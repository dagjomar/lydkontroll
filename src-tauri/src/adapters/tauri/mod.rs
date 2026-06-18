//! Thin Tauri command, path, persistence, and lifecycle adapter boundary.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use tauri::{AppHandle, Manager, Runtime, State};

use crate::{
    adapters::{
        audio::LocalAudioBackend,
        network::{ControlServerInfo, ControlServerRuntime, EmbeddedWebAsset, WebAssetProvider},
        persistence::{JsonLibraryRepository, PersistenceError},
    },
    application::{
        AppSnapshot, ApplicationService, ApplicationServiceError, CommandAck, CommandEnvelope,
        PreflightFacts, PreflightStatus,
    },
    domain::{CueLibrary, ManagedAudioFile},
    ports::{AudioBackend, AudioImporter, LibraryRepository},
};

pub type DesktopApplicationService = ApplicationService<LocalAudioBackend>;

pub struct TauriWebAssets<R: Runtime> {
    app: AppHandle<R>,
}

impl<R: Runtime> TauriWebAssets<R> {
    pub fn new(app: AppHandle<R>) -> Self {
        Self { app }
    }
}

impl<R: Runtime> WebAssetProvider for TauriWebAssets<R> {
    fn get(&self, path: &str) -> Option<EmbeddedWebAsset> {
        self.app
            .asset_resolver()
            .get(path.to_owned())
            .map(|asset| EmbeddedWebAsset {
                bytes: asset.bytes,
                mime_type: asset.mime_type,
                csp_header: asset.csp_header,
            })
    }
}

/// Serializes local library imports/saves around the shared authoritative core.
pub struct DesktopCoordinator {
    operations: Mutex<()>,
    repository: JsonLibraryRepository,
    service: Arc<DesktopApplicationService>,
}

impl DesktopCoordinator {
    pub fn new(
        repository: JsonLibraryRepository,
        library: CueLibrary,
        backend: LocalAudioBackend,
    ) -> Self {
        let audio_dir = repository.paths().audio_dir();
        Self {
            operations: Mutex::new(()),
            repository,
            service: Arc::new(ApplicationService::new(library, audio_dir, backend)),
        }
    }

    pub fn service(&self) -> &DesktopApplicationService {
        &self.service
    }

    pub fn shared_service(&self) -> Arc<DesktopApplicationService> {
        Arc::clone(&self.service)
    }

    pub fn snapshot(&self) -> Result<AppSnapshot, String> {
        self.service.poll().map_err(service_error)?;
        self.service.snapshot().map_err(service_error)
    }

    pub fn execute(&self, envelope: CommandEnvelope) -> Result<CommandAck, String> {
        self.service.execute(envelope).map_err(service_error)
    }

    pub fn save_library(&self, library: CueLibrary) -> Result<AppSnapshot, String> {
        let _operation = self
            .operations
            .lock()
            .map_err(|_| "desktop library operation is unavailable".to_owned())?;
        self.repository
            .save(&library)
            .map_err(|error| error.to_string())?;
        self.service.replace_library(library).map_err(service_error)
    }

    pub fn import_audio(&self, source: &Path) -> Result<ManagedAudioFile, String> {
        let _operation = self
            .operations
            .lock()
            .map_err(|_| "desktop library operation is unavailable".to_owned())?;
        let imported = self
            .repository
            .import(source)
            .map_err(|error| error.to_string())?;
        let mut library = self.service.library().map_err(service_error)?;
        library.audio_files.push(imported.clone());
        self.repository
            .save(&library)
            .map_err(|error| error.to_string())?;
        self.service
            .replace_library(library)
            .map_err(service_error)?;
        Ok(imported)
    }

    pub fn refresh_preflight(
        &self,
        control_server: PreflightStatus,
    ) -> Result<AppSnapshot, String> {
        let current = self.snapshot()?;
        let audio_files = managed_audio_preflight(&self.repository, &current_library(&current));
        self.service
            .set_preflight(PreflightFacts {
                control_server,
                audio_output: current.preflight.audio_output,
                audio_files,
            })
            .map_err(service_error)
    }
}

pub fn managed_audio_preflight(
    repository: &JsonLibraryRepository,
    library: &CueLibrary,
) -> PreflightStatus {
    let missing = library
        .audio_files
        .iter()
        .filter(|audio| {
            !repository
                .paths()
                .audio_dir()
                .join(&audio.file_name)
                .is_file()
        })
        .map(|audio| {
            let cue_names = library
                .scenes
                .iter()
                .flat_map(|scene| &scene.cues)
                .filter(|cue| cue.audio_file_id == audio.id)
                .map(|cue| cue.name.as_str())
                .collect::<Vec<_>>();
            if cue_names.is_empty() {
                audio.original_name.clone()
            } else {
                cue_names.join(", ")
            }
        })
        .collect::<Vec<_>>();

    if missing.is_empty() {
        PreflightStatus::Ready
    } else {
        PreflightStatus::Unavailable {
            message: format!("Mangler administrert lydfil for: {}.", missing.join("; ")),
        }
    }
}

/// Resolves Tauri's app-data directory at the outer boundary.
pub fn persistence_repository<R: Runtime>(
    app: &AppHandle<R>,
) -> Result<JsonLibraryRepository, PersistenceError> {
    let root = app
        .path()
        .app_data_dir()
        .map_err(|error| PersistenceError::Io {
            operation: "resolve app data directory",
            path: "<tauri-app-data>".into(),
            source: std::io::Error::other(error),
        })?;
    Ok(JsonLibraryRepository::new(root))
}

pub fn load_or_create_library(
    repository: &JsonLibraryRepository,
) -> Result<(CueLibrary, Option<String>), PersistenceError> {
    match repository.load() {
        Ok(library) => Ok((library, None)),
        Err(PersistenceError::LibraryMissing {
            backup_available: false,
        }) => {
            let library = CueLibrary::default();
            repository.save(&library)?;
            Ok((library, None))
        }
        Err(error @ PersistenceError::MissingManagedFile { .. }) => {
            let library = repository.load_metadata()?;
            Ok((library, Some(error.to_string())))
        }
        Err(error) => Err(error),
    }
}

#[tauri::command]
pub fn get_snapshot(state: State<'_, DesktopCoordinator>) -> Result<AppSnapshot, String> {
    state.snapshot()
}

#[tauri::command]
pub fn execute_desktop_command(
    state: State<'_, DesktopCoordinator>,
    envelope: CommandEnvelope,
) -> Result<CommandAck, String> {
    state.execute(envelope)
}

#[tauri::command]
pub fn save_library(
    state: State<'_, DesktopCoordinator>,
    library: CueLibrary,
) -> Result<AppSnapshot, String> {
    state.save_library(library)
}

#[tauri::command]
pub fn import_audio(
    state: State<'_, DesktopCoordinator>,
    source_path: PathBuf,
) -> Result<ManagedAudioFile, String> {
    state.import_audio(&source_path)
}

#[tauri::command]
pub fn get_control_server_info(
    state: State<'_, ControlServerRuntime>,
) -> Option<ControlServerInfo> {
    state.info()
}

#[tauri::command]
pub fn refresh_preflight(
    coordinator: State<'_, DesktopCoordinator>,
    control_server: State<'_, ControlServerRuntime>,
) -> Result<AppSnapshot, String> {
    let control_server_status = match control_server.info() {
        Some(_) => PreflightStatus::Ready,
        None => PreflightStatus::Unavailable {
            message: control_server
                .startup_error()
                .unwrap_or("Kontrollserveren kjører ikke.")
                .to_owned(),
        },
    };
    coordinator.refresh_preflight(control_server_status)
}

/// Generic helpers retained for transport-level tests.
pub fn execute_command<B>(
    service: &ApplicationService<B>,
    envelope: CommandEnvelope,
) -> Result<CommandAck, ApplicationServiceError>
where
    B: AudioBackend,
    B::Error: std::fmt::Display,
{
    service.execute(envelope)
}

pub fn snapshot<B>(service: &ApplicationService<B>) -> Result<AppSnapshot, ApplicationServiceError>
where
    B: AudioBackend,
    B::Error: std::fmt::Display,
{
    service.snapshot()
}

fn service_error(error: ApplicationServiceError) -> String {
    error.to_string()
}

fn current_library(snapshot: &AppSnapshot) -> CueLibrary {
    CueLibrary {
        schema_version: crate::domain::LIBRARY_SCHEMA_VERSION,
        scenes: snapshot.scenes.clone(),
        audio_files: snapshot.audio_files.clone(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::managed_audio_preflight;
    use crate::{
        adapters::persistence::JsonLibraryRepository,
        application::PreflightStatus,
        domain::{AudioFormat, Cue, CueLibrary, CueMode, ManagedAudioFile, Scene},
    };

    #[test]
    fn managed_audio_preflight_names_cues_with_missing_files() {
        let root = tempdir().expect("temp app data");
        let repository = JsonLibraryRepository::new(root.path());
        let audio = ManagedAudioFile {
            id: "audio-1".to_owned(),
            file_name: "audio-1.wav".to_owned(),
            original_name: "intro.wav".to_owned(),
            format: AudioFormat::Wav,
            byte_length: 44,
        };
        let library = CueLibrary {
            schema_version: 1,
            scenes: vec![Scene {
                id: "scene-1".to_owned(),
                name: "Middag".to_owned(),
                cues: vec![Cue {
                    id: "cue-1".to_owned(),
                    name: "Introduksjon".to_owned(),
                    color: "#ffffff".to_owned(),
                    audio_file_id: audio.id.clone(),
                    volume: 1.0,
                    mode: CueMode::Overlap,
                    fade_ms: 500,
                }],
            }],
            audio_files: vec![audio],
        };

        let status = managed_audio_preflight(&repository, &library);

        assert_eq!(
            status,
            PreflightStatus::Unavailable {
                message: "Mangler administrert lydfil for: Introduksjon.".to_owned()
            }
        );
    }

    #[test]
    fn managed_audio_preflight_is_ready_when_every_file_exists() {
        let root = tempdir().expect("temp app data");
        let repository = JsonLibraryRepository::new(root.path());
        fs::create_dir_all(repository.paths().audio_dir()).expect("audio directory");
        fs::write(repository.paths().audio_dir().join("audio-1.wav"), b"RIFF")
            .expect("managed file");
        let library = CueLibrary {
            schema_version: 1,
            scenes: Vec::new(),
            audio_files: vec![ManagedAudioFile {
                id: "audio-1".to_owned(),
                file_name: "audio-1.wav".to_owned(),
                original_name: "intro.wav".to_owned(),
                format: AudioFormat::Wav,
                byte_length: 4,
            }],
        };

        assert_eq!(
            managed_audio_preflight(&repository, &library),
            PreflightStatus::Ready
        );
    }
}

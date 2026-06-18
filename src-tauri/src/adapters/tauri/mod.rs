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

//! Versioned JSON persistence and managed-file import.

use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use symphonia::core::{
    codecs::{DecoderOptions, CODEC_TYPE_NULL},
    errors::Error as SymphoniaError,
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    domain::{AudioFormat, CueLibrary, ManagedAudioFile, LIBRARY_SCHEMA_VERSION},
    ports::{AudioImporter, LibraryRepository},
};

const LIBRARY_FILE: &str = "library.json";
const BACKUP_FILE: &str = "library.json.bak";
const TEMP_LIBRARY_FILE: &str = "library.json.tmp";

/// All paths owned by the persistence adapter.
#[derive(Debug, Clone)]
pub struct PersistencePaths {
    root: PathBuf,
}

impl PersistencePaths {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn library(&self) -> PathBuf {
        self.root.join(LIBRARY_FILE)
    }

    pub fn backup(&self) -> PathBuf {
        self.root.join(BACKUP_FILE)
    }

    pub fn audio_dir(&self) -> PathBuf {
        self.root.join("audio")
    }

    pub fn staging_dir(&self) -> PathBuf {
        self.root.join("staging")
    }
}

/// Recoverable storage/import failures shown to the operator.
#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("library does not exist")]
    LibraryMissing { backup_available: bool },
    #[error("library JSON is corrupt: {message}")]
    CorruptLibrary {
        message: String,
        backup_available: bool,
    },
    #[error("library schema version {found} is newer than supported version {supported}")]
    UnsupportedSchemaVersion {
        found: u32,
        supported: u32,
        backup_available: bool,
    },
    #[error("managed audio file is missing: {file_name}")]
    MissingManagedFile { file_name: String },
    #[error("managed audio filename is invalid: {file_name}")]
    InvalidManagedFileName { file_name: String },
    #[error("cue {cue_id} references unknown audio file {audio_file_id}")]
    MissingAudioReference {
        cue_id: String,
        audio_file_id: String,
    },
    #[error("source audio file is missing: {file_name}")]
    MissingSourceFile { file_name: String },
    #[error("unsupported audio extension: {extension}")]
    UnsupportedAudioFormat { extension: String },
    #[error("audio file could not be decoded: {file_name}: {message}")]
    InvalidAudio { file_name: String, message: String },
    #[error("filesystem operation {operation} failed for {path}: {source}")]
    Io {
        operation: &'static str,
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

impl PersistenceError {
    fn io(operation: &'static str, path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            operation,
            path: path.into(),
            source,
        }
    }
}

/// Filesystem-backed implementation used by the application service.
#[derive(Debug, Clone)]
pub struct JsonLibraryRepository {
    paths: PersistencePaths,
}

impl JsonLibraryRepository {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            paths: PersistencePaths::new(root),
        }
    }

    pub fn paths(&self) -> &PersistencePaths {
        &self.paths
    }

    fn ensure_directories(&self) -> Result<(), PersistenceError> {
        fs::create_dir_all(self.paths.root()).map_err(|error| {
            PersistenceError::io("create app data directory", self.paths.root(), error)
        })?;
        for path in [self.paths.audio_dir(), self.paths.staging_dir()] {
            fs::create_dir_all(&path)
                .map_err(|error| PersistenceError::io("create directory", path, error))?;
        }
        Ok(())
    }

    fn read_library(
        &self,
        path: &Path,
        backup_available: bool,
    ) -> Result<CueLibrary, PersistenceError> {
        let file = File::open(path).map_err(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                PersistenceError::LibraryMissing { backup_available }
            } else {
                PersistenceError::io("open library", path, error)
            }
        })?;
        let value: serde_json::Value =
            serde_json::from_reader(BufReader::new(file)).map_err(|error| {
                PersistenceError::CorruptLibrary {
                    message: error.to_string(),
                    backup_available,
                }
            })?;
        let found = value
            .get("schemaVersion")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| PersistenceError::CorruptLibrary {
                message: "schemaVersion is missing or invalid".to_owned(),
                backup_available,
            })?;
        let found = u32::try_from(found).map_err(|_| PersistenceError::CorruptLibrary {
            message: "schemaVersion is outside the supported integer range".to_owned(),
            backup_available,
        })?;
        if found != LIBRARY_SCHEMA_VERSION {
            return Err(PersistenceError::UnsupportedSchemaVersion {
                found,
                supported: LIBRARY_SCHEMA_VERSION,
                backup_available,
            });
        }
        let library: CueLibrary =
            serde_json::from_value(value).map_err(|error| PersistenceError::CorruptLibrary {
                message: error.to_string(),
                backup_available,
            })?;
        self.validate_managed_files(&library)?;
        Ok(library)
    }

    fn validate_managed_files(&self, library: &CueLibrary) -> Result<(), PersistenceError> {
        let mut audio_ids = HashSet::with_capacity(library.audio_files.len());
        for audio in &library.audio_files {
            let relative = Path::new(&audio.file_name);
            if relative.file_name().and_then(|value| value.to_str())
                != Some(audio.file_name.as_str())
            {
                return Err(PersistenceError::InvalidManagedFileName {
                    file_name: audio.file_name.clone(),
                });
            }
            audio_ids.insert(audio.id.as_str());
            if !self.paths.audio_dir().join(&audio.file_name).is_file() {
                return Err(PersistenceError::MissingManagedFile {
                    file_name: audio.file_name.clone(),
                });
            }
        }
        for cue in library.scenes.iter().flat_map(|scene| &scene.cues) {
            if !audio_ids.contains(cue.audio_file_id.as_str()) {
                return Err(PersistenceError::MissingAudioReference {
                    cue_id: cue.id.clone(),
                    audio_file_id: cue.audio_file_id.clone(),
                });
            }
        }
        Ok(())
    }

    fn write_primary_without_rotation(&self, library: &CueLibrary) -> Result<(), PersistenceError> {
        self.ensure_directories()?;
        let temporary = self.paths.root().join(TEMP_LIBRARY_FILE);
        write_json_file(&temporary, library)?;
        replace_file(&temporary, &self.paths.library())?;
        sync_directory(self.paths.root())
    }
}

impl LibraryRepository for JsonLibraryRepository {
    type Error = PersistenceError;

    fn load(&self) -> Result<CueLibrary, Self::Error> {
        self.read_library(&self.paths.library(), self.paths.backup().is_file())
    }

    fn load_backup(&self) -> Result<CueLibrary, Self::Error> {
        self.read_library(&self.paths.backup(), false)
    }

    fn recover_backup(&self) -> Result<CueLibrary, Self::Error> {
        let library = self.load_backup()?;
        self.write_primary_without_rotation(&library)?;
        Ok(library)
    }

    fn save(&self, library: &CueLibrary) -> Result<(), Self::Error> {
        if library.schema_version != LIBRARY_SCHEMA_VERSION {
            return Err(PersistenceError::UnsupportedSchemaVersion {
                found: library.schema_version,
                supported: LIBRARY_SCHEMA_VERSION,
                backup_available: self.paths.backup().is_file(),
            });
        }
        self.validate_managed_files(library)?;
        self.ensure_directories()?;

        let temporary = self.paths.root().join(TEMP_LIBRARY_FILE);
        write_json_file(&temporary, library)?;

        let primary = self.paths.library();
        if primary.is_file() {
            // Only a readable, supported primary is eligible to become backup.
            self.read_library(&primary, self.paths.backup().is_file())?;
            replace_file(&primary, &self.paths.backup())?;
        }
        replace_file(&temporary, &primary)?;
        sync_directory(self.paths.root())
    }
}

impl AudioImporter for JsonLibraryRepository {
    type Error = PersistenceError;

    fn import(&self, source: &Path) -> Result<ManagedAudioFile, Self::Error> {
        if !source.is_file() {
            return Err(PersistenceError::MissingSourceFile {
                file_name: display_name(source),
            });
        }
        let (format, extension) = audio_format(source)?;
        self.ensure_directories()?;

        let id = Uuid::new_v4().to_string();
        let file_name = format!("{id}.{extension}");
        let staged = self.paths.staging_dir().join(format!("{file_name}.tmp"));
        let managed = self.paths.audio_dir().join(&file_name);

        let import_result = (|| {
            copy_and_flush(source, &staged)?;
            validate_audio(&staged, extension)?;
            fs::rename(&staged, &managed)
                .map_err(|error| PersistenceError::io("move imported audio", &managed, error))?;
            sync_directory(&self.paths.audio_dir())?;
            let byte_length = fs::metadata(&managed)
                .map_err(|error| PersistenceError::io("read audio metadata", &managed, error))?
                .len();
            Ok(ManagedAudioFile {
                id,
                file_name,
                original_name: display_name(source),
                format,
                byte_length,
            })
        })();

        if import_result.is_err() {
            let _ = fs::remove_file(&staged);
        }
        import_result
    }
}

fn audio_format(source: &Path) -> Result<(AudioFormat, &'static str), PersistenceError> {
    let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match extension.as_str() {
        "mp3" => Ok((AudioFormat::Mp3, "mp3")),
        "wav" => Ok((AudioFormat::Wav, "wav")),
        _ => Err(PersistenceError::UnsupportedAudioFormat { extension }),
    }
}

fn copy_and_flush(source: &Path, destination: &Path) -> Result<(), PersistenceError> {
    let input = File::open(source)
        .map_err(|error| PersistenceError::io("open source audio", source, error))?;
    let output = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(destination)
        .map_err(|error| PersistenceError::io("create staged audio", destination, error))?;
    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);
    io::copy(&mut reader, &mut writer)
        .map_err(|error| PersistenceError::io("copy source audio", destination, error))?;
    writer
        .flush()
        .map_err(|error| PersistenceError::io("flush staged audio", destination, error))?;
    writer
        .get_ref()
        .sync_all()
        .map_err(|error| PersistenceError::io("sync staged audio", destination, error))
}

fn validate_audio(path: &Path, extension: &str) -> Result<(), PersistenceError> {
    let file =
        File::open(path).map_err(|error| PersistenceError::io("open staged audio", path, error))?;
    let stream = MediaSourceStream::new(Box::new(file), Default::default());
    let mut hint = Hint::new();
    hint.with_extension(extension);
    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            stream,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|error| invalid_audio(path, error))?;
    let mut format = probed.format;
    let track = format
        .default_track()
        .filter(|track| track.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| PersistenceError::InvalidAudio {
            file_name: display_name(path),
            message: "no decodable default track".to_owned(),
        })?;
    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|error| invalid_audio(path, error))?;

    loop {
        let packet = format
            .next_packet()
            .map_err(|error| invalid_audio(path, error))?;
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) if decoded.frames() > 0 => return Ok(()),
            Ok(_) => continue,
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(error) => return Err(invalid_audio(path, error)),
        }
    }
}

fn invalid_audio(path: &Path, error: SymphoniaError) -> PersistenceError {
    PersistenceError::InvalidAudio {
        file_name: display_name(path),
        message: error.to_string(),
    }
}

fn write_json_file(path: &Path, library: &CueLibrary) -> Result<(), PersistenceError> {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .map_err(|error| PersistenceError::io("create temporary library", path, error))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, library).map_err(|error| {
        PersistenceError::io(
            "serialize library",
            path,
            io::Error::new(io::ErrorKind::InvalidData, error),
        )
    })?;
    writer
        .write_all(b"\n")
        .map_err(|error| PersistenceError::io("write library", path, error))?;
    writer
        .flush()
        .map_err(|error| PersistenceError::io("flush library", path, error))?;
    writer
        .get_ref()
        .sync_all()
        .map_err(|error| PersistenceError::io("sync library", path, error))
}

fn replace_file(source: &Path, destination: &Path) -> Result<(), PersistenceError> {
    fs::rename(source, destination)
        .map_err(|error| PersistenceError::io("atomically replace file", destination, error))
}

fn sync_directory(path: &Path) -> Result<(), PersistenceError> {
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| PersistenceError::io("sync directory", path, error))
}

fn display_name(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("<unknown>")
        .to_owned()
}

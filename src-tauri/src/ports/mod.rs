//! Inward-facing interfaces implemented by infrastructure adapters.

use std::{path::Path, time::Duration};

use crate::domain::{CueLibrary, ManagedAudioFile};

/// Serialized storage for the versioned cue library.
pub trait LibraryRepository {
    type Error;

    fn load(&self) -> Result<CueLibrary, Self::Error>;
    fn load_backup(&self) -> Result<CueLibrary, Self::Error>;
    fn recover_backup(&self) -> Result<CueLibrary, Self::Error>;
    fn save(&self, library: &CueLibrary) -> Result<(), Self::Error>;
}

/// Copies and validates operator-selected media into managed storage.
pub trait AudioImporter {
    type Error;

    fn import(&self, source: &Path) -> Result<ManagedAudioFile, Self::Error>;
}

/// Opaque identifier assigned by an audio backend to one sound instance.
pub type BackendPlaybackId = String;

/// Terminal information reported while polling the audio backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioBackendEvent {
    Finished {
        backend_id: BackendPlaybackId,
    },
    Failed {
        backend_id: BackendPlaybackId,
        message: String,
    },
}

/// Narrow playback boundary implemented by Kira and deterministic test fakes.
pub trait AudioBackend {
    type Error;

    fn play(&mut self, managed_path: &Path, volume: f32) -> Result<BackendPlaybackId, Self::Error>;
    fn stop(&mut self, backend_id: &BackendPlaybackId, fade: Duration) -> Result<(), Self::Error>;
    fn set_master_volume(&mut self, volume: f32) -> Result<(), Self::Error>;
    fn poll_events(&mut self) -> Vec<AudioBackendEvent>;
}

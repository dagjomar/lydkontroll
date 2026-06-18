//! Inward-facing interfaces implemented by infrastructure adapters.

use std::path::Path;

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

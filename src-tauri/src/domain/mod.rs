//! Pure domain data and rules.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const LIBRARY_SCHEMA_VERSION: u32 = 1;
pub const DEFAULT_EVENT_TITLE: &str = "Mitt arrangement";

fn default_event_title() -> String {
    DEFAULT_EVENT_TITLE.to_owned()
}

/// Identifies which presentation surface consumes a shared snapshot.
///
/// This small contract establishes the deterministic Rust-to-TypeScript export
/// path. Product protocol types replace and extend it in later slices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum AppMode {
    Desktop,
    Mobile,
}

/// The complete persisted cue library.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CueLibrary {
    pub schema_version: u32,
    #[serde(default = "default_event_title")]
    pub event_title: String,
    pub scenes: Vec<Scene>,
    pub audio_files: Vec<ManagedAudioFile>,
}

impl Default for CueLibrary {
    fn default() -> Self {
        Self {
            schema_version: LIBRARY_SCHEMA_VERSION,
            event_title: default_event_title(),
            scenes: Vec::new(),
            audio_files: Vec::new(),
        }
    }
}

/// A named group of cues shown together to the operator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub cues: Vec<Cue>,
}

/// One operator-triggered sound.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Cue {
    pub id: String,
    pub name: String,
    pub color: String,
    pub audio_file_id: String,
    pub volume: f32,
    pub mode: CueMode,
    pub fade_ms: u32,
}

/// Whether a cue may overlap active audio or replaces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum CueMode {
    Overlap,
    Exclusive,
}

/// Metadata for a file copied into application-managed storage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ManagedAudioFile {
    pub id: String,
    pub file_name: String,
    pub original_name: String,
    pub format: AudioFormat,
    #[ts(type = "number")]
    pub byte_length: u64,
}

/// Audio containers accepted by the first release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum AudioFormat {
    Mp3,
    Wav,
}

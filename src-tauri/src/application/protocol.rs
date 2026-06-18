use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::domain::{ManagedAudioFile, Scene};

use super::PlaybackInstance;

pub const APPLICATION_PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommandEnvelope {
    pub protocol_version: u32,
    pub command_id: String,
    pub command: Command,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[ts(export)]
pub enum Command {
    TriggerCue { cue_id: String },
    StopPlayback { playback_id: String },
    FadePlayback { playback_id: String },
    StopAll,
    FadeAll { duration_ms: u32 },
    SetMasterVolume { volume: f32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CommandAck {
    pub protocol_version: u32,
    pub command_id: String,
    pub outcome: CommandOutcome,
}

impl CommandAck {
    pub fn revision(&self) -> u64 {
        match &self.outcome {
            CommandOutcome::Success { revision } | CommandOutcome::Failure { revision, .. } => {
                *revision
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(tag = "status", rename_all = "camelCase")]
#[ts(export)]
pub enum CommandOutcome {
    Success {
        #[ts(type = "number")]
        revision: u64,
    },
    Failure {
        #[ts(type = "number")]
        revision: u64,
        error: CommandError,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(
    tag = "code",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[ts(export)]
pub enum CommandError {
    UnsupportedProtocol {
        received: u32,
        supported: u32,
    },
    InvalidCommandId,
    UnknownCue {
        cue_id: String,
    },
    MissingAudioMetadata {
        cue_id: String,
        audio_file_id: String,
    },
    UnknownPlayback {
        playback_id: String,
    },
    AudioBackend {
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AppSnapshot {
    #[ts(type = "number")]
    pub revision: u64,
    pub scenes: Vec<Scene>,
    pub audio_files: Vec<ManagedAudioFile>,
    pub active_playback: Vec<PlaybackInstance>,
    pub pending_cue_id: Option<String>,
    pub master_volume: f32,
    pub preflight: PreflightFacts,
    pub errors: Vec<OperatorError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PreflightFacts {
    pub control_server: PreflightStatus,
    pub audio_output: PreflightStatus,
    pub audio_files: PreflightStatus,
}

impl Default for PreflightFacts {
    fn default() -> Self {
        Self {
            control_server: PreflightStatus::Unknown,
            audio_output: PreflightStatus::Unknown,
            audio_files: PreflightStatus::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "status", rename_all = "camelCase")]
#[ts(export)]
pub enum PreflightStatus {
    Unknown,
    Ready,
    Unavailable { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct OperatorError {
    pub id: String,
    pub kind: OperatorErrorKind,
    pub message: String,
    pub playback_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum OperatorErrorKind {
    AudioBackend,
    PlaybackFailed,
    Persistence,
}

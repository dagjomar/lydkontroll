//! Deterministic playback use cases independent of Tauri and audio hardware.

use std::{collections::BTreeMap, fmt, path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{
    domain::CueMode,
    ports::{AudioBackend, AudioBackendEvent, BackendPlaybackId},
};

mod protocol;
mod service;

pub use protocol::{
    AppSnapshot, Command, CommandAck, CommandEnvelope, CommandError, CommandOutcome, OperatorError,
    OperatorErrorKind, PreflightFacts, PreflightStatus, APPLICATION_PROTOCOL_VERSION,
};
pub use service::{ApplicationService, ApplicationServiceError};

/// Everything required to start one cue from managed storage.
#[derive(Debug, Clone, PartialEq)]
pub struct CuePlaybackRequest {
    pub cue_id: String,
    pub managed_path: PathBuf,
    pub volume: f32,
    pub mode: CueMode,
    pub fade_ms: u32,
}

/// Operator-visible lifecycle of one playback instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum PlaybackStatus {
    Playing,
    Fading,
}

/// Authoritative playback data owned by the engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PlaybackInstance {
    pub id: String,
    pub cue_id: String,
    pub volume: f32,
    pub fade_ms: u32,
    pub status: PlaybackStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackEvent {
    Finished {
        playback_id: String,
    },
    Failed {
        playback_id: String,
        message: String,
    },
    Started {
        playback_id: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum PlaybackError<E> {
    Backend(E),
    UnknownPlayback(String),
}

impl<E: fmt::Display> fmt::Display for PlaybackError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(error) => write!(formatter, "audio backend failed: {error}"),
            Self::UnknownPlayback(id) => write!(formatter, "unknown playback instance: {id}"),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> std::error::Error for PlaybackError<E> {}

#[derive(Debug)]
struct ActivePlayback {
    public: PlaybackInstance,
    backend_id: BackendPlaybackId,
}

/// Serialized state machine for overlap, exclusive, retrigger, stop, and fade.
#[derive(Debug)]
pub struct PlaybackEngine<B> {
    backend: B,
    active: BTreeMap<String, ActivePlayback>,
    pending_exclusive: Option<CuePlaybackRequest>,
    master_volume: f32,
}

impl<B: AudioBackend> PlaybackEngine<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            active: BTreeMap::new(),
            pending_exclusive: None,
            master_volume: 1.0,
        }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn active(&self) -> Vec<PlaybackInstance> {
        self.active
            .values()
            .map(|active| active.public.clone())
            .collect()
    }

    pub fn pending_exclusive(&self) -> Option<&CuePlaybackRequest> {
        self.pending_exclusive.as_ref()
    }

    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn trigger(
        &mut self,
        mut request: CuePlaybackRequest,
    ) -> Result<Option<String>, PlaybackError<B::Error>> {
        request.volume = clamp_volume(request.volume);
        self.stop_existing_cue(&request.cue_id)?;
        if self
            .pending_exclusive
            .as_ref()
            .is_some_and(|pending| pending.cue_id == request.cue_id)
        {
            self.pending_exclusive = None;
        }

        match request.mode {
            CueMode::Overlap => {
                let playback_id = self.start(request)?;
                if self.pending_exclusive.is_some() {
                    self.fade_one_for_exclusive(&playback_id)?;
                    self.start_pending_if_ready()?;
                }
                Ok(Some(playback_id))
            }
            CueMode::Exclusive => {
                self.pending_exclusive = Some(request);
                self.fade_all_for_exclusive()?;
                self.start_pending_if_ready()
            }
        }
    }

    pub fn stop(&mut self, playback_id: &str) -> Result<(), PlaybackError<B::Error>> {
        self.stop_active(playback_id)?;
        self.start_pending_if_ready()?;
        Ok(())
    }

    fn stop_active(&mut self, playback_id: &str) -> Result<(), PlaybackError<B::Error>> {
        let backend_id = self
            .active
            .get(playback_id)
            .ok_or_else(|| PlaybackError::UnknownPlayback(playback_id.to_owned()))?
            .backend_id
            .clone();
        self.backend
            .stop(&backend_id, Duration::ZERO)
            .map_err(PlaybackError::Backend)?;
        self.active.remove(playback_id);
        Ok(())
    }

    pub fn fade(&mut self, playback_id: &str) -> Result<(), PlaybackError<B::Error>> {
        let duration = self
            .active
            .get(playback_id)
            .ok_or_else(|| PlaybackError::UnknownPlayback(playback_id.to_owned()))?
            .public
            .fade_ms;
        self.fade_with_duration(playback_id, duration)
    }

    pub fn stop_all(&mut self) -> Result<(), PlaybackError<B::Error>> {
        self.pending_exclusive = None;
        let ids: Vec<_> = self.active.keys().cloned().collect();
        for id in ids {
            self.stop_active(&id)?;
        }
        Ok(())
    }

    pub fn fade_all(&mut self, duration_ms: u32) -> Result<(), PlaybackError<B::Error>> {
        self.pending_exclusive = None;
        let ids: Vec<_> = self.active.keys().cloned().collect();
        for id in ids {
            self.fade_with_duration(&id, duration_ms)?;
        }
        Ok(())
    }

    pub fn set_master_volume(&mut self, volume: f32) -> Result<(), PlaybackError<B::Error>> {
        let volume = clamp_volume(volume);
        self.backend
            .set_master_volume(volume)
            .map_err(PlaybackError::Backend)?;
        self.master_volume = volume;
        Ok(())
    }

    pub fn poll(&mut self) -> Result<Vec<PlaybackEvent>, PlaybackError<B::Error>> {
        let mut events = Vec::new();
        for event in self.backend.poll_events() {
            let (backend_id, failure) = match event {
                AudioBackendEvent::Finished { backend_id } => (backend_id, None),
                AudioBackendEvent::Failed {
                    backend_id,
                    message,
                } => (backend_id, Some(message)),
            };
            let playback_id = self
                .active
                .iter()
                .find_map(|(id, active)| (active.backend_id == backend_id).then(|| id.clone()));
            let Some(playback_id) = playback_id else {
                continue;
            };
            self.active.remove(&playback_id);
            events.push(match failure {
                Some(message) => PlaybackEvent::Failed {
                    playback_id,
                    message,
                },
                None => PlaybackEvent::Finished { playback_id },
            });
        }
        if let Some(playback_id) = self.start_pending_if_ready()? {
            events.push(PlaybackEvent::Started { playback_id });
        }
        Ok(events)
    }

    fn start(&mut self, request: CuePlaybackRequest) -> Result<String, PlaybackError<B::Error>> {
        let playback_id = Uuid::new_v4().to_string();
        let backend_id = self
            .backend
            .play(&request.managed_path, request.volume)
            .map_err(PlaybackError::Backend)?;
        self.active.insert(
            playback_id.clone(),
            ActivePlayback {
                backend_id,
                public: PlaybackInstance {
                    id: playback_id.clone(),
                    cue_id: request.cue_id,
                    volume: request.volume,
                    fade_ms: request.fade_ms,
                    status: PlaybackStatus::Playing,
                },
            },
        );
        Ok(playback_id)
    }

    fn stop_existing_cue(&mut self, cue_id: &str) -> Result<(), PlaybackError<B::Error>> {
        let ids: Vec<_> = self
            .active
            .iter()
            .filter(|(_, active)| active.public.cue_id == cue_id)
            .map(|(id, _)| id.clone())
            .collect();
        for id in ids {
            self.stop_active(&id)?;
        }
        Ok(())
    }

    fn fade_all_for_exclusive(&mut self) -> Result<(), PlaybackError<B::Error>> {
        let ids: Vec<_> = self.active.keys().cloned().collect();
        for id in ids {
            self.fade_one_for_exclusive(&id)?;
        }
        Ok(())
    }

    fn fade_one_for_exclusive(&mut self, playback_id: &str) -> Result<(), PlaybackError<B::Error>> {
        let duration = self.active[playback_id].public.fade_ms;
        self.fade_with_duration(playback_id, duration)
    }

    fn fade_with_duration(
        &mut self,
        playback_id: &str,
        duration_ms: u32,
    ) -> Result<(), PlaybackError<B::Error>> {
        if duration_ms == 0 {
            return self.stop_active(playback_id);
        }
        let active = self
            .active
            .get_mut(playback_id)
            .ok_or_else(|| PlaybackError::UnknownPlayback(playback_id.to_owned()))?;
        self.backend
            .stop(
                &active.backend_id,
                Duration::from_millis(u64::from(duration_ms)),
            )
            .map_err(PlaybackError::Backend)?;
        active.public.status = PlaybackStatus::Fading;
        Ok(())
    }

    fn start_pending_if_ready(&mut self) -> Result<Option<String>, PlaybackError<B::Error>> {
        if !self.active.is_empty() {
            return Ok(None);
        }
        let Some(request) = self.pending_exclusive.take() else {
            return Ok(None);
        };
        self.start(request).map(Some)
    }
}

fn clamp_volume(volume: f32) -> f32 {
    if volume.is_nan() {
        0.0
    } else {
        volume.clamp(0.0, 1.0)
    }
}

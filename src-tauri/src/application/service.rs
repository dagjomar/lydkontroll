use std::{
    collections::{HashMap, VecDeque},
    fmt,
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex, MutexGuard,
    },
};

use thiserror::Error;
use uuid::Uuid;

use crate::{
    domain::{Cue, CueLibrary},
    ports::AudioBackend,
};

use super::{
    AppSnapshot, Command, CommandAck, CommandEnvelope, CommandError, CommandOutcome,
    CuePlaybackRequest, OperatorError, OperatorErrorKind, PlaybackEngine, PlaybackError,
    PlaybackEvent, PlaybackInstance, PreflightFacts, APPLICATION_PROTOCOL_VERSION,
};

const DEFAULT_DEDUPLICATION_CAPACITY: usize = 256;
const DEFAULT_ERROR_CAPACITY: usize = 64;

#[derive(Debug, Error)]
pub enum ApplicationServiceError {
    #[error("application state is unavailable")]
    StateUnavailable,
}

#[derive(Debug)]
pub struct ApplicationService<B> {
    state: Mutex<ServiceState<B>>,
}

#[derive(Debug)]
struct ServiceState<B> {
    library: CueLibrary,
    audio_dir: PathBuf,
    playback: PlaybackEngine<B>,
    revision: u64,
    preflight: PreflightFacts,
    errors: VecDeque<OperatorError>,
    error_capacity: usize,
    acknowledgements: HashMap<String, CommandAck>,
    acknowledgement_order: VecDeque<String>,
    deduplication_capacity: usize,
    subscribers: Vec<Sender<AppSnapshot>>,
}

#[derive(Debug, PartialEq)]
struct PlaybackProjection {
    active: Vec<PlaybackInstance>,
    pending_cue_id: Option<String>,
    master_volume: f32,
}

impl<B> ApplicationService<B>
where
    B: AudioBackend,
    B::Error: fmt::Display,
{
    pub fn new(library: CueLibrary, audio_dir: impl Into<PathBuf>, backend: B) -> Self {
        Self::with_capacities(
            library,
            audio_dir,
            backend,
            DEFAULT_DEDUPLICATION_CAPACITY,
            DEFAULT_ERROR_CAPACITY,
        )
    }

    pub fn execute(
        &self,
        envelope: CommandEnvelope,
    ) -> Result<CommandAck, ApplicationServiceError> {
        let mut state = self.lock_state()?;
        if let Some(acknowledgement) = state.acknowledgements.get(&envelope.command_id) {
            return Ok(acknowledgement.clone());
        }

        let before = state.playback_projection();
        let result = state.validate_envelope(&envelope).and_then(|()| {
            let command = envelope.command.clone();
            state.apply(command)
        });
        let mut changed = before != state.playback_projection();
        if let Err(CommandError::AudioBackend { message }) = &result {
            state.push_error(OperatorError {
                id: Uuid::new_v4().to_string(),
                kind: OperatorErrorKind::AudioBackend,
                message: message.clone(),
                playback_id: None,
            });
            changed = true;
        }
        if changed {
            state.advance_revision();
        }

        let acknowledgement = CommandAck {
            protocol_version: APPLICATION_PROTOCOL_VERSION,
            command_id: envelope.command_id.clone(),
            outcome: match result {
                Ok(()) => CommandOutcome::Success {
                    revision: state.revision,
                },
                Err(error) => CommandOutcome::Failure {
                    revision: state.revision,
                    error,
                },
            },
        };
        state.remember_acknowledgement(acknowledgement.clone());
        Ok(acknowledgement)
    }

    pub fn poll(&self) -> Result<Option<AppSnapshot>, ApplicationServiceError> {
        let mut state = self.lock_state()?;
        let before = state.playback_projection();
        let result = state.playback.poll();
        let mut changed = before != state.playback_projection();
        match result {
            Ok(events) => {
                for event in events {
                    if let PlaybackEvent::Failed {
                        playback_id,
                        message,
                    } = event
                    {
                        state.push_error(OperatorError {
                            id: Uuid::new_v4().to_string(),
                            kind: OperatorErrorKind::PlaybackFailed,
                            message,
                            playback_id: Some(playback_id),
                        });
                        changed = true;
                    }
                }
            }
            Err(error) => {
                state.push_error(OperatorError {
                    id: Uuid::new_v4().to_string(),
                    kind: OperatorErrorKind::AudioBackend,
                    message: error.to_string(),
                    playback_id: None,
                });
                changed = true;
            }
        }
        if !changed {
            return Ok(None);
        }
        state.advance_revision();
        Ok(Some(state.snapshot()))
    }

    pub fn snapshot(&self) -> Result<AppSnapshot, ApplicationServiceError> {
        Ok(self.lock_state()?.snapshot())
    }

    pub fn library(&self) -> Result<CueLibrary, ApplicationServiceError> {
        Ok(self.lock_state()?.library.clone())
    }

    pub fn replace_library(
        &self,
        library: CueLibrary,
    ) -> Result<AppSnapshot, ApplicationServiceError> {
        let mut state = self.lock_state()?;
        if state.library != library {
            state.library = library;
            state.advance_revision();
        }
        Ok(state.snapshot())
    }

    pub fn subscribe(&self) -> Result<Receiver<AppSnapshot>, ApplicationServiceError> {
        let (sender, receiver) = mpsc::channel();
        let mut state = self.lock_state()?;
        sender
            .send(state.snapshot())
            .map_err(|_| ApplicationServiceError::StateUnavailable)?;
        state.subscribers.push(sender);
        Ok(receiver)
    }

    pub fn set_preflight(
        &self,
        preflight: PreflightFacts,
    ) -> Result<AppSnapshot, ApplicationServiceError> {
        let mut state = self.lock_state()?;
        if state.preflight != preflight {
            state.preflight = preflight;
            state.advance_revision();
        }
        Ok(state.snapshot())
    }

    pub fn report_error(
        &self,
        kind: OperatorErrorKind,
        message: impl Into<String>,
    ) -> Result<AppSnapshot, ApplicationServiceError> {
        let mut state = self.lock_state()?;
        state.push_error(OperatorError {
            id: Uuid::new_v4().to_string(),
            kind,
            message: message.into(),
            playback_id: None,
        });
        state.advance_revision();
        Ok(state.snapshot())
    }

    fn with_capacities(
        library: CueLibrary,
        audio_dir: impl Into<PathBuf>,
        backend: B,
        deduplication_capacity: usize,
        error_capacity: usize,
    ) -> Self {
        Self {
            state: Mutex::new(ServiceState {
                library,
                audio_dir: audio_dir.into(),
                playback: PlaybackEngine::new(backend),
                revision: 0,
                preflight: PreflightFacts::default(),
                errors: VecDeque::new(),
                error_capacity,
                acknowledgements: HashMap::new(),
                acknowledgement_order: VecDeque::new(),
                deduplication_capacity,
                subscribers: Vec::new(),
            }),
        }
    }

    fn lock_state(&self) -> Result<MutexGuard<'_, ServiceState<B>>, ApplicationServiceError> {
        self.state
            .lock()
            .map_err(|_| ApplicationServiceError::StateUnavailable)
    }
}

impl<B> ServiceState<B>
where
    B: AudioBackend,
    B::Error: fmt::Display,
{
    fn validate_envelope(&self, envelope: &CommandEnvelope) -> Result<(), CommandError> {
        if envelope.protocol_version != APPLICATION_PROTOCOL_VERSION {
            return Err(CommandError::UnsupportedProtocol {
                received: envelope.protocol_version,
                supported: APPLICATION_PROTOCOL_VERSION,
            });
        }
        Uuid::parse_str(&envelope.command_id)
            .map(|_| ())
            .map_err(|_| CommandError::InvalidCommandId)
    }

    fn apply(&mut self, command: Command) -> Result<(), CommandError> {
        match command {
            Command::TriggerCue { cue_id } => {
                let cue =
                    self.find_cue(&cue_id)
                        .cloned()
                        .ok_or_else(|| CommandError::UnknownCue {
                            cue_id: cue_id.clone(),
                        })?;
                let audio = self
                    .library
                    .audio_files
                    .iter()
                    .find(|audio| audio.id == cue.audio_file_id)
                    .ok_or_else(|| CommandError::MissingAudioMetadata {
                        cue_id: cue.id.clone(),
                        audio_file_id: cue.audio_file_id.clone(),
                    })?;
                self.playback
                    .trigger(CuePlaybackRequest {
                        cue_id: cue.id,
                        managed_path: self.audio_dir.join(&audio.file_name),
                        volume: cue.volume,
                        mode: cue.mode,
                        fade_ms: cue.fade_ms,
                    })
                    .map(|_| ())
                    .map_err(map_playback_error)
            }
            Command::StopPlayback { playback_id } => {
                self.playback.stop(&playback_id).map_err(map_playback_error)
            }
            Command::FadePlayback { playback_id } => {
                self.playback.fade(&playback_id).map_err(map_playback_error)
            }
            Command::StopAll => self.playback.stop_all().map_err(map_playback_error),
            Command::FadeAll { duration_ms } => self
                .playback
                .fade_all(duration_ms)
                .map_err(map_playback_error),
            Command::SetMasterVolume { volume } => {
                if self.playback.master_volume() == normalized_volume(volume) {
                    return Ok(());
                }
                self.playback
                    .set_master_volume(volume)
                    .map_err(map_playback_error)
            }
        }
    }

    fn find_cue(&self, cue_id: &str) -> Option<&Cue> {
        self.library
            .scenes
            .iter()
            .flat_map(|scene| &scene.cues)
            .find(|cue| cue.id == cue_id)
    }

    fn playback_projection(&self) -> PlaybackProjection {
        PlaybackProjection {
            active: self.playback.active(),
            pending_cue_id: self
                .playback
                .pending_exclusive()
                .map(|request| request.cue_id.clone()),
            master_volume: self.playback.master_volume(),
        }
    }

    fn snapshot(&self) -> AppSnapshot {
        AppSnapshot {
            revision: self.revision,
            event_title: self.library.event_title.clone(),
            scenes: self.library.scenes.clone(),
            audio_files: self.library.audio_files.clone(),
            active_playback: self.playback.active(),
            pending_cue_id: self
                .playback
                .pending_exclusive()
                .map(|request| request.cue_id.clone()),
            master_volume: self.playback.master_volume(),
            preflight: self.preflight.clone(),
            errors: self.errors.iter().cloned().collect(),
        }
    }

    fn advance_revision(&mut self) {
        self.revision = self.revision.saturating_add(1);
        let snapshot = self.snapshot();
        self.subscribers
            .retain(|subscriber| subscriber.send(snapshot.clone()).is_ok());
    }

    fn push_error(&mut self, error: OperatorError) {
        if self.error_capacity == 0 {
            return;
        }
        while self.errors.len() >= self.error_capacity {
            self.errors.pop_front();
        }
        self.errors.push_back(error);
    }

    fn remember_acknowledgement(&mut self, acknowledgement: CommandAck) {
        if self.deduplication_capacity == 0 {
            return;
        }
        while self.acknowledgement_order.len() >= self.deduplication_capacity {
            if let Some(command_id) = self.acknowledgement_order.pop_front() {
                self.acknowledgements.remove(&command_id);
            }
        }
        self.acknowledgement_order
            .push_back(acknowledgement.command_id.clone());
        self.acknowledgements
            .insert(acknowledgement.command_id.clone(), acknowledgement);
    }
}

fn map_playback_error<E: fmt::Display>(error: PlaybackError<E>) -> CommandError {
    match error {
        PlaybackError::Backend(error) => CommandError::AudioBackend {
            message: error.to_string(),
        },
        PlaybackError::UnknownPlayback(playback_id) => {
            CommandError::UnknownPlayback { playback_id }
        }
    }
}

fn normalized_volume(volume: f32) -> f32 {
    if volume.is_nan() {
        0.0
    } else {
        volume.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_capacity_error_queue_does_not_store_errors() {
        let mut queue = ServiceState {
            library: CueLibrary::default(),
            audio_dir: PathBuf::new(),
            playback: PlaybackEngine::new(NoopBackend),
            revision: 0,
            preflight: PreflightFacts::default(),
            errors: VecDeque::new(),
            error_capacity: 0,
            acknowledgements: HashMap::new(),
            acknowledgement_order: VecDeque::new(),
            deduplication_capacity: 0,
            subscribers: Vec::new(),
        };
        queue.push_error(OperatorError {
            id: "error".to_owned(),
            kind: OperatorErrorKind::AudioBackend,
            message: "failure".to_owned(),
            playback_id: None,
        });
        assert!(queue.errors.is_empty());
    }

    #[derive(Debug)]
    struct NoopBackend;

    impl AudioBackend for NoopBackend {
        type Error = std::convert::Infallible;

        fn play(
            &mut self,
            _managed_path: &std::path::Path,
            _volume: f32,
        ) -> Result<String, Self::Error> {
            Ok("backend".to_owned())
        }

        fn stop(
            &mut self,
            _backend_id: &String,
            _fade: std::time::Duration,
        ) -> Result<(), Self::Error> {
            Ok(())
        }

        fn set_master_volume(&mut self, _volume: f32) -> Result<(), Self::Error> {
            Ok(())
        }

        fn poll_events(&mut self) -> Vec<crate::ports::AudioBackendEvent> {
            Vec::new()
        }
    }
}

use std::{
    fmt,
    path::{Path, PathBuf},
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

use lydkontroll_lib::{
    adapters::tauri,
    application::{
        ApplicationService, Command, CommandEnvelope, CommandError, CommandOutcome,
        OperatorErrorKind, PreflightFacts, PreflightStatus, APPLICATION_PROTOCOL_VERSION,
    },
    domain::{
        AudioFormat, Cue, CueLibrary, CueMode, ManagedAudioFile, Scene, LIBRARY_SCHEMA_VERSION,
    },
    ports::{AudioBackend, AudioBackendEvent, BackendPlaybackId},
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
struct FakeError(&'static str);

impl fmt::Display for FakeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[derive(Debug, Default)]
struct Trace {
    plays: Vec<(BackendPlaybackId, PathBuf, f32)>,
    stops: Vec<(BackendPlaybackId, Duration)>,
    master_volumes: Vec<f32>,
    events: Vec<AudioBackendEvent>,
    fail_next_play: bool,
}

#[derive(Debug, Clone)]
struct FakeBackend {
    trace: Arc<Mutex<Trace>>,
}

impl FakeBackend {
    fn new(trace: Arc<Mutex<Trace>>) -> Self {
        Self { trace }
    }
}

impl AudioBackend for FakeBackend {
    type Error = FakeError;

    fn play(&mut self, managed_path: &Path, volume: f32) -> Result<BackendPlaybackId, Self::Error> {
        let mut trace = self.trace.lock().expect("trace lock");
        if trace.fail_next_play {
            trace.fail_next_play = false;
            return Err(FakeError("play failed"));
        }
        let id = format!("backend-{}", trace.plays.len() + 1);
        trace
            .plays
            .push((id.clone(), managed_path.to_path_buf(), volume));
        Ok(id)
    }

    fn stop(&mut self, backend_id: &BackendPlaybackId, fade: Duration) -> Result<(), Self::Error> {
        self.trace
            .lock()
            .expect("trace lock")
            .stops
            .push((backend_id.clone(), fade));
        Ok(())
    }

    fn set_master_volume(&mut self, volume: f32) -> Result<(), Self::Error> {
        self.trace
            .lock()
            .expect("trace lock")
            .master_volumes
            .push(volume);
        Ok(())
    }

    fn poll_events(&mut self) -> Vec<AudioBackendEvent> {
        std::mem::take(&mut self.trace.lock().expect("trace lock").events)
    }
}

#[test]
fn commands_publish_authoritative_snapshots_and_duplicate_ids_are_idempotent() {
    let trace = Arc::new(Mutex::new(Trace::default()));
    let service = ApplicationService::new(
        library_with_cues(&[("cue-a", CueMode::Overlap)]),
        "/managed/audio",
        FakeBackend::new(trace.clone()),
    );
    let updates = service.subscribe().expect("subscribe");
    assert_eq!(
        updates.recv().expect("initial snapshot").revision,
        0,
        "subscribers bootstrap from current authoritative state"
    );

    let command_id = Uuid::new_v4().to_string();
    let first = tauri::execute_command(
        &service,
        envelope_with_id(
            &command_id,
            Command::TriggerCue {
                cue_id: "cue-a".to_owned(),
            },
        ),
    )
    .expect("trigger command");
    assert_eq!(first.revision(), 1);
    let update = updates.recv().expect("trigger update");
    assert_eq!(update.revision, 1);
    assert_eq!(update.active_playback.len(), 1);
    assert_eq!(update.active_playback[0].cue_id, "cue-a");

    let duplicate = service
        .execute(envelope_with_id(&command_id, Command::StopAll))
        .expect("duplicate acknowledgement");
    assert_eq!(duplicate, first);
    assert_eq!(trace.lock().expect("trace lock").plays.len(), 1);
    assert_eq!(
        tauri::snapshot(&service)
            .expect("authoritative snapshot")
            .active_playback
            .len(),
        1,
        "a duplicate ID must not replay a different side effect"
    );
    assert!(updates.try_recv().is_err());
}

#[test]
fn validation_failures_are_typed_and_do_not_advance_revision() {
    let service = service();

    let unsupported = service
        .execute(CommandEnvelope {
            protocol_version: 99,
            command_id: Uuid::new_v4().to_string(),
            command: Command::StopAll,
        })
        .expect("typed acknowledgement");
    assert!(matches!(
        unsupported.outcome,
        CommandOutcome::Failure {
            revision: 0,
            error: CommandError::UnsupportedProtocol {
                received: 99,
                supported: APPLICATION_PROTOCOL_VERSION
            }
        }
    ));

    let invalid_id = service
        .execute(CommandEnvelope {
            protocol_version: APPLICATION_PROTOCOL_VERSION,
            command_id: "not-a-uuid".to_owned(),
            command: Command::StopAll,
        })
        .expect("typed acknowledgement");
    assert!(matches!(
        invalid_id.outcome,
        CommandOutcome::Failure {
            revision: 0,
            error: CommandError::InvalidCommandId
        }
    ));

    let unknown_cue = execute(
        &service,
        Command::TriggerCue {
            cue_id: "missing".to_owned(),
        },
    );
    assert!(matches!(
        unknown_cue.outcome,
        CommandOutcome::Failure {
            revision: 0,
            error: CommandError::UnknownCue { .. }
        }
    ));
    assert_eq!(service.snapshot().expect("snapshot").revision, 0);
}

#[test]
fn playback_commands_and_polling_advance_one_revision_per_transition() {
    let trace = Arc::new(Mutex::new(Trace::default()));
    let service = ApplicationService::new(
        library_with_cues(&[
            ("cue-a", CueMode::Overlap),
            ("cue-exclusive", CueMode::Exclusive),
        ]),
        "/managed/audio",
        FakeBackend::new(trace.clone()),
    );

    assert_eq!(
        execute(
            &service,
            Command::TriggerCue {
                cue_id: "cue-a".to_owned()
            }
        )
        .revision(),
        1
    );
    assert_eq!(
        execute(
            &service,
            Command::TriggerCue {
                cue_id: "cue-exclusive".to_owned()
            }
        )
        .revision(),
        2
    );
    let fading = service.snapshot().expect("fading snapshot");
    assert_eq!(fading.pending_cue_id.as_deref(), Some("cue-exclusive"));
    assert_eq!(fading.active_playback.len(), 1);

    trace
        .lock()
        .expect("trace lock")
        .events
        .push(AudioBackendEvent::Finished {
            backend_id: "backend-1".to_owned(),
        });
    let polled = service.poll().expect("poll").expect("changed snapshot");
    assert_eq!(polled.revision, 3);
    assert_eq!(polled.pending_cue_id, None);
    assert_eq!(polled.active_playback.len(), 1);
    assert_eq!(polled.active_playback[0].cue_id, "cue-exclusive");

    let playback_id = polled.active_playback[0].id.clone();
    assert_eq!(
        execute(
            &service,
            Command::FadePlayback {
                playback_id: playback_id.clone()
            }
        )
        .revision(),
        4
    );
    assert_eq!(
        execute(&service, Command::StopPlayback { playback_id }).revision(),
        5
    );
    assert_eq!(
        execute(&service, Command::SetMasterVolume { volume: 0.4 }).revision(),
        6
    );
    assert_eq!(
        execute(&service, Command::SetMasterVolume { volume: 0.4 }).revision(),
        6,
        "an accepted no-op retains the current revision"
    );
    assert_eq!(
        execute(&service, Command::FadeAll { duration_ms: 2000 }).revision(),
        6
    );
    assert_eq!(execute(&service, Command::StopAll).revision(), 6);
    assert!(service.poll().expect("empty poll").is_none());
}

#[test]
fn backend_failures_are_recoverable_snapshot_errors() {
    let trace = Arc::new(Mutex::new(Trace {
        fail_next_play: true,
        ..Trace::default()
    }));
    let service = ApplicationService::new(
        library_with_cues(&[("cue-a", CueMode::Overlap)]),
        "/managed/audio",
        FakeBackend::new(trace.clone()),
    );

    let acknowledgement = execute(
        &service,
        Command::TriggerCue {
            cue_id: "cue-a".to_owned(),
        },
    );
    assert!(matches!(
        acknowledgement.outcome,
        CommandOutcome::Failure {
            revision: 1,
            error: CommandError::AudioBackend { .. }
        }
    ));
    let failed_command = service.snapshot().expect("failure snapshot");
    assert_eq!(failed_command.errors.len(), 1);
    assert_eq!(
        failed_command.errors[0].kind,
        OperatorErrorKind::AudioBackend
    );

    execute(
        &service,
        Command::TriggerCue {
            cue_id: "cue-a".to_owned(),
        },
    );
    let playback_id = service
        .snapshot()
        .expect("playing snapshot")
        .active_playback[0]
        .id
        .clone();
    trace
        .lock()
        .expect("trace lock")
        .events
        .push(AudioBackendEvent::Failed {
            backend_id: "backend-1".to_owned(),
            message: "output disappeared".to_owned(),
        });
    let polled = service.poll().expect("poll").expect("changed snapshot");
    assert!(polled.active_playback.is_empty());
    assert_eq!(polled.errors.len(), 2);
    assert_eq!(polled.errors[1].kind, OperatorErrorKind::PlaybackFailed);
    assert_eq!(
        polled.errors[1].playback_id.as_deref(),
        Some(playback_id.as_str())
    );
}

#[test]
fn deduplication_cache_is_bounded_and_evicts_the_oldest_command() {
    let service = service();
    let oldest_id = Uuid::new_v4().to_string();
    let first = service
        .execute(envelope_with_id(
            &oldest_id,
            Command::SetMasterVolume { volume: 0.25 },
        ))
        .expect("first command");
    assert_eq!(first.revision(), 1);

    for index in 0..256 {
        let volume = if index % 2 == 0 { 0.5 } else { 0.75 };
        execute(&service, Command::SetMasterVolume { volume });
    }
    let revision_before_reuse = service.snapshot().expect("snapshot").revision;
    let reused = service
        .execute(envelope_with_id(
            &oldest_id,
            Command::SetMasterVolume { volume: 0.1 },
        ))
        .expect("evicted command ID can be accepted again");
    assert_eq!(reused.revision(), revision_before_reuse + 1);
    assert_ne!(reused, first);
}

#[test]
fn preflight_updates_are_revisioned_and_published() {
    let service = service();
    let updates = service.subscribe().expect("subscribe");
    updates.recv().expect("initial");
    let facts = PreflightFacts {
        control_server: PreflightStatus::Unavailable {
            message: "Tailscale unavailable".to_owned(),
        },
        audio_output: PreflightStatus::Ready,
        audio_files: PreflightStatus::Ready,
    };

    let snapshot = service
        .set_preflight(facts.clone())
        .expect("preflight update");
    assert_eq!(snapshot.revision, 1);
    assert_eq!(snapshot.preflight, facts);
    assert_eq!(updates.recv().expect("published").revision, 1);
    assert_eq!(
        service
            .set_preflight(facts)
            .expect("unchanged preflight")
            .revision,
        1
    );
    assert!(updates.try_recv().is_err());
}

#[test]
fn concurrent_callers_are_serialized_behind_the_in_flight_command() {
    let gate = Arc::new((Mutex::new(GateState::default()), Condvar::new()));
    let service = Arc::new(ApplicationService::new(
        library_with_cues(&[("cue-a", CueMode::Overlap)]),
        "/managed/audio",
        GateBackend { gate: gate.clone() },
    ));

    let first_service = service.clone();
    let first = thread::spawn(move || {
        execute(
            &first_service,
            Command::TriggerCue {
                cue_id: "cue-a".to_owned(),
            },
        )
    });
    let (lock, condition) = &*gate;
    let mut state = lock.lock().expect("gate lock");
    while !state.entered {
        state = condition.wait(state).expect("gate wait");
    }

    let second_service = service.clone();
    let second =
        thread::spawn(move || execute(&second_service, Command::SetMasterVolume { volume: 0.5 }));
    thread::sleep(Duration::from_millis(20));
    assert!(
        !second.is_finished(),
        "second caller must wait for the service"
    );
    state.released = true;
    condition.notify_all();
    drop(state);

    assert_eq!(first.join().expect("first thread").revision(), 1);
    assert_eq!(second.join().expect("second thread").revision(), 2);
}

#[derive(Debug, Default)]
struct GateState {
    entered: bool,
    released: bool,
}

#[derive(Debug)]
struct GateBackend {
    gate: Arc<(Mutex<GateState>, Condvar)>,
}

impl AudioBackend for GateBackend {
    type Error = FakeError;

    fn play(
        &mut self,
        _managed_path: &Path,
        _volume: f32,
    ) -> Result<BackendPlaybackId, Self::Error> {
        let (lock, condition) = &*self.gate;
        let mut state = lock.lock().expect("gate lock");
        state.entered = true;
        condition.notify_all();
        while !state.released {
            state = condition.wait(state).expect("gate wait");
        }
        Ok("backend-1".to_owned())
    }

    fn stop(
        &mut self,
        _backend_id: &BackendPlaybackId,
        _fade: Duration,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_master_volume(&mut self, _volume: f32) -> Result<(), Self::Error> {
        Ok(())
    }

    fn poll_events(&mut self) -> Vec<AudioBackendEvent> {
        Vec::new()
    }
}

fn service() -> ApplicationService<FakeBackend> {
    ApplicationService::new(
        library_with_cues(&[("cue-a", CueMode::Overlap)]),
        "/managed/audio",
        FakeBackend::new(Arc::new(Mutex::new(Trace::default()))),
    )
}

fn execute<B>(
    service: &ApplicationService<B>,
    command: Command,
) -> lydkontroll_lib::application::CommandAck
where
    B: AudioBackend,
    B::Error: fmt::Display,
{
    service
        .execute(CommandEnvelope {
            protocol_version: APPLICATION_PROTOCOL_VERSION,
            command_id: Uuid::new_v4().to_string(),
            command,
        })
        .expect("application command")
}

fn envelope_with_id(command_id: &str, command: Command) -> CommandEnvelope {
    CommandEnvelope {
        protocol_version: APPLICATION_PROTOCOL_VERSION,
        command_id: command_id.to_owned(),
        command,
    }
}

fn library_with_cues(cues: &[(&str, CueMode)]) -> CueLibrary {
    CueLibrary {
        schema_version: LIBRARY_SCHEMA_VERSION,
        scenes: vec![Scene {
            id: "scene-1".to_owned(),
            name: "Wedding".to_owned(),
            cues: cues
                .iter()
                .map(|(id, mode)| Cue {
                    id: (*id).to_owned(),
                    name: (*id).to_owned(),
                    color: "#663399".to_owned(),
                    audio_file_id: format!("audio-{id}"),
                    volume: 0.8,
                    mode: *mode,
                    fade_ms: 250,
                })
                .collect(),
        }],
        audio_files: cues
            .iter()
            .map(|(id, _)| ManagedAudioFile {
                id: format!("audio-{id}"),
                file_name: format!("{id}.wav"),
                original_name: format!("{id}.wav"),
                format: AudioFormat::Wav,
                byte_length: 44,
            })
            .collect(),
    }
}

use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    time::Duration,
};

use lydkontroll_lib::{
    application::{
        CuePlaybackRequest, PlaybackEngine, PlaybackError, PlaybackEvent, PlaybackStatus,
    },
    domain::CueMode,
    ports::{AudioBackend, AudioBackendEvent, BackendPlaybackId},
};

#[derive(Debug, Default)]
struct FakeBackend {
    next_id: usize,
    plays: Vec<(PathBuf, f32, String)>,
    stops: Vec<(String, Duration)>,
    master_volumes: Vec<f32>,
    events: VecDeque<AudioBackendEvent>,
    fail_next: Option<&'static str>,
}

impl AudioBackend for FakeBackend {
    type Error = &'static str;

    fn play(&mut self, path: &Path, volume: f32) -> Result<BackendPlaybackId, Self::Error> {
        if let Some(error) = self.fail_next.take() {
            return Err(error);
        }
        self.next_id += 1;
        let id = format!("backend-{}", self.next_id);
        self.plays.push((path.to_path_buf(), volume, id.clone()));
        Ok(id)
    }

    fn stop(&mut self, backend_id: &BackendPlaybackId, fade: Duration) -> Result<(), Self::Error> {
        if let Some(error) = self.fail_next.take() {
            return Err(error);
        }
        self.stops.push((backend_id.clone(), fade));
        Ok(())
    }

    fn set_master_volume(&mut self, volume: f32) -> Result<(), Self::Error> {
        if let Some(error) = self.fail_next.take() {
            return Err(error);
        }
        self.master_volumes.push(volume);
        Ok(())
    }

    fn poll_events(&mut self) -> Vec<AudioBackendEvent> {
        self.events.drain(..).collect()
    }
}

fn cue(id: &str, mode: CueMode, fade_ms: u32) -> CuePlaybackRequest {
    CuePlaybackRequest {
        cue_id: id.to_owned(),
        managed_path: PathBuf::from(format!("/managed/{id}.wav")),
        volume: 0.75,
        mode,
        fade_ms,
    }
}

#[test]
fn overlap_cues_play_together() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    engine.trigger(cue("one", CueMode::Overlap, 500)).unwrap();
    engine.trigger(cue("two", CueMode::Overlap, 500)).unwrap();

    assert_eq!(engine.active().len(), 2);
    assert_eq!(engine.backend().plays.len(), 2);
    assert!(engine.backend().stops.is_empty());
}

#[test]
fn retrigger_stops_old_instance_before_starting_replacement() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    let first = engine
        .trigger(cue("one", CueMode::Overlap, 500))
        .unwrap()
        .unwrap();
    let second = engine
        .trigger(cue("one", CueMode::Overlap, 500))
        .unwrap()
        .unwrap();

    assert_ne!(first, second);
    assert_eq!(engine.active().len(), 1);
    assert_eq!(
        engine.backend().stops,
        vec![("backend-1".to_owned(), Duration::ZERO)]
    );
}

#[test]
fn exclusive_waits_for_active_fades_then_starts() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    engine.trigger(cue("one", CueMode::Overlap, 500)).unwrap();
    engine.trigger(cue("two", CueMode::Overlap, 750)).unwrap();

    assert_eq!(
        engine
            .trigger(cue("exclusive", CueMode::Exclusive, 1000))
            .unwrap(),
        None
    );
    assert_eq!(
        engine
            .active()
            .iter()
            .map(|item| item.status)
            .collect::<Vec<_>>(),
        vec![PlaybackStatus::Fading, PlaybackStatus::Fading]
    );
    assert_eq!(engine.backend().plays.len(), 2);

    engine
        .backend_mut()
        .events
        .push_back(AudioBackendEvent::Finished {
            backend_id: "backend-1".to_owned(),
        });
    assert_eq!(engine.poll().unwrap().len(), 1);
    assert_eq!(engine.backend().plays.len(), 2);

    engine
        .backend_mut()
        .events
        .push_back(AudioBackendEvent::Finished {
            backend_id: "backend-2".to_owned(),
        });
    let events = engine.poll().unwrap();
    assert!(matches!(events.last(), Some(PlaybackEvent::Started { .. })));
    assert_eq!(engine.backend().plays.len(), 3);
    assert_eq!(engine.active()[0].cue_id, "exclusive");
}

#[test]
fn newer_pending_exclusive_replaces_older_one() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    engine.trigger(cue("one", CueMode::Overlap, 500)).unwrap();
    engine.trigger(cue("old", CueMode::Exclusive, 500)).unwrap();
    engine.trigger(cue("new", CueMode::Exclusive, 500)).unwrap();

    assert_eq!(engine.pending_exclusive().unwrap().cue_id, "new");
    engine
        .backend_mut()
        .events
        .push_back(AudioBackendEvent::Finished {
            backend_id: "backend-1".to_owned(),
        });
    engine.poll().unwrap();
    assert_eq!(engine.active()[0].cue_id, "new");
}

#[test]
fn overlap_during_exclusive_barrier_is_faded_before_exclusive_starts() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    engine.trigger(cue("one", CueMode::Overlap, 500)).unwrap();
    engine
        .trigger(cue("exclusive", CueMode::Exclusive, 500))
        .unwrap();
    engine.trigger(cue("late", CueMode::Overlap, 250)).unwrap();

    assert_eq!(engine.active().len(), 2);
    assert!(engine
        .active()
        .iter()
        .all(|item| item.status == PlaybackStatus::Fading));
    assert_eq!(
        engine.backend().stops.last(),
        Some(&("backend-2".to_owned(), Duration::from_millis(250)))
    );
}

#[test]
fn zero_fade_exclusive_starts_after_all_active_instances_are_stopped() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    engine.trigger(cue("one", CueMode::Overlap, 0)).unwrap();
    engine.trigger(cue("two", CueMode::Overlap, 0)).unwrap();

    let started = engine
        .trigger(cue("exclusive", CueMode::Exclusive, 500))
        .unwrap();

    assert!(started.is_some());
    assert_eq!(engine.active().len(), 1);
    assert_eq!(engine.active()[0].cue_id, "exclusive");
    assert_eq!(engine.backend().stops.len(), 2);
}

#[test]
fn retrigger_during_exclusive_barrier_does_not_start_pending_mid_command() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    engine.trigger(cue("one", CueMode::Overlap, 500)).unwrap();
    engine
        .trigger(cue("exclusive", CueMode::Exclusive, 500))
        .unwrap();

    engine.trigger(cue("one", CueMode::Overlap, 250)).unwrap();

    assert_eq!(engine.backend().plays.len(), 2);
    assert_eq!(engine.active().len(), 1);
    assert_eq!(engine.active()[0].cue_id, "one");
    assert_eq!(engine.active()[0].status, PlaybackStatus::Fading);
    assert_eq!(
        engine
            .pending_exclusive()
            .map(|pending| pending.cue_id.as_str()),
        Some("exclusive")
    );
}

#[test]
fn stop_fade_and_fade_all_update_state_deterministically() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    let first = engine
        .trigger(cue("one", CueMode::Overlap, 400))
        .unwrap()
        .unwrap();
    let second = engine
        .trigger(cue("two", CueMode::Overlap, 600))
        .unwrap()
        .unwrap();

    engine.fade(&first).unwrap();
    assert_eq!(
        engine
            .active()
            .into_iter()
            .find(|item| item.id == first)
            .unwrap()
            .status,
        PlaybackStatus::Fading
    );
    engine.stop(&second).unwrap();
    assert_eq!(engine.active().len(), 1);
    engine.fade_all(2000).unwrap();
    assert_eq!(
        engine.backend().stops.last(),
        Some(&("backend-1".to_owned(), Duration::from_millis(2000)))
    );
}

#[test]
fn natural_completion_and_failure_remove_authoritative_state() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    engine.trigger(cue("one", CueMode::Overlap, 500)).unwrap();
    let failed = engine
        .trigger(cue("two", CueMode::Overlap, 500))
        .unwrap()
        .unwrap();
    engine.backend_mut().events.extend([
        AudioBackendEvent::Finished {
            backend_id: "backend-1".to_owned(),
        },
        AudioBackendEvent::Failed {
            backend_id: "backend-2".to_owned(),
            message: "decoder stopped".to_owned(),
        },
    ]);

    let events = engine.poll().unwrap();
    assert_eq!(engine.active().len(), 0);
    assert!(events.contains(&PlaybackEvent::Failed {
        playback_id: failed,
        message: "decoder stopped".to_owned(),
    }));
}

#[test]
fn volume_is_clamped_and_backend_failure_does_not_publish_false_state() {
    let backend = FakeBackend {
        fail_next: Some("no output"),
        ..FakeBackend::default()
    };
    let mut engine = PlaybackEngine::new(backend);
    let mut request = cue("one", CueMode::Overlap, 500);
    request.volume = 4.0;

    assert_eq!(
        engine.trigger(request),
        Err(PlaybackError::Backend("no output"))
    );
    assert!(engine.active().is_empty());

    engine.set_master_volume(-1.0).unwrap();
    assert_eq!(engine.master_volume(), 0.0);
    assert_eq!(engine.backend().master_volumes, vec![0.0]);
}

#[test]
fn late_backend_events_for_retriggered_instances_are_ignored() {
    let mut engine = PlaybackEngine::new(FakeBackend::default());
    engine.trigger(cue("one", CueMode::Overlap, 500)).unwrap();
    engine.trigger(cue("one", CueMode::Overlap, 500)).unwrap();
    engine
        .backend_mut()
        .events
        .push_back(AudioBackendEvent::Finished {
            backend_id: "backend-1".to_owned(),
        });

    assert!(engine.poll().unwrap().is_empty());
    assert_eq!(engine.active().len(), 1);
}

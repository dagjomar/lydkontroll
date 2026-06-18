//! Kira/CPAL playback through the selected macOS system output.

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    time::Duration,
};

use kira::{
    sound::{
        streaming::{StreamingSoundData, StreamingSoundHandle},
        FromFileError, PlaybackState,
    },
    AudioManager, AudioManagerSettings, Decibels, DefaultBackend, Easing, Tween,
};
use thiserror::Error;
use uuid::Uuid;

use crate::ports::{AudioBackend, AudioBackendEvent, BackendPlaybackId};

#[derive(Debug, Error)]
pub enum KiraAudioError {
    #[error("could not initialize the system audio output: {0}")]
    Initialize(String),
    #[error("managed audio path is outside the audio directory: {0}")]
    InvalidManagedPath(PathBuf),
    #[error("managed audio file is missing: {0}")]
    MissingManagedFile(PathBuf),
    #[error("could not open or decode managed audio {path}: {message}")]
    InvalidAudio { path: PathBuf, message: String },
    #[error("audio playback resources are exhausted: {0}")]
    ResourceLimit(String),
    #[error("unknown backend playback instance: {0}")]
    UnknownPlayback(String),
}

pub struct KiraAudioBackend {
    audio_dir: PathBuf,
    manager: AudioManager<DefaultBackend>,
    handles: BTreeMap<BackendPlaybackId, StreamingSoundHandle<FromFileError>>,
}

pub enum LocalAudioBackend {
    Ready(KiraAudioBackend),
    Unavailable(String),
}

impl LocalAudioBackend {
    pub fn new(audio_dir: impl Into<PathBuf>) -> Self {
        match KiraAudioBackend::new(audio_dir) {
            Ok(backend) => Self::Ready(backend),
            Err(error) => Self::Unavailable(error.to_string()),
        }
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }

    pub fn unavailable_message(&self) -> Option<&str> {
        match self {
            Self::Ready(_) => None,
            Self::Unavailable(message) => Some(message),
        }
    }
}

impl KiraAudioBackend {
    pub fn new(audio_dir: impl Into<PathBuf>) -> Result<Self, KiraAudioError> {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .map_err(|error| KiraAudioError::Initialize(error.to_string()))?;
        Ok(Self {
            audio_dir: audio_dir.into(),
            manager,
            handles: BTreeMap::new(),
        })
    }

    fn validate_path(&self, path: &Path) -> Result<(), KiraAudioError> {
        let file_name = path
            .file_name()
            .ok_or_else(|| KiraAudioError::InvalidManagedPath(path.to_path_buf()))?;
        if path.parent() != Some(self.audio_dir.as_path()) || self.audio_dir.join(file_name) != path
        {
            return Err(KiraAudioError::InvalidManagedPath(path.to_path_buf()));
        }
        if !path.is_file() {
            return Err(KiraAudioError::MissingManagedFile(path.to_path_buf()));
        }
        Ok(())
    }
}

impl AudioBackend for KiraAudioBackend {
    type Error = KiraAudioError;

    fn play(&mut self, managed_path: &Path, volume: f32) -> Result<BackendPlaybackId, Self::Error> {
        self.validate_path(managed_path)?;
        let sound = StreamingSoundData::from_file(managed_path)
            .map_err(|error| KiraAudioError::InvalidAudio {
                path: managed_path.to_path_buf(),
                message: error.to_string(),
            })?
            .volume(linear_to_decibels(volume));
        let handle = self
            .manager
            .play(sound)
            .map_err(|error| KiraAudioError::ResourceLimit(error.to_string()))?;
        let id = Uuid::new_v4().to_string();
        self.handles.insert(id.clone(), handle);
        Ok(id)
    }

    fn stop(&mut self, backend_id: &BackendPlaybackId, fade: Duration) -> Result<(), Self::Error> {
        let handle = self
            .handles
            .get_mut(backend_id)
            .ok_or_else(|| KiraAudioError::UnknownPlayback(backend_id.clone()))?;
        handle.stop(Tween {
            duration: fade,
            easing: Easing::Linear,
            ..Tween::default()
        });
        Ok(())
    }

    fn set_master_volume(&mut self, volume: f32) -> Result<(), Self::Error> {
        self.manager
            .main_track()
            .set_volume(linear_to_decibels(volume), Tween::default());
        Ok(())
    }

    fn poll_events(&mut self) -> Vec<AudioBackendEvent> {
        let mut events = Vec::new();
        for (id, handle) in &mut self.handles {
            if let Some(error) = handle.pop_error() {
                events.push(AudioBackendEvent::Failed {
                    backend_id: id.clone(),
                    message: error.to_string(),
                });
            } else if handle.state() == PlaybackState::Stopped {
                events.push(AudioBackendEvent::Finished {
                    backend_id: id.clone(),
                });
            }
        }
        for event in &events {
            let id = match event {
                AudioBackendEvent::Finished { backend_id }
                | AudioBackendEvent::Failed { backend_id, .. } => backend_id,
            };
            self.handles.remove(id);
        }
        events
    }
}

impl AudioBackend for LocalAudioBackend {
    type Error = KiraAudioError;

    fn play(&mut self, managed_path: &Path, volume: f32) -> Result<BackendPlaybackId, Self::Error> {
        match self {
            Self::Ready(backend) => backend.play(managed_path, volume),
            Self::Unavailable(message) => Err(KiraAudioError::Initialize(message.clone())),
        }
    }

    fn stop(&mut self, backend_id: &BackendPlaybackId, fade: Duration) -> Result<(), Self::Error> {
        match self {
            Self::Ready(backend) => backend.stop(backend_id, fade),
            Self::Unavailable(message) => Err(KiraAudioError::Initialize(message.clone())),
        }
    }

    fn set_master_volume(&mut self, volume: f32) -> Result<(), Self::Error> {
        match self {
            Self::Ready(backend) => backend.set_master_volume(volume),
            Self::Unavailable(message) => Err(KiraAudioError::Initialize(message.clone())),
        }
    }

    fn poll_events(&mut self) -> Vec<AudioBackendEvent> {
        match self {
            Self::Ready(backend) => backend.poll_events(),
            Self::Unavailable(_) => Vec::new(),
        }
    }
}

fn linear_to_decibels(volume: f32) -> Decibels {
    if volume <= 0.0 || volume.is_nan() {
        Decibels::SILENCE
    } else {
        Decibels(20.0 * volume.clamp(0.0, 1.0).log10())
    }
}

#[cfg(test)]
mod tests {
    use super::linear_to_decibels;
    use kira::Decibels;

    #[test]
    fn converts_linear_volume_to_kira_decibels() {
        assert_eq!(linear_to_decibels(0.0), Decibels::SILENCE);
        assert_eq!(linear_to_decibels(1.0), Decibels::IDENTITY);
        assert!((linear_to_decibels(0.5).0 - -6.020_600_3).abs() < 0.000_1);
    }
}

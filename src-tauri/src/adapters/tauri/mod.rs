//! Thin Tauri command, path, and lifecycle adapter boundary.

use tauri::{AppHandle, Manager, Runtime};

use crate::{
    adapters::persistence::{JsonLibraryRepository, PersistenceError},
    application::{
        AppSnapshot, ApplicationService, ApplicationServiceError, CommandAck, CommandEnvelope,
    },
    ports::AudioBackend,
};

/// Resolves Tauri's app-data directory at the outer boundary.
pub fn persistence_repository<R: Runtime>(
    app: &AppHandle<R>,
) -> Result<JsonLibraryRepository, PersistenceError> {
    let root = app
        .path()
        .app_data_dir()
        .map_err(|error| PersistenceError::Io {
            operation: "resolve app data directory",
            path: "<tauri-app-data>".into(),
            source: std::io::Error::other(error),
        })?;
    Ok(JsonLibraryRepository::new(root))
}

/// Thin desktop adapter: all command behavior remains in `ApplicationService`.
pub fn execute_command<B>(
    service: &ApplicationService<B>,
    envelope: CommandEnvelope,
) -> Result<CommandAck, ApplicationServiceError>
where
    B: AudioBackend,
    B::Error: std::fmt::Display,
{
    service.execute(envelope)
}

/// Thin desktop adapter for authoritative state reads.
pub fn snapshot<B>(service: &ApplicationService<B>) -> Result<AppSnapshot, ApplicationServiceError>
where
    B: AudioBackend,
    B::Error: std::fmt::Display,
{
    service.snapshot()
}

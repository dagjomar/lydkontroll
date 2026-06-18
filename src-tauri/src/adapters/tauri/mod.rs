//! Thin Tauri command, path, and lifecycle adapter boundary.

use tauri::{AppHandle, Manager, Runtime};

use crate::adapters::persistence::{JsonLibraryRepository, PersistenceError};

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

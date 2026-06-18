//! Tauri composition root.
//!
//! Business rules live in inward-facing modules. Tauri setup stays here so the
//! same builder can be exercised with the mock runtime in integration tests.

pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

use tauri::{Builder, Runtime};

/// Applies all application wiring to a supplied Tauri builder.
///
/// Later slices add adapters and managed state here without coupling domain or
/// application modules to Tauri.
pub fn configure<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder
}

/// Runs the native desktop application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    configure(tauri::Builder::default())
        .run(tauri::generate_context!())
        .expect("failed to run Lydkontroll");
}

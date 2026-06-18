//! Tauri composition root.
//!
//! Business rules live in inward-facing modules. Tauri setup stays here so the
//! same builder can be exercised with the mock runtime in integration tests.

pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

use adapters::{
    audio::LocalAudioBackend,
    tauri::{load_or_create_library, persistence_repository, DesktopCoordinator},
};
use application::{OperatorErrorKind, PreflightFacts, PreflightStatus};
use tauri::{Builder, Manager, Runtime};

/// Applies all application wiring to a supplied Tauri builder.
///
/// Later slices add adapters and managed state here without coupling domain or
/// application modules to Tauri.
pub fn configure<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let repository = persistence_repository(app.handle())?;
            let (library, startup_error) = load_or_create_library(&repository)?;
            let backend = LocalAudioBackend::new(repository.paths().audio_dir());
            let audio_output = match backend.unavailable_message() {
                Some(message) => PreflightStatus::Unavailable {
                    message: message.to_owned(),
                },
                None => PreflightStatus::Ready,
            };
            let coordinator = DesktopCoordinator::new(repository, library, backend);
            coordinator.service().set_preflight(PreflightFacts {
                control_server: PreflightStatus::Unknown,
                audio_output,
                audio_files: PreflightStatus::Ready,
            })?;
            if let Some(message) = startup_error {
                coordinator
                    .service()
                    .report_error(OperatorErrorKind::Persistence, message)?;
            }
            app.manage(coordinator);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            adapters::tauri::get_snapshot,
            adapters::tauri::execute_desktop_command,
            adapters::tauri::save_library,
            adapters::tauri::import_audio,
        ])
}

/// Runs the native desktop application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    configure(tauri::Builder::default())
        .run(tauri::generate_context!())
        .expect("failed to run Lydkontroll");
}

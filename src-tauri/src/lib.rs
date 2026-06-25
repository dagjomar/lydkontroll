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
    network::{
        discover_tailscale_ipv4, local_ipv4_addresses, start_control_server, ControlServerRuntime,
        NetworkApplication, SystemProcessRunner, DEFAULT_TAILSCALE_TIMEOUT,
    },
    tauri::{
        load_or_create_library, managed_audio_preflight, persistence_repository,
        DesktopCoordinator, TauriWebAssets,
    },
};
use application::{OperatorErrorKind, PreflightFacts, PreflightStatus};
use std::sync::Arc;
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
                None => PreflightStatus::Warning {
                    message: "Kontroller valgt lydutgang i Systeminnstillinger på Mac-en."
                        .to_owned(),
                },
            };
            let audio_files = managed_audio_preflight(&repository, &library);
            let coordinator = DesktopCoordinator::new(repository, library, backend);
            let shared_service = coordinator.shared_service();
            let control_result = local_ipv4_addresses()
                .and_then(|local_addresses| {
                    discover_tailscale_ipv4(
                        &SystemProcessRunner,
                        None,
                        &local_addresses,
                        DEFAULT_TAILSCALE_TIMEOUT,
                    )
                })
                .map_err(|error| error.to_string())
                .and_then(|address| {
                    let application: Arc<dyn NetworkApplication> = shared_service.clone();
                    let assets = Arc::new(TauriWebAssets::new(app.handle().clone()));
                    tauri::async_runtime::block_on(start_control_server(
                        address,
                        application,
                        assets,
                    ))
                    .map_err(|error| error.to_string())
                });
            let (control_server, control_server_preflight) = match control_result {
                Ok(server) => (
                    ControlServerRuntime::running(server),
                    PreflightStatus::Ready,
                ),
                Err(error) => (
                    ControlServerRuntime::unavailable(error.clone()),
                    PreflightStatus::Unavailable { message: error },
                ),
            };
            coordinator.service().set_preflight(PreflightFacts {
                control_server: control_server_preflight,
                audio_output,
                audio_files,
            })?;
            if let Some(message) = startup_error {
                coordinator
                    .service()
                    .report_error(OperatorErrorKind::Persistence, message)?;
            }
            app.manage(coordinator);
            app.manage(control_server);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            adapters::tauri::get_snapshot,
            adapters::tauri::execute_desktop_command,
            adapters::tauri::save_library,
            adapters::tauri::import_audio,
            adapters::tauri::delete_managed_audio,
            adapters::tauri::get_control_server_info,
            adapters::tauri::refresh_preflight,
        ])
}

/// Runs the native desktop application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    configure(tauri::Builder::default())
        .run(tauri::generate_context!())
        .expect("failed to run Lydkontroll");
}

use lydkontroll_lib::configure;
use tauri::test::{mock_builder, mock_context, noop_assets};

#[test]
fn composition_root_builds_with_tauri_mock_runtime() {
    let app = configure(mock_builder())
        .build(mock_context(noop_assets()))
        .expect("mock Tauri app should build");

    assert!(!app.package_info().name.is_empty());
}

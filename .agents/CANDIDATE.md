# Replacement Candidate Record

Date/time: 2026-06-19 08:15:31 CEST
Application source commit: 20871d5d2b8928af77d3ae3246eee4f4ab4be23c
App version: 0.1.0
Artifact path: src-tauri/target/release/bundle/macos/Lydkontroll.app
Executable SHA-256: 4b8656e4a9b0ab279ae2dec4d5c51b2de58edaf8865969929dbbba85dfc7ea5b
Architecture: Mach-O 64-bit executable arm64
Bundle size: 5.2 MB
Signature: ad-hoc

Automated release build: PASS
Frontend regression tests: PASS (14 tests)
Rust regression tests: PASS (63 tests)
Collapsed ready and blocked states: PASS
Expanded diagnostics and QR: PASS
Manual refresh action: PASS in frontend regression coverage
Three-second trigger and fade sequence: PASS in frontend regression coverage
iPhone slider rapid-input/coalescing regression: PASS
390x844 responsive mobile render: PASS
Packaged native launch, audible test play, and physical iPhone drag: NOT YET CHECKED

## Release Status

This candidate contains the compact desktop status control from `TASK-014` and
the smooth iPhone master-volume behavior from `TASK-017`. Keep the previously
rehearsed candidate from `.agents/REHEARSAL.md` until the native desktop,
audible-output, and physical iPhone Safari checks in `TASK-016` pass.

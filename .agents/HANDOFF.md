# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-007` is complete. The Mac app now supports native MP3/WAV import,
persisted scene/cue editing, local playback controls through the shared
`ApplicationService`, active-state polling, and visible recovery errors.

## Exact Next Action

Claim `TASK-006` for planning. Replace its placeholder plan with concrete
Tailscale probe, local-address validation, Axum lifecycle, embedded-asset,
WebSocket snapshot/acknowledgement, reconnect, and QR URL slices. Move it to
`ready`, then implement the fail-closed transport.

```text
python3 scripts/ralph.py claim TASK-006 --owner "<agent/session>"
```

## Important Context

- `DesktopCoordinator` in `src-tauri/src/adapters/tauri/mod.rs` owns local
  persistence/import sequencing around the shared service.
- `ApplicationService<LocalAudioBackend>` remains the only playback, command,
  polling, retry, revision, operator-error, and snapshot owner.
- `get_snapshot` calls serialized backend polling before returning current
  state; WebSocket transport should call the same service rather than duplicate
  this behavior.
- `AppSnapshot` now includes `audioFiles`; generated TypeScript contracts must
  not be edited manually.
- Library candidates are saved before `replace_library` publishes them. Remote
  library editing is still deferred; TASK-006 only needs control/state
  transport.
- `LocalAudioBackend` keeps the editor available if CPAL initialization fails,
  and startup preserves valid library metadata when a managed file is missing.
- `npm run dev -- --` is not needed for visual QA; append `?preview=1` to the
  Vite URL for the in-memory standalone operator preview.
- Real analog playback and output-loss recovery still require target-Mac
  rehearsal even though fake-backend and adapter tests pass.

## Validation to Run

```text
cargo test --manifest-path src-tauri/Cargo.toml
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```

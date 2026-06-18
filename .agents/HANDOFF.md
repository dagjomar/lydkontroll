# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-005` was completed, then delivery order was changed to make local Mac
testing the next milestone. `TASK-007` is now P0 and `needs-planning`;
`TASK-006` remote transport is P1 and follows it.

## Exact Next Action

Claim `TASK-007` for planning. Define the local vertical slice from managed
audio import and library mutation through Tauri commands to desktop cue
triggering, active playback, stop/fade, and visible errors. Create a linked
plan, move the task to `ready`, then implement it.

```text
python3 scripts/ralph.py claim TASK-007 --owner "<agent/session>"
```

## Important Context

- `ApplicationService<B>` in `src-tauri/src/application/service.rs` is the only
  command, polling, retry, revision, error, and snapshot owner.
- `CommandEnvelope`, `CommandAck`, and `AppSnapshot` are Rust-owned contracts
  under `src-tauri/src/application/protocol.rs`; generated files must not be
  edited manually.
- Duplicate command IDs return the original acknowledgement from a 256-entry
  FIFO cache without replaying side effects.
- `subscribe()` immediately sends the current complete snapshot, then ordered
  updates after snapshot-visible transitions.
- Backend polling must remain serialized through `ApplicationService::poll()`;
  WebSocket handlers must never call `PlaybackEngine` or Kira directly.
- `set_preflight()` is the service seam for publishing Tailscale/server facts.
- `TASK-007` must extend the shared service rather than introducing
  React-owned library or playback state.
- Persistence already exposes `LibraryRepository` and `AudioImporter`, but
  library mutation/import commands are not yet part of `ApplicationService`.
- The first useful manual checkpoint is: import one WAV/MP3, create a cue,
  trigger it locally, then stop/fade it on the Mac.
- `TASK-006` remains independent at the code level but is deliberately queued
  after the desktop workflow.
- Real analog playback and output-loss recovery remain documented target-Mac
  rehearsal gates rather than automated proof.

## Validation to Run

```text
cargo test --manifest-path src-tauri/Cargo.toml
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```

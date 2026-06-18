# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-005` was completed. Versioned commands, acknowledgements, generated
TypeScript contracts, revisioned snapshots, bounded retries/errors, polling,
and playback mutation now live behind one serialized `ApplicationService`.

## Exact Next Action

Claim `TASK-006` for refinement. Replace its placeholder plan with the exact
Tailscale probe/refresh, local-address validation, bind refusal, server
lifecycle, embedded-assets, WebSocket bootstrap, and reconnect semantics. Move
it to `ready`, then start implementation.

```text
python3 scripts/ralph.py claim TASK-006 --owner "<agent/session>"
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
- The existing `TASK-006` plan is still a placeholder and must be completed
  before implementation.
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

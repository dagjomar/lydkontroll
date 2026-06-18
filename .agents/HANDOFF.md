# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-006` is complete. The app now discovers and validates one local Tailscale
IPv4 address, fails closed, serves embedded production assets through Axum, and
provides authoritative WebSocket snapshots and idempotent command
acknowledgements without coupling transport health to local playback.

## Exact Next Action

Refine `TASK-008` into a plan for the reconnecting iPhone projection, move it to
`ready`, then implement the mobile control interface against `/ws`.

```text
python3 scripts/ralph.py claim TASK-008 --owner "<agent/session>"
```

## Important Context

- `adapters/network/discovery.rs` is the strict CLI/local-address boundary.
  Production tries `tailscale` from `PATH`, the macOS app-bundle executable,
  and `/usr/local/bin/tailscale`; packaged-path behavior still needs target-Mac
  verification.
- `adapters/network/server.rs` sends `{type:"snapshot"}` on connect, accepts
  Rust-owned `CommandEnvelope` JSON, sends `{type:"acknowledgement"}`, then
  publishes the current snapshot. Protocol errors are tagged and recoverable.
- `src/services/desktopApi.ts` selects Tauri only when
  `window.__TAURI_INTERNALS__` exists; ordinary Safari uses the WebSocket
  adapter with command timeouts and reconnect backoff.
- Frontend identifiers must use `createUuid` from `src/services/uuid.ts`; it
  supports Safari versions without `crypto.randomUUID`.
- Reconnect intentionally starts from a fresh full snapshot. There is no delta
  replay; duplicate command IDs are handled by `ApplicationService`.
- HTTP uses `TauriWebAssets`, backed by Tauri's embedded production
  `frontendDist`. `TASK-008` should add a mobile-responsive projection to the
  existing frontend build rather than introduce runtime asset files.
- `get_control_server_info` returns the generated `ControlServerInfo` URL for
  later QR rendering. `TASK-009` owns the full desktop preflight/QR view and
  periodic Tailscale re-probing.
- Network adapter tests bind temporary loopback ports and may require elevated
  sandbox permission. They never bind a non-loopback address in tests.

## Validation to Run

```text
cargo test --manifest-path src-tauri/Cargo.toml
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```

---
id: TASK-006
title: Serve authenticated-by-Tailscale mobile control transport
status: done
priority: P1
type: feature
owner: unassigned
depends_on: [TASK-005]
plan: .agents/plans/TASK-006-control-server.md
updated: 2026-06-18
---

# TASK-006: Serve authenticated-by-Tailscale mobile control transport

## Context

The iPhone needs direct control without relying on Bluetooth, venue LAN access,
or an external cloud service.

## Outcome

Rust serves the mobile app and WebSocket protocol only on the active Tailscale
address at port 17321, with acknowledgements, snapshots, and reconnect support.

## Acceptance Criteria

- [x] An injectable, timeout-bounded `tailscale ip -4` probe returns exactly one
      locally assigned address in `100.64.0.0/10` or a typed diagnostic.
- [x] The server refuses to bind when no valid Tailscale address is available.
- [x] It never binds wildcard, loopback-only, or ordinary LAN interfaces as a
      fallback.
- [x] Axum runs as a gracefully stoppable/rebindable task on Tauri's async
      runtime and a server failure does not stop local playback.
- [x] HTTP serves mobile control assets embedded from the production frontend
      build.
- [x] WebSocket commands use the TASK-005 command/state core.
- [x] Connect/reconnect obtains an authoritative current-state snapshot and
      duplicate command IDs do not replay side effects.
- [x] QR-code URL data is exposed to the desktop app.

## Validation

```text
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Create the linked threat/failure-mode plan before marking this task ready.
Follow ADR-004 and ADR-005; packaged CLI path discovery must be verified on the
target Mac and must fail closed.

- 2026-06-18: Reprioritized after `TASK-007` so local Mac editing and playback
  can be tested before remote transport is introduced. Its technical dependency
  remains `TASK-005`; this is delivery ordering rather than a new coupling.
- 2026-06-18: Planning resolved CLI candidates and timeout behavior, strict
  local CGNAT validation, graceful Axum lifecycle, embedded asset boundaries,
  fresh-snapshot reconnects, idempotent command acknowledgements, and QR URL
  data. Automatic periodic re-probing is deferred to `TASK-009`.
- 2026-06-18: Task claimed.
- 2026-06-18: Added strict Tailscale CLI discovery with explicit executable
  candidates, three-second timeout, exactly-one-address parsing, CGNAT and
  local-interface validation, and typed diagnostics.
- 2026-06-18: Added Axum HTTP/WebSocket transport over the shared
  `ApplicationService`, Tauri-embedded production asset serving, initial and
  revisioned snapshots, acknowledgements, idempotent retries, malformed-frame
  recovery, graceful shutdown, and desktop URL/QR payload access.
- 2026-06-18: Rust tests, Vitest, production build, generated bindings,
  formatting, ESLint, Clippy, Ralph validation, and diff hygiene pass.
  Loopback transport tests require permission to bind temporary local ports.
- 2026-06-18: Implemented and validated fail-closed Tailscale discovery,
  embedded Axum HTTP/WebSocket transport, reconnect snapshots, idempotent
  acknowledgements, graceful lifecycle, and desktop URL data.
- 2026-06-18: Fixed the embedded Safari runtime to select WebSocket commands
  instead of Tauri `invoke`; remote snapshots, acknowledgements, command
  timeouts, and reconnect backoff now work in an ordinary browser.
- 2026-06-18: Added a frontend UUID-v4 compatibility helper for Safari versions
  without `crypto.randomUUID`, using `crypto.getRandomValues` when available
  and preserving valid command IDs for Rust protocol validation.

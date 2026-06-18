---
id: TASK-006
title: Serve authenticated-by-Tailscale mobile control transport
status: idea
priority: P0
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

- [ ] An injectable, timeout-bounded `tailscale ip -4` probe returns exactly one
      locally assigned address in `100.64.0.0/10` or a typed diagnostic.
- [ ] The server refuses to bind when no valid Tailscale address is available.
- [ ] It never binds wildcard, loopback-only, or ordinary LAN interfaces as a
      fallback.
- [ ] Axum runs as a gracefully stoppable/rebindable task on Tauri's async
      runtime and a server failure does not stop local playback.
- [ ] HTTP serves mobile control assets embedded from the production frontend
      build.
- [ ] WebSocket commands use the TASK-005 command/state core.
- [ ] Connect/reconnect obtains an authoritative current-state snapshot and
      duplicate command IDs do not replay side effects.
- [ ] QR-code URL data is exposed to the desktop app.

## Validation

```text
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Create the linked threat/failure-mode plan before marking this task ready.
Follow ADR-004 and ADR-005; packaged CLI path discovery must be verified on the
target Mac and must fail closed.

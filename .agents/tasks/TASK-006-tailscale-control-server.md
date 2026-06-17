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

- [ ] The server refuses to bind when no valid Tailscale address is available.
- [ ] It never binds wildcard, loopback-only, or ordinary LAN interfaces as a
      fallback.
- [ ] HTTP serves the mobile control assets.
- [ ] WebSocket commands use the TASK-005 command/state core.
- [ ] Reconnect obtains an authoritative current-state snapshot.
- [ ] QR-code URL data is exposed to the desktop app.

## Validation

```text
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Create the linked threat/failure-mode plan before marking this task ready.

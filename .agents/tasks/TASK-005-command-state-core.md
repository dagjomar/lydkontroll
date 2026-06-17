---
id: TASK-005
title: Create one authoritative command and playback state core
status: idea
priority: P0
type: feature
owner: unassigned
depends_on: [TASK-003, TASK-004]
plan: none
updated: 2026-06-18
---

# TASK-005: Create one authoritative command and playback state core

## Context

Desktop and mobile controls must invoke the same behavior and observe the same
state without duplicating playback rules.

## Outcome

Rust exposes a single command path for cue and master actions and publishes an
authoritative state model usable by both Tauri commands and WebSocket clients.

## Acceptance Criteria

- [ ] Commands have unique IDs and typed success/failure acknowledgements.
- [ ] State includes active sounds, scenes, master volume, and relevant errors.
- [ ] Desktop and future network adapters call the same application service.
- [ ] Shared TypeScript types are generated or otherwise derived from the
      recorded source of truth.
- [ ] Tests cover duplicate/retried command handling and state transitions.

## Validation

```text
npm test
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

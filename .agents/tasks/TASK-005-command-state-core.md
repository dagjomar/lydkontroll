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

- [ ] Versioned command envelopes have UUIDs and typed success/failure
      acknowledgements that echo the ID and resulting state revision.
- [ ] State has a monotonically increasing revision and includes active
      playback instances, scenes, master volume, preflight facts, and relevant
      recoverable errors.
- [ ] Desktop and future network adapters call the same application service.
- [ ] Shared TypeScript types are generated from Rust Serde types with ts-rs and
      checked for drift.
- [ ] Tests cover serialized ordering, bounded duplicate/retried command
      handling, state revisions, and transitions.

## Validation

```text
npm test
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Follow ADR-003. React, Tauri commands, and WebSocket handlers must not bypass
the `ApplicationService`.

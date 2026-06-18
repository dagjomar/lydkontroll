---
id: TASK-005
title: Create one authoritative command and playback state core
status: done
priority: P0
type: feature
owner: unassigned
depends_on: [TASK-003, TASK-004]
plan: .agents/plans/TASK-005-command-state-core.md
updated: 2026-06-18
---

# TASK-005: Create one authoritative command and playback state core

## Context

Desktop and mobile controls must invoke the same behavior and observe the same
state without duplicating playback rules.

## Outcome

Rust exposes a single command path for cue and master actions and publishes an
authoritative state model usable by both Tauri commands and WebSocket clients.

## Scope

- Versioned cue/playback command protocol and typed acknowledgements.
- Serialized library, playback, polling, revision, retry, error, and snapshot
  ownership in one application service.
- Thin adapter helpers shared by desktop and future network transports.

## Non-goals

- Tailscale discovery, HTTP/WebSocket serving, or reconnect behavior.
- Desktop cue editing UI and persistence mutation commands.
- Hardware rehearsal or output-device recovery.

## Acceptance Criteria

- [x] Versioned command envelopes have UUIDs and typed success/failure
      acknowledgements that echo the ID and resulting state revision.
- [x] State has a monotonically increasing revision and includes active
      playback instances, scenes, master volume, preflight facts, and relevant
      recoverable errors.
- [x] Desktop and future network adapters call the same application service.
- [x] Shared TypeScript types are generated from Rust Serde types with ts-rs and
      checked for drift.
- [x] Tests cover serialized ordering, bounded duplicate/retried command
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

- 2026-06-18: Planning resolved service ownership, bounded deduplication,
  polling, revision, snapshot publication, and recoverable error semantics.
  The linked plan is accepted and the task is ready for implementation.
- 2026-06-18: Task claimed.
- 2026-06-18: Added versioned Rust command/snapshot contracts, a mutex-serialized
  `ApplicationService`, 256-entry retry deduplication, revisioned snapshot
  subscribers, bounded operator errors, preflight updates, and thin Tauri
  adapter helpers. Generated TypeScript contracts are committed.
- 2026-06-18: Seven service integration tests cover ordering, transitions,
  polling, retries, eviction, typed failures, publication, and concurrent
  callers. Rust/frontend tests, build, bindings drift, formatting, ESLint, and
  Clippy all pass.

- 2026-06-18: Implemented and validated authoritative commands, snapshots,
  revisions, bounded retries/errors, polling, generated bindings, and shared
  adapter seams.

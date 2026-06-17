---
id: TASK-003
title: Persist cue scenes and import audio into managed storage
status: idea
priority: P0
type: feature
owner: unassigned
depends_on: [TASK-002]
plan: none
updated: 2026-06-18
---

# TASK-003: Persist cue scenes and import audio into managed storage

## Context

Cue configuration and audio references must survive restarts and source-file
moves without storing user audio in the repository or web assets.

## Outcome

Rust can import MP3/WAV files into application-managed storage and round-trip
scene/cue configuration with clear missing or invalid-file errors.

## Acceptance Criteria

- [ ] Cue data covers name, color, file, volume, mode, and fade time.
- [ ] Imported files are copied into the application data directory.
- [ ] Saving and reopening preserves scenes and cues.
- [ ] Missing and invalid files produce recoverable typed errors.
- [ ] Rust integration tests cover import and persistence behavior.

## Validation

```text
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Refine scope after TASK-001 defines serialization and shared-type ownership.

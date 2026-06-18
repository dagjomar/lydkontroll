---
id: TASK-003
title: Persist cue scenes and import audio into managed storage
status: ready
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

- [ ] A `schemaVersion: 1` Rust aggregate covers stable IDs, scenes, cue name,
      color, managed file, volume, mode, and fade time and generates its
      TypeScript contract with ts-rs.
- [ ] Imported MP3/WAV files are staged, decode-validated, flushed, and
      atomically moved into `<app_data_dir>/audio`.
- [ ] Atomic save, backup, reopen, and version handling preserve scenes/cues and
      never persist source-file paths.
- [ ] Missing and invalid files produce recoverable typed errors.
- [ ] Rust integration tests cover import, round trip, corrupt/future schemas,
      interrupted-write recovery, and missing managed files.

## Validation

```text
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Follow ADR-002 and ADR-003. Inject the app-data root in tests and keep Tauri path
resolution in an adapter.

- 2026-06-18: TASK-002 dependency completed; ADR-002/003 and the accepted foundation plan fully constrain this implementation slice.

---
id: TASK-020
title: Remove deprecated files from managed audio storage
status: done
priority: P1
type: feature
owner: unassigned
depends_on: [TASK-003, TASK-007]
plan: none
updated: 2026-06-25
---

# TASK-020: Remove deprecated files from managed audio storage

## Context

Each import copies a new managed MP3/WAV into application storage, but the
desktop currently has no way to remove old versions. Re-importing corrected
audio therefore leaves deprecated entries in the cue file picker and unused
files on disk.

## Outcome

The desktop operator can safely remove an obsolete managed audio file, while
the app protects files still used by cues and keeps persisted metadata and
managed storage consistent.

## Scope

- Show imported files in a compact desktop file-management view or section.
- Identify each file by original name and enough metadata to distinguish
  repeated imports.
- Allow deletion only after explicit confirmation.
- Refuse deletion while any saved cue references the file, and name the
  blocking cue or cues.
- Remove an eligible file from both the persisted library and managed audio
  directory as one coordinated operation.
- Refresh authoritative snapshots and preflight facts after deletion.
- Surface recoverable deletion errors without losing the previous valid
  library or silently orphaning metadata.

## Non-goals

- Editing audio content, tags, or filenames.
- Automatically deleting older files merely because names match.
- Bulk cleanup, duplicate-content detection, or storage quotas.
- Deleting source files outside application-managed storage.
- Removing cues automatically to make a file deletable.

## Acceptance Criteria

- [x] The desktop lists all managed audio files and distinguishes repeated
      imports with the same original filename.
- [x] An unreferenced file can be confirmed for deletion and no longer appears
      after app restart.
- [x] The corresponding managed file is removed from disk.
- [x] A file referenced by one or more cues cannot be deleted, and the UI names
      every blocking cue.
- [x] Cancelling confirmation changes neither metadata nor disk contents.
- [x] Persistence or filesystem failure leaves a recoverable, internally
      consistent state and shows an operator-facing error.
- [x] Existing cue playback, import, persistence, and missing-file diagnostics
      continue to pass.

## Validation

```text
cargo test --manifest-path src-tauri/Cargo.toml
npm test -- --run
npm run build
npm run lint
manual: import two same-named versions, delete only the unreferenced version,
restart, and confirm the referenced version and its cue still play
python3 scripts/ralph.py check
```

## Notes

- Classified as P1 because it improves library hygiene and prevents operator
  confusion, but it does not block playback of the current candidate.
- Ready as a minimal safe-delete result; a broader file manager can be proposed
  separately if sorting, preview, replacement, or bulk operations are needed.
- Preserve the persistence rule that only application-managed files may be
  deleted. Never follow or delete the original import source path.

- 2026-06-25: Task claimed.
- 2026-06-25: Added a collapsed desktop managed-file section with original
  name, format, size, and managed filename suffix so repeated imports remain
  distinguishable without taking cue workspace.
- 2026-06-25: Added explicit confirmation, saved-and-draft cue reference
  blockers, and authoritative snapshot/preflight refresh after deletion.
- 2026-06-25: The repository stages deletion outside managed audio, atomically
  saves updated metadata, and restores the file if persistence fails.
- 2026-06-25: Full Rust tests, frontend tests, production build, lint,
  generated-binding checks, rustfmt, and Clippy passed. The local in-app
  browser connection was unavailable; duplicate-name, confirmation, restart,
  disk removal, and rollback behavior are covered by automated regressions.
- 2026-06-25: Task completed.

- 2026-06-25: Implemented confirmed deletion of unreferenced managed audio with cue-reference blocking, rollback-safe persistence, refreshed snapshots/preflight, and regression coverage.

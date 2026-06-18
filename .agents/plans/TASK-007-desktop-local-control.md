# Plan: TASK-007 — Desktop cue editing and resilient local control

Status: accepted
Updated: 2026-06-18

## Problem

The existing shell can play a library supplied at construction time, but the
operator cannot yet create that library, import managed audio, or invoke the
authoritative command path from the Mac UI. The first hands-on milestone must
work without Tailscale and must persist edits before exposing them as current
state.

## Current Evidence

- `ApplicationService<B>` already serializes playback commands, polling,
  revisions, retry acknowledgements, errors, and complete snapshots.
- `JsonLibraryRepository` already validates, imports, saves, backs up, and
  reloads schema-v1 libraries and managed MP3/WAV files.
- `adapters/tauri` is intentionally thin, but the Tauri composition root does
  not yet manage a concrete service or expose commands.
- The React shell is still a scaffold and generated `AppSnapshot` contracts do
  not expose managed audio metadata needed by the cue editor.
- Tauri's official dialog plugin returns filesystem paths on macOS and can
  restrict the picker to MP3/WAV files.

## Open Questions

- How are edits made durable without creating a second authoritative frontend
  store? Resolve by saving a complete candidate library through the desktop
  coordinator, then replacing the service library and publishing its snapshot.
- Where does file selection live? Resolve in the desktop UI through the native
  dialog plugin; validation, copying, metadata, and persistence remain Rust
  responsibilities.
- How does the UI observe playback completion before WebSockets exist? Resolve
  with a lightweight local snapshot poll that calls the service's serialized
  backend poll before returning the latest snapshot.
- What is deferred? Multi-scene drag-and-drop, waveform previews, keyboard
  shortcuts, and remote library editing are not required for the first usable
  Mac workflow.

## Options Considered

### Option A: Frontend-owned library with ad hoc playback calls

- Benefits: least Rust work.
- Costs and risks: duplicates authoritative state, bypasses persistence rules,
  and creates a transport-specific command path.

### Option B: Put dialogs and filesystem persistence inside React

- Benefits: familiar browser-shaped implementation.
- Costs and risks: violates the Rust ownership boundary and cannot safely
  preserve atomic save/import behavior.

### Option C: Thin desktop coordinator around the shared service

- Benefits: reuses repository and command semantics, keeps React declarative,
  publishes only persisted libraries, and gives future transports one service
  to observe.
- Costs and risks: complete-library saves are coarse-grained and local library
  mutations must remain serialized by the coordinator until command-level
  mutation is needed remotely.

## Decision

Choose Option C. Add managed audio metadata and a controlled library replacement
operation to `ApplicationService`; serialize desktop save/import operations in
one Tauri-managed coordinator; persist candidates before publishing them.
Playback continues to use versioned `CommandEnvelope` values. Record this as
ADR-009.

## Implementation Slices

1. Extend authoritative snapshots and service tests for persisted library
   replacement and managed audio metadata.
2. Add a concrete desktop coordinator, Kira-backed managed state, native dialog
   setup, Tauri commands, and periodic backend polling through the service.
3. Build a desktop React workflow for scene/cue CRUD, ordering, MP3/WAV import,
   cue configuration, trigger/stop/fade controls, master volume, and errors.
4. Add component/state-flow tests and run generated-binding, frontend, Rust,
   lint, and Ralph validation.

## Test Strategy

- Automated: Rust service tests for library publication; Tauri mock smoke tests;
  React tests for scene/cue editing, import, triggering, active playback, and
  visible failures; full build/lint suites.
- Manual: launch on the target Mac, import one known WAV and MP3, create cues,
  trigger overlap/exclusive playback through analog output, then stop/fade.
- Failure modes: canceled dialog, invalid/unsupported audio, persistence error,
  deleted managed file, unavailable output, and failed playback command.

## Rollback or Recovery

Library writes retain the existing atomic primary/backup behavior. If a save or
import fails, the candidate is not published to `ApplicationService`; the last
authoritative snapshot remains active and the UI displays the error.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.

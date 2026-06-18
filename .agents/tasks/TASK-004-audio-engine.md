---
id: TASK-004
title: Implement deterministic local audio playback
status: done
priority: P0
type: feature
owner: unassigned
depends_on: [TASK-002, TASK-003]
plan: .agents/plans/TASK-004-audio-engine.md
updated: 2026-06-18
---

# TASK-004: Implement deterministic local audio playback

## Context

Reliable playback is the core event-critical behavior and needs isolated tests
and explicit concurrency semantics.

## Outcome

The Rust audio engine plays managed MP3/WAV files and implements overlap,
exclusive, retrigger, volume, stop, individual fade, and fade-all behavior.

## Acceptance Criteria

- [x] MP3 and WAV cues play through the selected macOS system output.
- [x] Overlap and exclusive rules match `PLAN.md`.
- [x] Retriggering restarts the same cue deterministically.
- [x] Fade and stop update authoritative playback state.
- [x] Engine behavior is tested without React or networking.
- [x] Manual analog-output checks are documented.

## Validation

```text
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Follow ADR-001: Kira/CPAL is the production adapter, while ordering and fade
semantics belong to an application-owned state machine tested with a fake audio
backend. The accepted linked plan defines exclusive barriers, retrigger
ordering, event-driven completion, cancellation, and output-loss behavior.

- 2026-06-18: Planning completed; the linked plan is accepted and the task is
  ready for implementation.
- 2026-06-18: Added the deterministic `PlaybackEngine`, fake-backend behavior
  suite, and Kira 0.12 streaming adapter for managed MP3/WAV playback through
  CPAL's default output. The target-Mac analog/output-loss procedure is recorded
  in the linked plan and remains a release rehearsal gate.
- 2026-06-18: Validation passed: Rust tests, repeated audio-engine tests,
  frontend tests/build, generated bindings, formatting, ESLint, and Clippy with
  warnings denied.
- 2026-06-18: Task claimed.
- 2026-06-18: Acceptance criteria and automated validation completed;
  target-Mac analog checks remain documented for release rehearsal.

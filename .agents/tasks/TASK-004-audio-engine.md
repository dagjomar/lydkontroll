---
id: TASK-004
title: Implement deterministic local audio playback
status: idea
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

- [ ] MP3 and WAV cues play through the selected macOS system output.
- [ ] Overlap and exclusive rules match `PLAN.md`.
- [ ] Retriggering restarts the same cue deterministically.
- [ ] Fade and stop update authoritative playback state.
- [ ] Engine behavior is tested without React or networking.
- [ ] Manual analog-output checks are documented.

## Validation

```text
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Create the linked plan before changing this task to `ready`; concurrency, clock,
decoder, and test-double choices deserve explicit design. Follow ADR-001:
Kira/CPAL is the production adapter, while ordering and fade semantics belong to
an application-owned state machine tested with fake audio and clock ports.

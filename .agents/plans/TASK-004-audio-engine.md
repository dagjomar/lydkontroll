# Plan: TASK-004 — Audio engine

Status: placeholder
Updated: 2026-06-18

## Problem

Playback concurrency, timing, fade semantics, decoder behavior, and hardware
testing need a detailed plan before this event-critical subsystem is built.

## Open Questions

- How are playback instances, clocks, cancellation, and state updates modeled?
- How are exclusive transitions and retriggers ordered?
- Which behavior can run under deterministic tests without audio hardware?

## Decision

Deferred until the task's dependencies and TASK-001 decisions are complete.

## Implementation Slices

1. Refine this plan after persistence types and the chosen audio stack exist.

## Test Strategy

- Automated: deterministic state-machine and engine-adapter tests.
- Manual: MP3/WAV playback, fades, overlaps, and analog output on target Mac.

## Ready Checklist

- [ ] Open questions are resolved or explicitly deferred.
- [ ] Decisions and tradeoffs are recorded.
- [ ] Slices are small and dependency ordered.
- [ ] Tests cover the important failure modes.
- [ ] The parent task can move to `ready`.

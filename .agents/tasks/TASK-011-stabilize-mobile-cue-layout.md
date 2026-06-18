---
id: TASK-011
title: Keep iPhone cue positions stable during playback
status: done
priority: P0
type: bug
owner: codex-2026-06-18-stable-mobile-layout
depends_on: [TASK-008]
plan: none
updated: 2026-06-18
---

# TASK-011: Keep iPhone cue positions stable during playback

## Context

The iPhone view currently inserts the `Spiller nå` section only when playback
is active. That shifts every cue downward at the moment the operator taps,
which creates a risk that a follow-up tap triggers the wrong cue.

## Outcome

Cue buttons remain at identical screen coordinates when playback starts,
changes, fades, stops, or becomes idle.

## Scope

- Reserve a fixed-height playback-status region in both idle and active states.
- Show a clear, quiet empty state when no audio is active.
- Render active playback details and controls inside the reserved region
  without changing its outer height.
- Reduce the vertical footprint of `Fade alt` and `Stopp alt` while preserving
  touch-safe targets and clear emergency-control styling.
- Keep scene and cue controls usable at the supported iPhone viewport.

## Non-goals

- Redesign cue ordering, colors, or playback semantics.
- Add new playback commands or protocol state.
- Replace the existing connection and acknowledgement status.

## Acceptance Criteria

- [x] The first cue has the same vertical position with zero, one, or multiple
      active playback instances.
- [x] The playback-status region has a useful empty state and fixed outer
      height.
- [x] Multiple active sounds remain readable and controllable within the fixed
      region without pushing cues.
- [x] `Fade alt` and `Stopp alt` use less vertical space while retaining at
      least 44-point touch targets.
- [x] Automated tests cover idle-to-playing and playing-to-idle layout states.
- [x] A 390x844 rendered check confirms cue positions do not move.

## Validation

```text
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- 2026-06-18: Added from operator feedback after successful iPhone Safari use.
  This is a safety/usability bug and should be completed before preflight work.
- The behavior and constraints are sufficiently specific for direct
  implementation; no separate planning task is required.

- 2026-06-18: Task claimed.
- 2026-06-18: Kept the playback panel mounted at a fixed 148-pixel rendered
  height with an internal scrolling list and quiet idle state. Emergency and
  per-playback controls render at 44 pixels. At 390x844, the first cue's
  document position remained 585.9296875 pixels through idle, playing, and
  stopped states.
- 2026-06-18: `npm test -- --run`, `npm run build`, and `npm run lint` passed.
- 2026-06-18: Fixed-height playback region, 44px controls, regression
  coverage, and 390x844 rendered position check completed.

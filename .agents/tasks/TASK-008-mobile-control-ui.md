---
id: TASK-008
title: Build reconnecting iPhone control interface
status: idea
priority: P1
type: feature
owner: unassigned
depends_on: [TASK-006]
plan: none
updated: 2026-06-18
---

# TASK-008: Build reconnecting iPhone control interface

## Outcome

iPhone Safari provides large, status-aware controls for scenes, cues, active
audio, master volume, stop-all, and fade-all.

## Acceptance Criteria

- [ ] Controls are touch-friendly and usable on the supported iPhone viewport.
- [ ] Connection and command acknowledgement state are always visible.
- [ ] Reconnect restores authoritative state without stopping Mac playback.
- [ ] Duplicate taps/retries do not produce ambiguous state.
- [ ] Tests cover disconnect, reconnect, and stale-state behavior.

## Validation

```text
npm test
npm run build
npm run lint
python3 scripts/ralph.py check
```

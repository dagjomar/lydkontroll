---
id: TASK-008
title: Build reconnecting iPhone control interface
status: done
priority: P1
type: feature
owner: codex-2026-06-18-mobile
depends_on: [TASK-006]
plan: .agents/plans/TASK-008-mobile-control.md
updated: 2026-06-18
---

# TASK-008: Build reconnecting iPhone control interface

## Outcome

iPhone Safari provides large, status-aware controls for scenes, cues, active
audio, master volume, stop-all, and fade-all.

## Acceptance Criteria

- [x] Controls are touch-friendly and usable on the supported iPhone viewport.
- [x] Connection and command acknowledgement state are always visible.
- [x] Reconnect restores authoritative state without stopping Mac playback.
- [x] Duplicate taps/retries do not produce ambiguous state.
- [x] Tests cover disconnect, reconnect, and stale-state behavior.

## Validation

```text
npm test
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- 2026-06-18: Refined around a dedicated Safari projection, observable
  WebSocket connection state, fresh-snapshot reconnects, stale-socket guards,
  and duplicate in-flight action suppression.
- 2026-06-18: Task claimed.
- 2026-06-18: Added touch-safe scenes, cues, active playback, master volume,
  stop, and fade controls. Disconnects invalidate cached state and reconnect
  from a fresh snapshot while ignoring replaced socket events.
- 2026-06-18: Added regressions for visible status, duplicate taps, reconnect,
  authoritative replacement, and stale sockets. Validated Vitest, production
  build, lint, Ralph metadata, and a 390x844 rendered interaction check.

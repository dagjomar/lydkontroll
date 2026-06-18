---
id: TASK-007
title: Build desktop cue editing and resilient local control
status: needs-planning
priority: P0
type: feature
owner: unassigned
depends_on: [TASK-005]
plan: none
updated: 2026-06-18
---

# TASK-007: Build desktop cue editing and resilient local control

## Context

The operator needs an end-to-end local Mac workflow before remote control is
added: import audio, configure scenes and cues, trigger playback, and recover
from visible errors without depending on Tailscale or an iPhone.

## Outcome

The Mac app can edit scenes/cues and operate all playback behavior locally,
independent of mobile connectivity.

## Acceptance Criteria

- [ ] Users can create, edit, reorder, and remove scenes and cues.
- [ ] Users can import files and configure color, volume, mode, and fade.
- [ ] Local controls expose cue triggering, active sounds, stop, and fade.
- [ ] Missing-file and command errors are visible and recoverable.
- [ ] Component and state-flow tests cover critical interactions.

## Validation

```text
npm test
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- 2026-06-18: Promoted ahead of `TASK-006` so the next hands-on milestone is a
  fully testable local Mac workflow. Planning must define library mutation and
  import commands, Tauri dialog boundaries, desktop state flow, and the
  smallest vertical slice that reaches real local playback.

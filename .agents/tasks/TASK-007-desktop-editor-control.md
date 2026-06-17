---
id: TASK-007
title: Build desktop cue editing and resilient local control
status: idea
priority: P1
type: feature
owner: unassigned
depends_on: [TASK-005]
plan: none
updated: 2026-06-18
---

# TASK-007: Build desktop cue editing and resilient local control

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

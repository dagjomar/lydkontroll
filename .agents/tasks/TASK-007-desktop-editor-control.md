---
id: TASK-007
title: Build desktop cue editing and resilient local control
status: done
priority: P0
type: feature
owner: unassigned
depends_on: [TASK-005]
plan: .agents/plans/TASK-007-desktop-local-control.md
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

- [x] Users can create, edit, reorder, and remove scenes and cues.
- [x] Users can import files and configure color, volume, mode, and fade.
- [x] Local controls expose cue triggering, active sounds, stop, and fade.
- [x] Missing-file and command errors are visible and recoverable.
- [x] Component and state-flow tests cover critical interactions.

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
- 2026-06-18: Planning accepted. The desktop coordinator serializes complete
  library save/import operations, persists candidates before publishing them
  through `ApplicationService`, and keeps playback on versioned command
  envelopes. Native file selection uses the Tauri dialog plugin.
- 2026-06-18: Implemented the complete local workflow: native MP3/WAV picker,
  managed import, atomic library saves, revisioned snapshots, scene/cue CRUD and
  ordering, cue configuration, master/instance playback controls, resilient
  audio initialization, missing-file recovery, and visible command errors.
  Rust, frontend, build, lint, binding drift, and browser preview checks pass.
  Real analog playback remains a documented target-Mac rehearsal gate.

- 2026-06-18: Task claimed.

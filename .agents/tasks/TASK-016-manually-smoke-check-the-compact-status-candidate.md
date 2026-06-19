---
id: TASK-016
title: Manually smoke-check the compact-status candidate
status: blocked
priority: P0
type: chore
owner: unassigned
depends_on: [TASK-018]
plan: none
updated: 2026-06-18
---

# TASK-016: Manually smoke-check the compact-status candidate

## Context

The replacement bundle must include the compact status control and the iPhone
master-volume drag fix. macOS has no native Tauri WebDriver, so a human must
confirm the packaged window, audible three-second test play, and physical
iPhone Safari gesture before it replaces the previously rehearsed candidate.

## Outcome

The packaged replacement launches on the event Mac, the compact status control
works in the native window, and test playback is heard and fades after three
seconds.

## Scope

- Launch the packaged `Lydkontroll.app`.
- Confirm preflight starts collapsed and expands from the status pill.
- Confirm the QR and diagnostics are visible and refresh completes.
- Run the three-second test play with a saved cue and confirm audible fade.
- Drag master volume through several values in iPhone Safari and confirm the
  thumb follows the finger without releasing after each revision.

## Non-goals

- Repeating the full 60-minute rehearsal unless the smoke check exposes a
  broader regression.

## Acceptance Criteria

- [ ] Packaged app launches without a blocking error.
- [ ] Status pill, expansion, QR, and refresh work in the native window.
- [ ] A saved cue is audible during test play and fades after three seconds.
- [ ] Master volume drags continuously in iPhone Safari and finishes at the
      selected value.
- [ ] `.agents/CANDIDATE.md` is promoted to the accepted event candidate.

## Validation

```text
open src-tauri/target/release/bundle/macos/Lydkontroll.app
manual native desktop and audible-output check
python3 scripts/ralph.py check
```

## Notes

- Blocker: native window interaction and audible output require the user at the
  event Mac. Automated coverage and packaging have already passed.
- Exact next action: open the bundle, expand the status pill, run refresh, and
  use `Spill i 3 sekunder` on a saved cue, then drag master volume on the phone;
  report whether all five checks pass.
- 2026-06-19: The candidate recorded before `TASK-017` does not contain the
  slider fix. `TASK-018` must replace its bundle identity before this check.

# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-012` produced three functional iPhone alternatives on one shared runtime:
an expanding now-playing overlay, a cue-first layout with global controls
below, and a two-tab cues/status layout. All can be tested with mock data or the
real Tailscale/WebSocket control URL.

## Exact Next Action

Ask the operator to test the three variants in `MOBILE_LAYOUT_PROTOTYPES.md` and
choose one, or describe a combination. Then refine and start `TASK-013`.

```text
python3 scripts/ralph.py claim TASK-013 --owner "<agent/session>"
```

## Important Context

- Normal mobile URLs still use the fixed stable production layout.
- Add `?layout=overlay`, `?layout=controls`, or `?layout=tabs` to the real mobile
  URL to test against actual cues and playback.
- The bottom switcher preserves current playback state while comparing.
- At 390x844 with three active rows, overlay left cues enabled and visible,
  cue-first required scrolling for all global controls, and tabs fit the cue
  operation view most compactly while moving master/status to a second tab.
- `TASK-013` must not move to `ready` until the operator chooses a direction.
- After `TASK-013`, return to `TASK-009` event preflight.

## Validation to Run

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

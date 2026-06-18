# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-011` was added from real iPhone Safari feedback. The conditional
`Spiller nå` section currently shifts cue positions when playback begins,
creating an accidental-trigger risk.

## Exact Next Action

Start and implement the ready P0 layout-safety bug.

```text
python3 scripts/ralph.py start TASK-011 --owner "<agent/session>"
```

## Important Context

- Keep the playback-status section mounted in both idle and active states with
  a fixed outer height.
- Give idle playback a quiet empty state instead of removing the section.
- Multiple active sounds must remain usable inside the reserved space without
  pushing the scene tabs or cue grid.
- Make `Fade alt` and `Stopp alt` shorter, but retain at least 44-point touch
  targets and obvious emergency-control styling.
- Add tests around idle-to-playing and playing-to-idle state changes.
- Use the mobile preview at 390x844 to compare cue coordinates before and after
  playback changes.
- After `TASK-011`, return to refining `TASK-009` event preflight.

## Validation to Run

```text
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```

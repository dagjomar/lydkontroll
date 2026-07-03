# Latest Handoff

Updated: 2026-07-03

## What Just Happened

`TASK-021` was claimed and its conflict-aware shortlist was completed. The
recommended package retains `Lydkontroll`, positions it for live events, makes
the displayed event title optional with `Mitt arrangement` as the fallback,
and revises the heart into a neutral rounded waveform. `SoundMastah` remains
the only alternate offered; every original candidate has an explicit recorded
disposition.

## Exact Next Action

Ask the owner to accept the complete `Lydkontroll` recommendation, choose the
`SoundMastah` alternate, or name one specific change. Then record the accepted
choice in `.agents/DECISIONS.md`, complete the linked plan, move `TASK-021` to
done, and proceed to `TASK-022`.

## Important Context

- Conflict checking is an obvious-collision screen, not trademark clearance.
- `npm run icons:generate` reproducibly rebuilds icons from the SVG source;
  `npm run icons:check` validates transparent macOS corners.
- Finder/Dock/app-switcher inspection is still a useful human check, but the
  built `.app` icon byte-matches the asset whose alpha was verified.
- `TASK-024` must settle license and source-only versus signed/notarized binary
  releases and audit the intended Git history.
- Do not create or push a public GitHub remote until `TASK-022`, `TASK-023`, and
  `TASK-024` are done.
- The old manual event-candidate gate `TASK-016` remains truthfully blocked.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

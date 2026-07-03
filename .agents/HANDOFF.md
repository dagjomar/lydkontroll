# Latest Handoff

Updated: 2026-07-03

## What Just Happened

Public-launch feedback was deduplicated into five independently schedulable
results: `TASK-021` brand/customization decision, blocked `TASK-022` reusable
identity implementation, ready `TASK-023` transparent macOS icon corners,
`TASK-024` safe public-repository preparation, and blocked `TASK-025` GitHub
publication. No product code changed.

## Exact Next Action

Claim `TASK-021`, research a short conflict-aware naming shortlist based on the
user's original candidates, and present one compact product decision covering:

1. wedding/toastmaster-specific versus event-generic positioning;
2. the public name and repository slug;
3. fixed versus configurable displayed event title, including default text;
4. retaining, revising, or replacing the waveform-heart identity.

Record the accepted choice in `.agents/DECISIONS.md`, complete the linked plan,
and move `TASK-021` to done. That unblocks `TASK-022`.

## Important Context

- `Marius + Wenche` is hard-coded in both desktop and mobile UI; wedding/event
  identity also appears in Cargo, HTML, Tauri, README, PLAN/project context,
  server fixtures, and release documentation.
- `TASK-023` is already ready and independent: fix alpha at the outer corners,
  preserve the current heart artwork, regenerate `.icns`, build, and inspect it
  in Finder/Dock/app switcher.
- ADR-013 says the known-Mac unsigned build is not ready for public
  distribution. `TASK-024` must explicitly settle license and source-only
  versus signed/notarized binary releases and audit the intended Git history.
- Do not create or push a public GitHub remote until `TASK-022`, `TASK-023`, and
  `TASK-024` are done.
- The old manual event-candidate gate `TASK-016` remains truthfully blocked but
  is no longer the current project focus now that the toastmaster job is over.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

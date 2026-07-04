# Latest Handoff

Updated: 2026-07-04

## What Just Happened

Completed `TASK-031`. The repository README now links prominently to the live
Norwegian marketing site and shows compact, event-generic Mac and iPhone views
using the canonical website assets.

## Exact Next Action

Claim and plan `TASK-026` without implementing it. Resolve the linked plan's
language-selection, translation ownership, persistence, generated-contract,
website/README, and validation choices, then leave the task ready if the plan
can settle them from repository evidence.

## Important Context

- `TASK-030` and `TASK-031` complete the Norwegian marketing track. The site is
  live at `https://dagjomar.github.io/lydkontroll/`, and the README reuses its
  512x326 Mac and 263x512 iPhone captures without duplicating assets.
- The Pages URL returned HTTP 200; `npm run public:check`, `git diff --check`,
  and Ralph validation passed for the README presentation.
- `TASK-026` now owns complete English localization and language selection.
- Current mobile control requires iPhone/Safari and Tailscale. Desktop control
  remains local and independent of mobile connectivity.
- The project is source-available under PolyForm Noncommercial and has no
  public signed/notarized download. Preserve those claim boundaries.
- `TASK-027` remains future research; do not market speculative LAN or fallback
  modes.
- The README still accurately states that no signed/notarized public bundle is
  distributed and that iPhone control requires Tailscale.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

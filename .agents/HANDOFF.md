# Latest Handoff

Updated: 2026-07-04

## What Just Happened

Completed `TASK-030`. The Norwegian marketing site is live at
`https://dagjomar.github.io/lydkontroll/` with event-generic app captures,
accurate claims, isolated dependencies, and a passing Pages workflow.

## Exact Next Action

Start ready `TASK-031` and add the verified Pages URL plus compact,
event-generic Mac/iPhone screenshots near the top of `README.md`. Preserve the
existing technical, licensing, support, and distribution guidance, then verify
the rendered README at desktop and narrow widths.

## Important Context

- `TASK-030` is complete. Pages workflow run `28690125409` and live visual
  checks at 1280x720 and 390x844 passed.
- The marketing site remains Norwegian-first; `TASK-026` follows it and owns
  complete English localization.
- Current mobile control requires iPhone/Safari and Tailscale. Desktop control
  remains local and independent of mobile connectivity.
- The project is source-available under PolyForm Noncommercial and has no
  public signed/notarized download. Preserve those claim boundaries.
- `TASK-027` remains future research; do not market speculative LAN or fallback
  modes.
- `TASK-031` is now ready and owns the README screenshots and prominent Pages
  link.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

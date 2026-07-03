# Latest Handoff

Updated: 2026-07-04

## What Just Happened

Completed `TASK-029`. `DESIGN.md` is a self-contained Norwegian product,
visual, content, screenshot, and Google Stitch handoff grounded in current
source and accepted decisions. It clearly separates fixed claims and brand
constraints from areas Stitch may explore.

## Exact Next Action

Copy the prompt under «Kopierbar brief til Google Stitch» in `DESIGN.md` into
Google Stitch. The owner should review the result and choose or revise a
concept. After that review, plan `TASK-030` by resolving its CTA, Pages
structure/URL, analytics/privacy, and publishable screenshot choices.

## Important Context

- Do not start `TASK-030` before the owner has reviewed a design concept.
- The marketing site remains Norwegian-first; `TASK-026` follows it and owns
  complete English localization.
- Current mobile control requires iPhone/Safari and Tailscale. Desktop control
  remains local and independent of mobile connectivity.
- The project is source-available under PolyForm Noncommercial and has no
  public signed/notarized download. Preserve those claim boundaries.
- `TASK-027` remains future research; do not market speculative LAN or fallback
  modes.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

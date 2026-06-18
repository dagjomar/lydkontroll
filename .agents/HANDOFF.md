# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-013` is complete. The operator selected the expanding now-playing
overlay, ADR-011 records the choice, and all prototype-only switchers,
alternative layouts, query options, and documentation were removed.

## Exact Next Action

Claim and refine event preflight into a dependency-safe implementation plan.

```text
python3 scripts/ralph.py claim TASK-009 --owner "<agent/session>"
```

## Important Context

- The no-query mobile URL now uses the overlay automatically.
- The overlay appears only while playback is active, grows to 38 percent of the
  viewport, then scrolls its active list so cues remain uncovered.
- `TASK-009` is still an `idea`; use `.agents/plans/TEMPLATE.md`, accepted
  architecture decisions, and the existing `PreflightFacts` projection before
  moving it to `ready`.
- Preserve the accepted boundary: Rust owns readiness facts; React renders
  operator guidance and safe actions.

## Validation to Run

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

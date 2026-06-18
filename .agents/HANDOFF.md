# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-011` is complete. The iPhone view now keeps a fixed-height `Spiller nå`
region mounted in idle and active states, scrolls multiple active rows inside
that region, and keeps emergency/playback actions at measured 44-pixel touch
targets. The first cue stayed at the same document position in a 390x844
idle-playing-stopped check.

## Exact Next Action

Claim and refine the event-preflight idea into a dependency-safe implementation
plan.

```text
python3 scripts/ralph.py claim TASK-009 --owner "<agent/session>"
```

## Important Context

- `TASK-009` is still an `idea`; use `.agents/plans/TEMPLATE.md`, accepted
  architecture decisions, and the existing `PreflightFacts` projection before
  moving it to `ready`.
- The preflight outcome must cover managed files, validated Tailscale binding
  and reachability, mobile URL/QR presentation, audio-output expectations, safe
  test playback, and blocking-versus-warning diagnostics.
- Preserve the accepted boundary: Rust owns facts and readiness; React renders
  operator guidance and actions.
- Hardware-dependent output naming may require an explicit manual verification
  step if the supported platform APIs cannot expose it reliably.

## Validation to Run

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

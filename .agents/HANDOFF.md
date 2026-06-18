# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-009` is complete. The Mac now has a single preflight view for managed
files, audio-output guidance, Tailscale/control-server readiness, mobile URL/QR
access, and an instance-scoped three-second test play. ADR-012 records the
Rust-owned readiness boundary.

## Exact Next Action

Claim and refine release rehearsal and recovery into an implementation-ready
plan.

```text
python3 scripts/ralph.py claim TASK-010 --owner "<agent/session>"
```

## Important Context

- `TASK-010` dependencies are complete, but its linked plan is still a
  placeholder with unresolved rehearsal hardware, pass/fail, evidence, and
  recovery details.
- Preflight warnings are manual checks; unavailable statuses are blockers.
- Output selection remains a manual macOS System Settings step.
- Real packaged Tailscale discovery, iPhone network transitions, analog output,
  and output-loss recovery remain rehearsal gates.

## Validation to Run

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

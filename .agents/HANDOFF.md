# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-010` is complete. The operator confirmed that every target-hardware
rehearsal gate passed, including the 60-minute run, iPhone/Tailscale failure
injection, Mac-local fallback, and analog-output recovery. The result is
recorded in `.agents/REHEARSAL.md` against the exact commit and executable
checksum.

## Exact Next Action

Preserve and use the rehearsed candidate for the event. Follow the event-day
checklist in `RELEASE_RUNBOOK.md`. If code or release configuration changes,
create a new task, rebuild the candidate, and repeat the rehearsal before use.

## Important Context

- Rehearsed commit:
  `885422112d554da5f22001a39115d2afcdf30e46`.
- Rehearsed executable SHA-256:
  `1347aacdd57fa9b6639114b6595c997c4b9ceac128796759074ec4a96b228c61`.
- The build is arm64 and ad-hoc signed; no Apple signing credentials are
  required for the known event Mac.
- The release bundle and local audio/data remain ignored and must not be
  committed.
- Mac-local control is the fallback for any phone/network failure. Do not
  restart the app during active audio.

## Validation to Run

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

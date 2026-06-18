# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-014` replaces the permanently expanded desktop preflight area with a
compact system-status pill. It is green when no blocking check exists, red when
action is required, and expands to the unchanged diagnostics, QR, refresh, and
test-play tools.

## Exact Next Action

Build the new candidate and perform a short desktop regression check: confirm
the preflight panel starts collapsed, expands from the status pill, shows the
QR, refreshes, and runs the three-second test play. Record the new commit and
checksum before replacing the previously rehearsed build.

## Important Context

- The previously rehearsed commit remains `8854221` until this UI-only change
  is rebuilt and checked.
- `TASK-014` does not change Rust preflight facts, audio behavior, networking,
  or the iPhone interface.
- The release bundle and local audio/data remain ignored and must not be
  committed.
- Mac-local control is the fallback for any phone/network failure. Do not
  restart the app during active audio.

## Validation to Run

```text
npm test
npm run build
npm run lint
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

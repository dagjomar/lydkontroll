# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-010` release automation and documentation are implemented. The complete
release gate passes and builds a 5.2 MB Apple Silicon `Lydkontroll.app`.
ADR-013 and `RELEASE_RUNBOOK.md` define exact evidence, failure injection,
recovery, and event-day operation.

## Exact Next Action

On the target event Mac with the iPhone, production cue library, Tailscale
accounts, and analog output, build the committed candidate and complete
`RELEASE_RUNBOOK.md`. Copy its rehearsal-record template to
`.agents/REHEARSAL.md` and fill every field.

```text
npm ci
npm run release:build
```

If every manual gate passes, mark the final two `TASK-010` acceptance criteria
complete, set the task to `done`, update state/progress/handoff, validate, and
commit the rehearsal record.

## Important Context

- The current automated build is arm64 and ad-hoc signed; no Apple signing
  credentials are required for the known event Mac.
- The release bundle and local audio/data remain ignored and must not be
  committed.
- Network tests need permission to bind temporary loopback ports.
- Mac-local control is the fallback for any phone/network failure. Do not
  restart the app during active audio.

## Validation to Run

```text
npm run release:build
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-015` built and automatically verified the compact-status replacement
candidate. The arm64 bundle identity is recorded in `.agents/CANDIDATE.md`, and
focused desktop coverage now explicitly checks collapsed/expanded states, QR,
refresh, and the three-second trigger/fade sequence.

## Exact Next Action

Complete `TASK-016` on the event Mac:

1. Open `src-tauri/target/release/bundle/macos/Lydkontroll.app`.
2. Confirm preflight starts collapsed and expands from the status pill.
3. Confirm the QR appears and `Kjør sjekk på nytt` completes.
4. With a saved cue, run `Spill i 3 sekunder` and confirm audible playback
   fades after three seconds.

Report whether all four checks pass. If they do, close `TASK-016` and promote
`.agents/CANDIDATE.md` as the accepted event build.

## Important Context

- Replacement source commit:
  `ad35ac35d0b6d241458f745cb5e6e99becbb7cb6`.
- Replacement executable SHA-256:
  `01679888c466b2231ecb2e85d995d7320da4e31534539644aaa5ebab0f236055`.
- The previously rehearsed candidate in `.agents/REHEARSAL.md` remains the
  accepted fallback until `TASK-016` passes.
- The release bundle and local audio/data remain ignored and must not be
  committed.

## Validation to Run After the Manual Check

```text
python3 scripts/ralph.py set-status TASK-016 done --note "Native compact-status smoke check passed."
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

# Latest Handoff

Updated: 2026-06-19

## What Just Happened

`TASK-017` fixed the iPhone master-volume slider so rapid input stays local and
draggable while ordered, coalesced commands are confirmed by the Mac.
`TASK-018` passed the complete release gate and replaced the arm64 bundle
identity in `.agents/CANDIDATE.md`.

## Exact Next Action

Complete `TASK-016` on the event Mac:

1. Open `src-tauri/target/release/bundle/macos/Lydkontroll.app`.
2. Confirm preflight starts collapsed and expands from the status pill.
3. Confirm the QR appears and `Kjør sjekk på nytt` completes.
4. With a saved cue, run `Spill i 3 sekunder` and confirm audible playback
   fades after three seconds.
5. Open the QR URL in iPhone Safari and drag master volume through several
   values without lifting; confirm the thumb follows continuously and finishes
   at the chosen value.

Report whether all five checks pass. If they do, close `TASK-016` and promote
`.agents/CANDIDATE.md` as the accepted event build.

## Important Context

- Replacement source commit:
  `20871d5d2b8928af77d3ae3246eee4f4ab4be23c`.
- Replacement executable SHA-256:
  `4b8656e4a9b0ab279ae2dec4d5c51b2de58edaf8865969929dbbba85dfc7ea5b`.
- The previously rehearsed candidate in `.agents/REHEARSAL.md` remains the
  accepted fallback until `TASK-016` passes.
- The release bundle and local audio/data remain ignored and must not be
  committed.

## Validation to Run After the Manual Check

```text
python3 scripts/ralph.py set-status TASK-016 done --note "Native compact-status, audio, and iPhone slider smoke check passed."
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

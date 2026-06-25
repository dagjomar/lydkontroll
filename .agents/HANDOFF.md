# Latest Handoff

Updated: 2026-06-25

## What Just Happened

`TASK-020` is complete. The desktop now has a compact managed-audio section
that distinguishes repeated imports, names cue references, requires
confirmation, and safely deletes only unreferenced managed files. Persistence
rollback, restart durability, disk removal, cancellation, and UI behavior have
regression coverage. The candidate bundle was not rebuilt by this task.

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

## Important Context

- The audio adapter uses Kira's default backend and selected macOS output. It
  applies volume only and performs no app-specific polarity, downmix, or M/S
  transform.
- Affected source files can be corrected and rendered in Logic. After importing,
  select the new managed file and save the configuration before playback tests.
- No global or per-cue phase/M/S switch is needed.
- Managed-file deletion is desktop-only and never follows the original import
  source path. Files used by saved or draft cues remain protected.
- The previously rehearsed candidate in `.agents/REHEARSAL.md` remains the
  accepted fallback until `TASK-016` passes.

## Validation to Run After the Manual Check

```text
python3 scripts/ralph.py set-status TASK-016 done --note "Native compact-status, audio, and iPhone slider smoke check passed."
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

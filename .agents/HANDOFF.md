# Latest Handoff

Updated: 2026-07-03

## What Just Happened

`TASK-023` corrected the macOS icon's opaque corners. All tracked icon assets
now regenerate from `src-tauri/icons/icon.svg`; the release gate checks corner
alpha in the desktop PNGs and every representation embedded in `icon.icns`.
The packaged app contains the checked icon and the full release build passes.

## Exact Next Action

Claim `TASK-021`, research a short conflict-aware naming shortlist based on the
user's original candidates, and present one compact product decision covering:

1. wedding/toastmaster-specific versus event-generic positioning;
2. the public name and repository slug;
3. fixed versus configurable displayed event title, including default text;
4. retaining, revising, or replacing the waveform-heart identity.

Record the accepted choice in `.agents/DECISIONS.md`, complete the linked plan,
and move `TASK-021` to done. That unblocks `TASK-022`.

## Important Context

- `npm run icons:generate` reproducibly rebuilds icons from the SVG source;
  `npm run icons:check` validates transparent macOS corners.
- Finder/Dock/app-switcher inspection is still a useful human check, but the
  built `.app` icon byte-matches the asset whose alpha was verified.
- `TASK-024` must settle license and source-only versus signed/notarized binary
  releases and audit the intended Git history.
- Do not create or push a public GitHub remote until `TASK-022`, `TASK-023`, and
  `TASK-024` are done.
- The old manual event-candidate gate `TASK-016` remains truthfully blocked.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

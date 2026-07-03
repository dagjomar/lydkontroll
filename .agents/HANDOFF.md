# Latest Handoff

Updated: 2026-07-04

## What Just Happened

`TASK-021` and `TASK-022` are complete. `Lydkontroll` now launches as a
Norwegian-first, event-generic project. The event title is editable, persisted,
projected to desktop/mobile, and backward-compatible with existing schema-v1
libraries. Couple-specific metadata is gone and every platform icon now derives
from the neutral rounded-waveform SVG. `TASK-026` holds future English
localization and language selection.

## Exact Next Action

Claim `TASK-024`, finish its public-readiness plan, and obtain only the genuine
owner decisions it identifies: license and source-only versus signed/notarized
binary distribution. Then audit the tracked tree and intended Git history
before allowing `TASK-025` to create a public remote.

## Important Context

- `eventTitle` remains inside schema version 1 and uses a serde default, so old
  libraries load as `Mitt arrangement` without reset.
- Conflict checking is an obvious-collision screen, not trademark clearance.
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
npm run lint
npm test -- --run
cargo test --manifest-path src-tauri/Cargo.toml
```

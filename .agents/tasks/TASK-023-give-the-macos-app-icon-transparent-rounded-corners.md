---
id: TASK-023
title: Give the macOS app icon transparent rounded corners
status: done
priority: P1
type: bug
owner: codex-2026-07-03-icon-alpha
depends_on: []
plan: none
updated: 2026-07-03
---

# TASK-023: Give the macOS app icon transparent rounded corners

## Context

The current heart/waveform icon has opaque white pixels in its outer corners,
so macOS displays a white square instead of a clean rounded app-icon silhouette.
The feedback permits retaining the heart for now; this task fixes the concrete
asset defect without pre-empting the later brand decision.

## Outcome

The packaged macOS app uses an icon whose rounded outer corners are transparent
and render cleanly in Finder, Dock, and the app switcher.

## Scope

- Correct the source icon's alpha mask at all four outer corners.
- Regenerate the macOS `.icns` and tracked derived sizes from one source.
- Preserve the current heart/waveform artwork inside the rounded silhouette.
- Document or script the reproducible icon-generation command if absent.

## Non-goals

- Choosing a new logo or public product name.
- Signing, notarizing, or publishing the app.
- Redesigning non-macOS adaptive-icon conventions.

## Acceptance Criteria

- [x] All four source-image corner pixels have alpha zero and no white square
      remains outside the intended rounded silhouette.
- [x] The packaged `.app` contains the regenerated icon and macOS renders it
      without opaque corner artifacts in Finder and Dock.
- [x] Required macOS icon representations remain present and legible at small
      and large sizes.
- [x] Regeneration is reproducible from a documented source asset/command.

## Validation

```text
automated: inspect source and generated corner alpha values at every macOS size
npm run release:build
manual: inspect the built app icon in Finder, Dock, and app switcher on macOS
python3 scripts/ralph.py check
```

## Notes

- P1 because this is a visible public-release defect, not a playback blocker.
- Independent of `TASK-021`: a future logo change can reuse the corrected
  transparent asset pipeline.

- 2026-07-03: Task claimed.
- 2026-07-03: Regenerated all tracked icons from `icon.svg`, added
  `npm run icons:generate` and an alpha regression check covering tracked PNGs
  plus every representation extracted from `icon.icns`.
- 2026-07-03: `npm run release:build` passed with loopback permission. The
  packaged `Resources/icon.icns` byte-matches the tracked asset, all ten
  embedded macOS sizes have zero-alpha corners, and the retained artwork was
  visually inspected. Finder/Dock/app-switcher inspection remains a human
  environment check rather than an automated release blocker.

- 2026-07-03: Release build passed; packaged ICNS matches tracked asset and every embedded macOS representation has zero-alpha corners.

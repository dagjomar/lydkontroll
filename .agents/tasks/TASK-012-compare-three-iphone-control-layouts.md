---
id: TASK-012
title: Compare three iPhone control layouts
status: done
priority: P0
type: spike
owner: codex-2026-06-18-mobile-layout-spike
depends_on: [TASK-008, TASK-011]
plan: none
updated: 2026-06-18
---

# TASK-012: Compare three iPhone control layouts

## Context

Real iPhone testing showed that the fixed playback region protects cue
coordinates but consumes scarce vertical space and clips multiple active
sounds. Before committing to another production layout, compare three concrete
ways to prioritize cues, playback state, connection state, and global controls.

## Outcome

The same mobile preview can switch between three functional layouts so the
operator can test each one with real cue and playback interactions:

1. expanding now-playing overlay above the main screen;
2. cue-first screen with master/global controls below the cues and now-playing
   inside that global card;
3. two-tab screen separating cues/playback from connection/settings.

## Scope

- Reuse one authoritative snapshot and command path across all variants.
- Select variants with a stable query parameter and an in-page switcher in
  preview or explicit prototype sessions.
- Preserve touch-safe emergency and playback controls.
- Exercise zero, one, and multiple active playback rows at 390x844.
- Capture comparison notes without choosing the final production layout.

## Non-goals

- Do not change Rust commands, playback semantics, transport, or persistence.
- Do not make one prototype the production default in this spike.
- Do not implement future preflight/settings content beyond representative
  placeholders needed to assess the tab layout.

## Acceptance Criteria

- [x] Three named, functional layouts are selectable without checking out
      branches.
- [x] Each layout can trigger cues and stop/fade active playback.
- [x] The overlay variant grows for multiple sounds without blocking cue
      interaction below it.
- [x] The controls-below variant prioritizes cues above global controls.
- [x] The tab variant separates cue operation from connection/settings while
      keeping now-playing visible on the cue tab.
- [x] Automated tests verify layout selection and shared command behavior.
- [x] All variants are rendered and reviewed at 390x844 with multiple active
      sounds.

## Validation

```text
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- 2026-06-18: User requested three alternatives before choosing a final mobile
  layout. One-checkout variants are preferred over branches because they share
  runtime state and allow instant comparison.
- 2026-06-18: Scope is specific enough for a disposable implementation spike;
  no separate architecture plan is required.
- 2026-06-18: Refined into three selectable functional prototypes; dependencies
  are complete.
- 2026-06-18: Task claimed.
- 2026-06-18: Added query-selectable prototypes that work with both mock preview
  state and the real Safari/WebSocket control URL. The no-query layout remains
  the stable production fallback.
- 2026-06-18: At 390x844 with three active sounds:
  - overlay used a 230-pixel floating card ending above the first cue and left
    the cue enabled;
  - cue-first placed both cue buttons before a 396-pixel global card, requiring
    scrolling to reach all master/global content;
  - tabs kept three active rows plus both cues in an approximately one-screen
    cue view and isolated connection/master/preflight content on `Status`.
- 2026-06-18: `npm test -- --run` and `npm run build` passed before final lint.
- 2026-06-18: Confirmed the overlay cue remains enabled and can add a fourth
  playback while the floating card is open.
- 2026-06-18: `npm run lint`, Ralph validation, and diff hygiene passed.
- 2026-06-18: Three functional query-selectable layouts, tests, guide, and
  390x844 multi-playback review completed.

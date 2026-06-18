---
id: TASK-013
title: Adopt the selected iPhone control layout
status: done
priority: P0
type: feature
owner: codex-2026-06-18-adopt-overlay
depends_on: [TASK-012]
plan: none
updated: 2026-06-18
---

# TASK-013: Adopt the selected iPhone control layout

## Context

Three functional layouts exist after real iPhone feedback. A product choice is
required before one can replace the stable fixed-region layout.

## Outcome

The selected prototype becomes the single production iPhone layout, prototype
switching code is removed, and the chosen behavior has regression and rendered
coverage.

## Scope

- Record the selected prototype and any requested combination or adjustment.
- Promote the chosen structure to the default no-query mobile view.
- Remove discarded prototype-only UI and documentation.
- Preserve shared WebSocket commands, reconnect behavior, touch targets, and
  cue safety.

## Non-goals

- Do not choose a layout without operator testing.
- Do not expand into event preflight implementation beyond layout space needed
  by the selected option.

## Acceptance Criteria

- [x] The operator-selected layout is recorded.
- [x] The no-query iPhone URL renders only the selected production layout.
- [x] Prototype switcher and discarded layouts are removed.
- [x] Playback, reconnect, and touch-safety tests pass.
- [x] A final 390x844 real-device-oriented check passes.

## Validation

```text
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- 2026-06-18: Operator selected the expanding overlay layout after comparing
  all three prototypes.
- 2026-06-18: Implementation is fully specified: make overlay the sole
  no-query mobile layout, remove the switcher and discarded alternatives, and
  retain the expanding multi-playback card with interactive cues beneath it.
- 2026-06-18: Accepted ADR-011 and removed query layout selection, the
  comparison switcher, cue-first/tab implementations, and the prototype guide.
- 2026-06-18: At 390x844, nine active playback rows used a 321-pixel overlay
  with an internally scrolling 249-pixel list. The first cue remained
  physically uncovered, enabled, and successfully triggered the ninth row.
  Emergency and playback actions measured 44 pixels.
- 2026-06-18: `npm test -- --run` and `npm run build` passed before final lint.
- 2026-06-18: `npm run lint`, Ralph validation, and diff hygiene passed.
- 2026-06-18: Overlay promoted to the sole production mobile layout;
  alternatives were removed and crowded-state validation passed.

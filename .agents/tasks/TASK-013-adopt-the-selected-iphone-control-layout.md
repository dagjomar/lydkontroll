---
id: TASK-013
title: Adopt the selected iPhone control layout
status: idea
priority: P0
type: feature
owner: unassigned
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

- [ ] The operator-selected layout is recorded.
- [ ] The no-query iPhone URL renders only the selected production layout.
- [ ] Prototype switcher and discarded layouts are removed.
- [ ] Playback, reconnect, and touch-safety tests pass.
- [ ] A final 390x844 real-device-oriented check passes.

## Validation

```text
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- Waiting for the operator to compare `overlay`, `controls`, and `tabs` using
  `MOBILE_LAYOUT_PROTOTYPES.md`.

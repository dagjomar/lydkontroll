---
id: TASK-017
title: Make iPhone master volume drag smoothly
status: done
priority: P0
type: bug
owner: codex-2026-06-19-mobile-volume
depends_on: [TASK-008]
plan: none
updated: 2026-06-19
---

# TASK-017: Make iPhone master volume drag smoothly

## Context

The iPhone master-volume slider sends a command for its first changed value,
then becomes disabled while awaiting acknowledgement. Safari consequently
loses the active drag gesture, so the operator can click the track but cannot
drag the thumb smoothly.

## Outcome

The iPhone master-volume thumb follows the operator's finger continuously while
authoritative volume updates are sent to and confirmed by the Mac.

## Scope

- Keep a local display value while the slider is being manipulated.
- Serialize or coalesce volume commands so rapid input does not create
  conflicting in-flight updates.
- Reconcile the local value with authoritative snapshots after interaction.
- Preserve connection safety and the existing revision/acknowledgement model.

## Non-goals

- Change Rust playback semantics or revision behavior.
- Change the desktop master-volume control.
- Redesign the mobile control layout.

## Acceptance Criteria

- [x] The slider remains enabled and its thumb advances through multiple input
      values while a prior volume acknowledgement is pending.
- [x] Rapid input is reduced to ordered, bounded command traffic and finishes
      at the latest requested value.
- [x] External authoritative volume changes appear when no local interaction
      is active.
- [x] Existing duplicate-action and reconnect safeguards continue to pass.

## Validation

```text
npm test -- --run
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- 2026-06-19: Added from an iPhone Safari report that clicking changes volume
  but dragging is interrupted after the first revision.
- 2026-06-19: Added an optimistic local slider projection, one-in-flight/latest-
  queued command coalescing, and authoritative snapshot reconciliation. The
  slider no longer participates in the ordinary pending-command disable lock.
- 2026-06-19: Added regressions for rapid input before acknowledgement, latest-
  value delivery, no post-ack snapback, and idle authoritative updates.
- 2026-06-19: Passed all frontend tests, production build, lint, generated
  binding checks, rustfmt, Clippy, Ralph metadata preparation, and a 390x844
  responsive render check. A physical iPhone Safari drag remains the final
  gesture verification.

- 2026-06-19: Task claimed.

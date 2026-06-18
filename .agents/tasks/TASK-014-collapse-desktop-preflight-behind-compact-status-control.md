---
id: TASK-014
title: Collapse desktop preflight behind compact status control
status: done
priority: P1
type: feature
owner: unassigned
depends_on: []
plan: none
updated: 2026-06-18
---

# TASK-014: Collapse desktop preflight behind compact status control

## Context

The full desktop preflight panel permanently consumes a large part of the
operator workspace after the release rehearsal is complete. The checks and
recovery tools must remain available without displacing everyday cue controls.

## Outcome

The desktop opens with a compact system-status control. It communicates whether
there is a blocking readiness problem and expands the existing preflight panel
on demand.

## Scope

- Collapse preflight details by default.
- Keep a compact, accessible status control visible.
- Show a ready state when there are no blocking checks and a blocked state when
  any preflight check is unavailable.
- Preserve refresh, QR, diagnostics, and test-play behavior when expanded.

## Non-goals

- Changing Rust preflight facts or release criteria.
- Changing the iPhone interface.

## Acceptance Criteria

- [x] Preflight details are hidden by default and toggled by the compact status
      control.
- [x] The compact control accurately exposes ready and blocked states.
- [x] Existing refresh, QR, diagnostics, and test-play behavior remains
      available when expanded.
- [x] Frontend tests and the production build pass.

## Validation

```bash
npm test
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- 2026-06-18: User requested moving the rehearsal/preflight area behind a
  settings-style control, preferably represented by a small green status light.
- 2026-06-18: Chosen behavior is a collapsed-by-default status button. Green
  means no blocking `unavailable` facts; red means at least one blocker.

- 2026-06-18: Task claimed.

- 2026-06-18: Implemented a collapsed-by-default system-status pill with an
  accessible expand/collapse state. The indicator is green when no preflight
  fact is blocked and red when an unavailable fact requires action.
- 2026-06-18: Preserved the complete preflight panel, including refresh,
  diagnostics, mobile QR access, and safe three-second test playback. Added
  regression coverage for collapsed ready/blocked states and expansion.

- 2026-06-18: Compact status control implemented and validated.

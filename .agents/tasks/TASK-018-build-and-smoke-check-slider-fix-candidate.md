---
id: TASK-018
title: Build and smoke-check the slider-fix candidate
status: in-progress
priority: P0
type: chore
owner: codex-2026-06-19-slider-release
depends_on: [TASK-017]
plan: none
updated: 2026-06-19
---

# TASK-018: Build and smoke-check the slider-fix candidate

## Context

`TASK-017` changes the mobile controls after the compact-status candidate was
built. The event bundle must be rebuilt and its identity recorded before the
user can verify the corrected slider on iPhone Safari.

## Outcome

A validated Apple Silicon application bundle contains the smooth iPhone
master-volume behavior and is ready for one short desktop, audio, and phone
smoke check.

## Scope

- Run the complete release build gate.
- Record the replacement bundle identity.
- Verify the mobile preview at the supported viewport.
- Leave exact physical-device checks for the user.

## Non-goals

- Repeat the full 60-minute rehearsal unless smoke testing exposes a broader
  regression.

## Acceptance Criteria

- [ ] `npm run release:build` passes.
- [ ] The arm64 bundle identity and checksum are recorded.
- [ ] The exact native Mac and iPhone Safari checks are handed off.

## Validation

```text
npm run release:build
python3 scripts/ralph.py check
git diff --check
```

## Notes

- The previous candidate does not contain `TASK-017` and must not be promoted.

- 2026-06-19: Task claimed.

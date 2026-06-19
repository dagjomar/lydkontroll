---
id: TASK-018
title: Build and smoke-check the slider-fix candidate
status: done
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

- [x] `npm run release:build` passes.
- [x] The arm64 bundle identity and checksum are recorded.
- [x] The exact native Mac and iPhone Safari checks are handed off.

## Validation

```text
npm run release:build
python3 scripts/ralph.py check
git diff --check
```

## Notes

- The previous candidate does not contain `TASK-017` and must not be promoted.
- 2026-06-19: The restricted first release run passed frontend checks but was
  denied temporary loopback binds in five network tests. The complete gate
  passed with local bind permission: 14 frontend tests, 63 Rust tests, lint,
  bindings, formatting, Clippy, release-tree safety, and arm64 packaging.
- 2026-06-19: Recorded commit
  `20871d5d2b8928af77d3ae3246eee4f4ab4be23c` and executable SHA-256
  `4b8656e4a9b0ab279ae2dec4d5c51b2de58edaf8865969929dbbba85dfc7ea5b`.

- 2026-06-19: Task claimed.

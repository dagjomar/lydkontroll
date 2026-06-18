---
id: TASK-015
title: Build and verify the compact-status release candidate
status: done
priority: P0
type: chore
owner: unassigned
depends_on: [TASK-014]
plan: none
updated: 2026-06-18
---

# TASK-015: Build and verify the compact-status release candidate

## Context

`TASK-014` changed the desktop operator UI after the previously rehearsed event
candidate was locked. ADR-013 requires a changed candidate to be rebuilt,
checked, and identified before it can replace the known-good build.

## Outcome

A new Apple Silicon application bundle passes the automated release gate. Its
commit and executable checksum are recorded, and the compact status control is
checked against the focused desktop regression criteria.

## Scope

- Run the complete release build from the committed `TASK-014` implementation.
- Verify collapsed, expanded, QR, refresh, and safe test-play behavior using
  automated coverage and any available local runtime checks.
- Record the candidate identity without overwriting the prior rehearsal record.

## Non-goals

- Repeating the full 60-minute iPhone/Tailscale/analog-output rehearsal.
- Changing product behavior or release criteria.

## Acceptance Criteria

- [x] `npm run release:build` passes and produces an arm64 macOS bundle.
- [x] Focused desktop regression coverage passes for collapsed/expanded state,
      QR access, refresh, and the three-second test-play sequence.
- [x] The candidate commit, app version, path, and executable SHA-256 are
      recorded while the previous rehearsed candidate remains identifiable.
- [x] Ralph state, progress, and handoff records agree on the next release
      action.

## Validation

```text
npm run release:build
npm test -- --run src/components/Shell.test.tsx
file src-tauri/target/release/bundle/macos/Lydkontroll.app/Contents/MacOS/lydkontroll
shasum -a 256 src-tauri/target/release/bundle/macos/Lydkontroll.app/Contents/MacOS/lydkontroll
python3 scripts/ralph.py check
python3 scripts/ralph.py next
git diff --check
```

## Notes

- 2026-06-18: Created from the exact next action in `.agents/HANDOFF.md` after
  Ralph reported no actionable task. The work is a release-validation slice,
  not a product decision.
- 2026-06-18: The first release run was stopped only by sandbox denial of
  temporary loopback listeners. Re-running with local bind permission passed
  every frontend and Rust gate and produced the arm64 `.app`.
- 2026-06-18: Recorded source commit `ad35ac35d0b6d241458f745cb5e6e99becbb7cb6`
  and executable SHA-256
  `01679888c466b2231ecb2e85d995d7320da4e31534539644aaa5ebab0f236055`
  in `.agents/CANDIDATE.md`.
- 2026-06-18: Extended the focused desktop regression test to exercise the
  expanded panel's manual refresh action. Native launch and audible test play
  remain a short operator check in `TASK-016`.

- 2026-06-18: Task claimed.

- 2026-06-18: Release build, focused regression coverage, and candidate identity recorded.

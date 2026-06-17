---
id: TASK-002
title: Scaffold a runnable and tested Tauri application shell
status: idea
priority: P0
type: feature
owner: unassigned
depends_on: [TASK-001]
plan: none
updated: 2026-06-18
---

# TASK-002: Scaffold a runnable and tested Tauri application shell

## Context

All product slices need a reproducible Tauri, React, TypeScript, and Rust
foundation.

## Outcome

The repository launches a minimal desktop app and exposes consistent build,
lint, frontend-test, and Rust-test commands.

## Scope

- Scaffold Tauri 2 with React and TypeScript using the decisions from TASK-001.
- Establish the intended source and test directories.
- Add formatting, linting, and baseline frontend/Rust tests.
- Document exact local development commands.

## Non-goals

- Audio playback, persistence, networking, or production UI.

## Acceptance Criteria

- [ ] `npm run tauri dev` launches the shell.
- [ ] `npm run build`, `npm test`, and `npm run lint` pass.
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] `AGENTS.md` commands match the actual scaffold.
- [ ] No generated bundles or local application data are tracked.

## Validation

```text
npm run build
npm test
npm run lint
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Move this task to `ready` after TASK-001 is done and its plan confirms the
scaffold choices.

---
id: TASK-002
title: Scaffold a runnable and tested Tauri application shell
status: done
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
- Use Vite, strict TypeScript, npm, and committed JavaScript/Rust lock files.
- Establish the planned frontend and Rust module boundaries without adding
  persistence, audio, or networking behavior.
- Add ESLint, Prettier, rustfmt/clippy checks, Vitest, React Testing Library,
  a Tauri mock-runtime smoke test, and a ts-rs generated-type smoke contract.
- Document exact local development commands.

## Non-goals

- Audio playback, persistence, networking, or production UI.

## Acceptance Criteria

- [x] `npm run tauri dev` launches the shell.
- [x] `npm run build`, `npm test`, and `npm run lint` pass.
- [x] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [x] The Rust crate is a testable library with a minimal executable entry and
      the domain/application/ports/adapters boundaries from TASK-001.
- [x] Generated TypeScript bindings have one documented, deterministic output
      location and are checked for drift.
- [x] `AGENTS.md` commands match the actual scaffold.
- [x] No generated bundles or local application data are tracked.

## Validation

```text
npm run build
npm test
npm run lint
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
```

## Notes

Implement the five TASK-002 slices in the accepted TASK-001 plan. Do not pull
audio, persistence, Axum, or Tailscale into this scaffold.

- 2026-06-18: Foundation decisions accepted in TASK-001; implement the five scaffold slices from its plan.

- 2026-06-18: Task claimed.

- 2026-06-18: Completed the Tauri 2/Vite/React/TypeScript/Rust shell with
  committed npm and Cargo lock files, strict frontend build, Vitest/RTL
  behavior coverage, Tauri mock-runtime coverage, Clippy/rustfmt/Prettier/
  ESLint checks, and a deterministic ts-rs contract export.
- 2026-06-18: Validated `npm run build`, `npm test`, `npm run lint`,
  `cargo test --manifest-path src-tauri/Cargo.toml`, and a native
  `npm run tauri dev` launch. The user visually confirmed the running macOS
  window from a screenshot.

- 2026-06-18: Acceptance criteria complete; automated validation and native macOS launch passed.

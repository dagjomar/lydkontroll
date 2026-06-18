---
id: TASK-002
title: Scaffold a runnable and tested Tauri application shell
status: ready
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

- [ ] `npm run tauri dev` launches the shell.
- [ ] `npm run build`, `npm test`, and `npm run lint` pass.
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] The Rust crate is a testable library with a minimal executable entry and
      the domain/application/ports/adapters boundaries from TASK-001.
- [ ] Generated TypeScript bindings have one documented, deterministic output
      location and are checked for drift.
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

Implement the five TASK-002 slices in the accepted TASK-001 plan. Do not pull
audio, persistence, Axum, or Tailscale into this scaffold.

- 2026-06-18: Foundation decisions accepted in TASK-001; implement the five scaffold slices from its plan.

# Repository Guidelines

## Agent Startup Protocol

This repository uses the workflow in `.agents/` as its durable project memory.
At the beginning of every session:

1. Read `PLAN.md`, `.agents/PROJECT.md`, and `.agents/STATE.md`.
2. Run `python3 scripts/ralph.py status` and `python3 scripts/ralph.py next`.
3. Read the selected task file and any linked plan or decision records.
4. Claim planning work with
   `python3 scripts/ralph.py claim TASK-NNN --owner "<agent/session>"`.
5. Before implementation, start a `ready` task with
   `python3 scripts/ralph.py start TASK-NNN --owner "<agent/session>"`.

Do not begin a task whose dependencies are incomplete. If a task is marked
`needs-planning`, create its linked plan from `.agents/plans/TEMPLATE.md`, resolve
the open questions, then move it to `ready` for implementation or `done` if the
task's entire outcome was the plan itself.

Before ending a working session:

1. Update the task's acceptance criteria, notes, and status.
2. Update `.agents/STATE.md` if the current focus, blocker, or project-level
   understanding changed.
3. Append a concise entry to `.agents/PROGRESS.md`.
4. Replace `.agents/HANDOFF.md` with the exact next action and useful context.
5. Run `python3 scripts/ralph.py check`.

Task files are the source of truth for backlog status. Do not maintain a
separate hand-written backlog table.

## Project Structure & Module Organization

`PLAN.md` is the product source of truth. The implementation is a Tauri 2
desktop application with a React/TypeScript frontend and a Rust backend.

When scaffolding the project, keep this layout:

- `src/`: shared React UI, state, and mobile/desktop views.
- `src/components/`: reusable controls such as cue buttons and status panels.
- `src-tauri/src/`: Rust commands, audio engine, persistence, and HTTP/WebSocket server.
- `src-tauri/tests/`: Rust integration tests.
- `tests/`: frontend and end-to-end tests.
- `public/`: static web assets only; do not place user audio files here.

Keep playback and networking logic out of React components. Shared command and state types should have one clearly documented source of truth.

## Build, Test, and Development Commands

- `npm install`: install JavaScript dependencies.
- `npm run tauri dev`: run the desktop app with hot reload.
- `npm run build`: type-check and build the frontend.
- `npm run tauri build`: create the macOS application bundle.
- `npm test`: run frontend tests.
- `cargo test --manifest-path src-tauri/Cargo.toml`: run Rust tests.
- `npm run lint`: run ESLint, Prettier checks, generated-binding drift checks,
  `cargo fmt --check`, and Clippy with warnings denied.
- `npm run format`: format frontend, configuration, Markdown, and Rust files.
- `npm run bindings:generate`: regenerate committed TypeScript contracts from
  Rust.
- `npm run bindings:check`: fail if committed generated contracts are stale.

## Coding Style & Naming Conventions

Use two-space indentation in TypeScript, CSS, JSON, and Markdown. Let `rustfmt` format Rust code. Use `PascalCase` for React components and TypeScript types, `camelCase` for functions and variables, and `snake_case` for Rust modules and functions. Name tests after observable behavior, for example `exclusive_cue_stops_active_audio`.

Run the configured formatter, linter, and `cargo fmt --check` before submitting changes. Avoid duplicating protocol constants between Rust and TypeScript.

## Testing Guidelines

Test cue persistence, MP3/WAV playback commands, overlap/exclusive behavior, fades, WebSocket acknowledgements, reconnects, and missing-file handling. Add a regression test with every bug fix. Hardware-dependent audio behavior must also be checked manually using an iPhone over Tailscale and the Mac’s analog output.

## Commit & Pull Request Guidelines

No Git history exists yet. Use Conventional Commits, such as `feat: add cue scene editor` or `fix: preserve playback during reconnect`. Keep commits focused.

Pull requests should summarize behavior changes, list tests performed, link relevant issues, and include screenshots for UI changes. Call out changes to networking, Tailscale binding, file storage, or audio behavior explicitly.

## Security & Configuration

Bind the control server only to the active Tailscale address. Never commit audio files, credentials, signing keys, generated bundles, or local app data.

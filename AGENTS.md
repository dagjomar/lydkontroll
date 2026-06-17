# Repository Guidelines

## Project Structure & Module Organization

This repository is currently in the planning stage; `PLAN.md` is the product and architecture source of truth. The intended implementation is a Tauri 2 desktop application with a React/TypeScript frontend and a Rust backend.

When scaffolding the project, keep this layout:

- `src/`: shared React UI, state, and mobile/desktop views.
- `src/components/`: reusable controls such as cue buttons and status panels.
- `src-tauri/src/`: Rust commands, audio engine, persistence, and HTTP/WebSocket server.
- `src-tauri/tests/`: Rust integration tests.
- `tests/`: frontend and end-to-end tests.
- `public/`: static web assets only; do not place user audio files here.

Keep playback and networking logic out of React components. Shared command and state types should have one clearly documented source of truth.

## Build, Test, and Development Commands

After the Tauri project is scaffolded, expose these scripts through `package.json`:

- `npm install`: install JavaScript dependencies.
- `npm run tauri dev`: run the desktop app with hot reload.
- `npm run build`: type-check and build the frontend.
- `npm run tauri build`: create the macOS application bundle.
- `npm test`: run frontend tests.
- `cargo test --manifest-path src-tauri/Cargo.toml`: run Rust tests.
- `npm run lint`: check TypeScript and React code.

Update this section if the actual scaffold uses different commands.

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

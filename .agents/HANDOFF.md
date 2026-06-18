# Latest Handoff

Updated: 2026-06-18

## What Just Happened

`TASK-001` was completed. The accepted foundation plan and ADR-001 through
ADR-006 now define the audio, persistence, command/state, server, Tailscale, and
test architecture. Downstream foundation tasks were refined, and `TASK-002` is
ready.

## Exact Next Action

Start `TASK-002`, then scaffold the five slices listed in the TASK-001 plan.

```text
python3 scripts/ralph.py start TASK-002 --owner "<agent/session>"
```

## Important Context

- Keep `src-tauri/src/lib.rs` as the composition root and `main.rs` minimal.
- Establish domain/application/ports/adapters boundaries without implementing
  audio, persistence, Axum, or Tailscale behavior yet.
- Use Vite, strict TypeScript, npm lock files, Vitest/React Testing Library,
  Rust tests with a Tauri mock-runtime smoke test, and a deterministic ts-rs
  generated-binding location.
- Read ADR-001 through ADR-006 and the TASK-002 slice in
  `.agents/plans/TASK-001-foundation-architecture.md` before scaffolding.

## Validation to Run

```text
npm run build
npm test
npm run lint
cargo test --manifest-path src-tauri/Cargo.toml
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```

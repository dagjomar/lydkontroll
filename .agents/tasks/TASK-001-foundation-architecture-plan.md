---
id: TASK-001
title: Decide foundation architecture and implementation slices
status: needs-planning
priority: P0
type: research
owner: unassigned
depends_on: []
plan: .agents/plans/TASK-001-foundation-architecture.md
updated: 2026-06-18
---

# TASK-001: Decide foundation architecture and implementation slices

## Context

The product plan names Tauri, React, TypeScript, and Rust, but leaves several
choices that will shape every later task. Making them implicitly in the scaffold
would be expensive to reverse.

## Outcome

An implementation-ready plan records the core libraries, module boundaries,
protocol/type ownership, state model, and test approach for the first vertical
slice.

## Scope

- Choose and justify audio playback/decoding and fade strategy.
- Choose persistence format and application-data layout.
- Choose HTTP/WebSocket server integration compatible with Tauri.
- Define the authoritative command/state model and TypeScript type-generation
  approach.
- Define how the active Tailscale address will be discovered and validated.
- Select frontend, Rust, and end-to-end test tools.
- Break `TASK-002` and the next foundation work into safe implementation slices.

## Non-goals

- Scaffold or implement production code.
- Finalize visual design.
- Optimize for platforms beyond Apple Silicon macOS and iPhone Safari.

## Acceptance Criteria

- [ ] `.agents/plans/TASK-001-foundation-architecture.md` resolves the scoped
      choices with evidence and tradeoffs.
- [ ] Durable choices are recorded in `.agents/DECISIONS.md`.
- [ ] The plan includes module boundaries, data flow, failure modes, and tests.
- [ ] Downstream tasks are split or refined where the findings require it.
- [ ] This task is moved to `done`, making `TASK-002` actionable.

## Validation

```text
python3 scripts/ralph.py check
python3 scripts/ralph.py next
```

## Notes

Start by creating the linked plan from `.agents/plans/TEMPLATE.md`. Prefer
primary documentation and small local experiments for risky library claims.

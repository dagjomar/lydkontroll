# Plan: TASK-001 — Foundation architecture

Status: draft
Updated: 2026-06-18

## Problem

The scaffold needs explicit choices for audio, persistence, networking, shared
types, Tailscale discovery, and testing. These choices cross most later tasks
and are costly to reverse.

## Current Evidence

- `PLAN.md` defines product behavior and supported platforms.
- `.agents/PROJECT.md` records the stable invariants and boundaries.
- No application code or dependency constraints exist yet.

## Open Questions

- Which Rust audio stack gives reliable MP3/WAV mixing, stopping, and fades on
  Apple Silicon macOS while remaining testable?
- Which persistence representation and migration strategy are sufficient for
  event reliability?
- How should a Rust HTTP/WebSocket server coexist with Tauri's async runtime?
- Which Rust definition generates or validates frontend command/state types?
- How can the process identify the active Tailscale IPv4/IPv6 address without
  ever falling back to a public bind?
- Which frontend, Rust, browser, and end-to-end test tools should be established
  in the first scaffold?

## Options Considered

To be completed from primary documentation and focused local experiments.

## Decision

Not decided. Record accepted choices in `../DECISIONS.md`.

## Implementation Slices

1. Research and compare the core library options.
2. Sketch module boundaries and the command/state data flow.
3. Define test seams and the minimum scaffold.
4. Record accepted decisions and refine downstream tasks.
5. Mark TASK-001 done and TASK-002 ready.

## Test Strategy

- Automated: identify unit, integration, contract, component, and end-to-end
  layers plus their commands.
- Manual: identify the smallest early macOS audio and Tailscale experiments.
- Failure modes: include decoder errors, output loss, missing files, bind
  refusal, duplicate commands, stale clients, and reconnects.

## Rollback or Recovery

Prefer replaceable adapters around audio, storage, and transport libraries so a
failed choice does not rewrite application behavior.

## Ready Checklist

- [ ] Open questions are resolved or explicitly deferred.
- [ ] Decisions and tradeoffs are recorded.
- [ ] Slices are small and dependency ordered.
- [ ] Tests cover the important failure modes.
- [ ] The parent task can move to `ready`.

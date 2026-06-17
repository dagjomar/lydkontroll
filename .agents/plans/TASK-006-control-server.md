# Plan: TASK-006 — Tailscale control server

Status: placeholder
Updated: 2026-06-18

## Problem

The server is intentionally protected by its bind boundary, so interface
discovery, refusal behavior, reconnects, and protocol failure modes need explicit
design.

## Open Questions

- How is the active Tailscale address discovered and refreshed?
- What exact conditions must refuse server startup?
- How are snapshots, acknowledgements, retries, and stale clients handled?

## Decision

Deferred until TASK-001 and the command/state core are complete.

## Implementation Slices

1. Refine this plan when the transport library and protocol types are known.

## Test Strategy

- Automated: interface-selection, bind-refusal, protocol, and reconnect tests.
- Manual: iPhone over Tailscale with venue-LAN independence simulated.

## Ready Checklist

- [ ] Open questions are resolved or explicitly deferred.
- [ ] Decisions and tradeoffs are recorded.
- [ ] Slices are small and dependency ordered.
- [ ] Tests cover the important failure modes.
- [ ] The parent task can move to `ready`.

# Plan: TASK-010 — Release rehearsal and recovery

Status: placeholder
Updated: 2026-06-18

## Problem

Event readiness needs a repeatable rehearsal, explicit pass/fail gates, and
recovery procedures rather than an informal final check.

## Open Questions

- What exact hardware, network transitions, cue set, and run duration represent
  the event?
- What failures block release, and what fallback steps can the operator perform?
- How will rehearsal evidence and the final build identity be recorded?

## Decision

Deferred until the working vertical slices expose concrete operational risks.

## Implementation Slices

1. Refine this plan after preflight and mobile control are functional.

## Test Strategy

- Automated: run the complete repository check suite.
- Manual: iPhone on mobile data, Mac on Wi-Fi, Tailscale, analog output, and
  deliberate disconnect/reconnect scenarios.

## Ready Checklist

- [ ] Open questions are resolved or explicitly deferred.
- [ ] Decisions and tradeoffs are recorded.
- [ ] Slices are small and dependency ordered.
- [ ] Tests cover the important failure modes.
- [ ] The parent task can move to `ready`.

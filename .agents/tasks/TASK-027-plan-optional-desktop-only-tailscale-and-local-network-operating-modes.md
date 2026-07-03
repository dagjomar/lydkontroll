---
id: TASK-027
title: Plan optional desktop-only, Tailscale, and local-network operating modes
status: needs-planning
priority: P3
type: research
owner: codex-2026-07-04-feedback-intake
depends_on: []
plan: .agents/plans/TASK-027-operating-modes.md
updated: 2026-07-04
---

# TASK-027: Plan optional desktop-only, Tailscale, and local-network operating modes

## Context

The current mobile-control path intentionally requires Tailscale, but that
prerequisite is unfamiliar to many prospective operators. The product already
supports reliable desktop-only operation without a phone or network, and a
phone could technically control the Mac on the same local network if a future
mode safely allowed it. Same-LAN control is less resilient when the phone
leaves Wi-Fi coverage, so it should be optional and clearly distinguished from
the recommended Tailscale path rather than silently becoming a fallback.

Original feedback: most people may not know what Tailscale is; desktop-only use
can make sense without any network, and same-network mobile use could be useful
even though it is not recommended because control can disappear outside Wi-Fi
coverage. The app structure, settings, marketing, and instructions should
eventually account for all three scenarios, while the first public version
should require minimal extra work and remain mostly as-is.

## Outcome

An accepted product and architecture plan defines the supported operating
modes, their security/reliability promises, how an operator deliberately
selects and understands them, and the dependency-ordered implementation and
documentation work for a later release without expanding the minimal first
public release.

## Scope

- Inventory current desktop-only and Tailscale behavior and terminology.
- Define explicit modes for desktop only, recommended Tailscale mobile control,
  and optional same-LAN mobile control.
- Threat-model LAN exposure and decide authentication, bind-address, discovery,
  consent, and fail-closed behavior before allowing a non-Tailscale listener.
- Plan settings, preflight/status presentation, onboarding, instructions, and
  marketing language for the three modes.
- Split accepted follow-up work into independently schedulable implementation
  and documentation tasks with dependencies and validation.

## Non-goals

- Implementing a LAN listener, mode selector, authentication, or UI changes in
  this research task.
- Delaying `TASK-024`/`TASK-025` or broadening the first public release beyond
  its current Tailscale-first behavior and existing desktop fallback.
- Automatic insecure LAN fallback when Tailscale is unavailable.
- Supporting Android, Windows, cloud relay, internet port forwarding, or venue
  networks that block device-to-device traffic.
- Reworking language catalogs owned by `TASK-026`; this task defines concepts
  and required copy, while localization owns translated strings.

## Acceptance Criteria

- [ ] Current behavior is documented accurately: desktop playback/editing does
      not require Tailscale or a network, while mobile control in v1 does.
- [ ] Each proposed mode has an explicit user promise, prerequisites, setup
      flow, failure behavior, and recommendation level.
- [ ] Same-LAN control has an accepted threat model and access-control decision;
      no plan relies on binding broadly without deliberate operator consent.
- [ ] Settings and preflight concepts make the active mode and mobile URL/status
      understandable without requiring prior Tailscale knowledge.
- [ ] README/onboarding/marketing guidance explains Tailscale in plain language,
      presents desktop-only use honestly, and labels LAN range limitations.
- [ ] The minimal first public release remains unchanged except for accurate
      documentation already required by `TASK-024`.
- [ ] Accepted follow-up tasks define automated and manual validation, including
      loss of Wi-Fi, loss of Tailscale, hostile/untrusted LAN assumptions, app
      restart, and continued local playback.

## Validation

```text
manual: review accepted plan against PLAN.md, TASK-006, TASK-007, TASK-009, TASK-024, and TASK-026
python3 scripts/ralph.py check
```

## Notes

- P3 because this is valuable product expansion but explicitly must not delay
  the minimal first public release.
- No dependency is required to research and decide the model. Any resulting
  implementation tasks should follow public-release work unless reprioritized.
- Needs planning because exposing a server on an ordinary LAN changes the
  current Tailscale access-control boundary and requires product/security
  choices before implementation can be scoped safely.
- Planning owner is only for this intake session; the next planning session may
  reclaim the task.

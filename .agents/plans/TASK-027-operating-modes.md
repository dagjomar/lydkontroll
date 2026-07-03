# Plan: TASK-027 — Optional operating modes

Status: draft
Updated: 2026-07-04

## Problem

The app's local-first architecture already permits desktop-only use, but the
public story centers on Tailscale mobile control. A same-LAN mobile option may
lower setup friction, yet it weakens reachability outside Wi-Fi and changes the
security boundary. The product needs a deliberate mode model before settings,
network binding, onboarding, or marketing encode contradictory promises.

## Current Evidence

- `PLAN.md` and `.agents/PROJECT.md` make Tailscale the only mobile-control
  access boundary for version one.
- `TASK-006` deliberately rejects LAN, wildcard, and loopback fallback.
- `TASK-007` established desktop editing and playback that remain operational
  without Tailscale or an iPhone.
- `TASK-009` preflight currently treats Tailscale/control-server readiness as a
  mobile-control fact while local playback remains available.
- `TASK-024` already requires public documentation of current prerequisites,
  limitations, and the Tailscale trust boundary.
- `TASK-026` owns complete English localization and translated marketing copy,
  not the underlying operating-mode decision.

## Open Questions

- Is desktop-only a selectable persistent mode, or simply the always-available
  baseline when mobile control is disabled or unavailable?
- Should Tailscale remain the recommended default once multiple modes exist,
  and should existing installations migrate to that mode automatically?
- What authentication or pairing is required before binding to a normal LAN,
  where other devices may be untrusted?
- Which interface/address may the LAN server bind, and how does it react to
  network changes, multiple interfaces, captive portals, or client isolation?
- Is mode selection global app configuration, per-event configuration, or an
  explicit temporary session choice?
- What terminology and onboarding explain Tailscale, LAN range limits, and
  desktop-only operation without overwhelming first-run users?
- Which documentation clarification belongs in the minimal public release, and
  which settings/marketing work waits for the later multi-mode release?

## Options Considered

### Preserve Tailscale-only mobile control and document desktop fallback

- Benefits: minimal work; retains the reviewed security boundary and reliable
  cellular/Wi-Fi transitions.
- Costs and risks: Tailscale remains a setup barrier; no same-LAN phone option.

### Add an explicit, authenticated same-LAN mode

- Benefits: lower-friction phone setup where Mac and phone share trusted Wi-Fi.
- Costs and risks: requires pairing/access control, interface lifecycle work,
  clearer warnings, and testing on varied LANs; phone control stops at Wi-Fi
  range and some networks prevent peer access.

### Add automatic LAN fallback

- Benefits: superficially seamless when Tailscale is absent.
- Costs and risks: can expose controls unexpectedly and makes active security
  posture unclear. This is excluded unless later evidence overturns the
  explicit-consent requirement.

## Decision

Pending. Keep the first public release as-is: desktop operation always remains
available, and mobile control remains Tailscale-only. Research should recommend
the later explicit mode and security model; it must not implement automatic LAN
fallback.

## Implementation Slices

1. Map current server, preflight, settings, docs, and first-run behavior to the
   three proposed operator scenarios.
2. Threat-model and prototype only the minimum needed to choose LAN access
   control, binding, discovery, and lifecycle semantics.
3. Decide mode persistence/defaults and plain-language product terminology.
4. Define settings, preflight, onboarding, instructions, and marketing changes.
5. Create dependency-ordered implementation and documentation tasks; leave the
   first public release scope unchanged.

## Test Strategy

- Automated: plan unit/integration coverage for address selection, explicit
  consent, authentication, restart persistence, mode switching, and rejection
  of unintended interfaces.
- Manual: verify each mode on a Mac; for mobile modes test iPhone Safari, Wi-Fi
  loss, Tailscale loss, network switching, QR/setup clarity, and continued
  desktop playback.
- Failure modes: accidental LAN exposure, stale QR/URL, unauthorized control,
  multiple-interface ambiguity, captive/client-isolated Wi-Fi, mode confusion,
  or network failure interrupting local audio.

## Rollback or Recovery

Keep the existing fail-closed Tailscale adapter and local desktop path intact.
Any later LAN work must be independently disableable so rollback restores the
current v1 behavior without migrating cue/audio data.

## Ready Checklist

- [ ] Open questions are resolved or explicitly deferred.
- [ ] Decisions and tradeoffs are recorded.
- [ ] Slices are small and dependency ordered.
- [ ] Tests cover the important failure modes.
- [ ] The parent task can move to `ready` or `done` if planning is its full
      outcome.

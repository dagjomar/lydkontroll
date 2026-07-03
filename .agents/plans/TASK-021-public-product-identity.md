# Plan: TASK-021 — Public product identity and customization

Status: draft
Updated: 2026-07-03

## Problem

The current identity is partly generic and partly tied to one couple. Public
positioning, naming, logo direction, and event-title customization are coupled
choices; downstream implementation should not encode contradictory answers.

## Current Evidence

- UI: `Marius + Wenche` plus `Lydkontroll` appears in desktop and mobile.
- Metadata/docs contain both wedding-specific and generic descriptions.
- The current source icon is a waveform heart.
- Original candidates: `The Wedding MC`, `The Toastmaster`, `Toastmastah`, and
  `SoundMastah`.
- The reliable local-first soundboard behavior works beyond weddings, while
  Tailscale and Apple-platform requirements remain real scope constraints.

## Open Questions

- Should positioning be wedding/toastmaster-specific or event-generic?
- Which public name and repository slug are distinctive, understandable, and
  free of obvious conflicts?
- Should operators customize an event title, and where is it edited/stored?
- Does the heart communicate the intended niche or unnecessarily narrow it?

## Options Considered

### Wedding-specific public brand

- Benefits: immediately legible to the original audience; heart fits naturally.
- Costs and risks: narrows reuse for conferences, theatre, and other events.

### Generic sound-control brand with configurable event title

- Benefits: broad reuse while retaining personal/event context in the UI.
- Costs and risks: adds a small persisted setting and needs a strong default.

### Fully generic fixed identity

- Benefits: smallest implementation and clearest product consistency.
- Costs and risks: loses useful event personalization and may feel sterile.

## Decision

Pending user product choice after a concise, conflict-aware shortlist is
prepared. Record the accepted result in `.agents/DECISIONS.md`.

## Implementation Slices

1. Prepare a shortlist and conflict/repository-slug check.
2. Select positioning, name, logo direction, and customization model.
3. Record exact strings/defaults and unblock `TASK-022` and `TASK-025`.

## Test Strategy

- Automated: none for the decision itself.
- Manual: verify all downstream identity fields have an unambiguous value.
- Failure modes: name conflict, mixed wedding/generic language, undefined empty
  title, or customization that shifts the mobile cue layout.

## Rollback or Recovery

Keep the current `Lydkontroll` identity until the decision is accepted; no
product files change during planning.

## Ready Checklist

- [ ] Open questions are resolved or explicitly deferred.
- [ ] Decisions and tradeoffs are recorded.
- [ ] Slices are small and dependency ordered.
- [ ] Tests cover the important failure modes.
- [ ] The parent task can move to `ready`.

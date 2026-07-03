# Plan: TASK-021 — Public product identity and customization

Status: accepted
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
- A 2026-07-03 web conflict check found `The Wedding MC` already used by
  multiple wedding-MC services, `ToastMaster` used by a current Norwegian App
  Store product and strongly associated with Toastmasters International, and
  `CueDeck`/`EventCue` used by directly adjacent live-event software.
- Exact searches for `Toastmastah` and `SoundMastah` found no obvious adjacent
  software products, but both spellings weaken immediate comprehension and
  professional trust. Exact `Lydkontroll` app/software searches found only
  descriptive uses of the Norwegian common noun, not an obvious competing
  product.

## Resolved Questions

- Positioning is event-generic but Norwegian-first.
- The public name is `Lydkontroll`; repository slug and package stem are
  `lydkontroll`.
- Operators can customize an event title stored with the cue library. It is
  edited on desktop and projected read-only on mobile.
- The heart is revised into a neutral rounded waveform mark.
- English and additional UI languages are deferred to a dedicated
  internationalization task. English is the first added language, Norwegian
  remains available, and language selection must not be mixed into branding.

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

### Accepted decision package: retain Lydkontroll, personalize the event

- Public name: `Lydkontroll`; repository slug and package stem: `lydkontroll`.
- Positioning (Norwegian): `En lokal lydtavle for trygg avspilling av lyd-cues
på arrangementer fra Mac, med iPhone-kontroll over Tailscale.`
- Marketing scope: launch as a Norwegian project. A later localization slice
  will add complete English UI/marketing copy and a language selector before
  positioning it broadly to international users.
- Event title: optional configurable text, default `Mitt arrangement`; trim
  whitespace and fall back to that default when empty. Show it in the existing
  eyebrow slot on desktop and mobile so cue coordinates remain stable.
- Identity: retain the waveform idea but revise the heart into a neutral rounded
  waveform mark. This keeps visual continuity without implying weddings only.
- Why: it is the least-conflicted, most honest name for the current Norwegian
  UI, preserves repository continuity, and lets weddings remain a first-class
  use case without excluding other events.

### Original candidate disposition

- `The Wedding MC`: reject; wedding-only positioning and active exact-name
  service use make it both narrow and crowded.
- `The Toastmaster`: reject; active `ToastMaster` app use plus the dominant
  Toastmasters association creates confusion, and the app controls audio rather
  than the full toastmaster role.
- `Toastmastah`: reject; distinctive spelling but still inherits the
  toastmaster ambiguity and reads less professionally.
- `SoundMastah`: viable alternate if an English playful brand is preferred, but
  reject in the recommendation because the spelling obscures purpose and does
  not match the product's calm, reliability-first character.

## Decision

Accepted by the owner on 2026-07-03: launch as the Norwegian-first,
event-generic `Lydkontroll`; support a configurable event title; revise the
heart into a neutral waveform; defer complete English and additional-language
support to a separate internationalization task.

## Implementation Slices

1. Prepare a shortlist and conflict/repository-slug check. (complete)
2. Select positioning, name, logo direction, and customization model. (complete)
3. Record exact strings/defaults and unblock `TASK-022` and `TASK-025`.
   (complete)

## Test Strategy

- Automated: none for the decision itself.
- Manual: verify all downstream identity fields have an unambiguous value.
- Failure modes: name conflict, mixed wedding/generic language, undefined empty
  title, or customization that shifts the mobile cue layout.

## Rollback or Recovery

Keep the current `Lydkontroll` identity until the decision is accepted; no
product files change during planning.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task is complete.

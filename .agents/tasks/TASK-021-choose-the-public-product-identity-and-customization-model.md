---
id: TASK-021
title: Choose the public product identity and customization model
status: needs-planning
priority: P1
type: research
owner: unassigned
depends_on: []
plan: .agents/plans/TASK-021-public-product-identity.md
updated: 2026-07-03
---

# TASK-021: Choose the public product identity and customization model

## Context

The original event is over and the app should become reusable before it is
published. Its visible identity currently mixes the generic `Lydkontroll` name
with `Marius + Wenche`, wedding-specific descriptions, and a waveform-heart
icon. The raw feedback proposed names including `The Wedding MC`, `The
Toastmaster`, `Toastmastah`, and `SoundMastah`, and questioned whether the app
should be fully generic or allow each operator to customize its displayed
title.

## Outcome

A recorded product decision defines the public name, positioning, retained or
replaced logo concept, and whether the desktop/mobile event title is fixed,
configurable, or absent, so implementation and repository publication use one
coherent identity.

## Scope

- Compare a small shortlist of generic and event-oriented names, including the
  user's original candidates.
- Check obvious naming conflicts and practical repository/package-name fit.
- Decide whether the heart remains part of the public identity.
- Decide the event-title customization model and its default/fallback text.
- Record the decision and exact strings/assets consumed by downstream tasks.

## Non-goals

- Implementing UI, persistence, metadata, or icon changes.
- Designing a full marketing site or paid brand campaign.
- Guaranteeing trademark availability through legal advice.

## Acceptance Criteria

- [ ] The public app name and one-sentence positioning are selected.
- [ ] The decision explicitly accepts or rejects each original candidate and
      gives the reason for the final choice.
- [ ] Fixed versus configurable event title is decided, including default and
      empty-state behavior on desktop and mobile.
- [ ] The waveform-heart direction is retained, revised, or replaced.
- [ ] Repository slug, package/product name, and user-facing title strings are
      specified for `TASK-022` and `TASK-025`.
- [ ] The result is recorded in `.agents/DECISIONS.md` and the linked plan is
      completed.

## Validation

```text
manual: verify the decision names every public and configurable identity field
manual: verify TASK-022 and TASK-025 can proceed without inventing brand text
python3 scripts/ralph.py check
```

## Notes

- P1 because this gates coherent genericization and naming of the public repo,
  but does not affect the already-working playback system.
- Preserved observation: the wedding/heart theme may still be appropriate; it
  is not assumed to be wrong merely because the original names are specific.
- Needs planning because choosing generic versus wedding-specific positioning,
  and fixed versus customizable title, materially changes the product.

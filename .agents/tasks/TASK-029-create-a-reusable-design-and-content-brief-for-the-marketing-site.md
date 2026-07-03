---
id: TASK-029
title: Create a reusable design and content brief for the marketing site
status: done
priority: P1
type: docs
owner: unassigned
depends_on: []
plan: none
updated: 2026-07-04
---

# TASK-029: Create a reusable design and content brief for the marketing site

## Context

The owner wants to use Google Stitch to explore a marketing-site design for
`Lydkontroll`, then refine the resulting concept before building a GitHub Pages
site. Stitch needs an accurate, reusable handoff rather than loose screenshots
or claims inferred from implementation details. The working app already has a
distinct warm, dark visual language and accepted Norwegian-first,
event-generic positioning that should carry into the site.

## Outcome

The repository contains a concise `DESIGN.md` that can be handed directly to a
designer or Google Stitch to generate a credible Norwegian marketing-site
concept while preserving the app's identity and product truth.

## Scope

- Document the accepted product name, positioning, audience, purpose,
  reliability principles, current functionality, platform/network model, and
  important limitations in plain Norwegian.
- Extract the app's current visual system from source: core palette with
  semantic roles, typography characteristics, spacing/shape/motion character,
  icon/logo usage, and accessibility considerations.
- Propose a first-pass site hierarchy and content goals, including hero,
  problem/benefit story, feature groups, how it works, trust/reliability,
  requirements, source/licensing context, and appropriate calls to action.
- Include a practical asset and screenshot shot list for desktop and iPhone,
  with notes about what each image should communicate.
- Include one copy-paste-ready Google Stitch brief that clearly distinguishes
  established facts from design freedom.
- Cross-reference the relevant source files and accepted decisions so future
  edits can be checked for drift.

## Non-goals

- Building or deploying the marketing site.
- Generating the final visual design or treating a Stitch proposal as accepted.
- Final polished marketing copy, pricing, sales promises, testimonials,
  analytics, or a custom domain.
- English website content; `TASK-026` owns complete English localization and
  translated positioning.
- Changing the app UI, icon, or accepted `Lydkontroll` identity.

## Acceptance Criteria

- [x] `DESIGN.md` gives a designer enough product context to understand the
      audience, use case, workflow, differentiators, requirements, and limits
      without reading the codebase.
- [x] The visual section records exact source-backed colors and explains their
      roles instead of presenting an unstructured swatch list.
- [x] The brief preserves the accepted Norwegian-first, event-generic,
      calm/reliability-first identity and does not imply unsupported app-store,
      multi-platform, LAN, cloud, or international availability.
- [x] A proposed information architecture and screenshot/asset checklist cover
      both Mac operation and iPhone control over Tailscale.
- [x] A copy-paste Google Stitch prompt identifies fixed brand constraints and
      areas where Stitch may explore alternatives.
- [x] The document points to the canonical implementation/decision sources and
      notes that final copy and design remain subject to owner review.

## Validation

```text
manual: compare DESIGN.md product claims with PLAN.md, README.md, ADR-014, and current UI
manual: paste the standalone Stitch brief into a blank document and verify it makes sense without repository context
git diff --check
python3 scripts/ralph.py check
```

## Notes

- Source palette evidence currently lives in `src/styles.css`; the default cue
  color is `#d88c68` in `src/components/Shell.tsx`.
- Reuse the neutral rounded-waveform icon and accepted positioning from
  `TASK-021`/ADR-014.
- The next action is to claim and start this task, then inventory source-backed
  product and visual facts before drafting `DESIGN.md`.
- Reprioritized by the owner on 2026-07-04: marketing and the Norwegian website
  come before English localization.

- 2026-07-04: Task claimed.
- 2026-07-04: Added and manually reviewed `DESIGN.md` against the product,
  public documentation, ADR-014/ADR-015, current desktop/mobile UI, palette,
  and icon sources. The standalone Stitch prompt preserves fixed facts while
  leaving composition and presentation open for exploration. `git diff
--check` and Ralph validation pass.

- 2026-07-04: Acceptance criteria complete: DESIGN.md contains the source-backed Norwegian product and visual brief, site hierarchy, screenshot list, standalone Stitch prompt, review boundary, and canonical references; formatting, diff, and Ralph checks pass.

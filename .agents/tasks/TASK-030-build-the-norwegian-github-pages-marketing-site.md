---
id: TASK-030
title: Build the Norwegian GitHub Pages marketing site
status: needs-planning
priority: P1
type: feature
owner: unassigned
depends_on: [TASK-029]
plan: .agents/plans/TASK-030-github-pages-marketing-site.md
updated: 2026-07-04
---

# TASK-030: Build the Norwegian GitHub Pages marketing site

## Context

After the design/content handoff and an owner-reviewed concept exist,
`Lydkontroll` needs a public Norwegian marketing site hosted through GitHub
Pages. The site should explain the product more clearly than the repository
README while remaining honest about current distribution, Apple-platform
support, Tailscale requirements, and source-available licensing.

## Outcome

Visitors can open a polished, responsive Norwegian `Lydkontroll` marketing site
on GitHub Pages, understand what the product does and requires, and reach the
appropriate repository or future distribution action.

## Scope

- Convert the accepted design proposal and approved content direction into a
  responsive, accessible static site tracked in this repository.
- Configure a maintainable GitHub Pages build/deployment path without mixing
  marketing assets into the Tauri runtime bundle.
- Cover Mac and mobile layouts, metadata/social sharing, favicon/brand assets,
  sensible performance, and a useful not-found path where applicable.
- State current platform, Tailscale, licensing, and distribution boundaries
  accurately and link to canonical project information.

## Non-goals

- Creating the initial design brief or choosing among unreviewed Stitch
  concepts; that is `TASK-029` plus owner review.
- English localization; coordinate later with `TASK-026` rather than shipping
  a partial mixed-language site.
- Checkout, subscriptions, account systems, cloud services, or collecting
  visitor data by default.
- Changing product behavior or weakening the Tailscale-only mobile boundary.

## Acceptance Criteria

- [ ] The accepted Norwegian content and design render correctly at desktop
      and representative iPhone widths.
- [ ] GitHub Pages deploys from a documented, reproducible repository workflow.
- [ ] The site has accessible landmarks, keyboard behavior, contrast, image
      alternatives, metadata, and no avoidable layout shift.
- [ ] Public claims match the current product, source-available terms, and
      supported distribution model.
- [ ] Marketing-site source and assets are clearly separated from the embedded
      Tauri application frontend.
- [ ] Automated validation covers build integrity, links, and the agreed page
      quality gates; representative layouts receive visual/manual review.

## Validation

```text
site-specific build and test commands selected by the accepted plan
manual: verify the deployed GitHub Pages URL on desktop and iPhone Safari
npm run public:check
git diff --check
python3 scripts/ralph.py check
```

## Notes

- Planning must wait for `TASK-029` and an owner-reviewed Stitch/design concept.
- Open choices are recorded in the linked plan; no site implementation should
  begin until the design direction, content/CTA, repository layout, and Pages
  deployment model are accepted.
- Reprioritized by the owner on 2026-07-04: complete this Norwegian marketing
  track after `TASK-029` and before starting `TASK-026` English localization.

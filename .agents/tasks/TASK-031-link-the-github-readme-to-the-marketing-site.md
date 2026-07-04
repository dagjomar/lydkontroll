---
id: TASK-031
title: Add product screenshots and the marketing-site link to the README
status: needs-planning
priority: P1
type: docs
owner: unassigned
depends_on: [TASK-030]
plan: none
updated: 2026-07-04
---

# TASK-031: Add product screenshots and the marketing-site link to the README

## Context

The repository README is the default landing page for visitors who reach the
GitHub project directly. It should show the product clearly and make the
polished Norwegian marketing site easy to find without obscuring the
repository's source, build, license, and support information.

## Outcome

Visitors opening the GitHub repository immediately see representative Mac and
iPhone product imagery and can follow a prominent, accurate link to the
verified GitHub Pages marketing site.

## Scope

- Add a concise Norwegian-first marketing-site link near the top of `README.md`.
- Use the production Pages URL verified by `TASK-030`.
- Add a compact, responsive screenshot presentation covering both Mac operation
  and iPhone control.
- Use final event-generic images without private names, licensed audio details,
  or obsolete interface states; reuse canonical website assets where sensible.
- Give every image useful alternative text and keep repository-page weight
  reasonable.
- Preserve the README's current technical, limitation, license, and support
  guidance.

## Non-goals

- Rewriting the README as a duplicate marketing page.
- Adding a link before the Pages deployment is live and verified.
- Publishing the temporary `Marius + Wenche` mockups as final README imagery.
- Changing the website, product behavior, licensing, or distribution claims.

## Acceptance Criteria

- [ ] The link is visible without searching through the full README.
- [ ] The link targets the verified canonical Pages URL and returns the
      marketing site.
- [ ] Surrounding copy does not imply a public signed download or unsupported
      platform availability.
- [ ] Existing technical setup, limitations, license, contribution, support,
      and security information remains intact.
- [ ] The README shows representative Mac and iPhone views near the product
      introduction, with event-generic content and descriptive alternative
      text.
- [ ] Screenshots render cleanly on GitHub at desktop and narrow widths without
      dominating the technical documentation or adding unnecessary large files.

## Validation

```text
manual: open the README link and verify the production Pages site
manual: review the rendered README on GitHub at desktop and narrow widths
npm run public:check
git diff --check
python3 scripts/ralph.py check
```

## Notes

- Requested by the owner on 2026-07-04.
- Expanded by the owner on 2026-07-04 to include final product screenshots as
  well as the marketing-site link; this replaces rather than duplicates the
  original task.
- Fully scoped but intentionally left `needs-planning` behind `TASK-030`; the
  only remaining input is its verified production URL. Move it to `ready` and
  implement only after the Pages deployment is live.

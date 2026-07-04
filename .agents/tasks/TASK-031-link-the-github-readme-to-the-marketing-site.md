---
id: TASK-031
title: Link the GitHub README to the marketing site
status: needs-planning
priority: P1
type: docs
owner: unassigned
depends_on: [TASK-030]
plan: none
updated: 2026-07-04
---

# TASK-031: Link the GitHub README to the marketing site

## Context

The repository README is the default landing page for visitors who reach the
GitHub project directly. Once the Norwegian marketing site is live, the README
should make that polished product introduction easy to find without obscuring
the repository's source, build, license, and support information.

## Outcome

Visitors opening the GitHub repository can follow a prominent, accurate link
to the verified GitHub Pages marketing site.

## Scope

- Add a concise Norwegian-first marketing-site link near the top of `README.md`.
- Use the production Pages URL verified by `TASK-030`.
- Preserve the README's current technical, limitation, license, and support
  guidance.

## Non-goals

- Rewriting the README as a duplicate marketing page.
- Adding a link before the Pages deployment is live and verified.
- Changing the website, product behavior, licensing, or distribution claims.

## Acceptance Criteria

- [ ] The link is visible without searching through the full README.
- [ ] The link targets the verified canonical Pages URL and returns the
      marketing site.
- [ ] Surrounding copy does not imply a public signed download or unsupported
      platform availability.
- [ ] Existing technical setup, limitations, license, contribution, support,
      and security information remains intact.

## Validation

```text
manual: open the README link and verify the production Pages site
npm run public:check
git diff --check
python3 scripts/ralph.py check
```

## Notes

- Requested by the owner on 2026-07-04.
- Fully scoped but intentionally left `needs-planning` behind `TASK-030`; the
  only remaining input is its verified production URL. Move it to `ready` and
  implement only after the Pages deployment is live.

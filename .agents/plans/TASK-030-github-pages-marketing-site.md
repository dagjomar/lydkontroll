# Plan: TASK-030 — Norwegian GitHub Pages marketing site

Status: accepted
Updated: 2026-07-04

## Problem

The project needs a public-facing Norwegian product presentation, but the
repository does not yet have an accepted page design, final site copy,
distribution call to action, or a decision about how marketing-site source and
GitHub Pages deployment should coexist with the Tauri frontend. Implementing
before those choices would risk duplicating build systems, publishing
unsupported claims, or hardening an exploratory Stitch concept prematurely.

## Current Evidence

- `TASK-029` will produce the source-backed design/content handoff and a Google
  Stitch prompt.
- ADR-014 fixes the `Lydkontroll` name, Norwegian-first/event-generic
  positioning, calm reliability-first character, and neutral waveform mark.
- `src/styles.css` contains the current warm dark interface palette and visual
  treatment.
- The public repository is `https://github.com/dagjomar/lydkontroll` and the
  source is PolyForm Noncommercial source-available, not OSI open source.
- Desktop use works without phone/network; current mobile control requires an
  iPhone/Safari path over Tailscale.
- `TASK-026` separately owns full English UI and marketing localization.

## Open Questions

- Which owner-reviewed Stitch concept, page hierarchy, and content tone become
  the implementation reference?
- What is the primary call to action at launch: inspect/fork the source, follow
  development, request commercial access, or another explicitly supported
  action?
- Should the site live in a dedicated repository directory/build package or a
  separate branch-oriented Pages setup, and how will its dependencies remain
  isolated from the Tauri app?
- Should Pages publish at the repository URL only, or should the implementation
  prepare for a custom domain without requiring one now?
- Are privacy-friendly analytics needed at launch, or should the first version
  deliberately collect no visitor data?
- Which final screenshots or mockups may be published, and do any need fixture
  data to avoid exposing wedding/event details?
- How should later English pages share routes/content with `TASK-026` without
  forcing premature internationalization into the Norwegian launch?

## Options Considered

### Site package inside the main repository

- Benefits: content, assets, product facts, and public-readiness checks evolve
  with the application; one issue/task history and one Pages workflow.
- Costs and risks: build dependencies and output must be kept clearly separate
  from the embedded Tauri frontend.

### Generated static files on a Pages branch

- Benefits: minimal published surface and conventional Pages output.
- Costs and risks: generated branch drift, weaker reviewability, and a less
  obvious local authoring workflow unless automation owns it completely.

### Separate marketing repository

- Benefits: total deployment and dependency isolation.
- Costs and risks: duplicated identity/content governance and weaker coupling
  to product truth; unnecessary unless the site becomes independently managed.

## Decision

The owner accepted the Google Stitch/AI Studio concept delivered as
`lydkontroll-website.zip` on 2026-07-04, including its existing desktop and
mobile mockup images. Integrate it as an isolated `website/` package in this
repository and deploy it with GitHub Actions to the repository Pages URL. Use
the canonical GitHub repository as the primary CTA, collect no analytics, and
do not add a custom domain yet. Launch only the Norwegian presentation;
`TASK-026` retains the later complete English track. Download the approved
Google-hosted mockups into tracked static assets so production does not depend
on temporary external URLs. Remove unused AI Studio/Gemini/server scaffolding
and correct generated claims against `DESIGN.md` and public project records.

## Implementation Slices

1. Accept the visual concept, information architecture, launch CTA, and asset
   set; record any durable decisions.
2. Select and scaffold the isolated static-site/build/deployment structure.
3. Implement approved Norwegian content, responsive components, metadata, and
   accessible assets.
4. Add build, link, accessibility, performance, and public-claim checks.
5. Deploy to GitHub Pages and verify the production URL on desktop and iPhone
   Safari.
6. Record the later English route/content handoff to `TASK-026` without adding
   partial English pages.

## Test Strategy

- Automated: production build, internal/external link checks, accessibility
  checks, metadata validation, and agreed performance/asset budgets.
- Manual: compare representative desktop/iPhone renders with the accepted
  design; verify public claims and calls to action; test the deployed Pages URL.
- Failure modes: stale product claims, mixed languages, private event data in
  screenshots, broken base paths, Pages-only routing failures, inaccessible
  contrast/focus, oversized assets, or Tauri and site build outputs colliding.

## Rollback or Recovery

Keep the existing GitHub repository/README as the canonical public entrypoint
until the Pages deployment is verified. A failed deployment must not alter the
application release path; disable or revert only the Pages workflow/output.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.

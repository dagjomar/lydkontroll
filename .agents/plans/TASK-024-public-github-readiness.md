# Plan: TASK-024 — Public GitHub readiness

Status: draft — owner decisions pending
Updated: 2026-07-04

## Problem

The private event repository and unsigned known-Mac release were never scoped
for public source or binary distribution. Publication is irreversible enough
that privacy, licensing, support, security, and release promises need explicit
decisions and a history-wide audit first.

## Current Evidence

- ADR-013 explicitly defers signing/notarization and public distribution.
- Existing release checks reject tracked audio, secrets, bundles, and local app
  data, but have not established a history-wide public audit.
- README and PLAN still frame the product around one wedding.
- Tailscale is the only access-control boundary and must remain prominent.
- The 2026-07-04 preliminary audit found no audio, app bundles, signing
  material, local library data, or absolute `/Users/...` paths in any reachable
  Git object. See `.agents/audits/TASK-024-public-readiness-audit.md`.
- Every existing commit publishes the author's full name and Gmail address.
- Older commits contain the couple-specific `Marius`/`Wenche` identity even
  though the current tree is generic. Publishing the existing history therefore
  publishes that retired identity unless the history is rewritten.

## Open Questions

- Which open-source license should govern reuse and contributions?
- Is the first public launch source-only, or must it ship downloadable macOS
  binaries with signing/notarization and release automation?
- Which event/rehearsal records are useful engineering history versus private
  material that should not be published?
- Which GitHub community, security, and automation files are warranted now?

## Options Considered

### Public source only

- Benefits: smallest credible launch; no unsigned-binary confusion or Apple
  credential requirement.
- Costs and risks: users must build locally; less approachable to non-developers.

### Source plus unsigned app bundle

- Benefits: easy artifact creation with the current workflow.
- Costs and risks: Gatekeeper friction and a weak public trust story.

### Source plus signed/notarized releases

- Benefits: strongest end-user installation path.
- Costs and risks: Apple account credentials, release automation, and ongoing
  distribution maintenance substantially expand scope.

## Decision

Pending owner confirmation. Planning recommendations are:

- launch source-only first; do not publish the current unsigned bundle and do
  not add Apple signing/notarization until binary distribution is an explicit
  product goal;
- use a permissive license if broad reuse is desired (MIT is the smallest
  simple option; Apache-2.0 adds an explicit patent grant); and
- publish a sanitized history only if the author email or retired couple names
  are not acceptable public information. Otherwise retain the engineering
  history and explicitly approve those disclosures.

These choices cannot be inferred from implementation evidence because they
change legal rights, maintenance scope, and irreversible privacy exposure.

## Implementation Slices

1. Audit current tree and intended Git history for public exposure.
2. Decide license, support boundary, and distribution level.
3. Genericize contributor-facing docs and add selected community/security files.
4. Strengthen ignore/public-readiness checks and validate a clean checkout.

## Test Strategy

- Automated: existing validation plus secret/private-artifact/history checks.
- Manual: license review, full tracked-file/history review, clean-clone README
  walkthrough, and Tailscale security-boundary review.
- Failure modes: leaked personal material, incompatible license, undocumented
  prerequisites, misleading binary promise, or unreviewed generated artifacts.

## Rollback or Recovery

Do not create or push a public remote until every acceptance criterion passes.
If audit findings require history rewriting, perform and re-audit it locally
before `TASK-025`; never sanitize after public publication as the primary plan.

## Ready Checklist

- [ ] Open questions are resolved or explicitly deferred.
- [ ] Decisions and tradeoffs are recorded.
- [ ] Slices are small and dependency ordered.
- [ ] Tests cover the important failure modes.
- [ ] The parent task can move to `ready`.

## Planning Progress

- [x] Current tracked tree and reachable Git objects received a preliminary
      path/content/large-object audit.
- [x] Distribution options were narrowed to a source-only recommendation; an
      unsigned public app bundle is rejected as a weak trust story.
- [x] History-specific privacy findings were identified for owner review.
- [ ] Owner selects the license.
- [ ] Owner confirms source-only launch.
- [ ] Owner chooses retained versus sanitized history after reviewing the
      author-email and retired-identity disclosures.

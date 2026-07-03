# Plan: TASK-024 — Public GitHub readiness

Status: accepted
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

## Resolved Questions

- Use PolyForm Noncommercial 1.0.0 for the public source. This is deliberately
  source-available rather than OSI open source: noncommercial use, forks, and
  modification are allowed, while commercial licensing stays with the owner.
- Launch GitHub source-only. Do not attach the current unsigned `.app`; a future
  signed/notarized bundle may be sold separately under a commercial license.
- Retain the 31-commit history. The owner accepts publication of the author
  email, retired couple names, event context, and non-sensitive rehearsal
  evidence found by the audit.
- Add README licensing/distribution/security boundaries, `SECURITY.md`,
  `SUPPORT.md`, `CONTRIBUTING.md`, and a contributor agreement that preserves
  contributor copyright while granting the project owner commercial and
  relicensing rights.

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

Publish the retained history under PolyForm Noncommercial 1.0.0 as a
source-only, source-available project. The owner reserves commercial licensing
and may later sell signed/notarized bundles, support, or other commercial
offerings. External contributions require a CLA broad enough for the owner to
commercially license and relicense accepted contributions while contributors
retain copyright. Public copy must not call this OSI open source.

The repository documents the intended model but does not claim that it replaces
professional legal review. Before selling builds or accepting substantive
outside contributions, the owner should have Norwegian counsel review the
license/CLA and applicable dependency-notice obligations.

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

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task can move to `ready`.

## Planning Progress

- [x] Current tracked tree and reachable Git objects received a preliminary
      path/content/large-object audit.
- [x] Distribution options were narrowed to a source-only recommendation; an
      unsigned public app bundle is rejected as a weak trust story.
- [x] History-specific privacy findings were identified for owner review.
- [x] Owner selects PolyForm Noncommercial 1.0.0 plus separate commercial
      licensing and a contributor agreement.
- [x] Owner confirms source-only launch.
- [x] Owner chooses retained history after reviewing the
      author-email and retired-identity disclosures.

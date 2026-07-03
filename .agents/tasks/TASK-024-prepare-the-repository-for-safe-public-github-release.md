---
id: TASK-024
title: Prepare the repository for safe public GitHub release
status: needs-planning
priority: P1
type: chore
owner: unassigned
depends_on: []
plan: .agents/plans/TASK-024-public-github-readiness.md
updated: 2026-07-03
---

# TASK-024: Prepare the repository for safe public GitHub release

## Context

The repository was built for one private event. ADR-013 explicitly says the
current unsigned, known-Mac release is not prepared for public distribution.
Before making source history public, the project needs a deliberate privacy,
license, documentation, contribution, artifact, and distribution-readiness
review rather than assuming the existing release gate covers public use.

## Outcome

The tracked repository and its history are demonstrably safe and understandable
to publish, with an explicit license and support/distribution scope, and a
repeatable pre-publication gate that passes.

## Scope

- Audit tracked files and Git history for personal data, audio, credentials,
  local paths, build products, and event-only evidence.
- Decide and add the source license and public support/distribution promise.
- Make README setup, architecture, limitations, security model, and validation
  instructions sufficient for a new contributor.
- Add standard public-repository community/security files where justified.
- Define whether the first launch is source-only or includes downloadable app
  artifacts, signing/notarization, and release automation.
- Add a repeatable public-readiness validation command/checklist.

## Non-goals

- Choosing or applying product branding (`TASK-021`/`TASK-022`).
- Creating the remote GitHub repository or pushing (`TASK-025`).
- Promising platforms beyond Apple Silicon macOS and iPhone Safari.

## Acceptance Criteria

- [ ] A documented audit covers the current tree and every commit intended for
      publication, with no secrets, personal audio, local app data, generated
      bundles, or unintended event-private material exposed.
- [ ] A deliberate license is present and compatible with direct dependencies.
- [ ] README and support/security documentation state prerequisites, Tailscale
      trust boundary, local-audio behavior, build/test commands, limitations,
      and how to report vulnerabilities.
- [ ] Source-only versus signed/notarized binary distribution is decided and
      the documented release process does not overclaim what is provided.
- [ ] Ignore rules and automated checks reject private artifacts and secrets
      likely to recur.
- [ ] Full project validation and the public-readiness gate pass from a clean
      checkout.

## Validation

```text
manual: review every tracked path and commit intended for publication
manual: review license and dependency-license compatibility
npm run release:check
npm test -- --run
cargo test --manifest-path src-tauri/Cargo.toml
npm run build
npm run lint
python3 scripts/ralph.py check
```

## Notes

- P1 because it gates publication and protects privacy/security, while not
  affecting the functioning local app.
- Needs planning because license choice and source-only versus distributed
  binaries materially change the public promise and release work.
- This deduplicates public-launch hygiene into one readiness result; the actual
  remote creation/push remains separately schedulable as `TASK-025`.


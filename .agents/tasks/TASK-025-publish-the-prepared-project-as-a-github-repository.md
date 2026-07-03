---
id: TASK-025
title: Publish the prepared project as a GitHub repository
status: in-progress
priority: P2
type: chore
owner: codex-2026-07-04-github-publication
depends_on: [TASK-022, TASK-023, TASK-024]
plan: none
updated: 2026-07-04
---

# TASK-025: Publish the prepared project as a GitHub repository

## Context

Once reusable branding, the corrected icon, and public-readiness work are
complete, the prepared history should be stored on GitHub under the selected
repository identity. Remote creation is kept separate because it is an
external, independently verifiable action and requires the owner's GitHub
account/organization and visibility confirmation.

## Outcome

The prepared project exists in the intended GitHub account as a public
repository with its default branch, description, topics, and release posture
matching the recorded decisions, and a fresh clone validates successfully.

## Scope

- Confirm the destination account/organization and public visibility at
  execution time.
- Create the repository using the slug/description chosen in `TASK-021`.
- Push the approved branch/history without force-overwriting an existing repo.
- Configure basic metadata and branch protections/settings justified by
  `TASK-024`.
- Verify links and validation from a fresh clone.

## Non-goals

- Rewriting or sanitizing history after it has been published.
- Publishing binaries unless `TASK-024` explicitly approves them.
- Ongoing issue triage, marketing, or multi-platform support.

## Acceptance Criteria

- [x] The remote URL, owner, visibility, default branch, description, and
      topics match the approved public identity.
- [x] Only the audited history and intended tracked files are present remotely.
- [x] README, license, security/support links, and any approved release assets
      render and resolve correctly on GitHub.
- [x] A fresh clone passes the documented public-readiness validation.
- [x] The local remote configuration and `.agents` handoff record the canonical
      repository URL and next maintenance action.

## Validation

```text
git remote -v
git ls-remote --heads <canonical-repository-url>
manual: inspect repository visibility, metadata, files, and rendered docs
manual: clone into a temporary directory and run the documented validation gate
python3 scripts/ralph.py check
```

## Notes

- P2 because it is the final publication action; P1 prerequisites contain the
  risk-reducing and user-visible work.
- Fully scoped but blocked until `TASK-022`, `TASK-023`, and `TASK-024` are
  done. Then move it to ready and ask for GitHub destination/visibility at
  execution time, not during backlog intake.

- 2026-07-04: Owner authorized creation of public `dagjomar/lydkontroll` from
  the audited `main` history. Created the repository without remote
  initialization, pushed `main`, and configured the event-generic description,
  `tauri`/`rust`/`react`/`soundboard`/`tailscale` topics, issues enabled, wiki
  disabled, and private vulnerability reporting enabled. No binary release was
  created. Canonical URL: https://github.com/dagjomar/lydkontroll

- 2026-07-04: TASK-022, TASK-023, and TASK-024 are complete. Publication now needs the owner's GitHub destination account/organization and explicit authorization to create/push the public remote.

- 2026-07-04: Task claimed.

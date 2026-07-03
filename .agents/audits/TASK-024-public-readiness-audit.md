# TASK-024 preliminary public-readiness audit

Date: 2026-07-04
Scope: current tracked tree and all objects reachable from local Git refs
Status: preliminary; repeat after implementation and from the exact history
intended for publication

## Checks performed

- Enumerated every currently tracked path and every path present in reachable
  commit history.
- Inspected commit authorship metadata.
- Searched the working tree and history for personal/event names, absolute user
  paths, credential-like terms, and private-key markers.
- Searched reachable object paths for MP3, WAV, M4A, app bundles, disk images,
  signing material, environment files, and local `library.json` data.
- Reviewed the largest reachable objects for unexpected binary payloads.
- Reviewed `.gitignore`, `scripts/check-release-tree.mjs`, README, candidate,
  rehearsal, and the public-release task/plan.

## Findings

### Blocking owner decisions

1. Every commit records `Dag Jomar Mersland <dagjomar@gmail.com>`. A public push
   exposes that address through Git history. The owner must explicitly accept
   this or choose a mailmap/history rewrite before publication.
2. Older commits contain the retired `Marius`/`Wenche` event identity. The
   current tree is generic, but an ordinary public push exposes the historical
   names and wedding context. The owner must accept that disclosure or approve
   a sanitized history.
3. The repository has no source license. License selection is a legal/product
   choice, not a technical default.

### Clean results

- No reachable path indicates committed MP3/WAV/M4A files, `.app`/`.dmg`
  bundles, signing keys, environment-secret files, or local cue-library data.
- No absolute `/Users/...` path was found in reachable textual history.
- The largest reachable objects are expected generated icon assets or project
  source/lock data; no unexpected audio or release bundle was identified.
- Current ignore rules cover common build outputs, audio, local data, and most
  signing material. The existing release-tree check rejects these categories
  only in the current tracked tree, not across history.
- Candidate/rehearsal records contain artifact hashes and coarse platform
  details, but no source audio paths, phone model, precise hardware identity, or
  production-library location. They are useful engineering evidence if the
  owner accepts publishing the event context.

## Required follow-up after owner decisions

1. Record the license, source-only distribution, and history policy in an ADR.
2. If sanitizing, create the proposed publication history locally and repeat
   this audit against every reachable object before any remote is created.
3. Add the selected license, README security/support/distribution statements,
   community files, and a history-aware public-readiness check.
4. Run the full task validation from a clean checkout of the exact publication
   history.

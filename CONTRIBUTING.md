# Contributing to Lydkontroll

Thank you for considering an improvement. Bug reports, focused proposals, and
small pull requests are welcome. This project is source-available under the
PolyForm Noncommercial License 1.0.0; it is not OSI open source.

## Before writing code

- Open an issue for substantial behavior, protocol, persistence, or UI changes.
- Keep playback, networking, and persistence logic out of React components.
- Do not add audio files, credentials, local data, generated app bundles, or
  signing material.
- Follow `AGENTS.md` and update the relevant `.agents/` task records for work
  that changes project scope or behavior.

## Contributor agreement

Every pull request containing a substantive contribution must include this
statement in its description:

> I have read and agree to the Contributor License Agreement in `CLA.md`, and I
> have the right to submit this contribution under those terms.

Do not merge a contribution without that recorded acceptance. Contributors
retain copyright in their work; the agreement grants the project owner the
rights needed to distribute accepted contributions under both noncommercial
and separate commercial terms.

## Validation

Install locked dependencies with `npm ci`, then run:

```bash
npm run public:check
npm test -- --run
cargo test --manifest-path src-tauri/Cargo.toml
npm run build
npm run lint
```

Bug fixes should include a regression test named after observable behavior.
Hardware-sensitive audio or iPhone Safari behavior also needs a manual check on
the supported Apple Silicon Mac/iPhone setup.

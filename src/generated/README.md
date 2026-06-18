# Generated Rust contracts

Files in this directory are generated from Rust types with `ts-rs` and must not
be edited by hand.

- Generate: `npm run bindings:generate`
- Check for drift: `npm run bindings:check`

The export directory is fixed by `.cargo/config.toml`. Rust remains the source
of truth for shared command and state contracts.

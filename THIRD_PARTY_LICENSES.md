# Direct dependency license review

Reviewed: 2026-07-04 against committed npm and Cargo locks.

Lydkontroll's PolyForm terms apply only to original project material. Every
dependency remains governed by its own license. The direct dependencies below
use permissive licenses except Symphonia, which uses MPL-2.0. Nothing found in
the direct dependency set prevents publishing Lydkontroll source under PolyForm
or commercially distributing a combined application, provided all dependency
license and notice obligations are honored.

This inventory is a release-planning review, not legal advice. Re-run it when
locks change and obtain legal review before commercial binary distribution.

## Runtime npm dependencies

| Package                     | Locked version | Declared license  |
| --------------------------- | -------------: | ----------------- |
| `@tauri-apps/api`           |         2.11.1 | Apache-2.0 OR MIT |
| `@tauri-apps/plugin-dialog` |          2.7.1 | MIT OR Apache-2.0 |
| `qrcode`                    |          1.5.4 | MIT               |
| `react`                     |         19.2.7 | MIT               |
| `react-dom`                 |         19.2.7 | MIT               |

## Direct Rust dependencies

| Crate                 | Locked version | Declared license    |
| --------------------- | -------------: | ------------------- |
| `axum`                |          0.8.9 | MIT                 |
| `futures-util`        |         0.3.32 | MIT OR Apache-2.0   |
| `if-addrs`            |         0.15.0 | MIT OR BSD-3-Clause |
| `kira`                |         0.12.0 | MIT OR Apache-2.0   |
| `serde`               |        1.0.228 | MIT OR Apache-2.0   |
| `serde_json`          |        1.0.150 | MIT OR Apache-2.0   |
| `symphonia`           |          0.5.5 | MPL-2.0             |
| `tauri`               |         2.11.3 | Apache-2.0 OR MIT   |
| `tauri-plugin-dialog` |          2.7.1 | Apache-2.0 OR MIT   |
| `thiserror`           |         1.0.69 | MIT OR Apache-2.0   |
| `tokio`               |         1.52.3 | MIT                 |
| `ts-rs`               |         12.0.1 | MIT                 |
| `uuid`                |         1.23.3 | Apache-2.0 OR MIT   |

Build-only direct dependency: `tauri-build` 2.6.3, Apache-2.0 OR MIT.

Before distributing a commercial binary, generate and ship a complete
transitive third-party notice bundle rather than relying on this direct-only
planning inventory.

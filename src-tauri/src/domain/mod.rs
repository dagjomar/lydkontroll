//! Pure domain data and rules.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Identifies which presentation surface consumes a shared snapshot.
///
/// This small contract establishes the deterministic Rust-to-TypeScript export
/// path. Product protocol types replace and extend it in later slices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum AppMode {
    Desktop,
    Mobile,
}

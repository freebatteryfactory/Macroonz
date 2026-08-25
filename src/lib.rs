//! The module-preserving front door to the complete Macroonz machine.
//!
//! [`compiler`] bakes a complete generation request, [`macros`] carries the six first-party declaration doors, and the feature-gated `harness` module judges what a caller hands it.
//! The facade adds no second API and flattens no owner's vocabulary: every item remains under the crate that defines it.

/// The ordinary callable generation compiler.
pub use macroonz_compiler as compiler;

/// The six first-party procedural declaration doors.
pub use macroonz_macros as macros;

/// The independent test harness.
#[cfg(feature = "harness")]
pub use macroonz_harness as harness;

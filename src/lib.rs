//! The recipe entrance and module-preserving front door to the complete Macroonz machine.
//!
//! `macroonz::recipe!` is the one root workflow entrance for ordinary Rust, declared structure, and requested projections.
//! [`compiler`] remains the callable expert owner, [`macros`] carries procedural doors, and the feature-gated `harness` module judges what a caller hands it.
//! The facade flattens no owner's expert vocabulary, and removing the harness feature makes only harness-owned evidence projections unavailable.

/// The ordinary callable generation compiler.
pub use macroonz_compiler as compiler;

/// The built-in procedural declaration doors.
pub use macroonz_macros as macros;

/// The independent test harness.
#[cfg(feature = "harness")]
pub use macroonz_harness as harness;

//! The recipe entrance and module-preserving front door to the complete Macroonz machine.
//!
//! `macroonz::recipe!` is the one root workflow entrance for ordinary Rust, declared structure, and requested projections.
//! [`compiler`] remains the callable expert owner, [`macros`] carries procedural doors, and the feature-gated `harness` module judges what a caller hands it.
//! The facade flattens no owner's expert vocabulary, and removing the harness feature makes only harness-owned evidence projections unavailable.

/// The ordinary callable generation compiler.
pub use macroonz_compiler as compiler;

/// The built-in procedural declaration doors.
pub use macroonz_macros as macros;

/// Bakes one ordinary Rust module from its declared structure and requested projections.
///
/// This is the only supported root recipe entrance.
/// Its procedural carrier is public only because Rust requires a public proc-macro entry behind this hygienic facade wrapper; direct carrier invocation is outside the compatibility contract.
///
/// # Example
///
#[doc = "```rust"]
#[doc = include_str!("../examples/recipe.rs")]
#[doc = "```"]
#[cfg(feature = "harness")]
#[macro_export]
macro_rules! recipe {
    ($($recipe:tt)*) => {
        $crate::macros::__macroonz_recipe_carrier! {
            { $crate }
            __macroonz_test_carrier_available
            { $($recipe)* }
        }
    };
}

/// Bakes one ordinary Rust module from its declared structure and requested projections.
///
/// This is the only supported root recipe entrance.
/// Harness-owned projections produce a typed declaration refusal in this facade posture.
/// Its procedural carrier is public only because Rust requires a public proc-macro entry behind this hygienic facade wrapper; direct carrier invocation is outside the compatibility contract.
///
/// # Example
///
#[doc = "```rust"]
#[doc = include_str!("../examples/recipe.rs")]
#[doc = "```"]
#[cfg(not(feature = "harness"))]
#[macro_export]
macro_rules! recipe {
    ($($recipe:tt)*) => {
        $crate::macros::__macroonz_recipe_carrier! {
            { $crate }
            __macroonz_test_carrier_unavailable
            { $($recipe)* }
        }
    };
}

/// The independent test harness.
#[cfg(feature = "harness")]
pub use macroonz_harness as harness;

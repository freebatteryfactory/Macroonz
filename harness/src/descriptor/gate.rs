//! The schema gate releases one staged carrier with the defining harness's own binding.
//!
//! Matching literal schema tokens releases one trial/deferred or benchmark/reporter delivery through its carrier's internal transcription.
//! A mismatch or malformed envelope releases no carrier input to that transcription.
//! The gate never interprets the carried vocabulary or caller-owned Rust.

/// The generated-support schema identity this harness publishes, as raw bytes.
///
/// The gate matches the same decimal literals directly; the independent schema-currency lane compares this publication with the current declaration.
pub const PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID: &[u8; 32] = &[
    185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5, 84, 120, 104, 25,
    150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
];

/// Releases a schema-compatible carrier with this harness's hygienic crate binding.
///
/// # Grammar
///
/// ```text
/// generated_support! {
///     expected: [<the thirty-two published decimal bytes>],
///     trials: <carrier macro path>,
///     with: { <opaque carrier input> },
/// }
/// ```
///
/// The benchmark form replaces `trials` with `benches`.
/// Every clause is required, occurs in that order, and ends with a comma.
/// The producer fixes the expectation; the consuming target selects the gate and supplies its carrier input.
///
/// # Authority
///
/// A matching arm invokes the carrier's corresponding `@trials` or `@benches` transcription with this crate's `$crate` identity, its table stamp and the opaque input.
/// Generated carriers bind every framework-owned harness reference from that identity, so an invocation cannot independently choose another harness type.
/// A reexport of this macro still binds this defining crate, regardless of neighboring types or the dependency's name.
/// An independent `harness` clause is not admitted and releases no cargo.
///
/// The [descriptor gate contract](crate::descriptor#the-gate) owns schema coherence, currency and the supported-carrier evidence ceiling.
#[macro_export]
macro_rules! generated_support {
    (
        expected: [
            185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
            84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
        ],
        trials: $carrier:path,
        with: { $($input:tt)* },
    ) => {
        $carrier! { @trials { $crate } { $crate::trial_table } { $($input)* } }
    };
    (
        expected: [
            185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
            84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
        ],
        benches: $carrier:path,
        with: { $($input:tt)* },
    ) => {
        $carrier! { @benches { $crate } { $crate::bench_table } { $($input)* } }
    };
    (
        expected: [$($expected:literal),* $(,)?],
        trials: $carrier:path,
        with: { $($input:tt)* },
    ) => {
        $crate::generated_support! { @mismatch [$($expected),*] }
    };
    (
        expected: [$($expected:literal),* $(,)?],
        benches: $carrier:path,
        with: { $($input:tt)* },
    ) => {
        $crate::generated_support! { @mismatch [$($expected),*] }
    };
    (@mismatch [$($expected:literal),*]) => {
        ::core::compile_error!(::core::concat!(
            "generated_support!: the producer's expected schema does not match this harness; ",
            "both delivery seats are withheld. Producer expected: ",
            ::core::stringify!([$($expected),*]),
            ". Published here: ",
            ::core::stringify!([
                185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
                84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
            ]),
            ". Derive the current identity from the harness schema declaration and rewrite ",
            "both published holders together."
        ));
    };
    ($($invalid:tt)*) => {
        ::core::compile_error!(
            "generated_support!: expected a staged trials or benches carrier and its input; \
             the gate supplies the harness binding, and no delivery is released"
        );
    };
}

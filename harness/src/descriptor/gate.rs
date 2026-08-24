//! The pre-typecheck gate: where a producer's expected schema identity meets the one this harness publishes, before either side's material reaches type checking.
//!
//! A mismatch is one precise loud break rather than a cascade of field errors inside a generated table.
//! That is why the comparison is a macro arm: a `macro_rules!` pattern matches tokens, so it can refuse before the tokens it guards are parsed as Rust at all.
//! It is also why the pin crosses as decimal byte values — an unsuffixed integer has exactly one rendering, and a byte string has many.
//!
//! The gate is semantically ignorant of the deferred seat on purpose: it transports or it withholds, and it never reads.
//! A door that parsed the cargo would be a second authority over a vocabulary it does not own.
//!
//! The three drifts and where each dies: pair incoherence dies here, with the refusing arm telling whoever meets it how the pin is rewritten; joint staleness dies in `harness/tests/published_schema_currency/`; a changed constructor shape dies at the compiler as ordinary type errors before any trial runs.

/// The generated-support schema identity this harness publishes, as raw bytes.
///
/// One side of the two-sided pin, and the same thirty-two values each gate arm carries as literal tokens.
/// A `macro_rules!` arm matches tokens and cannot read a constant, so each arm carries the digits itself and the currency lane is what keeps the copies coherent.
///
/// It is deliberately not a [`GeneratedSupportSchemaId`](crate::descriptor::GeneratedSupportSchemaId), even though these bytes came off that derivation: a raw checked-in copy neither derives from the current declaration nor reifies an address whose derivation a caller established.
pub const PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID: &[u8; 32] = &[
    185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5, 84, 120, 104, 25,
    150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
];

/// Guards one generated support delivery: compares the producer's expected schema identity against the published one, and releases either the trial/deferred pair or the benchmark/reporter pair only when the two agree.
///
/// # The grammar
///
/// ```text
/// generated_support! {
///     expected: [<the thirty-two published bytes, in decimal>],
///     harness: <identifier>,
///     trials: { <the trial_table! payload, verbatim> },
///     deferred: { <opaque token trees, verbatim> },
/// }
///
/// generated_support! {
///     expected: [<the thirty-two published bytes, in decimal>],
///     harness: <identifier>,
///     benches: { <the bench_table! payload, verbatim> },
///     reporter: { <opaque token trees, verbatim> },
/// }
/// ```
///
/// - `expected:` is the producer's own copy of the published identity, as thirty-two unsuffixed decimal byte values.
///   Exactly one roster opens the gate; every other roster reaches the refusing arm.
/// - `harness:` is the identifier this crate is reached by at the invocation site — the rename twin's own name.
///   It is an identifier rather than a path because a captured path fragment cannot be extended with further segments in an expansion.
/// - `trials:` is the row road, whose grammar is [`trial_table!`](crate::trial_table)'s exhaustively and which this gate neither reads nor rewrites.
///   The seat may be empty: a producer whose whole cargo was deferred writes `trials: { }`, the matched arm stamps no table, and nothing is missing.
/// - `deferred:` is the opaque seat, forwarded verbatim on the matched road and withheld entirely on the refusing one.
/// - `benches:` is the benchmark-table road, whose grammar is [`bench_table!`](crate::bench_table)'s and whose admitted table is nonempty.
/// - `reporter:` is opaque benchmark-target cargo, released inside the same matched arm as the benchmark table.
///
/// Both seats of the selected form are always written, in that order.
/// Neither is optional: a seat a producer could omit is a seat it could place somewhere the pin does not reach.
///
/// # Authority
///
/// The declared harness identifier is load-bearing rather than decorative: the expansion writes one item naming this crate's schema-identity type through both the declared path and `$crate`, and the two must be one type.
/// A declaration that names another crate, or a name the consuming crate does not have, refuses at the door.
///
/// Agreement here means the two published sides are coherent, and nothing more.
/// Releasing cargo is transport and never endorsement: the gate says the pin matched, and says nothing about opaque tokens it never read.
///
/// The `@`-prefixed rules below are internal transcriptions, not invocation forms.
/// The forms above are text rather than compiled examples, because compiled rows need bindings that live on the challenge side.
#[macro_export]
macro_rules! generated_support {
    (
        expected: [
            185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
            84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
        ],
        harness: $harness:ident,
        trials: { $($trials:tt)* },
        deferred: { $($deferred:tt)* },
    ) => {
        // The declared harness identifier, proven to name THIS crate: one type, reached both ways, and a
        // function pointer that exists only if the two roads arrive at it. A wrong name refuses here, at
        // the door, rather than as an unresolved path somewhere inside either seat.
        const _: fn(
            $harness::descriptor::GeneratedSupportSchemaId,
        ) -> $crate::descriptor::GeneratedSupportSchemaId = ::core::convert::identity;

        $crate::generated_support! { @trials $($trials)* }

        // The deferred seat, verbatim, released INSIDE the matched arm — so the pin that governs the
        // constructors above governs this cargo by the same act rather than by a second arm that agreed
        // with the first.
        $($deferred)*
    };

    (
        expected: [
            185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
            84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
        ],
        harness: $harness:ident,
        benches: { $($benches:tt)* },
        reporter: { $($reporter:tt)* },
    ) => {
        const _: fn(
            $harness::descriptor::GeneratedSupportSchemaId,
        ) -> $crate::descriptor::GeneratedSupportSchemaId = ::core::convert::identity;

        $crate::generated_support! { @benches $($benches)* }
        $($reporter)*
    };

    // Pair incoherence: the producer's expectation is a literal, and it is not the published one. ONE
    // diagnostic, both sides shown, and no seat forwarded — the constructors and the deferred cargo are
    // bound here and dropped here.
    //
    // The sentence names this macro and no crate path: a consumer may rename this dependency, and a
    // compiler message has no way to learn the name it was renamed to.
    (
        expected: [$($expected:literal),* $(,)?],
        harness: $harness:ident,
        trials: { $($trials:tt)* },
        deferred: { $($deferred:tt)* },
    ) => {
        ::core::compile_error!(::core::concat!(
            "generated_support!: the producer's expected generated-support schema identity is \
             not the one this harness publishes, so the published pair is incoherent and \
             nothing this door was handed reaches the compiler: the trial constructors and \
             the deferred cargo are withheld together, because one arm releases both seats \
             and this is not that arm. Producer expected: ",
            ::core::stringify!([$($expected),*]),
            ". Published here: ",
            ::core::stringify!([
                185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
                84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
            ]),
            ". Declared harness: ",
            ::core::stringify!($harness),
            ". Both sides are rewritten together, in one change: derive the current value \
             from the harness's own published schema declaration, write it into the \
             producer's expectation and into this harness's published literal, and commit \
             the pair. A version-mixed consumer, a partial rewrite, and a hand edit to one \
             side are the three shapes this refusal has."
        ));
    };

    (
        expected: [$($expected:literal),* $(,)?],
        harness: $harness:ident,
        benches: { $($benches:tt)* },
        reporter: { $($reporter:tt)* },
    ) => {
        ::core::compile_error!(::core::concat!(
            "generated_support!: the producer's expected generated-support schema identity is \
             not the one this harness publishes, so the benchmark table and reporter cargo are \
             withheld together. Producer expected: ",
            ::core::stringify!([$($expected),*]),
            ". Published here: ",
            ::core::stringify!([
                185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5,
                84, 120, 104, 25, 150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
            ]),
            ". Declared harness: ",
            ::core::stringify!($harness),
            ". Derive the current value from the harness's published schema declaration and \
             rewrite both published holders together."
        ));
    };

    // An empty trials seat carried no rows, so it stamps nothing at all: the alternative would be handing
    // the stamp an empty declaration, which is not a form the stamp's grammar has.
    (@trials) => {};

    // A carried seat is the stamp's grammar, forwarded whole. One or more tokens, so this rule and the one
    // above partition the seat between them.
    (@trials $($trials:tt)+) => {
        $crate::trial_table! { $($trials)+ }
    };

    (@benches $($benches:tt)+) => {
        $crate::bench_table! { $($benches)+ }
    };
}

//! The pre-typecheck gate: the one place a producer's expected schema identity
//! is compared against the identity this harness publishes, BEFORE any material
//! the producer emitted reaches type checking.
//!
//! A mismatch here is one precise loud break — a single owned diagnostic naming
//! both sides — rather than a cascade of field errors somewhere inside a
//! generated table. That is the whole reason the comparison is a macro arm: a
//! `macro_rules!` pattern matches TOKENS, so it can refuse before the tokens it
//! guards are parsed as Rust at all.
//!
//! # One coupled pair, one pin
//!
//! Each delivery form carries one coupled pair: `trials:` with `deferred:`, or `benches:` with `reporter:`.
//! The matched arm releases both seats of the selected form; the refusing arm releases neither.
//!
//! That is what makes the pin's reach a fact of the expansion rather than a
//! description of where a producer chose to put things: everything a delivery
//! carries into a consumption target is handed to this door, and the door
//! answers for all of it in one act. The two seats share one arm, so they cross
//! together or they stay together.
//!
//! The gate stays semantically ignorant of the deferred seat on purpose: it
//! transports or it withholds, and it never reads. A door that parsed the cargo
//! would be a second authority over a vocabulary it does not own, and the
//! transport law is the same for cargo this home has never heard of.
//!
//! # The published side of the pin
//!
//! Two values in two crates hold the same identity: the producer's own
//! expectation travels with the code it emits, and this crate's published
//! literal sits here. Their independence is across UPGRADE TIME — both sides are
//! rewritten together, git-visible and human-committed, when the declaration
//! moves.
//!
//! # Rewriting the pin
//!
//! There is no command to run.
//! The act is three steps and they are named here because a reader following a schema change needs them: derive the current value through [`GeneratedSupportSchema::published`](crate::descriptor::GeneratedSupportSchema::published) and [`GeneratedSupportSchema::identity`](crate::descriptor::GeneratedSupportSchema::identity), rewrite both crates' literals to it, and commit the pair in one change.
//! The currency lane is what says whether the act has been performed.
//!
//! # The derived posture
//!
//! The bytes below came off that derivation. They remain a raw published copy rather than a [`GeneratedSupportSchemaId`](crate::descriptor::GeneratedSupportSchemaId): the typed identity has both a fresh declaration-derivation road and a typed reification road for an address whose derivation the caller already established, while this literal establishes neither by itself. The currency lane derives the current identity and compares this copy against it.
//!
//! The first pair was hand-authored, spelled a sentence, and stood under a
//! declared-bootstrap posture. It does not any more, and neither side has a road
//! back to one.
//!
//! # What the gate's comparison claims, and what it cannot
//!
//! It detects PAIR INCOHERENCE: a version-mixed consumer, a partial rewrite, or
//! a hand edit to one side. Inside one workspace, where both sides move
//! together, the live protection is the last two — and that limit is stated
//! rather than hidden.
//!
//! It cannot detect a JOINTLY STALE PAIR: the declaration changed and neither
//! literal was rewritten, so two old values still agree and the gate opens. That
//! is the currency lane's job, and the lane exists —
//! `harness/tests/published_schema_currency.rs` derives the identity from the current declaration and requires both published spellings to equal it.
//!
//! **The lane owns currency and nothing else.** Descriptor field traversal, origin metadata, and root membership are joined structurally at their owner; this lane neither establishes nor restates those mechanisms.
//!
//! Each drift named below dies at its stated seat; only this gate's own claim is narrow.
//! The routes are exact.
//! Pair incoherence dies here, at the gate.
//! Joint staleness dies in the harness's own currency lane, which derives the identity from the current declaration and requires both published spellings to equal it.
//! A changed constructor shape dies at the compiler as ordinary type errors before any trial runs.
//!
//! # The two sides must be one TOKEN, so the form is one nobody chooses
//!
//! A `macro_rules!` arm matches a literal by its TOKEN, so the pattern below and
//! what a producer writes have to be spelled the same way and not merely carry
//! the same bytes.
//!
//! The pin therefore crosses as a bracketed roster of thirty-two DECIMAL byte
//! values, and that form is the whole of what makes the comparison sound. A
//! byte-string literal has many spellings of one value — `b"\x71"` and `b"q"`
//! are one value and two tokens — and the producer's side is rendered by the
//! compiler's own literal writer, whose escaping is its choice rather than
//! anybody's declaration. An unsuffixed integer has exactly one rendering, so the
//! two sides agree by construction rather than by a human matching an escaping
//! convention they do not control.
//!
//! That is not a preference discovered on paper. The first producer to walk
//! through this door rendered its expectation as a byte string and was refused
//! here, with both spellings printed, over a value both sides agreed on — which
//! is the gate working exactly as designed and the reason the form moved.
//!
//! # Bounds
//!
//! The producer-facing rosters and the published constant below state one fact in the forms their seats require.
//! The currency and compile-refusal lanes keep each form's opening and refusing literals coherent with the freshly derived identity.

/// The generated-support schema identity this harness PUBLISHES, as raw bytes.
///
/// # Authority
///
/// This is one side of the two-sided pin, and it is the same thirty-two bytes each gate form carries as literal tokens.
/// A `macro_rules!` arm matches tokens and cannot read a constant, so each opening pattern and refusing diagnostic carries the digits itself.
/// The currency and compile-refusal lanes keep those literals coherent with the identity the current declaration derives.
///
/// # Bounds
///
/// Written in DECIMAL, in the same order and layout every gate arm uses, so the published copies are character-identical.
/// The arms have no choice about their base because an unsuffixed integer is the one literal form with exactly one rendering.
///
/// # Nonclaims
///
/// It is deliberately not a [`GeneratedSupportSchemaId`](crate::descriptor::GeneratedSupportSchemaId), even though these bytes were derived when the pair was published. A typed identity can be derived freshly from the declaration or reified from a `ContentAddress` whose derivation the caller already established; this raw checked-in copy does neither by itself. The currency lane re-derives the current identity and compares this copy against it.
pub const PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID: &[u8; 32] = &[
    222, 149, 109, 97, 135, 230, 254, 180, 55, 195, 41, 161, 180, 186, 130, 96, 170, 30, 123, 48,
    131, 30, 77, 129, 225, 115, 89, 175, 105, 68, 31, 161,
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
/// - `expected:` is the producer's own copy of the published identity, as a
///   bracketed roster of thirty-two unsuffixed decimal byte values. Exactly one
///   roster opens the gate; every other roster reaches the refusing arm. The form
///   is decimal because an unsuffixed integer has ONE rendering and a byte string
///   has many, and the producer's side is spelled by the compiler's own literal
///   writer rather than by a declaration.
/// - `harness:` is the identifier this crate is reached by at the invocation
///   site — the rename twin's own name. It is an identifier rather than a
///   general path because a captured path fragment cannot be extended with
///   further segments in an expansion, while an identifier composes into one
///   freely; a consumer's rename is an identifier, so nothing is lost.
/// - `trials:` is the row road. Its grammar is
///   [`trial_table!`](crate::trial_table)'s, exhaustively, and this gate neither
///   reads nor rewrites one token of it. The seat may be EMPTY, and an empty one
///   is a lawful delivery rather than a refusal: a producer whose whole cargo was
///   deferred writes `trials: { }`, the matched arm stamps no table, and nothing
///   is missing. Vacuity is judged where a run can see it — a declared suite that
///   pairs with no row is caught at the SELECTION, by the seat the stamp writes —
///   and a seat that was never declared has no run to be vacuous in.
/// - `deferred:` is the opaque seat. Its token trees are forwarded verbatim on
///   the matched road and withheld entirely on the refusing one, and this gate
///   parses none of them. What rides there is whatever an expansion deferred into
///   the delivery — the evaluation cargo the mutation crossing carries
///   into a test target — and the gate's ignorance of that is the design rather
///   than a gap. A delivery with nothing deferred writes `deferred: { }`.
/// - `benches:` is the benchmark-table road. Its grammar is [`bench_table!`](crate::bench_table)'s and its admitted table is nonempty.
/// - `reporter:` is opaque benchmark-target cargo. It is released inside the same matched arm as the benchmark table and may be empty where the target needs no generated renderer.
///
/// Both seats of the selected form are always written, in that order. Neither is optional: a seat a producer could omit is a seat it could place somewhere the pin does not reach.
///
/// # The crossings this door carries
///
/// Two of the wall's three crossings land in a test target, and both are inside
/// this one invocation: the row constructors in `trials:`, the mutation
/// crossing's evaluation cargo in `deferred:`. So one pin governs both of the
/// live crossings physically — a mismatch withholds them together, in one arm.
///
/// The third crossing lands in a bench target through the `benches:`/`reporter:` form. It answers to the same identity: the root declaration's members are the descriptor's, the mutation discovery's, and the bench's, so a change to the bench roster moves the one pin.
///
/// # Authority
///
/// The opening arms carry the published literal in their pattern. A producer whose expectation is different reaches its form's refusing arm, so neither carried seat is released into type checking. Each form releases both seats or withholds both; there is no half-delivery outcome.
///
/// The declared harness identifier is load-bearing rather than decorative: the
/// expansion writes one item that names this crate's own schema-identity type
/// through BOTH the declared path and `$crate`, and the two must be one type. A
/// declaration that names another crate, or a name the consuming crate does not
/// have, refuses at the door.
///
/// # Nonclaims
///
/// Agreement here means the two published sides are COHERENT. It is not evidence
/// that either side is current: a pair that agrees because neither literal was
/// rewritten is exactly what this comparison cannot see, and pair currency is
/// the currency lane's. This page's module states the disposal routes for every
/// drift the gate does not catch.
///
/// Releasing deferred or reporter cargo is transport and never endorsement. The gate says the pin matched; it says nothing about opaque tokens it never read.
///
/// # Bounds
///
/// The `@`-prefixed rules below are the gate's internal transcriptions of trial and benchmark seats, not invocation forms.
///
/// The form above is shown as text rather than as a compiled example, for the
/// reason the two table stamps' pages give: compiled rows need challenge-side bindings.
#[macro_export]
macro_rules! generated_support {
    (
        expected: [
            222, 149, 109, 97, 135, 230, 254, 180, 55, 195, 41, 161, 180, 186, 130, 96,
            170, 30, 123, 48, 131, 30, 77, 129, 225, 115, 89, 175, 105, 68, 31, 161,
        ],
        harness: $harness:ident,
        trials: { $($trials:tt)* },
        deferred: { $($deferred:tt)* },
    ) => {
        // The declared harness identifier, proven to name THIS crate: one type,
        // reached both ways, and a function pointer that exists only if the two
        // roads arrive at it. A wrong name refuses here, at the door, rather
        // than as an unresolved path somewhere inside either seat.
        const _: fn(
            $harness::descriptor::GeneratedSupportSchemaId,
        ) -> $crate::descriptor::GeneratedSupportSchemaId = ::core::convert::identity;

        // The trials seat, through the one transcription that tells an empty
        // seat from a carried one: an empty seat stamps nothing, and a carried
        // one is the stamp's own grammar, forwarded whole and unread.
        $crate::generated_support! { @trials $($trials)* }

        // The deferred seat, verbatim, released INSIDE the matched arm. Written
        // once and released once, so the pin that governs the constructors above
        // governs this cargo by the same act rather than by a second arm that
        // agreed with the first.
        $($deferred)*
    };

    (
        expected: [
            222, 149, 109, 97, 135, 230, 254, 180, 55, 195, 41, 161, 180, 186, 130, 96,
            170, 30, 123, 48, 131, 30, 77, 129, 225, 115, 89, 175, 105, 68, 31, 161,
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

    // Pair incoherence: the producer's expectation is a literal, and it is not
    // the published one. ONE diagnostic, both sides shown, and no seat
    // forwarded — the constructors and the deferred cargo are bound here and
    // dropped here, so neither reaches the compiler.
    //
    // The sentence names this macro and no crate path: a consumer may rename
    // this dependency, and a compiler message has no way to learn the name it
    // was renamed to.
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
                222, 149, 109, 97, 135, 230, 254, 180, 55, 195, 41, 161, 180, 186, 130, 96,
                170, 30, 123, 48, 131, 30, 77, 129, 225, 115, 89, 175, 105, 68, 31, 161,
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
                222, 149, 109, 97, 135, 230, 254, 180, 55, 195, 41, 161, 180, 186, 130, 96,
                170, 30, 123, 48, 131, 30, 77, 129, 225, 115, 89, 175, 105, 68, 31, 161,
            ]),
            ". Declared harness: ",
            ::core::stringify!($harness),
            ". Derive the current value from the harness's published schema declaration and \
             rewrite both published holders together."
        ));
    };

    // THE TRANSCRIPTION of the trials seat. An empty seat is a delivery that
    // carried no rows, so it stamps nothing at all — the alternative would be
    // handing the stamp an empty declaration, which is not a form the stamp's
    // grammar has.
    (@trials) => {};

    // A carried seat is the stamp's grammar, forwarded whole. One or more
    // tokens, so this rule and the one above partition the seat between them.
    (@trials $($trials:tt)+) => {
        $crate::trial_table! { $($trials)+ }
    };

    (@benches $($benches:tt)+) => {
        $crate::bench_table! { $($benches)+ }
    };
}

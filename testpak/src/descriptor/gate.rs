//! The pre-typecheck gate: the one place a producer's expected schema identity
//! is compared against the identity this harness publishes, BEFORE any
//! constructor the producer emitted reaches type checking.
//!
//! A mismatch here is one precise loud break — a single owned diagnostic naming
//! both sides — rather than a cascade of field errors somewhere inside a
//! generated table. That is the whole reason the comparison is a macro arm: a
//! `macro_rules!` pattern matches TOKENS, so it can refuse before the tokens it
//! guards are parsed as Rust at all.
//!
//! # The published side of the pin
//!
//! Two values in two crates hold the same identity: the producer's own
//! expectation travels with the code it emits, and this crate's published
//! literal sits here. Their independence is across UPGRADE TIME — both sides are
//! written by one explicit publication operation at schema-change time,
//! git-visible, human-committed, under a receipt.
//!
//! # The bootstrap posture
//!
//! The first pair is HAND-AUTHORED, and this side of it says so out loud: the
//! bytes below are not a derived address and do not pretend to be one. They
//! spell a sentence rather than a digest, they are declared as raw bytes rather
//! than as a [`GeneratedSupportSchemaId`](crate::descriptor::GeneratedSupportSchemaId)
//! — whose only construction road is derivation — and the type system therefore
//! refuses to let a bootstrap placeholder impersonate a derived identity.
//!
//! What the pair claims under this posture is exactly one thing: the two sides
//! were written coherently by one hand. At the first toolchain contact the pair
//! becomes VERIFIED-DERIVED — the current declaration's identity is derived, both
//! sides are rewritten to it, and that flip is itself a receipted,
//! human-committed publication act rather than an edit somebody made.
//!
//! # What the gate's comparison claims, and what it cannot
//!
//! It detects PAIR INCOHERENCE: a version-mixed consumer, a partial publication,
//! or a hand edit to one side. Inside one workspace, where both sides move
//! together, the live protection is the last two — and that limit is stated
//! rather than hidden.
//!
//! It cannot detect a JOINTLY STALE PAIR: the declaration changed and
//! publication never ran, so two old literals still agree and the gate opens.
//! Pair currency is the conformance trial's job, which derives the current
//! declaration's identity and checks the published literal against it.
//!
//! Every drift still dies; only this gate's own claim is narrow. The routes,
//! exactly: pair incoherence dies here, at the gate. Joint staleness dies at
//! whichever tripwire the drift reaches first — a changed constructor shape is
//! rejected by the compiler as ordinary type errors before any trial runs, and a
//! stale surface that still typechecks is rejected by the conformance trial.

/// The generated-support schema identity this harness PUBLISHES, as raw bytes.
///
/// # Authority
///
/// This is one side of the two-sided pin, and it is the same thirty-two bytes
/// the gate's own arm carries as a literal token. Two spellings of one published
/// fact, stated rather than hidden: a `macro_rules!` arm matches tokens and
/// cannot read a constant, so the comparison must carry the literal itself. The
/// conformance trial is what holds the two spellings together, along with
/// holding both against the identity the current declaration actually derives.
///
/// # Nonclaims
///
/// It is deliberately NOT a
/// [`GeneratedSupportSchemaId`](crate::descriptor::GeneratedSupportSchemaId).
/// The only road to that type is derivation from a declaration's canonical
/// bytes, and these bytes were not derived from anything — they are the
/// declared-bootstrap placeholder the first hand-authored pair stands on. Being
/// unable to build the typed identity out of them is the point: nothing in this
/// crate can mistake the bootstrap value for a derived one.
pub const PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID: &[u8; 32] =
    b"threadpak-generated-support-sche";

/// Guards one generated support declaration: compares the producer's expected
/// schema identity against the published one, and releases the payload to
/// [`trial_table!`](crate::trial_table) only when the two agree.
///
/// # The grammar
///
/// ```text
/// generated_support! {
///     expected: b"<the thirty-two published bytes>",
///     harness: <identifier>,
///     <the trial_table! payload, verbatim>
/// }
/// ```
///
/// - `expected:` is the producer's own copy of the published identity, as a
///   literal. Exactly one literal opens the gate; every other literal reaches
///   the refusing arm.
/// - `harness:` is the identifier this crate is reached by at the invocation
///   site — the rename twin's own name. It is an identifier rather than a
///   general path because a captured path fragment cannot be extended with
///   further segments in an expansion, while an identifier composes into one
///   freely; a consumer's rename is an identifier, so nothing is lost.
/// - Everything after those two clauses is the payload, forwarded verbatim. Its
///   grammar is [`trial_table!`](crate::trial_table)'s, exhaustively, and this
///   gate neither reads nor rewrites one token of it. All three crossings the
///   wall declares pass this one pin, and the trial-table grammar is the only
///   payload published today: a bench payload becomes expressible through this
///   same door when the bench seat fills, and until it does this gate forwards
///   that one grammar and nothing else.
///
/// # Authority
///
/// The opening arm carries the published literal in its PATTERN. A producer
/// whose expectation is a different literal does not match that arm, so its
/// constructors are never released into type checking and the reader gets one
/// sentence instead of a field-error cascade.
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
/// that either side is current: a pair that agrees because publication never ran
/// is exactly what this comparison cannot see, and pair currency is the
/// conformance trial's. This page's module states the disposal routes for every
/// drift the gate does not catch.
///
/// # Bounds
///
/// The form above is shown as text rather than as a compiled example, for the
/// reason [`trial_table!`](crate::trial_table)'s page gives: a compiled one needs
/// a subject, a check, and a population, and those live on the challenge side.
#[macro_export]
macro_rules! generated_support {
    (
        expected: b"threadpak-generated-support-sche",
        harness: $harness:ident,
        $($payload:tt)*
    ) => {
        // The declared harness identifier, proven to name THIS crate: one type,
        // reached both ways, and a function pointer that exists only if the two
        // roads arrive at it. A wrong name refuses here, at the door, rather
        // than as an unresolved path somewhere inside the payload.
        const _: fn(
            $harness::descriptor::GeneratedSupportSchemaId,
        ) -> $crate::descriptor::GeneratedSupportSchemaId = ::core::convert::identity;

        $crate::trial_table! { $($payload)* }
    };

    // Pair incoherence: the producer's expectation is a literal, and it is not
    // the published one. ONE diagnostic, both sides shown, and no payload
    // forwarded — the constructors never reach the compiler.
    //
    // The sentence names this macro and no crate path: a consumer may rename
    // this dependency, and a compiler message has no way to learn the name it
    // was renamed to.
    (
        expected: $expected:literal,
        harness: $harness:ident,
        $($payload:tt)*
    ) => {
        ::core::compile_error!(::core::concat!(
            "generated_support!: the producer's expected generated-support schema identity is \
             not the one this harness publishes, so the published pair is incoherent and no \
             constructor is released into type checking. Producer expected: ",
            ::core::stringify!($expected),
            ". Published here: ",
            ::core::stringify!(b"threadpak-generated-support-sche"),
            ". Declared harness: ",
            ::core::stringify!($harness),
            ". Exactly one act writes both sides: re-run the publication operation that \
             rewrites the producer's expectation and this harness's published literal \
             together, under one receipt. A version-mixed consumer, a partial publication, \
             and a hand edit to one side are the three shapes this refusal has."
        ));
    };
}

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
//! # Two named seats, one pin
//!
//! The gate is handed everything a delivery carries into a consumption target,
//! in two named seats. `trials:` carries the row constructors, under
//! [`trial_table!`](crate::trial_table)'s own grammar. `deferred:` carries the
//! cargo an expansion deferred into the delivery, as opaque token trees. The
//! matched arm releases BOTH; the refusing arm releases NEITHER.
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
//! There is no command to run. The act is three steps and they are named here
//! because a reader following a schema change needs them: derive the current
//! value through [`GeneratedSupportSchema::published`] and
//! [`GeneratedSupportSchema::identity`](crate::descriptor::GeneratedSupportSchema::identity),
//! rewrite both crates' literals to it, and commit the pair in one change. The
//! currency lane is what says whether the act has been performed.
//!
//! # The derived posture
//!
//! The bytes below came off that derivation. They are declared as raw bytes
//! rather than as a
//! [`GeneratedSupportSchemaId`](crate::descriptor::GeneratedSupportSchemaId) —
//! whose only construction road is derivation performed at the moment it is
//! asked for — so a checked-in COPY cannot pass as a fresh derivation anywhere
//! that type is accepted.
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
//! `testpak/tests/published_schema_currency.rs` derives the identity from the
//! current declaration and requires both published spellings to equal it.
//!
//! **The lane owns currency and nothing else.** It does not establish that
//! `Row`, `DESCRIPTOR_FIELDS`, `Origin`, the encoder's slots, and the schema's
//! field rosters are one structural declaration. They are parallel facts, a
//! master declaration is what would join them, and no lane here pretends to.
//!
//! Every drift still dies; only this gate's own claim is narrow. The routes,
//! exactly: pair incoherence dies here, at the gate. Joint staleness dies in the
//! harness's own currency lane, which derives the identity from the current
//! declaration and requires both published spellings to equal it, and a changed
//! constructor shape dies at the compiler as ordinary type errors before any
//! trial runs.
//!
//! # The two spellings must be one TOKEN
//!
//! A `macro_rules!` arm matches a literal by its token, so the pattern below and
//! the literal a producer writes have to be spelled the same way and not merely
//! carry the same bytes. The published value is a derived identity now rather
//! than the readable phrase the first hand-authored pair used, so both sides
//! spell every byte as `\xNN`: one uniform form, chosen because it is the one a
//! writer cannot get subtly different.
//!
//! The producer renders its literal from the expectation's VALUE
//! (`GeneratedToken::ByteText`), so the spelling on that side is the token
//! tree's. Where the two forms disagree, this gate refuses and its own
//! diagnostic prints both — a visible refusal at the first invocation rather
//! than a silent mismatch.
//!
//! # Bounds
//!
//! **No caller invokes this gate today.** The crossing it guards is the row road
//! a generated support delivery carries, and the declaration family that would
//! emit one has not been admitted at the derive's door yet. The pin is current
//! and the gate is written; what it is waiting for is the producer that walks
//! through it.

/// The generated-support schema identity this harness PUBLISHES, as raw bytes.
///
/// # Authority
///
/// This is one side of the two-sided pin, and it is the same thirty-two bytes
/// the gate's own arm carries as a literal token. Two spellings of one published
/// fact, stated rather than hidden: a `macro_rules!` arm matches tokens and
/// cannot read a constant, so the comparison must carry the literal itself. The
/// currency lane is what holds the two spellings together, along with holding
/// both against the identity the current declaration actually derives.
///
/// # Nonclaims
///
/// It is deliberately NOT a
/// [`GeneratedSupportSchemaId`](crate::descriptor::GeneratedSupportSchemaId),
/// even though these bytes were derived. That type's only road is derivation
/// from a declaration's canonical bytes, performed at the moment it is asked
/// for; this is a checked-in COPY of one such derivation, and a copy that could
/// wear the typed identity would let a stale copy pass as a fresh derivation
/// anywhere the type is accepted. What holds the copy current is the currency
/// lane, which re-derives and compares.
pub const PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID: &[u8; 32] =
    b"\x71\x16\xd7\x1b\xc9\x53\x2d\xb1\xe4\x7b\x9a\xff\xef\x11\x63\x38\x96\x2d\x4e\x91\x90\xfa\x4b\x0a\x3c\x21\x4a\x93\x11\xbb\x4d\x93";

/// Guards one generated support delivery: compares the producer's expected
/// schema identity against the published one, and releases BOTH of its seats —
/// the trial payload to [`trial_table!`](crate::trial_table), and the deferred
/// cargo verbatim — only when the two agree.
///
/// # The grammar
///
/// ```text
/// generated_support! {
///     expected: b"<the thirty-two published bytes>",
///     harness: <identifier>,
///     trials: { <the trial_table! payload, verbatim> },
///     deferred: { <opaque token trees, verbatim> },
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
///   the delivery — today, the evaluation cargo the mutation crossing carries
///   into a test target — and the gate's ignorance of that is the design rather
///   than a gap. A delivery with nothing deferred writes `deferred: { }`.
///
/// Both seats are always written, in that order. Neither is optional, and that
/// is the point: a seat a producer could omit is a seat a producer could place
/// somewhere the pin does not reach.
///
/// # The crossings this door carries
///
/// Two of the wall's three crossings land in a test target, and both are inside
/// this one invocation: the row constructors in `trials:`, the mutation
/// crossing's evaluation cargo in `deferred:`. So one pin governs both of the
/// live crossings physically — a mismatch withholds them together, in one arm.
///
/// The third crossing lands in a BENCH target. It rides the same one delivery
/// and answers to the same one identity — the root declaration's members are the
/// descriptor's, the mutation point's, and the bench's, so a change to the bench
/// roster moves the pin like any other member — and the invocation arm that
/// carries a bench payload through this door arrives when the reserved bench
/// seat fills. Until it does, the two seats above are the two this door
/// declares, and the bench crossing's own opening condition is stated rather
/// than papered over.
///
/// # Authority
///
/// The opening arm carries the published literal in its PATTERN. A producer
/// whose expectation is a different literal does not match that arm, so neither
/// seat is released into type checking and the reader gets one sentence instead
/// of a field-error cascade. One arm releases both seats and one arm withholds
/// both, so there is no third outcome in which half the delivery got through.
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
/// Releasing the deferred seat is transport and never endorsement. The gate says
/// the pin matched; it says nothing about what those tokens mean, because it
/// never read them.
///
/// # Bounds
///
/// The `@`-prefixed rules below are the gate's internal transcription of the
/// trials seat — the one place an empty seat is told from a carried one — and
/// not invocation forms.
///
/// The form above is shown as text rather than as a compiled example, for the
/// reason [`trial_table!`](crate::trial_table)'s page gives: a compiled one needs
/// a subject, a check, and a population, and those live on the challenge side.
#[macro_export]
macro_rules! generated_support {
    (
        expected: b"\x71\x16\xd7\x1b\xc9\x53\x2d\xb1\xe4\x7b\x9a\xff\xef\x11\x63\x38\x96\x2d\x4e\x91\x90\xfa\x4b\x0a\x3c\x21\x4a\x93\x11\xbb\x4d\x93",
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

    // Pair incoherence: the producer's expectation is a literal, and it is not
    // the published one. ONE diagnostic, both sides shown, and no seat
    // forwarded — the constructors and the deferred cargo are bound here and
    // dropped here, so neither reaches the compiler.
    //
    // The sentence names this macro and no crate path: a consumer may rename
    // this dependency, and a compiler message has no way to learn the name it
    // was renamed to.
    (
        expected: $expected:literal,
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
            ::core::stringify!($expected),
            ". Published here: ",
            ::core::stringify!(b"\x71\x16\xd7\x1b\xc9\x53\x2d\xb1\xe4\x7b\x9a\xff\xef\x11\x63\x38\x96\x2d\x4e\x91\x90\xfa\x4b\x0a\x3c\x21\x4a\x93\x11\xbb\x4d\x93"),
            ". Declared harness: ",
            ::core::stringify!($harness),
            ". Both sides are rewritten together, in one change: derive the current value \
             from the harness's own published schema declaration, write it into the \
             producer's expectation and into this harness's published literal, and commit \
             the pair. A version-mixed consumer, a partial rewrite, and a hand edit to one \
             side are the three shapes this refusal has."
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
}

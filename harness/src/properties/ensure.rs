//! The `ensure_*` battery: a thin transparent stamp from an owner's outcome to
//! this home's typed conclusion.
//!
//! Each macro expands to exactly one call into the conclusion nucleus
//! ([`concluded`](crate::properties::concluded),
//! [`admitted`](crate::properties::admitted),
//! [`refused`](crate::properties::refused)) and adds nothing else. Nothing here
//! panics, asserts, unwraps, or indexes: a check that failed hands back a
//! [`TrialConclusion`](crate::report::TrialConclusion) carrying its evidence, the
//! way the rest of this harness fails.
//!
//! # Ambient provenance for free
//!
//! The calls these macros expand to are `#[track_caller]`, and a macro
//! expansion's caller is the site that INVOKED the macro. So every refusal is
//! born knowing the file and line of the owner's check, with no panic machinery
//! anywhere on the road — the caller-tracking on the value-returning nucleus is
//! the whole mechanism.
//!
//! # Refusals are matched by value
//!
//! [`ensure_refused_with!`](crate::ensure_refused_with) takes a PATTERN over the
//! owner's own refusal type, composed by the compiler. No macro here reads a
//! message, a rendering, or any other phrase: a check that matched on prose
//! would be judging a diagnostic rather than a value, and would keep passing
//! after the value it meant to name had changed underneath the words.

/// Concludes that one outcome answered where an answer was owed.
///
/// # The grammar
///
/// ```text
/// ensure_ok!(<outcome expression>, <cause expression>)
/// ```
///
/// - `<outcome expression>` evaluates to a `Result` in the owner's own
///   vocabulary. It is borrowed, never consumed, so the caller keeps it.
/// - `<cause expression>` evaluates to a
///   [`FindingCause`](crate::report::FindingCause) — the identity pair a refusal
///   is cited under. It is required rather than defaulted, so a finding always
///   names a cause its owner wrote down;
///   [`ANSWER_EXPECTED`](crate::properties::ANSWER_EXPECTED) is the paved value
///   for an owner with nothing more specific to say.
///
/// # What it expands to
///
/// One call to [`admitted`](crate::properties::admitted), which is what states
/// the failure class. Nothing else is written into the expansion.
#[macro_export]
macro_rules! ensure_ok {
    ($outcome:expr, $cause:expr $(,)?) => {
        $crate::properties::admitted(&$outcome, $cause)
    };
}

/// Concludes that one outcome refused where a refusal was owed.
///
/// # The grammar
///
/// ```text
/// ensure_refused!(<outcome expression>, <cause expression>)
/// ```
///
/// The two arguments are [`ensure_ok!`](crate::ensure_ok)'s, and
/// [`REFUSAL_EXPECTED`](crate::properties::REFUSAL_EXPECTED) is the paved cause.
///
/// # Nonclaims
///
/// It reads that the subject refused and nothing about WHICH refusal it
/// answered with. An owner who owes the exact refusal reaches for
/// [`ensure_refused_with!`](crate::ensure_refused_with), because a check that
/// admitted any refusal at all passes when the subject refused for a reason
/// nobody intended.
#[macro_export]
macro_rules! ensure_refused {
    ($outcome:expr, $cause:expr $(,)?) => {
        $crate::properties::refused(&$outcome, $cause)
    };
}

/// Concludes that one outcome refused with the refusal it owed.
///
/// # The grammar
///
/// ```text
/// ensure_refused_with!(<outcome expression>, <refusal pattern>, <cause expression>)
/// ensure_refused_with!(
///     <outcome expression>,
///     <refusal pattern> if <guard expression>,
///     <cause expression>
/// )
/// ```
///
/// - `<refusal pattern>` is an ordinary Rust pattern over the owner's own
///   refusal type, matched against the outcome's error seat. Naming a variant
///   pins the family; binding its payload and adding a guard pins a typed VALUE.
/// - `<guard expression>` is the optional guard, evaluated over whatever the
///   pattern bound. The bindings arrive by reference, because the outcome is
///   borrowed rather than consumed.
/// - `<cause expression>` is [`ensure_ok!`](crate::ensure_ok)'s.
///
/// # Authority
///
/// The match is composed by the compiler over the owner's typed refusal. A
/// pattern that names a variant the type does not have, or binds a payload it
/// does not carry, is a compile refusal rather than a check that quietly never
/// matches.
///
/// # Bounds
///
/// The pattern must be refutable — a pattern that matches every refusal makes
/// the expansion's second arm unreachable, and the compiler says so. That is the
/// intended reading: a check demanding "some refusal, any refusal" is
/// [`ensure_refused!`](crate::ensure_refused), which states that claim in its
/// name.
#[macro_export]
macro_rules! ensure_refused_with {
    ($outcome:expr, $refusal:pat $(if $guard:expr)?, $cause:expr $(,)?) => {
        $crate::properties::concluded(
            if ::core::matches!(
                &$outcome,
                ::core::result::Result::Err($refusal) $(if $guard)?
            ) {
                $crate::properties::Holding::Holds
            } else {
                $crate::properties::Holding::Fails
            },
            $crate::report::FailureClass::RefusedByCheck,
            $cause,
        )
    };
}

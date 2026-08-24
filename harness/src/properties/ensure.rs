//! The `ensure_*` battery: a thin transparent stamp from an owner's outcome to this home's typed conclusion.
//!
//! Each macro expands to exactly one call into the conclusion nucleus and adds nothing else, and nothing here panics, asserts, unwraps, or indexes.
//!
//! Those calls are `#[track_caller]` and a macro expansion's caller is the site that invoked the macro, so every refusal is born knowing the file and line of the owner's check.

/// Concludes that one outcome answered where an answer was owed.
///
/// ```text
/// ensure_ok!(<outcome expression>, <cause expression>)
/// ```
///
/// - `<outcome expression>` evaluates to a `Result` in the owner's own vocabulary, borrowed rather than consumed, so the caller keeps it.
/// - `<cause expression>` evaluates to a [`FindingCause`](crate::report::FindingCause), required rather than defaulted so a finding always names a cause its owner wrote down; [`ANSWER_EXPECTED`](crate::properties::ANSWER_EXPECTED) is the paved value.
#[macro_export]
macro_rules! ensure_ok {
    ($outcome:expr, $cause:expr $(,)?) => {
        $crate::properties::admitted(&$outcome, $cause)
    };
}

/// Concludes that one outcome refused where a refusal was owed.
///
/// The two arguments are [`ensure_ok!`](crate::ensure_ok)'s and [`REFUSAL_EXPECTED`](crate::properties::REFUSAL_EXPECTED) is the paved cause; it reads that the subject refused and nothing about which refusal, so an owner who owes the exact one reaches for [`ensure_refused_with!`](crate::ensure_refused_with).
#[macro_export]
macro_rules! ensure_refused {
    ($outcome:expr, $cause:expr $(,)?) => {
        $crate::properties::refused(&$outcome, $cause)
    };
}

/// Concludes that one outcome refused with the refusal it owed.
///
/// ```text
/// ensure_refused_with!(<outcome expression>, <refusal pattern>, <cause expression>)
/// ensure_refused_with!(<outcome expression>, <refusal pattern> if <guard>, <cause expression>)
/// ```
///
/// - `<refusal pattern>` is an ordinary Rust pattern over the owner's own refusal type, matched against the outcome's error seat; naming a variant pins the family, and binding its payload under a guard pins a typed value.
/// - `<guard>` is evaluated over whatever the pattern bound, and the bindings arrive by reference because the outcome is borrowed; the cause is [`ensure_ok!`](crate::ensure_ok)'s.
///
/// The match is composed by the compiler over the owner's typed refusal, so a pattern naming a variant the type does not have is a compile refusal rather than a check that quietly never matches.
/// The pattern must be refutable: one that matches every refusal makes the second arm unreachable, and "some refusal, any refusal" is [`ensure_refused!`](crate::ensure_refused), which states that claim in its name.
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

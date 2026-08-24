//! The damages a mutation may inflict on a subject, banked as rows rather than hidden inside a planner.
//!
//! Each row states what an operator of its family attacks and what a surviving mutant of it means for the suite that let it live; choosing a point and applying a damage are [`crate::muterprater`]'s.

use super::types::OperatorFamily;

/// Moves a comparison off the edge it was written on — `<` for `<=`, `>` for `>=`, or either for its negation.
///
/// The mutated subject agrees with the lawful one everywhere except at the single value the comparison exists to place, so a survivor names a declared bound whose edge no row in the table stands on; the candidate that closes it is an input rather than a new check.
pub const COMPARISON_BOUNDARIES: OperatorFamily =
    OperatorFamily::declared("comparison-boundaries", "the exact edge a comparison draws");

/// Rewrites the connective a decision is made through: `&&` becomes `||`, or a condition becomes its negation.
///
/// Every condition the lawful subject consults is still consulted, so what changed is which combination of them decides; a survivor names the condition no input ever made the deciding one.
pub const BOOLEAN_OPERATORS: OperatorFamily =
    OperatorFamily::declared("boolean-operators", "the shape of a decision");

/// Answers a refusal as a success: the `Err` road rewritten into the `Ok` one.
///
/// Where failure is a returned value rather than a raised one, a refusal is an ordinary value that one rewritten road can simply stop producing with nothing raised for a runner to notice; a survivor means the subject was only ever handed inputs it accepts, so every claim about how it says no rests on a road no row walks.
pub const RESULT_COLLAPSE: OperatorFamily =
    OperatorFamily::declared("result-collapse", "the road a refusal travels");

/// Answers an absence as a presence: the `None` road rewritten into a `Some`.
///
/// A road that refuses when a value is missing and proceeds when it is present is fail-closed and this is its exact reversal, so a survivor names a road whose closed direction no row walks — and the closed direction is the only one that carries evidence.
pub const OPTION_COLLAPSE: OperatorFamily =
    OperatorFamily::declared("option-collapse", "the road an absence travels");

/// Drops a returned value at a call site written to read it.
///
/// A producer can be word-perfect about how it refuses while one line at one call site throws the refusal away, and a refusal discarded is indistinguishable from one that never happened; a survivor names a call whose result the table never observes.
pub const IGNORED_RESULTS: OperatorFamily = OperatorFamily::declared(
    "ignored-results",
    "the fate of a value a caller was written to read",
);

/// Replaces an owner-declared input with a fact taken straight off the host — a clock, an environment value, an entropy source, a filesystem answer.
///
/// A subject holds this seam when no host fact reaches a semantic result except through a declared input, and one line of contact reverses it; a survivor names a result that comes out the same whichever fact arrives, which is exactly the shape under which ambient contact is invisible.
///
/// The family is about the subject's contact and never the harness's: a harness reads the host facts it needs in order to run, and what a damage of this family attacks is a host fact entering a semantic result.
pub const DIRECT_MACHINE_CONTACT: OperatorFamily = OperatorFamily::declared(
    "direct-machine-contact",
    "the seam between a declared input and the host it was declared instead of",
);

/// Replaces a constructor's body with a default value, or with a call back into itself.
///
/// One family and two damages, because both land in the same nucleus — a default-minted value never met the guard and a self-call never produces a value at all — and a survivor means the guard's refusal is required by no row, so the type's promise is carried by whoever calls the constructor rather than by the construction.
pub const CONSTRUCTOR_AND_DEFAULT_RECURSION: OperatorFamily = OperatorFamily::declared(
    "constructor-and-default-recursion",
    "the invariant nucleus a smart constructor holds",
);

/// Exchanges exactly one adjacent pair in an owner-declared semantic order.
///
/// Length, membership, every member identity and byte, and every fact that is not order stay as they were, so one order-bearing projection is attacked without disturbing a neighboring one that states the same roster for another purpose; a kill proves the witness distinguishes that exchange and a survivor says only that it did not, with neither saying which order is owner-correct without the owner's declaration.
pub const DECLARED_ORDER_PERMUTATION: OperatorFamily = OperatorFamily::declared(
    "declared-order-permutation",
    "the adjacency of an owner-declared semantic order",
);

/// The declared operator families, in the order this bank states them.
pub const OPERATOR_FAMILIES: &[OperatorFamily] = &[
    COMPARISON_BOUNDARIES,
    BOOLEAN_OPERATORS,
    RESULT_COLLAPSE,
    OPTION_COLLAPSE,
    IGNORED_RESULTS,
    DIRECT_MACHINE_CONTACT,
    CONSTRUCTOR_AND_DEFAULT_RECURSION,
    DECLARED_ORDER_PERMUTATION,
];

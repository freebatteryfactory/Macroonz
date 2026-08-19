//! The mutation operator families: which damages this harness is willing to
//! inflict on a subject, banked as rows rather than hidden inside a planner.
//!
//! # Targeting
//!
//! An operator family earns its seat by naming a way this machine's own laws
//! could be broken quietly. Four of the seven are aimed at three such ways, and
//! each names its target at its own row: [`RESULT_COLLAPSE`] and
//! [`IGNORED_RESULTS`] at error-swallowing, [`OPTION_COLLAPSE`] at fail-closed
//! erosion, and [`DIRECT_MACHINE_CONTACT`] at ambient contact. The remaining
//! three — [`COMPARISON_BOUNDARIES`], [`BOOLEAN_OPERATORS`], and
//! [`CONSTRUCTOR_AND_DEFAULT_RECURSION`] — are aimed at the edges, the
//! decisions, and the constructors every one of those laws is written in terms
//! of.
//!
//! A survivor is the finding, never the score: what a surviving mutant of a
//! family means for the suite that let it live is written at the family's row,
//! because that sentence is what a survivor explanation turns into a candidate
//! with.

use super::types::OperatorFamily;

/// Moves a comparison off the edge it was written on.
///
/// The damage is one token: `<` for `<=`, `>` for `>=`, or either for its
/// negation, so the mutated subject agrees with the lawful one everywhere
/// except at the single value the comparison exists to place.
///
/// A bound whose edge no input ever visits is a bound nobody proved, and a
/// declared bound yields its own edge population for free — the generation
/// contract owns that ladder. A survivor here therefore names a declared bound
/// whose edge no row in the table stands on, and the candidate that closes it
/// is an input, not a new check.
pub const COMPARISON_BOUNDARIES: OperatorFamily = OperatorFamily {
    slug: "comparison-boundaries",
    attacks: "the exact edge a comparison draws",
};

/// Rewrites the connective a decision is made through.
///
/// `&&` becomes `||`, or a condition becomes its negation. The mutated subject
/// still consults every condition the lawful one did; what changed is which
/// combination of them decides.
///
/// A decision over two conditions that the suite has only ever seen agree is a
/// decision the suite has never actually read. A survivor here names the
/// condition no input ever made the deciding one — so the explanation is
/// specific enough to author against: which condition, and which combination of
/// values the table never carries.
pub const BOOLEAN_OPERATORS: OperatorFamily = OperatorFamily {
    slug: "boolean-operators",
    attacks: "the shape of a decision",
};

/// Answers a refusal as a success: the `Err` road rewritten into the `Ok` one.
///
/// Its target is ERROR-SWALLOWING. Failure in this machine is a returned typed
/// value rather than a raised one, and the harness fails the same way — which
/// means every refusal anywhere in reach is an ordinary value that one rewritten
/// road can simply stop producing, with nothing raised for a runner to notice.
///
/// A survivor here means the subject was only ever handed inputs it accepts:
/// the refusal arm is prose, its typed cause is carried by nobody, and every
/// downstream claim about how the subject says no rests on a road no row walks.
pub const RESULT_COLLAPSE: OperatorFamily = OperatorFamily {
    slug: "result-collapse",
    attacks: "the road a refusal travels",
};

/// Answers an absence as a presence: the `None` road rewritten into a `Some`.
///
/// Its target is FAIL-CLOSED EROSION. A road that refuses when a value is
/// missing and proceeds when it is present is a fail-closed road, and this
/// operator is precisely its reversal — the mutated subject proceeds on a value
/// nobody supplied, manufactured at the one seat that was supposed to stop.
///
/// A survivor here names a road whose closed direction no row ever walks, and
/// the closed direction is the only one that carries evidence: an open road
/// that opens establishes nothing about what the road does when the value is
/// not there.
pub const OPTION_COLLAPSE: OperatorFamily = OperatorFamily {
    slug: "option-collapse",
    attacks: "the road an absence travels",
};

/// Drops a returned value at a call site written to read it.
///
/// Its target is ERROR-SWALLOWING as well, at the caller rather than at the
/// producer, and the two are separate families because they fail separately: a
/// producer can be word-perfect about how it refuses while one line at one call
/// site throws the refusal away, and a refusal discarded is indistinguishable
/// from a refusal that never happened.
///
/// A survivor here names a call whose result the table never observes, so the
/// producer's refusal contract — however well proven at the producer — is
/// unproven from the outside, where every consumer of it lives.
pub const IGNORED_RESULTS: OperatorFamily = OperatorFamily {
    slug: "ignored-results",
    attacks: "the fate of a value a caller was written to read",
};

/// Replaces an owner-declared input with a fact taken straight off the host — a
/// clock, an environment value, an entropy source, a filesystem answer.
///
/// Its target is AMBIENT CONTACT. The machine's standing law is that no host
/// fact influences a semantic result unless an owner-declared input carries it,
/// and the reversal of that law is one line of contact: the mutated subject
/// reads the host where the lawful one reads the declaration it was handed.
///
/// A survivor here names a result that comes out the same whichever fact
/// arrives, which is exactly the shape under which ambient contact is
/// invisible — nothing in the table ever made the declared value and the host
/// value disagree, so no row could have told the two subjects apart.
///
/// # Nonclaims
///
/// The family is about the SUBJECT's contact, never the harness's. A harness
/// reads the host facts it needs in order to run; what the law forbids is a
/// host fact entering a semantic identity or decision, and a row of this family
/// is a damage aimed at that entry.
pub const DIRECT_MACHINE_CONTACT: OperatorFamily = OperatorFamily {
    slug: "direct-machine-contact",
    attacks: "the seam between a declared input and the host it was declared instead of",
};

/// Replaces a constructor's body with a default value, or with a call back into
/// itself.
///
/// One family and two damages, because both land in the same nucleus and are
/// caught or missed together: parse-don't-validate seats an invariant in a smart
/// constructor, so a value that exists at all is supposed to be a value that
/// passed one. A default-minted value never met the guard; a self-call never
/// produces a value at all.
///
/// A survivor here means the guard's refusal is required by no row: the type's
/// promise is being carried by the discipline of whoever calls the constructor
/// rather than by the construction, and every later claim that reads the type as
/// already-informed is reading a promise nothing enforces.
pub const CONSTRUCTOR_AND_DEFAULT_RECURSION: OperatorFamily = OperatorFamily {
    slug: "constructor-and-default-recursion",
    attacks: "the invariant nucleus a smart constructor holds",
};

/// The declared operator families, in the order this bank states them.
pub const OPERATOR_FAMILIES: [OperatorFamily; 7] = [
    COMPARISON_BOUNDARIES,
    BOOLEAN_OPERATORS,
    RESULT_COLLAPSE,
    OPTION_COLLAPSE,
    IGNORED_RESULTS,
    DIRECT_MACHINE_CONTACT,
    CONSTRUCTOR_AND_DEFAULT_RECURSION,
];

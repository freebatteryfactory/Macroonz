//! Every public type of this home, and the causes its laws cite.
//!
//! No declaration here names the vocabulary of anything it judges, and none can: every seat a meaning could enter through is a type parameter with no bound, equality arrives as an [`Equivalence`], and order arrives as an [`Order`].

use crate::descriptor::NamespacedName;
use crate::generate::GeneratedSequences;
use crate::report::{FindingCause, TrialConclusion};
use core::cmp::Ordering;
use std::collections::BTreeSet;

#[path = "type_guard.rs"]
mod guard;

/// The owner every cause this home cites is declared under.
///
/// Qualified with the harness's own name, like every sibling family, so a consumer declaring a bare `properties` family cannot alias a fingerprint of this home's.
const CAUSE_FAMILY: &str = "macroonz.properties";

// The callable seams every law is written over.

/// One owner-supplied check over the material a trial supplies.
pub type Check<Input> = fn(&Input) -> TrialConclusion;

/// One owner-supplied road from a value to its image.
pub type Road<Domain, Image> = fn(&Domain) -> Image;

/// One owner-supplied reading of the quantity a conservation law weighs.
pub type Measure<Value, Quantity> = fn(&Value) -> Quantity;

/// Whether two values are the same under the owner's declared equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Agreement {
    /// The two values are the same.
    Agrees,
    /// They are not.
    Differs,
}

/// The owner-supplied equivalence a law compares under.
///
/// A function rather than a trait bound, because sameness for a semantic value is its owner's declaration.
pub type Equivalence<Value> = fn(&Value, &Value) -> Agreement;

/// The owner-supplied order a law ranks under.
pub type Order<Value> = fn(&Value, &Value) -> Ordering;

/// Whether one demand holds.
///
/// A predicate's answer, a law's answer, and a macro-supplied match's answer are all this one verdict, so exactly one road turns a demand into a conclusion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Holding {
    /// The demand holds.
    Holds,
    /// The demand does not hold.
    Fails,
}

/// One owner-supplied predicate over a state.
pub type StatePredicate<State> = fn(&State) -> Holding;

/// What one subject did with material it was supposed to refuse.
///
/// Two arms and no third, so a substituted default reads as [`PoisonResponse::Answered`]: a value stood where a refusal was owed, whether it was computed, remembered, or invented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoisonResponse {
    /// The subject refused.
    Refused,
    /// The subject answered with a value.
    Answered,
}

/// The owner-supplied reading of what a subject answered.
///
/// A reading rather than a shape this home imposes, because a refusal is spelled in the subject's own vocabulary.
pub type ResponseReading<Response> = fn(&Response) -> PoisonResponse;

// Parity: two roads to one meaning.

/// One thing two parity roads both stand on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubstrateRef(NamespacedName);

/// The foundations two parity roads share, at least one of them named.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstrateRoster {
    standing: BTreeSet<SubstrateRef>,
}

/// What two parity roads share, stated in full.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedSubstrate {
    /// The author declares the two roads stand on nothing in common, which is the loudest claim in this vocabulary and is reached by writing it and by no other road.
    DeclaredIndependent,
    /// The two roads stand on these foundations, and the suite is silent about every one of them.
    Standing(SubstrateRoster),
}

/// Why one shared-substrate roster was refused.
#[must_use = "a refusal is the reason a shared substrate was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubstrateRefusal {
    /// The roster names nothing at all, which is the opposite of [`SharedSubstrate::DeclaredIndependent`] rather than a quiet road to it.
    EmptyRoster,
    /// The roster names this substrate more than once, refused rather than folded away so that an authoring defect is not normalized out of sight.
    DuplicateSubstrate(SubstrateRef),
}

/// Which two roads a parity suite stands over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoadPairing {
    /// One fused implementation against the composition of the separate steps it fuses.
    FusedVersusSeparate,
    /// A live run against a second arrival at the same meaning, whether replayed from a record or rebuilt from cold.
    LiveVersusReplayed,
    /// Two roads the owner names, for a pairing this home has no shape for.
    Declared(NamespacedName),
}

/// Two roads to one meaning, the equivalence they are judged under, and the substrate they share.
///
/// The suite cannot decide which road is right: a disagreement names the pair, and which side moved is its owner's ruling.
pub struct ParitySuite<Input, Meaning> {
    pairing: RoadPairing,
    left: Road<Input, Meaning>,
    right: Road<Input, Meaning>,
    same: Equivalence<Meaning>,
    substrate: SharedSubstrate,
}

/// The suite, input, results, and conclusion from one parity comparison.
pub struct ParityReading<'suite, 'input, Input, Meaning> {
    suite: &'suite ParitySuite<Input, Meaning>,
    input: &'input Input,
    left: Meaning,
    right: Meaning,
    conclusion: TrialConclusion,
}

// Temporal: laws over a whole history.

/// What one temporal claim demands of a whole history.
///
/// Every arm is read across the opening state and the state after every command, rather than at whichever moment a driver happened to stop.
pub enum TemporalDemand<State> {
    /// The predicate holds of every state in the history.
    Always(StatePredicate<State>),
    /// The predicate holds of no state in the history.
    Never(StatePredicate<State>),
    /// The predicate holds of at least one state in the history.
    Eventually(StatePredicate<State>),
    /// Once the predicate holds it holds of every later state — the latch, and the monotonicity law over a predicate.
    OnceHoldingAlwaysHolding(StatePredicate<State>),
    /// No state ranks below the state before it — the monotonicity law over an order.
    NeverDecreases(Order<State>),
}

/// One temporal claim: what it demands of a history, and the cause a break in it is cited under.
///
/// The cause is the owner's, so a contract carrying several claims of one shape still tells its own breaks apart, and a fingerprint built from the cause can too.
pub struct TemporalClaim<State> {
    cause: FindingCause,
    demand: TemporalDemand<State>,
}

/// One owner-supplied transition system: where a history opens, how one command moves it, and the claims its histories owe.
///
/// The opening is a nullary road rather than a value, so one contract drives many sequences and each history opens where its owner declared rather than where the last one ended.
pub struct TransitionContract<State, Command> {
    opening: fn() -> State,
    apply: fn(&State, &Command) -> State,
    claims: Vec<TemporalClaim<State>>,
}

/// Whether a temporal drive earned a conclusion or stopped before an all-pass claim could be established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemporalDriveStanding {
    /// The generated evidence establishes this conclusion.
    Concluded(TrialConclusion),
    /// Every evaluated sequence passed, but generation stopped before the declared case budget was met.
    Incomplete,
}

/// The generation result and evaluated prefix behind one temporal-drive standing.
///
/// The retained [`GeneratedSequences`] stays the owner of what was generated; this reading adds only how far evaluation reached.
pub struct TemporalDriveReading<Command> {
    generated: GeneratedSequences<Command>,
    evaluated: usize,
    standing: TemporalDriveStanding,
}

/// Why one transition contract was refused.
#[must_use = "a refusal is the reason a transition contract was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractRefusal {
    /// The contract declares no claim, so every history driven through it would read as a pass with nothing demanded of it.
    NoClaimDeclared,
}

// Composition: laws over a wiring.

/// Two owner-supplied steps wired in a declared order, and the equivalence their composition is judged under.
///
/// The wiring is not itself a [`Road`], because a function pointer cannot carry the two steps it composes.
pub struct ComposedRoads<Entry, Middle, Exit> {
    first: Road<Entry, Middle>,
    second: Road<Middle, Exit>,
    same: Equivalence<Exit>,
}

// The causes this home's laws cite.

/// The cause a broken roundtrip law is cited under.
pub const ROUNDTRIP_DISAGREEMENT: FindingCause = FindingCause::named(CAUSE_FAMILY, "roundtrip");

/// The cause a broken idempotence law is cited under.
pub const IDEMPOTENCE_DISAGREEMENT: FindingCause = FindingCause::named(CAUSE_FAMILY, "idempotence");

/// The cause a broken conservation law is cited under.
pub const CONSERVATION_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "conservation");

/// The cause a broken monotonicity law is cited under.
pub const MONOTONICITY_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "monotonicity");

/// The cause a broken permutation-insensitivity law is cited under.
pub const PERMUTATION_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "permutation-insensitivity");

/// The cause a broken run-twice determinism law is cited under.
pub const DETERMINISM_DISAGREEMENT: FindingCause = FindingCause::named(CAUSE_FAMILY, "determinism");

/// The cause a broken ambient-pathway-invariance law is cited under.
pub const AMBIENT_PATHWAY_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "ambient-pathway-invariance");

/// The cause a fused road disagreeing with the separate composition is cited under.
pub const FUSED_VERSUS_SEPARATE_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "parity-fused-versus-separate");

/// The cause a reproduced run disagreeing with the live one is cited under.
pub const LIVE_VERSUS_REPLAYED_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "parity-live-versus-replayed");

/// The cause a broken roundtrip over a returning composition is cited under.
pub const COMPOSED_RETURN_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "composition-return");

/// The cause a broken idempotence law over a composition is cited under.
pub const COMPOSED_IDEMPOTENCE_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "composition-idempotence");

/// The cause a broken determinism law over a composition is cited under.
pub const COMPOSED_DETERMINISM_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "composition-determinism");

/// The cause a broken conservation law over a composition is cited under.
pub const COMPOSED_CONSERVATION_DISAGREEMENT: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "composition-conservation");

/// The cause a subject answering where it owed a refusal is cited under.
pub const FAIL_CLOSED_ANSWERED: FindingCause = FindingCause::named(CAUSE_FAMILY, "fail-closed");

/// The cause a subject refusing the lawful twin of a hostile case is cited under.
pub const LAWFUL_TWIN_REFUSED: FindingCause = FindingCause::named(CAUSE_FAMILY, "lawful-twin");

/// The paved cause an outcome that was owed an answer is cited under.
pub const ANSWER_EXPECTED: FindingCause = FindingCause::named(CAUSE_FAMILY, "answer-expected");

/// The paved cause an outcome that was owed a refusal is cited under.
pub const REFUSAL_EXPECTED: FindingCause = FindingCause::named(CAUSE_FAMILY, "refusal-expected");

/// The cause a drive that produced no sequence at all is cited under, because a law over an empty world is unexercised rather than satisfied.
pub const NO_SEQUENCE_DRIVEN: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "no-sequence-driven");

/// The paved cause a broken always-claim is cited under.
pub const ALWAYS_BROKEN: FindingCause = FindingCause::named(CAUSE_FAMILY, "temporal-always");

/// The paved cause a broken never-claim is cited under.
pub const NEVER_BROKEN: FindingCause = FindingCause::named(CAUSE_FAMILY, "temporal-never");

/// The paved cause an eventually-claim nothing in the history reached is cited under.
pub const EVENTUALLY_UNREACHED: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "temporal-eventually");

/// The paved cause a broken latch is cited under.
pub const LATCH_BROKEN: FindingCause = FindingCause::named(CAUSE_FAMILY, "temporal-latch");

/// The paved cause a history that ranked below its own past is cited under.
pub const ORDER_DECREASED: FindingCause = FindingCause::named(CAUSE_FAMILY, "temporal-order");

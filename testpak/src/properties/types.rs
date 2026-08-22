//! The property vocabulary's declarations: the check shape, the owner-supplied
//! comparison seams, the demand verdict, the parity suite and the substrate it
//! names, the transition contract and its temporal claims, the composed-roads
//! suite, and the typed causes this home cites.
//!
//! Declarations only. Every road that reaches a private field is this file's own
//! child, `type_guard.rs`; the laws themselves are the role-named function
//! modules beside it.
//!
//! # The neutrality law
//!
//! No declaration here names a product type, and none can: every subject seat is
//! a type parameter carrying no bound at all. A product adapts its own
//! vocabulary into these shapes at its own layer, and a suite that named a
//! product would be this home holding an opinion about a meaning it does not
//! own.
//!
//! # The comparison seam
//!
//! Equality is the OWNER's declaration, always: every law that compares takes an
//! [`Equivalence`] and every law that ranks takes an [`Order`]. No bound is
//! demanded of any subject type, so a product type never has to grow a derive to
//! be judged here, and two subjects that disagree about what sameness means
//! never share one.

use crate::descriptor::NamespacedName;
use crate::generate::GeneratedSequences;
use crate::report::{FindingCause, TrialConclusion};
use core::cmp::Ordering;
use std::collections::BTreeSet;

#[path = "type_guard.rs"]
mod guard;

/// The owner every cause this home cites is declared under.
const CAUSE_FAMILY: &str = "properties";

// ---------------------------------------------------------------------------
// The check shape and the owner-supplied seams.
// ---------------------------------------------------------------------------

/// One owner-supplied check over the material a trial supplies.
///
/// # Authority
///
/// This is the callable shape of a check: one borrowed input and one
/// [`TrialConclusion`] output. The owner remains responsible for the function's
/// effects and unwind behavior.
///
/// # Bounds
///
/// A function pointer rather than a closure, so a check carries no captured
/// state. That shape does not prevent the function from reaching globals, I/O,
/// or another ambient source. Every law in this home is written to be called
/// FROM one of these: an owner's check is the thin function that binds its
/// subject to a law and hands the conclusion back.
pub type Check<Input> = fn(&Input) -> TrialConclusion;

/// One owner-supplied road from a value to its image.
///
/// The subject seat of every law here. A fallible road is one whose image is the
/// owner's own outcome type, which the refusal-family checks read through a
/// declared reading rather than through a shape this home invented.
pub type Road<Domain, Image> = fn(&Domain) -> Image;

/// One owner-supplied reading over a value.
///
/// What a conservation law weighs: the quantity a transformation is claimed to
/// leave unmoved.
pub type Measure<Value, Quantity> = fn(&Value) -> Quantity;

/// Whether two values are the same under the owner's declared equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Agreement {
    /// The two values are the same under the declared equivalence.
    Agrees,
    /// They are not.
    Differs,
}

/// The owner-supplied equivalence a law compares under.
///
/// # Authority
///
/// An explicit function rather than a trait bound, deliberately. A law here may
/// never demand `Eq` of a product type: sameness for a semantic value is the
/// owner's declaration — which fields count, which are presentational, which
/// float comparison is the right one — and a derived equality would be this home
/// answering that question for somebody else.
pub type Equivalence<Value> = fn(&Value, &Value) -> Agreement;

/// The owner-supplied order a law ranks under.
///
/// The ordinary comparison vocabulary rather than a second one: a rank is a
/// [`Ordering`], and this alias states only who supplies it.
pub type Order<Value> = fn(&Value, &Value) -> Ordering;

/// Whether one demand holds.
///
/// The home's one two-arm demand verdict: a state predicate's answer, a law's
/// answer, and a macro-supplied match's answer are all this, so exactly one road
/// turns a demand into a conclusion.
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
/// # Authority
///
/// The two arms are the whole roster, and a substituted default is an
/// [`PoisonResponse::Answered`]: a value stood where a refusal was owed, whether
/// it was computed, remembered, or invented. That is exactly the failure the
/// fail-closed law exists to name, so it has no third arm to hide in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoisonResponse {
    /// The subject refused.
    Refused,
    /// The subject answered with a value.
    Answered,
}

/// The owner-supplied reading of what a subject answered.
///
/// A reading rather than a shape this home imposes: an owner whose subject
/// answers with a `Result`, with a typed outcome enum, or with a sentinel of its
/// own writes the one function that says which of the two things happened.
pub type ResponseReading<Response> = fn(&Response) -> PoisonResponse;

// ---------------------------------------------------------------------------
// The parity suite.
// ---------------------------------------------------------------------------

/// One thing two parity roads both stand on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubstrateRef(NamespacedName);

/// The foundations two parity roads share, at least one of them named.
///
/// # Authority
///
/// A roster exists where there is something to name, and it is never empty. An
/// empty roster is not a small roster: it is the OPPOSITE claim, and that claim
/// is [`SharedSubstrate::DeclaredIndependent`] — written by an author who means
/// it rather than reached by handing this constructor nothing.
///
/// # Construction
///
/// [`SubstrateRoster::declared`] is the only road. It refuses an empty roster,
/// then a substrate the roster names twice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstrateRoster {
    standing: BTreeSet<SubstrateRef>,
}

/// What two parity roads share, stated in full.
///
/// # Authority
///
/// The parity honesty clause made structural, and a SUM because its two arms
/// are two different claims. Agreement across a shared substrate is SILENCE
/// about that substrate: two roads that both stand on one declaration, one
/// parser, or one rendering engine agree with each other exactly as far as that
/// shared thing is right, and no further. A parity suite cannot be built
/// without stating which of the two claims it makes, so the ceiling travels
/// with the value rather than living in whoever remembers to say it.
///
/// # The claim ceiling
///
/// [`SharedSubstrate::DeclaredIndependent`] is the author's DECLARATION that
/// the two roads stand on nothing in common. It is the loudest thing this
/// vocabulary can say, and it is a declaration rather than a qualification:
/// nothing here establishes independence, and a suite carrying this arm claims
/// exactly what its author claimed and no more.
/// [`SharedSubstrate::Standing`] yields parity evidence with the shared
/// foundations named, which is the honest ceiling for two roads that share
/// anything at all.
///
/// # Construction
///
/// The independent arm is reached by writing it and by no other road: there is
/// no constructor that arrives at it from a roster, so an empty roster is a
/// typed refusal ([`SubstrateRefusal::EmptyRoster`]) rather than the loudest
/// claim in this vocabulary made without anybody saying it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedSubstrate {
    /// The author declares that the two roads stand on nothing in common, so
    /// their agreement is evidence about both of them.
    DeclaredIndependent,
    /// The two roads stand on these foundations, and the suite is silent about
    /// every one of them.
    Standing(SubstrateRoster),
}

/// Why one shared-substrate roster was refused.
///
/// Dependent checks in a declared order: the roster is read before its members
/// are weighed against each other.
#[must_use = "a refusal is the reason a shared substrate was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubstrateRefusal {
    /// The roster names nothing at all.
    ///
    /// An empty roster is not the independence declaration. That claim is
    /// [`SharedSubstrate::DeclaredIndependent`], and reaching it by handing a
    /// generic constructor no substrates would be the loudest claim here made
    /// by a caller who never said it.
    EmptyRoster,
    /// The roster names this substrate more than once.
    ///
    /// Refused rather than folded away, because collapsing a duplicate silently
    /// would be the harness normalizing an authoring defect out of sight.
    DuplicateSubstrate(SubstrateRef),
}

/// Which two roads a parity suite stands over.
///
/// # Authority
///
/// The pairing is carried rather than implied by which constructor was called,
/// so a disagreement is cited under the pairing that disagreed and a reader
/// never has to ask which two roads a refusal came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RoadPairing {
    /// One fused implementation against the composition of the separate steps it
    /// fuses.
    FusedVersusSeparate,
    /// A live run against the same run reproduced from its record.
    ///
    /// A reproduction rebuilt from cold rather than replayed from a record is
    /// the same pairing under a different reproduction road: both claim that a
    /// second arrival at the meaning reaches the meaning the first one did.
    LiveVersusReplayed,
    /// Two roads the owner names, for a pairing this home has no shape for.
    Declared(NamespacedName),
}

/// Two roads to one meaning, the equivalence they are judged under, and the
/// substrate they share.
///
/// # Authority
///
/// A parity law pins MEANING while leaving both roads free to change: whatever
/// either road becomes, the pair must still arrive at one answer. What it can
/// never do is decide which road is right — a disagreement names the pair, and
/// which side moved is the owner's ruling.
///
/// # Construction
///
/// [`ParitySuite::over`] takes the pairing explicitly; the two named
/// constructors fill it for the pairings this home has a shape for. The roads
/// are `left` and `right`, and the pairing states which is which.
///
/// # Nonclaims
///
/// Agreement is silence about everything the two roads share, which is why
/// [`SharedSubstrate`] is a required field rather than an optional note — and
/// why it is a sum rather than a roster that could arrive empty, so the claim
/// that the roads share nothing is one an author states rather than one a
/// caller reaches by passing nothing.
pub struct ParitySuite<Input, Meaning> {
    pairing: RoadPairing,
    left: Road<Input, Meaning>,
    right: Road<Input, Meaning>,
    same: Equivalence<Meaning>,
    substrate: SharedSubstrate,
}

/// The exact suite, input, results, and conclusion from one parity comparison.
///
/// # Authority
///
/// The retained [`ParitySuite`] remains the sole owner of the road pairing, equivalence, and shared-substrate ceiling.
/// This reading records what those roads returned for one exact input without copying or widening any suite claim.
pub struct ParityReading<'suite, 'input, Input, Meaning> {
    suite: &'suite ParitySuite<Input, Meaning>,
    input: &'input Input,
    left: Meaning,
    right: Meaning,
    conclusion: TrialConclusion,
}

// ---------------------------------------------------------------------------
// The temporal suite.
// ---------------------------------------------------------------------------

/// What one temporal claim demands of a whole history.
///
/// # Authority
///
/// Every arm is read across the COMPLETE history — the opening state and the
/// state after every command — rather than at whichever moment a driver happened
/// to stop. A law read at one moment is a law about that moment.
pub enum TemporalDemand<State> {
    /// The predicate holds of every state in the history.
    Always(StatePredicate<State>),
    /// The predicate holds of no state in the history.
    Never(StatePredicate<State>),
    /// The predicate holds of at least one state in the history.
    Eventually(StatePredicate<State>),
    /// Once the predicate holds, it holds of every later state — the latch, and
    /// the monotonicity law over a predicate.
    OnceHoldingAlwaysHolding(StatePredicate<State>),
    /// No state ranks below the state before it — the monotonicity law over an
    /// order.
    NeverDecreases(Order<State>),
}

/// One temporal claim: what it demands of a history, and the typed cause a
/// broken claim is cited under.
///
/// # Authority
///
/// The cause is the owner's, always. A contract carrying several claims of one
/// shape would otherwise report every break under one name, and a fingerprint
/// built from that name could not tell two of the owner's claims apart. The
/// paved causes this home publishes are values an owner may pass, never a
/// default that fills a seat nobody stated.
pub struct TemporalClaim<State> {
    cause: FindingCause,
    demand: TemporalDemand<State>,
}

/// One owner-supplied transition system: where a history opens, how one command
/// moves it, and the claims its histories owe.
///
/// # Authority
///
/// Generic and neutral in both seats. The state and the command are type
/// parameters carrying no bound, so the temporal machinery drives a product's
/// transition system without ever naming one — the product maps its own
/// vocabulary into this contract at its own layer.
///
/// # Construction
///
/// [`TransitionContract::declared`] refuses a contract with no claim: driving a
/// history under no claim reads as a pass and proves nothing, which is the one
/// shape of vacuity this home can refuse structurally.
///
/// # Bounds
///
/// The opening state is a nullary road rather than a value, so one contract
/// drives many sequences and each history opens where the owner declared rather
/// than where the previous one ended.
pub struct TransitionContract<State, Command> {
    opening: fn() -> State,
    apply: fn(&State, &Command) -> State,
    claims: Vec<TemporalClaim<State>>,
}

/// Whether a temporal generation drive earned a conclusion or stopped before an all-pass claim could be established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemporalDriveStanding {
    /// The generated evidence establishes the carried trial conclusion.
    Concluded(TrialConclusion),
    /// Every evaluated sequence passed, but generation stopped before completing the declared case budget.
    Incomplete,
}

/// The generation result and evaluated prefix behind one temporal-drive standing.
///
/// # Authority
///
/// The retained [`GeneratedSequences`] owns the admitted sequences, census, and halt; this reading adds only how far temporal evaluation reached and what that evidence can conclude.
/// A universal claim passes only after generation reaches its complete halt, while one concrete counterexample remains a refusal even when generation stopped early.
pub struct TemporalDriveReading<Command> {
    generated: GeneratedSequences<Command>,
    evaluated: usize,
    standing: TemporalDriveStanding,
}

/// Why one transition contract was refused.
#[must_use = "a refusal is the reason a transition contract was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractRefusal {
    /// The contract declares no claim, so every history driven through it would
    /// pass without anything having been demanded of it.
    NoClaimDeclared,
}

// ---------------------------------------------------------------------------
// The composed-roads suite.
// ---------------------------------------------------------------------------

/// Two owner-supplied steps wired in a declared order, and the equivalence their
/// composition is judged under.
///
/// # Authority
///
/// Composition owes its own laws: wiring correct operations in the wrong order
/// is still a defect, and neither step's own suite can see it. The value carries
/// the order, so a suite over it is a suite over the wiring rather than over the
/// parts.
///
/// # Bounds
///
/// The composition is not itself a [`Road`], because a function pointer cannot
/// carry the two steps it composes. An owner comparing a fused implementation
/// against this wiring writes one plain function that calls
/// [`composed`](crate::properties::composed) and passes it as the separate road
/// of a [`ParitySuite`].
pub struct ComposedRoads<Entry, Middle, Exit> {
    first: Road<Entry, Middle>,
    second: Road<Middle, Exit>,
    same: Equivalence<Exit>,
}

// ---------------------------------------------------------------------------
// The typed causes.
// ---------------------------------------------------------------------------

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

/// The cause a fused road disagreeing with the separate composition is cited
/// under.
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

/// The cause a subject refusing the lawful twin of a hostile case is cited
/// under.
pub const LAWFUL_TWIN_REFUSED: FindingCause = FindingCause::named(CAUSE_FAMILY, "lawful-twin");

/// The paved cause an outcome that was owed an answer is cited under.
pub const ANSWER_EXPECTED: FindingCause = FindingCause::named(CAUSE_FAMILY, "answer-expected");

/// The paved cause an outcome that was owed a refusal is cited under.
pub const REFUSAL_EXPECTED: FindingCause = FindingCause::named(CAUSE_FAMILY, "refusal-expected");

/// The cause a drive that produced no sequence at all is cited under.
///
/// A temporal law over an empty world is not satisfied; it is unexercised, and
/// reporting it as a pass would be the harness manufacturing evidence out of a
/// generator that gave it nothing.
pub const NO_SEQUENCE_DRIVEN: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "no-sequence-driven");

/// The paved cause a broken always-claim is cited under.
pub const ALWAYS_BROKEN: FindingCause = FindingCause::named(CAUSE_FAMILY, "temporal-always");

/// The paved cause a broken never-claim is cited under.
pub const NEVER_BROKEN: FindingCause = FindingCause::named(CAUSE_FAMILY, "temporal-never");

/// The paved cause an eventually-claim nothing in the history reached is cited
/// under.
pub const EVENTUALLY_UNREACHED: FindingCause =
    FindingCause::named(CAUSE_FAMILY, "temporal-eventually");

/// The paved cause a broken latch is cited under.
pub const LATCH_BROKEN: FindingCause = FindingCause::named(CAUSE_FAMILY, "temporal-latch");

/// The paved cause a history that ranked below its own past is cited under.
pub const ORDER_DECREASED: FindingCause = FindingCause::named(CAUSE_FAMILY, "temporal-order");

//! The proof-pressure engine's declarations: the verdict chain's axes, the
//! per-mutant record and its run, the mutation target and its owner mapping, the
//! wrap lane's adapter profile and reading vocabulary, the two facts a reading
//! opens trust with, the interpreted lane's evaluation surface and trust gate,
//! the rewrite lane's descriptors, the artifact-mutation seed roster, the
//! survivor explanation and the check gap, the scope shapes and the proof plan,
//! and the whole proposal road.
//!
//! Declarations only. Every road that reaches a private field is this file's own
//! child, `type_guard.rs`; the declarative tables are `type_contract.rs`; the
//! four lanes are the role-named modules beside them.
//!
//! # The borrowed vocabularies
//!
//! The row, the staged view, the candidate origin arm, and every namespaced
//! reference belong to the descriptor vocabulary ([`crate::descriptor`]). The
//! trial identity, the finding, the fingerprint, the replay capsule, the
//! execution key, and the run report belong to the record vocabulary
//! ([`crate::report`]). The selection and the invocation belong to the engine
//! ([`crate::runner`]), and the operator families belong to the fact bank
//! ([`crate::depot`]). Nothing here restates any of those contracts: this home
//! BINDS those values, and what they mean is written where they are declared.

use crate::depot::types::OperatorFamily;
use crate::descriptor::{
    CheckRef, ClaimRef, Classification, ExecutionSuite, MutationPointRef, NameRefusal,
    NamespacedName, PopulationRef, ProposalId, Row, RowRefusal, StagedTableRefusal, SubjectRoute,
};
use crate::identity::{ContentAddress, DomainTag};
use crate::properties::SubstrateRefusal;
use crate::report::{
    ClaimExercise, ExecutionKey, Fingerprint, ForeignText, InvocationProfile, ReplayCapsule,
    RunReport, TrialFinding, TrialId,
};
use crate::runner::Selection;

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The verdict chain's axes.
// ---------------------------------------------------------------------------

/// What the UNMUTATED subject's own suite did before any damage was inflicted.
///
/// # Authority
///
/// An unchanged passing baseline is the precondition every kill stands on. A
/// mutant "caught" by an already-failing suite proves nothing about the suite,
/// so this axis is read before any other and a lane that cannot read it mints no
/// kill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaselineAxis {
    /// The unchanged subject's suite ran and passed.
    Qualified,
    /// The unchanged subject's suite ran and did not pass.
    Failed,
    /// The unchanged subject's suite was not run at all.
    NotRun,
}

/// Whether one damage became a thing that could be executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterializationAxis {
    /// The damaged subject built.
    Built,
    /// The damaged subject is not buildable — the damage does not typecheck, or
    /// the site admits no such alternative.
    Unviable,
    /// The backend itself failed while materializing the damage, so nothing was
    /// established about the damage at all.
    ToolFailed,
}

/// Whether the planted damage was PROVEN to fire.
///
/// # Authority
///
/// An unactivated mutant is not a survivor. A damage nothing reached says
/// nothing about the suite that did not catch it, which is why this axis stands
/// between materialization and the verdict rather than being folded into either.
///
/// # Nonclaims
///
/// [`ActivationAxis::UnobservableUnderBackend`] is a fact about the BACKEND and
/// never about the damage: it states that no channel exists to observe firing at
/// all, so nothing about this mutant's activation was established either way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationAxis {
    /// An execution observed the damage fire.
    Observed,
    /// The backend can observe firing, and nothing observed this damage fire.
    NotObserved,
    /// The backend offers no activation channel, so firing is unobservable under
    /// it.
    UnobservableUnderBackend,
}

/// What became of the witness execution over the damaged subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionAxis {
    /// The witness ran to a conclusion.
    Completed,
    /// The witness passed the time bound it was given.
    TimedOut,
    /// The witness process died.
    Crashed,
    /// The harness or the backend failed around the witness, so nothing was
    /// learned about the damaged subject.
    InfrastructureFailed,
}

/// What one mutant earned, at axis width.
///
/// # Authority
///
/// This is the naming; the RECORD carries [`MutationOutcome`], whose arms carry
/// the evidence each one requires. A reader that needs the word takes this
/// projection, and a reader that needs the evidence takes the outcome.
///
/// # Nonclaims
///
/// A mutant unobservable under the backend can never earn
/// [`MutationVerdict::Survived`]: its non-kill result is
/// [`MutationVerdict::Inconclusive`], and that is a refusal in the record's
/// constructors rather than a rule somebody follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationVerdict {
    /// The suite rejected the damaged subject.
    Killed,
    /// The suite accepted a damage that was proven to fire.
    Survived,
    /// Nothing was learned about the suite from this mutant.
    Inconclusive,
}

/// What was established about the damaged subject meaning the same thing as the
/// lawful one.
///
/// # Nonclaims
///
/// [`EquivalenceAxis::ProvenInScope`] claims equivalence over the SCOPE the
/// proof was taken in and never in general: a mutant indistinguishable under one
/// declared population may be distinguishable under another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquivalenceAxis {
    /// No equivalence question was put.
    NotAssessed,
    /// The damaged subject was proven equivalent to the lawful one, in the scope
    /// the proof was taken in.
    ProvenInScope,
    /// The damaged subject was shown to differ from the lawful one.
    Refuted,
    /// The equivalence question was put and not answered.
    Inconclusive,
}

// ---------------------------------------------------------------------------
// The mutation target.
// ---------------------------------------------------------------------------

/// The domain tag every external mutant identity is derived under.
pub const MUTATION_TARGET_TAG: DomainTag = DomainTag::declared("mutation-target");

/// Where in a source text an external backend placed one damage.
///
/// # Bounds
///
/// The spelling is data a backend reported rather than a name this harness
/// authored, which is why it is owned text and deliberately not a
/// [`NamespacedName`]: that vocabulary is authored, and nothing here mints one
/// from a tool's output.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceCoordinate {
    file: String,
    line: u32,
    column: u32,
}

/// Why one source coordinate was refused.
#[must_use = "a refusal is the reason a coordinate was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoordinateRefusal {
    /// The coordinate names no file, so it places nothing.
    EmptyFile,
}

/// One external mutant's identity, derived from the coordinate and the damage
/// text the backend reported.
///
/// # Authority
///
/// External mutants arrive as source coordinates rather than as claims, so their
/// identity is a content address over exactly what arrived. Two runs of one
/// backend over one unchanged tree name the same mutant; a moved line names a
/// different one, which is honest — the coordinate is what the backend gave.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MutantId(ContentAddress);

/// How one damaged thing is identified, by the lane that damaged it.
///
/// # Authority
///
/// The two arms are the two lanes' two kinds of address and never
/// interchangeable: an external identity is derived from data a backend
/// reported, and an interpreted one is a reference a producer authored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationIdentity {
    /// An external backend's mutant, addressed by its reported coordinate and
    /// damage.
    External(MutantId),
    /// A point on an evaluation surface, addressed by the reference its producer
    /// authored.
    Interpreted(MutationPointRef),
}

/// Where one damage lives, as the lane that placed it can say.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationSite {
    /// A source coordinate an external backend reported.
    Reported(SourceCoordinate),
    /// The named activation site a producer declared.
    Declared(ActivationSite),
}

/// One row of the operator-family bank, cited by the slug that bank declares.
///
/// # Construction
///
/// [`OperatorFamilyRef::of_slug`] resolves against the bank's own roster, so a
/// reference can never name a family the bank does not declare. Nothing here
/// copies a family's prose: the row is carried, and what a survivor of the
/// family means is read at the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorFamilyRef(OperatorFamily);

/// Whether one damage is one the operator-family bank names.
///
/// # Authority
///
/// A backend applies its own operators, and attributing one of them to a family
/// this bank never declared would be the lane inventing a fact. The second arm
/// is that refusal made a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyAttribution {
    /// The damage realizes a family the bank declares.
    Declared(OperatorFamilyRef),
    /// The damage is not one the bank names, so no family is claimed for it.
    OutsideTheBank,
}

/// Whether the origin-graph reading named the claim that owns one damage's site.
///
/// # Authority
///
/// The owning claim rides the mapped arm, so a mapped target without a claim is
/// not a value anybody can hold. Where mapping is unavailable the lane reports
/// the second arm and runs a conservative witness selection; it never invents
/// the claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappingPosture {
    /// The reading named this claim as the owner of the damage's site.
    Mapped(ClaimRef),
    /// No mapping was available for the site.
    OwnerUnmapped,
}

/// One damaged thing this lane pressed: its identity, the family it realizes,
/// where it lives, and whether its owning claim is known.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationTarget {
    identity: MutationIdentity,
    family: FamilyAttribution,
    site: MutationSite,
    owner: MappingPosture,
}

// ---------------------------------------------------------------------------
// Activation evidence, and the dud plant.
// ---------------------------------------------------------------------------

/// That one planted damage was proven to fire, and what proved it.
///
/// # Construction
///
/// [`ActivationEvidence::observed`] refuses a firing count of zero, so evidence
/// of a plant that never fired is not a value that exists. What comes back
/// instead is a [`DudPlant`] — a finding, never a silent pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActivationEvidence {
    point: MutationPointRef,
    witness: TrialId,
    firings: u32,
}

/// A plant that never fired: the damage was selected and no execution reached
/// it.
///
/// # Authority
///
/// A finding in its own right. A harness that planted a damage, observed nothing
/// fire, and reported the run as ordinary would be grading itself on an alarm
/// that never rang.
#[must_use = "a dud plant is a finding, never a silent pass"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DudPlant {
    point: MutationPointRef,
    witness: TrialId,
}

/// The activation axis with the evidence its observed arm requires.
///
/// # Authority
///
/// Observed activation without evidence is unrepresentable: the arm carries the
/// proof. The axis alone is [`ActivationAxis`], and the projection between them
/// is declared once in `type_contract.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationDisposition {
    /// The damage was observed to fire, and this is the evidence.
    Observed(ActivationEvidence),
    /// The backend can observe firing, and nothing observed this damage fire.
    NotObserved,
    /// The backend offers no activation channel at all.
    UnobservableUnderBackend,
}

// ---------------------------------------------------------------------------
// The rejection a kill stands on.
// ---------------------------------------------------------------------------

/// One rejection this harness's own engine demonstrated: the trial that refused,
/// and the finding it refused with.
///
/// # Authority
///
/// The finding is carried whole, so the failure identity is derived rather than
/// remembered — [`DemonstratedRejection::fingerprint`] is the one road, and no
/// second naming of the same failure exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DemonstratedRejection {
    trial: TrialId,
    finding: TrialFinding,
}

/// The rejection one witness execution answered a damaged subject with.
///
/// # The ceilings
///
/// [`IntendedRejection::Demonstrated`] is this harness's own engine refusing:
/// the trial and the finding are both named, so the rejection carries a failure
/// fingerprint.
///
/// [`IntendedRejection::ReportedByBackend`] is an external backend's word. It
/// names no trial and no cause, so no fingerprint exists for it, and a kill
/// standing on it claims exactly what the backend stated and nothing more.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntendedRejection {
    /// This harness's engine ran the witness and the check refused.
    Demonstrated(DemonstratedRejection),
    /// An external backend ran its own command and reported a rejection.
    ReportedByBackend {
        /// The backend's own line, bounded and marked.
        stated: ForeignText,
    },
}

/// The failure identity a rejection carries, where it carries one.
///
/// # Nonclaims
///
/// [`RejectionIdentity::Unfingerprinted`] is not a missing value a caller should
/// fill in. It states that the rejection came from a backend that named neither
/// a trial nor a cause, so there is no failure to name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RejectionIdentity {
    /// The rejection names a fingerprint, derived from its trial and finding.
    Fingerprinted(Fingerprint),
    /// The rejection carries no trial and no cause, so it names no failure.
    Unfingerprinted,
}

// ---------------------------------------------------------------------------
// The per-mutant record.
// ---------------------------------------------------------------------------

/// Why nothing was learned about the suite from one mutant.
///
/// # Authority
///
/// Every arm names a link of the verdict chain that did not hold. An
/// inconclusive result is a statement about the RUN and never about the suite,
/// which is why none of these arms is a softer survivor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InconclusiveCause {
    /// The unchanged baseline did not qualify, so no rejection under it would
    /// have proven anything.
    BaselineNotQualified,
    /// The damage never became a thing that could be executed.
    NotMaterialized,
    /// The backend can observe firing and nothing observed this damage fire, so
    /// the suite was never asked about it.
    NotActivated,
    /// The witness execution did not complete.
    WitnessIncomplete,
    /// The backend offers no activation channel and the witness did not reject,
    /// so a non-kill here can never earn survived.
    UnobservableAndUnrejected,
    /// The damaged subject was proven equivalent in scope, so no suite could
    /// have rejected it.
    ProvenEquivalentInScope,
}

/// The verdict one mutant earned, with the evidence each arm requires.
///
/// # Authority
///
/// The killed arm carries the rejection that killed it, so a kill asserted
/// without a rejection is unrepresentable. The survived arm carries nothing
/// BECAUSE it is the absence of a rejection under an activation that was proven
/// — and the constructor is what establishes that proof.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationOutcome {
    /// The witness rejected the damaged subject, and this is the rejection.
    Killed(IntendedRejection),
    /// A damage that was proven to fire was accepted by the witness.
    Survived,
    /// Nothing was learned about the suite from this mutant.
    Inconclusive(InconclusiveCause),
}

/// One mutant's complete record: the target, every axis of the verdict chain,
/// and the outcome the chain earned.
///
/// # Construction
///
/// Three roads, and each one is a law. [`MutationReport::killed`] demands a
/// qualified unchanged baseline, correct materialization, activation that was
/// observed or a backend that cannot observe it, a witness that completed, and
/// the rejection itself. [`MutationReport::survived`] demands all of that plus
/// OBSERVED activation and refuses the unobservable arm outright, so a survivor
/// under a backend with no activation channel is not a value anybody can build.
/// [`MutationReport::inconclusive`] is total, because any chain can fail to
/// establish anything.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationReport {
    target: MutationTarget,
    baseline: BaselineAxis,
    materialization: MaterializationAxis,
    activation: ActivationDisposition,
    execution: ExecutionAxis,
    outcome: MutationOutcome,
    equivalence: EquivalenceAxis,
}

/// Why one mutant's record could not be minted as a lawful kill.
///
/// Dependent checks in a declared order — baseline, materialization, activation,
/// execution — so exactly one cause is true of any refused kill.
#[must_use = "a refusal is the reason a kill was not minted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KillRefusal {
    /// The unchanged baseline did not qualify, so a rejection under it proves
    /// nothing about the suite.
    BaselineNotQualified(BaselineAxis),
    /// The damage never materialized, so there was nothing for a witness to
    /// reject.
    NotMaterialized(MaterializationAxis),
    /// The backend can observe firing and nothing observed this damage fire.
    ActivationNotObserved,
    /// The witness execution did not complete, so its rejection is not the
    /// suite's answer.
    WitnessDidNotComplete(ExecutionAxis),
}

/// Why one mutant's record could not be minted as a survivor.
///
/// Dependent checks in a declared order — baseline, materialization, activation,
/// execution, equivalence.
#[must_use = "a refusal is the reason a survivor was not minted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurvivalRefusal {
    /// The unchanged baseline did not qualify, so the suite's acceptance says
    /// nothing.
    BaselineNotQualified(BaselineAxis),
    /// The damage never materialized, so nothing was accepted.
    NotMaterialized(MaterializationAxis),
    /// Nothing observed the damage fire. An unactivated mutant is not a
    /// survivor.
    ActivationNotObserved,
    /// The backend offers no activation channel, so this mutant can never earn
    /// survived — its non-kill result is inconclusive.
    ActivationUnobservable,
    /// The witness execution did not complete, so the suite never answered.
    WitnessDidNotComplete(ExecutionAxis),
    /// The damaged subject was proven equivalent in scope, so no suite could
    /// have rejected it.
    ProvenEquivalentInScope,
}

/// The honest accounting over one pressure run's mutants.
///
/// # Authority
///
/// One count seat per arm of [`MutationVerdict`], always, and
/// [`MutationCensus::pressed`] is the sum rather than a separately maintained
/// total that could disagree with its parts.
///
/// # Nonclaims
///
/// It counts mutants under one run. It is not the trial census, the generation
/// census, or the bench-sample census: each of those denominators answers its
/// own question, and none flattens into another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MutationCensus {
    killed: u32,
    survived: u32,
    inconclusive: u32,
}

/// That the unchanged subject's own suite ran and passed.
///
/// # Authority
///
/// The typed precondition every kill stands on, carried as a value so that "was
/// the baseline good" is not a question anywhere downstream.
/// [`BaselineQualification::read`] is the only road, and it refuses every
/// reading but [`BaselineAxis::Qualified`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaselineQualification {
    axis: BaselineAxis,
}

/// Why one baseline did not qualify.
#[must_use = "a refusal is the reason a baseline did not qualify"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaselinePrecondition {
    /// The unchanged subject's suite ran and did not pass.
    BaselineFailed,
    /// The unchanged subject's suite was not run at all.
    BaselineNotRun,
}

/// One pressure run's complete record: the baseline it stood on, every mutant's
/// report, and the census over them.
///
/// # Authority
///
/// The baseline is a [`BaselineQualification`] rather than an axis reading, so a
/// run whose baseline did not qualify is not a run this value can describe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationRun {
    baseline: BaselineQualification,
    reports: Vec<MutationReport>,
    census: MutationCensus,
}

// ---------------------------------------------------------------------------
// The wrap lane's reading vocabulary.
// ---------------------------------------------------------------------------

/// Which external mutation backend one reading was taken from.
///
/// # Bounds
///
/// One backend because one line grammar: the shapes the wrap lane reads are
/// that tool's own rendering. A second backend is a second grammar, and it
/// arrives as a second arm beside the line laws that read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrappedBackend {
    /// The `cargo-mutants` backend: it mutates the real source and invokes the
    /// test command itself.
    CargoMutants,
}

/// One backend's version, as the party that ran it states it.
///
/// # Bounds
///
/// The spelling is the party's own word rather than a name this harness
/// authored, which is why it is owned text and deliberately not a
/// [`NamespacedName`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendVersion(String);

/// Why one backend version was refused.
#[must_use = "a refusal is the reason a backend version was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendVersionRefusal {
    /// The spelling is empty, so it states no version.
    EmptySpelling,
}

/// Whether the party that ran a backend stated which version of it produced the
/// output a reading was taken from.
///
/// # Authority
///
/// The version is DECLARED rather than observed: the wrap lane reads text a
/// caller already holds and invokes nothing, so what a version is here is the
/// running party's own word about what wrote that text.
/// [`BackendVersionPosture::Unstated`] is the bootstrap posture — the grammar
/// assumption stands unbound to any version, and a reading under it is exactly
/// as good as the assumption.
///
/// # Nonclaims
///
/// [`BackendVersionPosture::Stated`] records which version the party names. It
/// is not a verification that the text matches that version's rendering, and
/// nothing anywhere reads a backend's output to discover its version.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackendVersionPosture {
    /// The party that ran the backend stated this version.
    Stated(BackendVersion),
    /// No party has stated a version, so the grammar assumption stands unbound.
    Unstated,
}

/// Which of a backend's outputs one reading was taken from.
///
/// # Bounds
///
/// A console stream is a rendering a tool writes for a person, so the shapes it
/// carries are the ones the reading's own page states and no schema stands
/// behind them. A machine-readable output is a second arm, admitted beside the
/// grammar that reads it and carrying whatever ceiling that output affords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadingSource {
    /// The line-oriented console stream the backend writes as it runs.
    ConsoleStream,
}

/// Which version of an adapter's stated line grammar one reading was taken
/// under.
///
/// # Authority
///
/// The adapter's own number, moving when and only when the line shapes its page
/// states move. It is neither the backend's version nor an encoding version:
/// three things move for three reasons, and a bump to one is never a bump to
/// another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GrammarVersion(u32);

/// The most one reading's evidence can establish, in the verdict vocabulary.
///
/// # Authority
///
/// A ceiling follows from what the reading's SOURCE carries, and it is applied
/// where a reading is built: a run carrying a verdict outside its profile's
/// ceiling is not a reading anybody can hold. Which verdicts a ceiling admits
/// is declared in `type_contract.rs`, so the ceiling is read in one place
/// rather than restated by each road that stands under it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaimCeiling {
    /// The strongest verdict is a kill that asserts witness rejection and
    /// states nothing about activation. The source carries no channel that
    /// could observe a damage firing, so no mutant read under it earns
    /// [`MutationVerdict::Survived`] and every non-kill is
    /// [`MutationVerdict::Inconclusive`].
    WitnessRejection,
}

/// What one reading of a backend's output is stated under: the backend, the
/// version posture that backend's run carries, the output the reading was taken
/// from, and the adapter grammar that read it.
///
/// # Authority
///
/// The reading's assumption, made a value. A [`WrapReading`] cannot be built
/// without one, so "which grammar was this read under, and what may it claim"
/// is answered at the reading rather than remembered around it.
///
/// # Construction
///
/// The claim ceiling is READ from the source ([`AdapterProfile::ceiling`])
/// rather than stated into the profile, so no profile grants its reading more
/// than its source affords.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdapterProfile {
    backend: WrappedBackend,
    version: BackendVersionPosture,
    source: ReadingSource,
    grammar: GrammarVersion,
}

/// The caller-supplied reading from one source coordinate to the claim that owns
/// it.
///
/// # Authority
///
/// The origin graph is READ on the generator side — a reading of the one join,
/// never a second structure — and this seam is where that reading arrives. A
/// function pointer rather than a closure, so the seam carries no captured state
/// and nothing ambient rides in with it.
///
/// # Nonclaims
///
/// Answering `None` states that no mapping was available for the coordinate. It
/// is not a claim that the coordinate owns no claim, which is exactly why the
/// posture it produces is [`MappingPosture::OwnerUnmapped`] rather than a
/// claim the lane picked.
pub type OwnerLookup = fn(&SourceCoordinate) -> Option<ClaimRef>;

/// The caller-supplied reading from one backend's damage text to the operator
/// family it realizes.
///
/// # Authority
///
/// The bank's families are declared by what they ATTACK, and a backend's damage
/// prose is not a family name. Attributing one from that prose would be this
/// lane inventing a fact, so the reading is the caller's — exactly as the owner
/// mapping is — and answering `None` produces
/// [`FamilyAttribution::OutsideTheBank`] rather than a family the lane picked.
pub type FamilyLookup = fn(&SourceCoordinate, &[u8]) -> Option<OperatorFamilyRef>;

/// The outcome word one line of a compiled-mutation backend's output states
/// about one mutant.
///
/// # Bounds
///
/// The roster is the set of outcomes this parser reads. A line whose leading
/// word is none of these is an [`UnparsedLine`] and is never guessed at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapOutcomeWord {
    /// The backend's own command rejected the mutant.
    Caught,
    /// The backend's own command accepted the mutant.
    Missed,
    /// The mutant did not build.
    Unviable,
    /// The mutant's run passed the backend's time bound.
    TimedOut,
    /// The backend itself failed around the mutant.
    ToolFailed,
}

/// One line of a backend's output this parser could not read.
///
/// # Authority
///
/// Never dropped. A parser that discarded what it did not understand would shrink
/// the denominator without anybody being able to read that it had, so every
/// unrecognized line becomes one of these and travels with the reading.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnparsedLine {
    ordinal: usize,
    text: ForeignText,
}

/// What the backend announced about its own roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnnouncedRoster {
    /// The backend stated how many mutants it found.
    Stated(u32),
    /// The output states no roster count.
    Unstated,
}

/// What one reading of a compiled-mutation backend's output recovered: the
/// profile it was read under, the run, the roster the backend announced, and
/// every line the parser could not read.
///
/// # Nonclaims
///
/// The announced roster and the run's census answer different questions. A
/// difference between them says the parse and the backend disagree about how
/// many mutants there were, which is a finding for a reader and never a number
/// this value reconciles on its own.
///
/// The reading claims exactly what its profile's ceiling affords. A reader that
/// wants to know what a reading may be stood on takes
/// [`AdapterProfile::ceiling`], and no road anywhere widens it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrapReading {
    profile: AdapterProfile,
    run: MutationRun,
    announced: AnnouncedRoster,
    unparsed: Vec<UnparsedLine>,
}

/// Why one reading of a backend's output was refused.
///
/// Dependent checks in a declared order: the baseline is read before any mutant
/// line, because a lane that minted kills under an unqualified baseline would be
/// minting evidence it does not have, and the run is weighed against the
/// profile's ceiling before a reading stands over it.
#[must_use = "a refusal is the reason a wrap reading was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapRefusal {
    /// The output states no unmutated-baseline line at all, so the precondition
    /// every kill stands on was never established.
    BaselineNotStated,
    /// The baseline the output states does not qualify.
    BaselineNotQualified(BaselinePrecondition),
    /// One mutant line's record was refused by the lawful-kill constructor.
    KillNotLawful {
        /// Which line of the output, counting from zero.
        ordinal: usize,
        /// What the constructor refused.
        cause: KillRefusal,
    },
    /// One record in the run carries a verdict the profile's ceiling does not
    /// admit, so the reading would state more than its source affords.
    VerdictPastCeiling {
        /// The record's position in the run, counting from zero.
        at: usize,
        /// The verdict the record carries.
        verdict: MutationVerdict,
        /// What the profile's source affords.
        ceiling: ClaimCeiling,
    },
}

// ---------------------------------------------------------------------------
// The two facts a reading opens trust with.
// ---------------------------------------------------------------------------

/// Whether the wrap-first pressure has reported, and what it reported.
///
/// # Authority
///
/// The first half of the trust order, carried as the whole PROFILED reading
/// rather than as a bare run. The backend, the version posture, the output the
/// reading was taken from, the adapter grammar, and the ceiling that source
/// affords all ride with the evidence — and they are exactly the facts the
/// trust-opening road weighs, so dropping them at the moment provenance decides
/// something is not a shape this vocabulary has.
///
/// # Nonclaims
///
/// A report is not the same fact as a report that killed something. A wrap pass
/// with no kill is not evidence that the properties bite, and
/// [`CompiledPressureWitness::shown`] reads it as the absence it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapStanding<'reading> {
    /// The wrap-first pressure reported, and this is the reading it reported.
    Reported(&'reading WrapReading),
    /// The wrap-first pressure has not reported.
    NotReported,
}

/// Whether anybody has checked one adapter's stated line grammar against output
/// the backend itself wrote.
///
/// # Authority
///
/// The [`BackendVersionPosture`] shape, for the same reason: what stands behind
/// a reading is a party's own word, and the bare arm is the BOOTSTRAP posture
/// rather than a value somebody forgot to fill in. Ahead of the first toolchain
/// contact nothing has been checked against anything, so
/// [`GrammarStanding::Unchecked`] is the only arm anybody can state honestly,
/// and a reading under it is exactly as good as the adapter's own page.
///
/// # Nonclaims
///
/// [`GrammarStanding::Checked`] records that a party checked this adapter's
/// stated shapes against output that version of the backend wrote. It is that
/// party's word rather than a verification this crate performed: nothing here
/// invokes a backend, and nothing here reads a backend's output to discover
/// what that backend renders.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrammarStanding {
    /// A party checked the adapter's stated line shapes against output this
    /// version of the backend wrote, and states so.
    Checked(BackendVersion),
    /// Nobody has checked them against real output, so the adapter's grammar
    /// stands as the assumption its own page states.
    Unchecked,
}

/// That one reading's backend, output, and grammar profile stands qualified to
/// open trust under — and how far that standing reaches.
///
/// # Authority
///
/// The typed fact the trust-opening road demands about the TOOL, carried as a
/// value so that "which adapter produced this evidence, and what may a reading
/// under it claim" is answered at the evidence rather than remembered around
/// it. The profile is taken from the reading's own
/// ([`AdapterQualification::of`]), so a qualification can never name a profile
/// some other reading was taken under.
///
/// # Nonclaims
///
/// Parser correctness is not suite bite. A qualification says the adapter is
/// fit to be read under; it says nothing about whether any property rejected
/// anything, which is [`CompiledPressureWitness`]'s fact on a different axis.
/// The claim ceiling rides with it unchanged: a qualified adapter over a
/// console stream is still an adapter whose source carries no activation
/// channel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdapterQualification {
    profile: AdapterProfile,
    standing: GrammarStanding,
}

/// That one qualified reading demonstrated at least one lawful witness
/// rejection.
///
/// # Authority
///
/// The typed fact the trust-opening road demands about the RUN: a suite
/// rejected a damaged subject, under an adapter that stands qualified. The
/// qualification rides inside, taken from the very reading the kill was read
/// out of, so a witness over an unqualified reading is not a value anybody can
/// hold and a witness can never be married to another adapter's profile.
///
/// # Construction
///
/// [`CompiledPressureWitness::shown`] is the only road, and it refuses a
/// standing that never reported, then a reported reading whose run demonstrated
/// no kill.
///
/// # Nonclaims
///
/// Suite bite is not campaign accounting. The witness states that at least one
/// lawful rejection happened; how many mutants a run pressed and how they
/// divide is the run's own census ([`MutationCensus`]), which answers a
/// different question and is never read as this one. Neither of the two is the
/// no-mutation parity ([`ParityStanding`]), which is about the evaluation
/// copy's faithfulness to the rendered production surface and about nothing
/// else.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompiledPressureWitness {
    qualification: AdapterQualification,
    kill: MutationReport,
}

/// Why one wrap standing demonstrated no compiled-pressure witness.
///
/// Dependent checks in a declared order: whether the pressure reported at all,
/// then whether what it reported carries a kill.
#[must_use = "a refusal is the reason no compiled-pressure witness was shown"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureWitnessRefusal {
    /// The wrap-first pressure has not reported, so there is no reading to
    /// stand on.
    WrapNotReported,
    /// The reading's run demonstrated no lawful kill, so nothing in it has
    /// shown a property biting.
    NoKillDemonstrated,
}

// ---------------------------------------------------------------------------
// The interpreted lane's evaluation surface.
// ---------------------------------------------------------------------------

/// Where a selected alternative fires, named rather than path-spelled.
///
/// The reason a trial's identity is not its site holds here too: a file move
/// must rename nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActivationSite(NamespacedName);

/// One mutation point on an evaluation surface, as a producer states it.
///
/// # Authority
///
/// The field shapes are this crate's own reading of the producer-facing
/// mutation-point roster the descriptor vocabulary publishes
/// ([`crate::descriptor::MUTATION_POINT_FIELDS`]). The RUNTIME types are this
/// lane's, exactly as that roster says, so a producer emits against the
/// published vocabulary rather than against another crate's shape and nothing
/// here imports a generator.
///
/// # Nonclaims
///
/// A roster of admitted alternatives states which damages the point ADMITS, and
/// never that any of them was materialized, activated, or killed. Those are
/// executed facts and they live in [`MutationReport`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MutationPoint {
    identity: MutationPointRef,
    owner_claim: ClaimRef,
    original_operation: &'static [u8],
    admitted_alternatives: &'static [&'static [u8]],
    activation_site: ActivationSite,
}

/// Why one mutation point was refused.
///
/// Dependent checks in a declared order — the original operation is read, then
/// each alternative against it, then each alternative against its predecessors.
#[must_use = "a refusal is the reason a mutation point was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointRefusal {
    /// The point states no original operation, so its no-mutation reading is
    /// empty.
    EmptyOriginalOperation,
    /// An admitted alternative is byte-identical to the original operation, so
    /// selecting it would be the no-mutation reading under another name.
    AlternativeIsOriginal {
        /// The alternative's position in the roster.
        at: usize,
    },
    /// Two admitted alternatives carry one damage.
    DuplicateAlternative {
        /// The second alternative's position in the roster.
        at: usize,
    },
}

/// One evaluation copy's complete point table.
///
/// # Authority
///
/// The walk over the declaration happened at generation time; this is the table
/// that walk produced, arriving as conforming DATA. Runtime is selection among
/// these points and never interpretation of arbitrary source, which would mint a
/// second meaning authority.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvaluationSurface {
    points: Vec<MutationPoint>,
}

/// Why one evaluation surface was refused.
#[must_use = "a refusal is the reason an evaluation surface was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceRefusal {
    /// The table states no point at all, so nothing on this surface could ever
    /// be selected.
    EmptyPointTable,
    /// Two points state one identity.
    DuplicatePoint(MutationPointRef),
}

/// Which admitted alternative of one point a selection names.
///
/// # Construction
///
/// Minted only by [`EvaluationSurface::select`], so an index that names no
/// admitted alternative is not a value anybody can hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AlternativeIndex(usize);

/// One point selected into one of the damages it admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActiveSelection {
    point: MutationPointRef,
    alternative: AlternativeIndex,
}

/// What one run of the evaluation copy selects among the surface's points.
///
/// # Authority
///
/// Every evaluation surface contains the no-mutation mutant, and it is this
/// arm rather than a point: no point is damaged, so the copy reads exactly what
/// the declaration says. It is the road the mandatory parity is driven over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveMutant {
    /// No point is damaged — the no-mutation mutant every surface contains.
    NoMutation,
    /// One point reads as one of its admitted alternatives.
    Active(ActiveSelection),
}

/// Why one active-mutant selection was refused.
#[must_use = "a refusal is the reason a mutant was not selected"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionRefusal {
    /// The surface states no point under this identity.
    NoSuchPoint(MutationPointRef),
    /// The point admits fewer alternatives than this index names.
    AlternativePastRoster {
        /// How many alternatives the point admits.
        admitted: usize,
        /// The index that was named.
        named: usize,
    },
}

/// Why the mandatory no-mutation parity suite could not be declared.
///
/// Dependent checks in a declared order: the substrate names are parsed, then
/// the roster they are declared into.
#[must_use = "a refusal is the reason a parity suite was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParityRefusal {
    /// A name this lane spells would not parse.
    NameNotParsed(NameRefusal),
    /// The shared-substrate roster refused the substrates it was given.
    SubstrateNotDeclared(SubstrateRefusal),
}

/// Whether the mandatory no-mutation parity has passed.
///
/// # Construction
///
/// [`ParityStanding::of`] reads it from the trial's own conclusion, so nobody
/// records this by hand from a run they remember.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParityStanding {
    /// The parity trial ran and both roads agreed.
    Passed,
    /// The parity trial has not passed.
    NotPassed,
}

/// Which of the trust order's facts the interpreted lane is still owed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MissingTrustEvidence {
    /// No qualified reading has demonstrated a witness rejection, so nothing
    /// has shown the properties bite.
    WrapEvidence,
    /// A witness stands, and it was shown under a different adapter
    /// qualification than the one trust is being opened under — so it is
    /// evidence about another adapter's reading and opens nothing here.
    WitnessUnderAnotherQualification,
    /// The mandatory no-mutation parity has not passed.
    NoMutationParity,
}

/// What the interpreted lane is available for right now.
///
/// # Authority
///
/// Absence is a typed disposition and never a crippled fake interpreter:
/// interpreted mutation is available exactly when a conforming evaluation
/// surface exists and the trust order has opened, and every other state names
/// itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InterpreterAvailability<'surface> {
    /// A conforming evaluation surface exists and trust has opened.
    Available {
        /// The surface selection runs over.
        surface: &'surface EvaluationSurface,
    },
    /// No conforming evaluation surface exists — neither a producer's nor a
    /// hand-authored one under the same contract.
    NoConformingSurface,
    /// A surface exists and the trust order still owes this evidence.
    TrustNotOpened {
        /// What the staging is still owed.
        missing: MissingTrustEvidence,
    },
}

// ---------------------------------------------------------------------------
// The rewrite lane.
// ---------------------------------------------------------------------------

/// One rewrite-mutation descriptor: the shape a damage matches, the shape it
/// rewrites to, and the operator family the pair realizes.
///
/// # Authority
///
/// Data rows, never programs. A descriptor states a pattern and its rewrite as
/// text a structural rewriter reads; nothing here compiles, executes, or
/// interprets either side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RewriteDescriptor {
    family: OperatorFamilyRef,
    pattern: &'static str,
    rewrite: &'static str,
}

/// Why one rewrite descriptor was refused.
///
/// Dependent checks in a declared order: the pattern, then the rewrite, then the
/// pair.
#[must_use = "a refusal is the reason a rewrite descriptor was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteRefusal {
    /// The pattern is empty, so the descriptor matches nothing.
    EmptyPattern,
    /// The rewrite is empty, so the descriptor states no damage.
    EmptyRewrite,
    /// The pattern and the rewrite are one shape, so applying it damages
    /// nothing.
    RewriteIsPattern,
}

/// The rewrite lane's declared descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RewriteRoster {
    descriptors: Vec<RewriteDescriptor>,
}

/// Why one rewrite roster was refused.
#[must_use = "a refusal is the reason a rewrite roster was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RosterRefusal {
    /// The roster states no descriptor at all.
    EmptyRoster,
    /// Two entries state one pattern-and-rewrite pair.
    DuplicateDescriptor {
        /// The second entry's position in the roster.
        at: usize,
    },
}

/// The trust posture every rewrite-produced descriptor stands under.
///
/// # Authority
///
/// Rewrite-produced descriptors are admitted LAST in the trust order: they are
/// candidates for the harness to audit and never evidence on their own.
///
/// # Bounds
///
/// A second posture would be a law change, and the trust order's sentence would
/// have to move first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteTrust {
    /// The descriptor awaits the harness's audit.
    AuditPending,
}

/// One rewrite descriptor planned for audit: the descriptor, the scope its
/// application was planned under, and the trust posture it stands under.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RewriteCandidate {
    descriptor: RewriteDescriptor,
    scope: ScopeShape,
    trust: RewriteTrust,
}

/// Why the rewrite lane's descriptors may not be admitted as evidence yet.
#[must_use = "a refusal is the reason the rewrite lane was withheld"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteWithheld {
    /// The interpreted lane — the execution substrate that makes rewrite
    /// families cheap — is not available.
    InterpreterUnavailable,
    /// The trust order still owes this evidence.
    TrustNotOpened(MissingTrustEvidence),
}

/// Whether the rewrite lane's descriptors are admitted as evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteAdmission {
    /// Admitted last, after the wrap report and the parity, with the interpreted
    /// lane standing under them.
    Admitted,
    /// Not yet, for a stated reason.
    Withheld(RewriteWithheld),
}

// ---------------------------------------------------------------------------
// The artifact-mutation seed roster.
// ---------------------------------------------------------------------------

/// One deliberate damage the artifact-mutation mode inflicts on a lawful
/// rendered artifact.
///
/// # Authority
///
/// Each arm is a LIE a damaged rendering tells about the declaration it claims
/// to project, and every one of them is this harness's own: the services carry
/// no road that renders a defective artifact, because a producer that writes its
/// own exam is rehearsed only against the defects it already imagined.
///
/// The roster is seed material and not a lane. It states WHICH damages the
/// self-attack mode plans for; the surgery that realizes one over a rendered
/// artifact is authored where the anchors are authored, so that a damage is cut
/// against the anchors a generator emits rather than against spellings a hand
/// restated beside them.
///
/// # Nonclaims
///
/// It says nothing about which reader CATCHES a damage. Ownership of a catching
/// claim belongs to the readers that exist — the independence annex's lanes
/// ([`crate::oracle`]) — and is stated there, against a seat that can hold it,
/// once the self-attack mode runs over generated anchors. A roster that carried
/// its own ownership table with no run to falsify it would be a green wall that
/// measures nothing, which is the defect this vocabulary is descended from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactMutation {
    /// The textual selection order is reversed while the typed order stands as
    /// declared — the projection no longer projects.
    OrderPermuted,
    /// Every cause is emitted under the first cause's local key: distinct causes
    /// inside one family made to share one identity.
    IdentityRecycled,
    /// One planned output is deleted from the artifact.
    PlannedOutputOmitted,
    /// An output nobody planned is appended.
    UnplannedOutputAdded,
    /// The implementation targets a different type than the one declared.
    ImplTargetAltered,
    /// The declared body shape is changed.
    ShapeAltered,
    /// A planned output is emitted twice.
    OutputDuplicated,
    /// The trait path names a contract the declaration did not realize.
    TraitPathWrong,
    /// A decoy carrying the anchored bytes is planted inside a comment while the
    /// real constant is damaged.
    DecoyInComment,
    /// One planned member constant is emitted twice inside one implementation.
    ImplMemberDuplicated,
    /// A member nobody planned is added inside one implementation.
    ImplMemberUnexpected,
    /// A declared value is carried through a constructor the declaration did not
    /// name.
    ConstructorPathAltered,
    /// The implementation is written under a posture the declaration did not
    /// name.
    ImplPostureAltered,
    /// An attribute that decides something is added to an implementation.
    MeaningBearingAttributeAdded,
    /// The artifact stops being well-formed Rust.
    MalformedRust,
}

/// The artifact-mutation roster, in the order this home states it.
///
/// # Authority
///
/// A declared table rather than a derived one: the order is the order a plan
/// reads the damages in, so it is written down once here instead of arriving
/// from whichever road happened to enumerate them.
pub const ARTIFACT_MUTATIONS: [ArtifactMutation; 15] = [
    ArtifactMutation::OrderPermuted,
    ArtifactMutation::IdentityRecycled,
    ArtifactMutation::PlannedOutputOmitted,
    ArtifactMutation::UnplannedOutputAdded,
    ArtifactMutation::ImplTargetAltered,
    ArtifactMutation::ShapeAltered,
    ArtifactMutation::OutputDuplicated,
    ArtifactMutation::TraitPathWrong,
    ArtifactMutation::DecoyInComment,
    ArtifactMutation::ImplMemberDuplicated,
    ArtifactMutation::ImplMemberUnexpected,
    ArtifactMutation::ConstructorPathAltered,
    ArtifactMutation::ImplPostureAltered,
    ArtifactMutation::MeaningBearingAttributeAdded,
    ArtifactMutation::MalformedRust,
];

// ---------------------------------------------------------------------------
// Survivor explanation and the check gap.
// ---------------------------------------------------------------------------

/// Which independence lane a survivor's explanation names as the missing judge.
///
/// # Authority
///
/// The roster is the independence annex's own lanes ([`crate::oracle`]), named
/// here so an explanation says WHICH kind of judge is absent rather than that
/// something is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OracleClass {
    /// The golden-vector lane: bytes a specification states for an input.
    GoldenVector,
    /// The independent transcript lane: a published identity re-derived from its
    /// published specification.
    IndependentTranscript,
    /// The structural read: what a rendered artifact declares.
    StructuralRead,
    /// The compiled read-back: what a compiled artifact hands back as values.
    CompiledReadBack,
}

/// One survivor, explained: the target that survived, the claim that owns it,
/// the oracle class no check of that claim supplies, and the check reference
/// that would close the opening.
///
/// # Authority
///
/// The whole hand-off into synthesis, in one value: survivor, owning claim,
/// missing oracle class, closing check. An explanation over an owner-unmapped
/// target is refused rather than guessed, so no candidate is ever cut against a
/// claim nobody established.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SurvivorExplanation {
    target: MutationTarget,
    claim: ClaimRef,
    missing: OracleClass,
    closing: CheckRef,
}

/// Why one survivor explanation was refused.
#[must_use = "a refusal is the reason a survivor was not explained"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplanationRefusal {
    /// The record's verdict is not survived, so there is no survivor to explain.
    NotASurvivor(MutationVerdict),
    /// The target's owning claim is unmapped, so the explanation would have to
    /// invent the claim it hands to synthesis.
    OwnerUnmapped,
}

/// The typed finding a synthesis raises instead of a candidate it cannot
/// honestly build.
///
/// # Authority
///
/// Synthesis is scoped to already-authored executable attachments — descriptors,
/// never programs. Where the check an explanation names has no attachment, the
/// opening is this finding, and a fake candidate that referenced a callable
/// nobody wrote is not a value this home can produce.
#[must_use = "a check gap is a finding, never a candidate"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckGap {
    claim: ClaimRef,
    check: CheckRef,
    missing: OracleClass,
}

/// The row coordinates a synthesis cannot read off a survivor.
///
/// The explanation names the claim and the check; the suite the candidate would
/// run under, its classification, the subject route it exercises, and the
/// population that supplies its inputs are the caller's to state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSketch {
    suite: ExecutionSuite,
    classification: Classification,
    subject: SubjectRoute,
    population: PopulationRef,
}

/// Why one candidate row could not be synthesized.
///
/// Dependent checks in a declared order: the attachment roster is read, then the
/// synthesis facts the origin arm needs, then the row itself.
#[must_use = "a refusal is the reason a candidate was not synthesized"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthesisRefusal {
    /// The check the explanation names has no authored executable attachment, so
    /// the opening is a check gap.
    CheckGapFound(CheckGap),
    /// The explained record's identity is an external backend's mutant, which
    /// names a coordinate rather than a mutation point, and the descriptor
    /// vocabulary's candidate arm carries a point or a proof gap and nothing
    /// else.
    ///
    /// A guard on the IDENTITY shape rather than on a lane. No external reading
    /// this crate declares can reach it today: earning the survived verdict
    /// takes observed activation, and the one wrapped backend offers no channel
    /// that could observe a damage firing.
    ExternalSurvivorNamesNoPoint,
    /// The row constructor refused the values the synthesis assembled.
    RowRefused(RowRefusal),
}

// ---------------------------------------------------------------------------
// Scope, budget, and the proof plan.
// ---------------------------------------------------------------------------

/// One path a diff touched, as the caller read it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DiffPath(String);

/// Why one diff path was refused.
#[must_use = "a refusal is the reason a diff path was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffPathRefusal {
    /// The path is empty, so it names nothing.
    EmptyPath,
}

/// What one invocation of a lane is scoped to.
///
/// # Authority
///
/// Scope shapes are INVOCATION parameters and never a second world: each one
/// narrows a run, and the denominator every report is stated over is the
/// complete table regardless.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeShape {
    /// One seam: the subject route the run is narrowed to.
    SeamScoped {
        /// The route the seam is entered through.
        route: SubjectRoute,
    },
    /// Every row the world presents.
    RepoWide,
    /// Only what a diff touched.
    DiffScoped {
        /// The paths the diff touched, in the order the caller read them.
        touched: Vec<DiffPath>,
    },
}

/// What one scoped pressure run may spend.
///
/// # Authority
///
/// The mutant bound is this home's; the per-trial budgets are the invocation
/// profile's, borrowed rather than restated, so no second budget authority
/// answers the question the record vocabulary already answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PressureBudget {
    mutants: u32,
    invocation: InvocationProfile,
}

/// Why one pressure budget was refused.
#[must_use = "a refusal is the reason a budget was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetRefusal {
    /// The budget admits no mutant, so the run it bounds would press nothing.
    ZeroMutants,
}

/// One scope shape with the budget its run may spend.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopedInvocation {
    scope: ScopeShape,
    budget: PressureBudget,
}

/// Which lane one planned run belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureLane {
    /// Compiled mutation, wrapped around an external backend.
    CompiledMutation,
    /// The compile-once interpreter's rapid loop.
    InterpretedMutation,
    /// Structure-aware generation with budgeted minimization.
    Fuzz,
    /// Campaigns over the fault adapters.
    Chaos,
}

/// Which damage one planned run presses.
///
/// # Authority
///
/// An external backend chooses its own damage and the mutant identity already
/// names it; an interpreted run states which admitted alternative of its point
/// it selects. Carrying the distinction is what keeps two planned runs over one
/// point from reading as one run stated twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlannedDamage {
    /// The backend's own damage, already named by the mutant identity.
    BackendChosen,
    /// One admitted alternative of an interpreted point.
    Alternative(AlternativeIndex),
}

/// One intended run: which lane presses which damage of which target, what the
/// run selects, and what it may spend.
///
/// # Authority
///
/// A planned run is a VALUE and spends nothing. It is what makes planning
/// inspectable before a budget is committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedRun {
    lane: PressureLane,
    target: MutationIdentity,
    damage: PlannedDamage,
    selection: Selection,
    budget: PressureBudget,
}

/// The complete statement of what one pressure pass intends to run, before any
/// budget is spent.
///
/// # Authority
///
/// Planning is a pure function and this is its image: a caller reads every
/// intended run and its budget, and decides, before the first mutant is pressed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPlan {
    scope: ScopedInvocation,
    runs: Vec<PlannedRun>,
}

/// Why one proof plan was refused.
///
/// Dependent checks in a declared order: the roster is read before it is
/// weighed against the budget.
#[must_use = "a refusal is the reason a proof plan was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanRefusal {
    /// The plan states no run at all, so it presses nothing.
    NoRunPlanned,
    /// The plan states more runs than the scope's mutant budget admits.
    BudgetOverspent {
        /// How many mutants the budget admits.
        admitted: u32,
        /// How many runs the plan states.
        planned: usize,
    },
}

// ---------------------------------------------------------------------------
// The obligation road.
// ---------------------------------------------------------------------------

/// A claim declared owed: its identity, and the opening condition its
/// declaration named.
///
/// # Authority
///
/// "Owed" is a POSTURE on a claim under the named-opening-condition rule, never
/// a genus. The claim is a typed reference, so a citation to nothing is refused
/// by the name parser before this value exists; the opening condition is refused
/// empty, so an obligation that never comes due is not a value anybody can hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwedClaim {
    claim: ClaimRef,
    opening_condition: &'static str,
}

/// Why one owed-claim posture was refused.
#[must_use = "a refusal is the reason an owed claim was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwedClaimRefusal {
    /// The posture names no opening condition, so nothing states when the claim
    /// comes due.
    NoOpeningCondition,
}

/// What shape of proof one opening asks for.
///
/// # Authority
///
/// The routing input. Which lane discharges an obligation follows from the shape
/// of proof it needs, and that map is declared once in `type_contract.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofShape {
    /// One stated input and its stated answer.
    StatedCase,
    /// A search over a generated population.
    GeneratedSearch,
    /// A fault placed at a sequence position.
    ScheduledFault,
}

/// Which lane one inferred obligation is routed to discharge in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObligationLane {
    /// A descriptor row in the authored table.
    TestRow,
    /// A seed in the fuzz lane's warm start.
    FuzzSeed,
    /// A scenario in the chaos lane's campaign.
    ChaosScenario,
}

/// One claim declared owed, and the shape of proof its opening asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwedDeclaration {
    owed: OwedClaim,
    shape: ProofShape,
}

/// One opening a coverage reading states: an owed claim the denominator names
/// and no report exercised.
///
/// # Authority
///
/// "Where is proof missing" is claim coverage over reports, never a structural
/// scan — so this value is born from a [`crate::report::ClaimCoverage`] entry
/// and carries the counts it was born from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InferredObligation {
    owed: OwedClaim,
    exercise: ClaimExercise,
    shape: ProofShape,
}

/// What discharged one owed claim.
///
/// # Authority
///
/// A discharge authors no capsule: the admitted row IS the discharge's permanent
/// record, and rerunning it regenerates the behavioral evidence. What is carried
/// here is the lane the obligation was routed to, the trial that discharged it,
/// and the key that trial ran under — every one of them reconstructible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DischargeEvidence {
    lane: ObligationLane,
    trial: TrialId,
    key: ExecutionKey,
}

// ---------------------------------------------------------------------------
// The proposal road.
// ---------------------------------------------------------------------------

/// One demonstrated kill: the report the staged run wrote, and the rejection
/// read out of it.
///
/// # Authority
///
/// A claimed kill is DEMONSTRATED on the evaluation surface with the mutant
/// active, never asserted. This value is what a demonstration leaves behind, and
/// the proposal road's mutant-killed ground cannot be built without one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Demonstration {
    report: RunReport,
    rejection: DemonstratedRejection,
}

/// Why no kill was demonstrated.
///
/// Dependent checks in a declared order: the view's posture, then the census,
/// then the candidate's own disposition. The construction arm stands first
/// because there is no report to read until the staged view has been built.
#[must_use = "a refusal is the reason a kill was not demonstrated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofRefusal {
    /// The staged view could not be built.
    StagingRefused(StagedTableRefusal),
    /// The report stands over the authored world rather than a staged view, so
    /// no candidate was proven by it.
    NotStaged,
    /// The report's census does not carry the candidate's trial at all.
    CandidateNotInCensus,
    /// The run's selection passed the candidate over, so it never executed.
    CandidateNotSelected,
    /// The candidate was selected and did not execute.
    CandidateDidNotExecute,
    /// The candidate executed and did not refuse, so the claimed kill is
    /// asserted rather than shown.
    CandidateDidNotRefuse,
}

/// How much proof one candidate adds to the claim it pins.
///
/// # Construction
///
/// [`ProofDelta::between`] takes the claim's exercised counts before and after
/// the candidate ran in staging, and refuses a pair that does not move: a pin
/// that adds nothing is not a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProofDelta {
    before: usize,
    after: usize,
}

/// Why one proof delta was refused.
#[must_use = "a refusal is the reason a proof delta was not stated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofDeltaRefusal {
    /// The candidate leaves the claim's exercised count where it was, so it pins
    /// nothing.
    NoProofAdded {
        /// The count before.
        before: usize,
        /// The count after.
        after: usize,
    },
}

/// The structural ground one proposal stands on, carrying exactly what that
/// ground possesses.
///
/// # The grounds
///
/// [`ProposalGround::MutantKilled`] carries the target, its activation
/// disposition, the replay capsule, and the demonstration — a kill shown on the
/// surface with the mutant active.
///
/// [`ProposalGround::ClaimPinned`] carries the claim, the replay capsule, and
/// the proof delta the pin moved.
///
/// [`ProposalGround::ObligationDischarged`] carries the owed claim's identity
/// and the discharge evidence, and no capsule at all: the admitted row is the
/// discharge's permanent record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalGround {
    /// The proposal killed a real mutant.
    MutantKilled {
        /// What was damaged.
        target: MutationTarget,
        /// What the damage's activation was.
        activation: ActivationDisposition,
        /// The reproduction account of the demonstrating run.
        capsule: ReplayCapsule,
        /// The demonstrated kill.
        demonstration: Demonstration,
    },
    /// The proposal pinned a named claim.
    ClaimPinned {
        /// The claim pinned.
        claim: ClaimRef,
        /// The reproduction account of the pinning run.
        capsule: ReplayCapsule,
        /// What the pin added to the claim's proof.
        delta: ProofDelta,
    },
    /// The proposal discharged a claim declared owed.
    ObligationDischarged {
        /// The owed claim's identity.
        owed: OwedClaim,
        /// What discharged it.
        discharge: DischargeEvidence,
    },
}

/// Why one comparison had no subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoComparisonReason {
    /// The ground carries no failure, so there is no fingerprint to compare.
    GroundCarriesNoFailure,
    /// Nothing comparable was kept: no previous fingerprint and no discharge
    /// roster.
    NoKnownMaterial,
}

/// The typed, ground-aware evidence that one proposal is not a duplicate.
///
/// # Authority
///
/// Never persuasive prose. A failure-bearing ground compares fingerprints, a
/// discharge ground compares the owed claim's known discharges, and anything
/// else states its reason. The comparison is performed where the value is built,
/// so a duplicate is a refusal rather than a paragraph a reader has to check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DuplicateEvidence {
    /// The candidate's fingerprint, against every fingerprint already known.
    FailureCompared {
        /// The fingerprint this candidate carries.
        candidate: Fingerprint,
        /// The fingerprints already known, in the order they were compared.
        known: Vec<Fingerprint>,
    },
    /// The owed claim, against the discharges already recorded for it.
    ObligationCompared {
        /// The owed claim.
        owed: ClaimRef,
        /// The trials already recorded as discharging it.
        discharges: Vec<TrialId>,
    },
    /// Neither comparison has a subject, for this reason.
    NotApplicable {
        /// Why nothing was compared.
        reason: NoComparisonReason,
    },
}

/// Why one duplicate comparison refused its candidate.
#[must_use = "a refusal is the reason a proposal was not offered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DuplicateRefusal {
    /// The candidate's fingerprint is one the known roster already carries, so
    /// it is a second spelling of a find already made.
    FingerprintAlreadyKnown(Fingerprint),
    /// The owed claim already carries a discharge, so this one discharges
    /// nothing new.
    ObligationAlreadyDischarged(TrialId),
}

/// Where an admitted row would land: a semantic owner and a suite, never a file
/// path.
///
/// # Authority
///
/// One field, because the suite's own namespace IS the semantic owner — the
/// descriptor vocabulary says so where the destination is declared
/// ([`crate::descriptor::AdmissionFacts`]), and a second owner field here would
/// be a second authority answering one question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProposalDestination {
    suite: ExecutionSuite,
}

/// The domain tag every proposal identity is derived under.
pub const PROPOSAL_TAG: DomainTag = DomainTag::declared("proposal");

/// One proposal: a candidate row, the typed ground it stands on, the evidence it
/// is not a duplicate, and where it would land.
///
/// # Authority
///
/// Process-local until a caller's own sink stores it. Constructing one asserts
/// nothing about admission: a human admits, and admission is a two-part
/// human-authored patch this crate never performs.
///
/// # Construction
///
/// [`Proposal::offered`] refuses a row that does not carry the candidate origin
/// arm, refuses duplicate evidence that does not match the ground, and refuses a
/// survivor synthesis fact that names a different point than the ground's
/// target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    candidate: Row,
    ground: ProposalGround,
    duplicate: DuplicateEvidence,
    destination: ProposalDestination,
}

/// Why one proposal was refused.
///
/// Dependent checks in a declared order: the row's origin, then the ground
/// against its evidence, then the survivor point against the target.
#[must_use = "a refusal is the reason a proposal was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProposalRefusal {
    /// The row does not carry the candidate origin arm, so it is an authored row
    /// entering by the proposal door.
    NotACandidate,
    /// The duplicate evidence does not match the ground: a failure-bearing
    /// ground was offered a comparison that compares no failure, or a discharge
    /// ground was offered anything but a discharge comparison.
    EvidenceDoesNotMatchGround,
    /// The row's synthesis facts name one mutation point and the ground's target
    /// names another.
    SurvivorPointMismatch {
        /// The point the row's synthesis facts name.
        synthesis: MutationPointRef,
        /// The point the ground's target names.
        target: MutationPointRef,
    },
}

/// Why one mutant-killed proposal was not offered.
///
/// Dependent checks in a declared order: the duplicate comparison is taken
/// before the proposal is assembled, so a find already made never reaches the
/// constructor at all.
#[must_use = "a refusal is the reason a mutant-killed proposal was not offered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KillProposalRefusal {
    /// The comparison found the candidate's failure already known.
    Duplicate(DuplicateRefusal),
    /// The proposal constructor refused the values that were assembled.
    Refused(ProposalRefusal),
}

/// The location one sink stored a proposal at.
///
/// # Authority
///
/// Opaque and MORTAL. The review artifact may die after any ruling, which is
/// exactly why an admitted origin cites the proposal's content identity
/// ([`ProposalId`]) and never this token — so nothing dangles when the artifact
/// is deleted.
///
/// # Nonclaims
///
/// It is not an identity, not a path this crate can interpret, and not evidence
/// that the destination is durable. What the token spells is the sink's own
/// business.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StoredProposalRef {
    token: String,
}

/// Why one sink did not store a proposal.
///
/// # Authority
///
/// The family a caller's sink answers with. The durability arm in particular is
/// the sink's own statement: this crate reaches no filesystem and can establish
/// nothing about where a sink writes.
#[must_use = "a refusal is the reason a proposal was not stored"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinkRefusal {
    /// The sink is not accepting proposals.
    Unavailable,
    /// The sink already holds a proposal under this content identity.
    AlreadyStored(ProposalId),
    /// The location offered is empty, so it names nowhere.
    EmptyLocation,
    /// The destination lies inside the repository tree or a build cache, where
    /// undischarged review evidence may never live: deleting a cache must change
    /// only cost, never truth.
    DestinationNotDurable,
}

/// The caller-supplied storage the proposal road ends at.
///
/// # Authority
///
/// The ONLY storage seam anywhere in this crate. No realization is declared
/// here, no filesystem is reached from this crate at all, and no scratch
/// directory exists: a proposal is process-local until a caller's own sink
/// stores it, and review-durable custody begins at the reference the sink hands
/// back.
///
/// # Nonclaims
///
/// Storing is not admitting. A stored proposal is review material a human rules
/// on — real, duplicate, tooling defect, or invalid baseline — and the ruling is
/// what discharges it.
pub trait ProposalSink {
    /// Store one proposal, and hand back the location custody begins at.
    ///
    /// # Errors
    ///
    /// The sink's own refusal: unavailable, already stored under this identity,
    /// an empty location, or a destination that is not durable.
    fn store(&mut self, proposal: &Proposal) -> Result<StoredProposalRef, SinkRefusal>;
}

/// The two parts a human's admission act authors, named so the proposal road's
/// exit is stated rather than implied.
///
/// # Authority
///
/// Admission is OUT OF SCOPE for this crate: it is a human act, and nothing here
/// performs either part. This value exists so a reader of a proposal can see
/// what admitting it would require, and so no road here is mistaken for one that
/// admits.
///
/// # Nonclaims
///
/// Holding one is not an admission and creates nothing. The authored row and —
/// for a replay-bearing ground — the depot capsule entry are both written by the
/// admission act itself, by hand, outside this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdmissionPatch {
    /// The admission authors the row and a depot capsule entry the row's replay
    /// reference points at.
    RowAndCapsule,
    /// The admission authors the row alone; the row IS the discharge's permanent
    /// record.
    RowAlone,
}

// ---------------------------------------------------------------------------
// The shared-substrate spelling the no-mutation parity names.
// ---------------------------------------------------------------------------

/// The owner every name this home spells is declared under.
pub const MUTERPRATER_NAMESPACE: &str = "muterprater";

/// The spelling of the declaration both no-mutation parity roads are projected
/// from.
pub const PARITY_DECLARATION_SUBSTRATE: &str = "one-declaration";

/// The spelling of the rendering engine both no-mutation parity roads stand on.
pub const PARITY_RENDERING_SUBSTRATE: &str = "rendering-engine";

/// The spelling the no-mutation parity's road pairing is declared under.
pub const NO_MUTATION_PAIRING: &str = "no-mutation-parity";

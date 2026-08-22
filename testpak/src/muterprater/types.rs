//! The proof-pressure engine's declarations: the verdict chain's axes, the
//! per-mutant record and its run, the mutation target and its owner mapping, the
//! wrap lane's adapter profile and reading vocabulary, the two facts a reading
//! opens trust with, the interpreted lane's evaluation surface and trust gate,
//! the rewrite lane's descriptors, the artifact-mutation seed roster, the
//! survivor explanation and the check gap, the scope shapes and the proof plan,
//! and the whole proposal road.
//!
//! Declarations only. Every road that reaches a private field is this file's own
//! child, `type_guard.rs`; declarative trait participation is in
//! `type_contract.rs`; the four lanes are the role-named modules beside them.
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
    AdmissionGround, CheckRef, ClaimRef, Classification, ExecutionSuite, MutationPointRef,
    NameRefusal, NamespacedName, PopulationRef, ProposalId, RevisionBinding, Row, RowRefusal,
    StagedTableRefusal, SubjectRoute,
};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::properties::{Equivalence, SharedSubstrate, SubstrateRefusal};
use crate::report::{
    ClaimExercise, ExecutionKey, Fingerprint, ForeignText, InvocationProfile, ReplayCapsule,
    RunReport, TrialConclusion, TrialFinding, TrialId, TrialReport,
};
use crate::runner::{ReportRecordingRefusal, Selection, TrialBinding};

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

/// What the backend or evaluation copy reported about one planted damage firing.
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
    /// An execution channel reported a positive firing count for the damage.
    Observed,
    /// The backend exposes an activation channel and supplied no positive activation observation.
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
    /// The witness was not executed.
    NotExecuted,
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
    /// The suite accepted a damage whose evaluation copy reported a positive firing count under the exact selection and witness.
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
pub const MUTATION_TARGET_TAG: DomainTag =
    DomainTag::declared("mutation-target", IdentityProfileVersion::declared(1));

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
    Interpreted {
        /// The stable point the producer discovered.
        point: MutationPointRef,
        /// The stable mutation meaning selected at that point.
        alternative: AlternativeId,
    },
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

/// A positive firing count reported for one exact active selection and witness trial.
///
/// # Construction
///
/// The receiver guard returns no activation value for a firing count of zero, so a zero-count plant cannot enter the observed arm. The receiver returns the exact [`DudPlant`] instead. The count is caller-callback output: this type binds it to selection and witness but does not independently instrument the callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActivationEvidence {
    selection: ActiveSelection,
    witness: TrialId,
    firings: u32,
}

/// A plant whose evaluation callback reported zero firings for one exact active selection and witness trial.
///
/// # Authority
///
/// A zero-count callback report is a finding in its own right and cannot enter the positive-count activation arm. The receiver binds the reported count to the exact selection and witness but does not independently instrument the callback.
#[must_use = "a dud plant is a finding, never a silent pass"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DudPlant {
    selection: ActiveSelection,
    witness: TrialId,
}

/// The activation axis with the evidence its observed arm requires.
///
/// # Authority
///
/// A positive-count activation observation without its bound reading is unrepresentable: the arm carries the exact selection, witness, and count. The axis alone is [`ActivationAxis`], and the projection between them is declared once in `type_contract.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationDisposition {
    /// The evaluation callback or backend reported a positive activation observation, and this is its bound reading.
    Observed(ActivationEvidence),
    /// The backend exposes an activation channel and supplied no positive activation observation.
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
    /// The backend exposes an activation channel but supplied no positive activation observation, so the suite was never asked about it.
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
/// without a rejection is unrepresentable. The survived arm carries nothing because it is the absence of a rejection after the evaluation callback reported positive activation under the exact bound selection and witness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationOutcome {
    /// The witness rejected the damaged subject, and this is the rejection.
    Killed(IntendedRejection),
    /// A damage with a positive firing count bound to the exact selection and witness was accepted by that witness.
    Survived,
    /// Nothing was learned about the suite from this mutant.
    Inconclusive(InconclusiveCause),
}

/// One mutant's complete record: the target, every axis of the verdict chain,
/// and the outcome the chain earned.
///
/// # Construction
///
/// The compiled adapter reaches the private killed/inconclusive roads under its
/// output ceiling. The interpreted receiver derives killed or survived only
/// from an active execution admitted through its exact pair, firing evidence,
/// trial report, and no-mutation qualification. No loose public constructor can
/// assemble these axes from neighboring values.
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
    /// The backend exposes an activation channel and supplied no positive activation observation.
    ActivationNotObserved,
    /// The witness execution did not complete, so its rejection is not the
    /// suite's answer.
    WitnessDidNotComplete(ExecutionAxis),
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
/// The baseline guard is the only construction road, and it refuses every
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
/// is enforced by the type's invariant readings, so every road that stands
/// under it reaches the same answer.
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
    family: EvaluationFamilyRef,
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
/// rather than a value somebody forgot to fill in. The standing is the whole of
/// what a party states to [`AdapterQualification::of`], and it is weighed
/// against the version the reading's own profile names.
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
    /// stands as the assumption its own page states — the honest posture, and
    /// worth exactly what it says. It states that nothing has been checked, so
    /// nothing is qualified: [`AdapterQualification::of`] refuses it, and an
    /// adapter under it is inspectable, statable, and reaches no gate.
    Unchecked,
}

/// One adapter profile qualified for readings taken under that exact profile — and how far that standing reaches.
///
/// # Authority
///
/// The typed fact the trust-opening road demands about the TOOL, carried as a
/// value so that "which adapter produced this evidence, and what may a reading
/// under it claim" is answered at the evidence rather than remembered around
/// it. The profile is taken from a reading ([`AdapterQualification::of`]); the qualification is reusable only for readings carrying the same profile and does not identify one reading instance.
///
/// # Construction
///
/// [`AdapterQualification::of`] is the only road, and exactly one pairing
/// travels it: the reading's profile states a backend version, and the standing
/// is [`GrammarStanding::Checked`] over that same version. Every other pairing
/// is a [`QualificationRefusal`] — so an unchecked adapter, a reading whose
/// version nobody stated, and a check made against another version each yield
/// no qualification at all. A qualification is therefore a value that could
/// only have come from somebody checking these shapes against the very version
/// of the backend that wrote the text.
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

/// Why one reading's profile was not qualified.
///
/// Dependent checks in a declared order: whether anybody checked the adapter's
/// shapes at all, then whether the reading's profile states a version a check
/// could be made against, then whether the check and the reading name one
/// version. Each arm is an absent qualification and never a weaker one.
#[must_use = "a refusal is the reason a reading's profile was not qualified"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QualificationRefusal {
    /// The standing is [`GrammarStanding::Unchecked`]: nobody has checked the
    /// adapter's stated shapes against output the backend wrote, so there is
    /// nothing to qualify the reading's profile on.
    GrammarUnchecked,
    /// The reading's profile states no backend version, so a check against a
    /// version names nothing the reading stands under.
    BackendVersionUnstated,
    /// The reading was taken under one backend version and the shapes were
    /// checked against another, so what was checked is a different version's
    /// rendering than the one that wrote this text.
    CheckedAgainstAnotherVersion {
        /// The version the reading's own profile states wrote the text.
        stated: BackendVersion,
        /// The version the standing states the shapes were checked against.
        checked: BackendVersion,
    },
}

/// One reported reading under a qualified adapter profile demonstrated at least one lawful witness rejection for an explicitly scoped evaluation pair.
///
/// # Authority
///
/// The typed fact the trust-opening road demands about the RUN: a suite
/// rejected a damaged subject, under an adapter that stands qualified. The
/// qualification rides inside — one that already exists for the reading's exact profile and is weighed against the reported reading the kill is read out of — so a witness over an unqualified adapter profile is not a value anybody can hold.
///
/// # Construction
///
/// [`CompiledPressureWitness::shown`] is the only road, and it refuses a standing that never reported, a qualification naming another adapter profile, a caller-scoped reading naming another family than the exact pair, then a reported reading whose run demonstrated no kill. The qualification is the caller's to supply and [`AdapterQualification`]'s own road to build, so this road adds a kill and an explicit pair scope to a standing already vouched for rather than vouching for backend execution or pair provenance itself.
///
/// # Nonclaims
///
/// Suite bite is not campaign accounting. The witness states that at least one
/// lawful rejection happened; how many mutants a run pressed and how they
/// divide is the run's own census ([`MutationCensus`]), which answers a
/// different question and is never read as this one. Neither of the two is the
/// no-mutation parity ([`NoMutationParityQualification`]), which is about the
/// exact evaluation pair's faithfulness for one retained input and about
/// nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledPressureWitness {
    pair: EvaluationPairStanding,
    qualification: AdapterQualification,
    kill: MutationReport,
}

/// Why one wrap standing demonstrated no compiled-pressure witness.
///
/// Dependent checks in a declared order: whether the pressure reported at all, whether the qualification carries the reading's exact adapter profile, whether the caller-scoped reading names the exact pair's family, then whether what it reported carries a kill.
#[must_use = "a refusal is the reason no compiled-pressure witness was shown"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureWitnessRefusal {
    /// The wrap-first pressure has not reported, so there is no reading to
    /// stand on.
    WrapNotReported,
    /// The qualification offered names another adapter profile and therefore stands behind nothing here.
    QualificationUnderAnotherProfile,
    /// The caller-scoped reading names another evaluation family than the
    /// exact production/evaluation pair offered for this witness.
    ReadingForAnotherFamily {
        /// The family retained by the exact evaluation pair.
        expected: EvaluationFamilyRef,
        /// The family the external reading was scoped to.
        found: EvaluationFamilyRef,
    },
    /// The reading's run demonstrated no lawful kill, so nothing in it has
    /// shown a property biting.
    NoKillDemonstrated,
}

// ---------------------------------------------------------------------------
// The interpreted lane's evaluation surface.
// ---------------------------------------------------------------------------

/// The domain tag of an owner-authored mutation policy.
pub const MUTATION_POLICY_TAG: DomainTag =
    DomainTag::declared("mutation-policy", IdentityProfileVersion::declared(1));

/// The domain tag of one admitted alternative's stable identity.
pub const MUTATION_ALTERNATIVE_TAG: DomainTag =
    DomainTag::declared("mutation-alternative", IdentityProfileVersion::declared(1));

/// The domain tag of one complete evaluation surface.
pub const EVALUATION_SURFACE_TAG: DomainTag =
    DomainTag::declared("evaluation-surface", IdentityProfileVersion::declared(1));

/// The owner-declared family that binds one production road, evaluation copy, policy, and evidence chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluationFamilyRef(NamespacedName);

/// The content identity of one owner-authored mutation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationPolicyId(ContentAddress);

/// One claim's permission to use a nonempty roster of operator families.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPermission {
    owner_claim: ClaimRef,
    admitted_families: Vec<OperatorFamilyRef>,
}

/// Why one mutation permission was refused.
#[must_use = "a refusal is the reason a mutation permission was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionRefusal {
    /// The permission names no operator family, so it permits no executable damage.
    NoOperatorFamily,
    /// One operator family appears twice in the permission.
    DuplicateOperatorFamily(OperatorFamilyRef),
}

/// One evaluation family's owner-authored mutation policy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPolicy {
    family: EvaluationFamilyRef,
    permissions: Vec<MutationPermission>,
    identity: MutationPolicyId,
}

/// Why one mutation policy was refused.
#[must_use = "a refusal is the reason a mutation policy was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyRefusal {
    /// Two permission rows name one owner claim.
    DuplicateClaim(ClaimRef),
}

/// The policy-issued membership carried by one admitted mutation point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolicyMembership {
    policy: MutationPolicyId,
    owner_claim: ClaimRef,
}

/// Where a selected alternative fires, named rather than path-spelled.
///
/// The reason a trial's identity is not its site holds here too: a file move
/// must rename nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActivationSite(NamespacedName);

/// One producer-discovered operator family and producer-declared canonical mutation meaning before owner-policy admission at a point.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlternativeDeclaration {
    family: OperatorFamilyRef,
    operation: Vec<u8>,
}

/// The stable identity of one point's admitted mutation meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlternativeId(ContentAddress);

/// One executable operator family and producer-declared canonical mutation meaning admitted under a point's policy membership.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdmittedAlternative {
    identity: AlternativeId,
    family: OperatorFamilyRef,
    operation: Vec<u8>,
}

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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPoint {
    identity: MutationPointRef,
    membership: PolicyMembership,
    original_operation: Vec<u8>,
    admitted_alternatives: Vec<AdmittedAlternative>,
    activation_site: ActivationSite,
}

/// Why one mutation point was refused.
///
/// Dependent checks in a declared order: original operation, nonempty alternative roster, owner-claim permission, then each alternative's family, bytes, difference from the original, and difference from earlier meanings under the same family.
#[must_use = "a refusal is the reason a mutation point was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointRefusal {
    /// The point states no original operation, so its no-mutation reading is
    /// empty.
    EmptyOriginalOperation,
    /// The discovery carries no executable alternative, so it is not a mutation point.
    NoAdmittedAlternative,
    /// The policy carries no permission row for the point's owner claim.
    ClaimNotPermitted(ClaimRef),
    /// One alternative's operator family is outside the owner permission.
    FamilyNotPermitted {
        /// The alternative's position in the supplied roster.
        at: usize,
        /// The family the owner policy did not admit for this claim.
        family: OperatorFamilyRef,
    },
    /// One alternative states no mutation meaning.
    EmptyAlternative {
        /// The alternative's position in the supplied roster.
        at: usize,
    },
    /// An admitted alternative is byte-identical to the original operation, so
    /// selecting it would be the no-mutation reading under another name.
    AlternativeIsOriginal {
        /// The alternative's position in the roster.
        at: usize,
    },
    /// Two admitted alternatives carry one operator family and canonical mutation meaning.
    DuplicateAlternativeMeaning {
        /// The second alternative's position in the roster.
        at: usize,
    },
}

/// The content identity of one complete evaluation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluationSurfaceId(ContentAddress);

/// One evaluation copy's complete point table.
///
/// # Authority
///
/// This is producer-shaped conforming data: a hand author may build the same public shape, and macroc may later emit it from one declaration walk. Runtime is selection among these points and never interpretation of arbitrary source, which would mint a second meaning authority.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvaluationSurface {
    family: EvaluationFamilyRef,
    policy: MutationPolicyId,
    identity: EvaluationSurfaceId,
    points: Vec<MutationPoint>,
}

/// Why one evaluation surface was refused.
#[must_use = "a refusal is the reason an evaluation surface was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceRefusal {
    /// A point carries membership issued by another policy.
    PointUnderAnotherPolicy {
        /// The point whose membership does not belong here.
        point: MutationPointRef,
        /// The policy the surface requires.
        expected: MutationPolicyId,
        /// The policy that issued the point's membership.
        found: MutationPolicyId,
    },
    /// Two points state one identity.
    DuplicatePoint(MutationPointRef),
}

/// Whether a complete evaluation surface admits executable points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointCatalogPosture {
    /// The evaluation copy is lawful but admits no active mutation control.
    NoAdmittedPoints,
    /// The evaluation copy admits at least one executable mutation point.
    Mutable,
}

/// One point selected into one of the damages it admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActiveSelection {
    surface: EvaluationSurfaceId,
    point: MutationPointRef,
    alternative: AlternativeId,
}

/// What one run of the evaluation copy selects among the surface's points.
///
/// # Authority
///
/// The no-mutation control is an evaluation-call posture rather than a point in the surface catalog. A point-free surface can therefore run parity without pretending it admits active mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationControl {
    /// No point is damaged; the evaluation copy reads its unmodified meaning.
    NoMutation,
    /// One point reads as one of its admitted alternatives.
    Active(ActiveSelection),
}

/// Why one active-mutant selection was refused.
#[must_use = "a refusal is the reason a mutant was not selected"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionRefusal {
    /// The selection was minted by another evaluation surface.
    SelectionFromAnotherSurface {
        /// The surface reading the selection.
        expected: EvaluationSurfaceId,
        /// The surface that issued the selection.
        found: EvaluationSurfaceId,
    },
    /// The surface states no point under this identity.
    NoSuchPoint(MutationPointRef),
    /// The point does not admit this mutation meaning.
    NoSuchAlternative {
        /// The point whose roster was read.
        point: MutationPointRef,
        /// The alternative identity absent from that roster.
        alternative: AlternativeId,
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

/// The production callable of one evaluation family.
pub type ProductionCall<Input, Meaning> = fn(&Input) -> Meaning;

/// The evaluation-copy callable of one evaluation family.
pub type EvaluationCall<Input, Meaning> =
    fn(&Input, EvaluationControl) -> EvaluationObservation<Meaning>;

/// The check that judges one meaning under the exact trial binding it is joined to.
pub type MeaningCheck<Meaning> = fn(&Meaning) -> TrialConclusion;

/// Raw output from one evaluation-copy call.
///
/// # Authority
///
/// This is caller output, not admitted evidence. The receiver validates the control, firing count, trial binding, report, and trust facts before a mutation evidence value exists.
pub struct EvaluationObservation<Meaning> {
    meaning: Meaning,
    firings: u32,
}

/// The production callable and revision an owner declares for one evaluation family.
pub struct ProductionBinding<Input, Meaning> {
    family: EvaluationFamilyRef,
    revision: RevisionBinding,
    call: ProductionCall<Input, Meaning>,
}

/// The evaluation callable and revision bound to one exact evaluation surface.
pub struct EvaluationBinding<Input, Meaning> {
    family: EvaluationFamilyRef,
    revision: RevisionBinding,
    surface: EvaluationSurfaceId,
    call: EvaluationCall<Input, Meaning>,
}

/// One production/evaluation pair under a shared owner declaration and equivalence.
///
/// # Nonclaims
///
/// Matching family references prove the declared relationship, not behavioral agreement. Only an executed no-mutation parity reading can establish that agreement for its exact input.
pub struct EvaluationPair<Input, Meaning> {
    production: ProductionBinding<Input, Meaning>,
    evaluation: EvaluationBinding<Input, Meaning>,
    same: Equivalence<Meaning>,
}

/// Why one production/evaluation pair was refused.
#[must_use = "a refusal is the reason an evaluation pair was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationPairRefusal {
    /// The production and evaluation bindings name different owner families.
    FamilyMismatch {
        /// The production binding's family.
        production: EvaluationFamilyRef,
        /// The evaluation binding's family.
        evaluation: EvaluationFamilyRef,
    },
}

/// The identity and revision facts retained by every reading over one evaluation pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationPairStanding {
    family: EvaluationFamilyRef,
    production_revision: RevisionBinding,
    evaluation_revision: RevisionBinding,
    surface: EvaluationSurfaceId,
}

/// One trial binding joined to the declared check identity and callable that judge mutation executions through it.
pub struct MutationWitness<Meaning> {
    binding: TrialBinding,
    check: MeaningCheck<Meaning>,
}

/// Why one mutation witness could not bind its check identity to its trial.
#[must_use = "a refusal is the reason a mutation witness was not bound"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationWitnessRefusal {
    /// The offered check identity is not the check identity retained by the trial row.
    CheckMismatch {
        /// The check identity retained by the row.
        expected: CheckRef,
        /// The check identity offered with the callable.
        found: CheckRef,
    },
}

/// The three returned facts compared by one no-mutation observation.
pub struct NoMutationResults<Meaning> {
    production: Meaning,
    evaluation: Meaning,
    evaluation_firings: u32,
}

/// The production and evaluation reports retained in their semantic roles for one no-mutation comparison.
pub(in crate::muterprater) struct NoMutationReports {
    production: TrialReport,
    evaluation: TrialReport,
}

/// The exact input, results, substrate, conclusions, and reports of one no-mutation comparison.
pub struct NoMutationParityReading<'pair, 'input, Input, Meaning> {
    pair: &'pair EvaluationPair<Input, Meaning>,
    witness: MutationWitness<Meaning>,
    input: &'input Input,
    results: NoMutationResults<Meaning>,
    substrate: SharedSubstrate,
    conclusion: TrialConclusion,
    reports: NoMutationReports,
}

/// Why one no-mutation observation could not be recorded.
#[must_use = "a refusal is the reason no no-mutation reading was recorded"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoMutationObservationRefusal {
    /// The shared-substrate declaration could not be built.
    Substrate(ParityRefusal),
    /// The production observation could not join its trial binding.
    ProductionReport(ReportRecordingRefusal),
    /// The evaluation observation could not join its trial binding.
    EvaluationReport(ReportRecordingRefusal),
}

/// Why one complete no-mutation reading did not qualify interpreted evidence.
#[must_use = "a refusal is the reason no-mutation parity did not qualify"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParityQualificationRefusal {
    /// The production report did not earn a lawful lens verdict.
    ProductionDidNotQualify,
    /// The evaluation report did not earn a lawful lens verdict.
    EvaluationDidNotQualify,
    /// The no-mutation control reported an activation.
    NoMutationActivated {
        /// How many firings the evaluation callable reported.
        firings: u32,
    },
    /// The owner-declared equivalence refused the two meanings.
    MeaningsDisagreed,
}

/// One no-mutation reading that earned scoped parity qualification.
pub struct NoMutationParityQualification<'pair, 'input, Input, Meaning> {
    reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
}

/// One complete no-mutation reading that did not earn qualification.
pub struct RejectedNoMutationParity<'pair, 'input, Input, Meaning> {
    cause: ParityQualificationRefusal,
    reading: NoMutationParityReading<'pair, 'input, Input, Meaning>,
}

/// The qualification disposition of one complete no-mutation reading.
pub enum NoMutationParityStanding<'pair, 'input, Input, Meaning> {
    /// The reading earned parity qualification for its exact pair and input.
    Qualified(NoMutationParityQualification<'pair, 'input, Input, Meaning>),
    /// The reading remains available with the exact reason it did not qualify.
    Rejected(RejectedNoMutationParity<'pair, 'input, Input, Meaning>),
}

/// Which of the trust order's facts the interpreted lane is still owed.
///
/// # Authority
///
/// The roster of what the gate can be owed, and it is short because the two
/// evidence types are strict: an [`AdapterQualification`] exists only over an
/// adapter somebody checked against the backend version the reading names, and
/// a [`CompiledPressureWitness`] only over a reading that qualification stands
/// on. So every arm here names an ABSENT fact, and the gate has no arm for
/// evidence that arrived weak.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MissingTrustEvidence {
    /// No qualified reading has demonstrated a witness rejection, so nothing
    /// has shown the properties bite.
    CompiledPressure,
    /// The compiled-pressure witness belongs to another exact evaluation pair.
    CompiledPressureForAnotherPair,
    /// The mandatory no-mutation parity has not passed.
    NoMutationParity,
    /// The parity qualification belongs to another pair or evaluation surface.
    ParityForAnotherSurface,
}

/// The exact compiled and parity evidence that opens interpreted execution for one surface.
pub struct InterpretedTrust<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning> {
    surface: &'surface EvaluationSurface,
    compiled: &'compiled CompiledPressureWitness,
    parity: &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning>,
}

/// The availability of interpreted evidence for one evaluation surface.
///
/// # Authority
///
/// A surface alone earns no trust. Availability requires a compiled-pressure witness and no-mutation qualification scoped to the same exact evaluation-pair standing: family, production revision, evaluation revision, and surface identity. Every absent fact names itself.
pub enum InterpreterAvailability<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning> {
    /// A conforming evaluation surface exists and trust has opened.
    Available(InterpretedTrust<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning>),
    /// No conforming evaluation surface exists — neither a producer's nor a
    /// hand-authored one under the same contract.
    NoConformingSurface,
    /// A surface exists and the trust order still owes this evidence.
    TrustNotOpened {
        /// What the staging is still owed.
        missing: MissingTrustEvidence,
    },
}

/// The admitted interpreted result of one active selection under an opened trust boundary.
pub struct InterpretedMutationEvidence<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning>
{
    trust: InterpretedTrust<'surface, 'compiled, 'parity, 'pair, 'input, Input, Meaning>,
    selection: ActiveSelection,
    meaning: Meaning,
    report: TrialReport,
    mutation: MutationReport,
}

/// Why one interpreted execution produced no admitted mutation evidence.
#[must_use = "a refusal is the reason interpreted mutation evidence was not built"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretedExecutionRefusal {
    /// The active selection does not belong to the opened surface.
    Selection(SelectionRefusal),
    /// The witness trial belongs to a claim other than the selected point's owner claim.
    WitnessForAnotherClaim {
        /// The claim that owns the selected point.
        expected: ClaimRef,
        /// The claim carried by the offered trial binding.
        found: ClaimRef,
    },
    /// The evaluation callback reported zero firings for the selected damage.
    DudPlant(DudPlant),
    /// The host observation could not join its exact trial binding.
    Report(ReportRecordingRefusal),
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

/// Why rewrite descriptors may not enter the interpreted audit road.
#[must_use = "a refusal is the reason the rewrite audit road was withheld"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteWithheld {
    /// The interpreted lane — the execution substrate that makes rewrite
    /// families cheap — is not available.
    InterpreterUnavailable,
    /// The trusted evaluation surface carries no executable mutation point.
    NoAdmittedPoint,
    /// The trust order still owes this evidence.
    TrustNotOpened(MissingTrustEvidence),
}

/// Whether rewrite descriptors may enter the interpreted audit road.
///
/// # Nonclaims
///
/// Admission here is execution availability, not evidence. A descriptor remains [`RewriteTrust::AuditPending`] until an actual execution establishes whatever a later evidence owner requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteAdmission {
    /// The interpreted audit road is available under compiled pressure, parity, and a mutable surface.
    Admitted,
    /// The audit road is unavailable for a stated reason.
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
    Alternative(AlternativeId),
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

/// The ground a mutant-killed proposal stands on: a kill shown on the surface
/// with the mutant active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantKilledGround {
    /// What was damaged.
    target: MutationTarget,
    /// What the damage's activation was.
    activation: ActivationDisposition,
    /// The reproduction account of the demonstrating run.
    capsule: ReplayCapsule,
    /// The demonstrated kill.
    demonstration: Demonstration,
}

/// The ground a claim-pinned proposal stands on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimPinnedGround {
    /// The claim pinned.
    claim: ClaimRef,
    /// The reproduction account of the pinning run.
    capsule: ReplayCapsule,
    /// What the pin added to the claim's proof.
    delta: ProofDelta,
}

/// The ground an obligation-discharged proposal stands on.
///
/// No capsule at all, and no seat for one: the admitted row is the discharge's
/// permanent record. The two grounds that DO author a capsule each carry it as
/// a field, so nothing here reads a capsule out of an option that one ground
/// would always answer empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationDischargedGround {
    /// The owed claim's identity.
    owed: OwedClaim,
    /// What discharged it.
    discharge: DischargeEvidence,
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

/// The evidence a failure-bearing proposal is not a duplicate: the candidate's
/// fingerprint, against every fingerprint already known.
///
/// # Authority
///
/// Never persuasive prose. The comparison is performed where the value is built,
/// so a duplicate is a refusal rather than a paragraph a reader has to check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureComparison {
    /// The fingerprint this candidate carries.
    candidate: Fingerprint,
    /// The fingerprints already known, in the order they were compared.
    known: Vec<Fingerprint>,
}

/// The evidence a discharge proposal is not a duplicate: the owed claim,
/// against the discharges already recorded for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationComparison {
    /// The owed claim.
    owed: ClaimRef,
    /// The trials already recorded as discharging it.
    discharges: Vec<TrialId>,
}

/// The statement a proposal with no comparable subject makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoComparison {
    /// Why nothing was compared.
    reason: NoComparisonReason,
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
pub const PROPOSAL_TAG: DomainTag =
    DomainTag::declared("proposal", IdentityProfileVersion::declared(1));

/// What every proposal is, whichever ground it stands on: a candidate row, a
/// ground word an admission act can state, a destination, and the identity those
/// three derive.
///
/// # The sealed shape
///
/// Sealed, so the proposals are the three this crate declares and no outside
/// crate can add a fourth by implementing anything. A road that stores or reports
/// a proposal takes one of these rather than a sum type every ground would have
/// to fit inside — which is what keeps a discharge proposal from being as large
/// as a kill's demonstration.
///
/// # Nonclaims
///
/// It reaches no ground's own contents. What a kill demonstrated and what a pin
/// moved are read off the concrete proposal that holds them, because they are
/// exactly the facts the three do not share.
pub trait ProposalDocument: sealed::Sealed {
    /// The candidate row.
    fn candidate(&self) -> &Row;

    /// The ground at summary width — the word an admission act states.
    fn ground_summary(&self) -> AdmissionGround;

    /// Where it would land.
    fn destination(&self) -> ProposalDestination;

    /// The proposal's content identity — permanent provenance.
    ///
    /// # The specification
    ///
    /// Two primitives: `u32be(n)`, and `bytes(x)` — `u64be(len(x))` followed by
    /// the bytes of `x`.
    ///
    /// The members, in exactly this order:
    ///
    /// | # | member | encoding |
    /// | - | ------ | -------- |
    /// | 1 | encoding version | `u32be` |
    /// | 2 | candidate row | `bytes(…)` of the descriptor home's canonical row bytes |
    /// | 3 | ground | one byte, [`AdmissionGround::slot`] |
    /// | 4 | destination namespace | `bytes(utf8)` |
    /// | 5 | destination stem | `bytes(utf8)` |
    ///
    /// # Nonclaims
    ///
    /// The evidence is deliberately absent: the replay capsule, the
    /// demonstration, and the duplicate comparison are what STANDS BEHIND the
    /// proposal rather than what it proposes. Two offers of one row on one
    /// ground into one destination therefore share an identity by design, which
    /// is what makes an admitted origin's citation stable across a rerun.
    ///
    /// Every member is one of the three readings above, so the three proposals
    /// derive their identity by one road rather than by three that agree.
    fn identity(&self) -> ProposalId;
}

pub(super) mod sealed {
    /// The seal. Implemented for this home's three proposals and nothing else.
    #[expect(
        unnameable_types,
        reason = "a seal is a bound that is reachable and not nameable: reachable so the public trait can require it, unnameable so no outside crate can satisfy it, and a seal an outsider could name would be the closure this roster has instead of an open sum"
    )]
    pub trait Sealed {}
}

/// One proposal on the mutant-killed ground.
///
/// # Authority
///
/// Process-local until a caller's own sink stores it. Constructing one asserts
/// nothing about admission: a human admits, and admission is a two-part
/// human-authored patch this crate never performs.
///
/// The comparison seat takes a [`FailureComparison`] and admits nothing else, so
/// "is this evidence the comparison this ground owes?" is not a question that can
/// be asked of a built value or of one being built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantKilledProposal {
    candidate: Row,
    ground: MutantKilledGround,
    duplicate: FailureComparison,
    destination: ProposalDestination,
}

/// One proposal on the claim-pinned ground.
///
/// The comparison seat takes a [`NoComparison`]: a pin carries no failure to
/// fingerprint and discharges no obligation, so what it offers is the stated
/// reason nothing was compared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimPinnedProposal {
    candidate: Row,
    ground: ClaimPinnedGround,
    duplicate: NoComparison,
    destination: ProposalDestination,
}

/// One proposal on the obligation-discharged ground.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationDischargedProposal {
    candidate: Row,
    ground: ObligationDischargedGround,
    duplicate: ObligationComparison,
    destination: ProposalDestination,
}

/// Why one proposal was refused.
///
/// Dependent checks in a declared order: the row's origin, then — where the
/// ground names a mutation point — the survivor point against the target.
///
/// # Nonclaims
///
/// There is no evidence-against-ground cause, because there is no such
/// disagreement to establish: each proposal's comparison seat admits exactly the
/// comparison its ground owes, so a mismatched pair is not a value that can be
/// written.
///
/// [`ProposalRefusal::SurvivorPointMismatch`] is reachable from the mutant-killed
/// road alone. The other two grounds name no mutation point for a synthesis fact
/// to disagree with, and their roads establish [`ProposalRefusal::NotACandidate`]
/// or nothing.
#[must_use = "a refusal is the reason a proposal was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProposalRefusal {
    /// The row does not carry the candidate origin arm, so it is an authored row
    /// entering by the proposal door.
    NotACandidate,
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
    /// Generic over the sealed proposal roster rather than over one sum type: a
    /// sink stores what every proposal is — a row, a ground word, a destination,
    /// and the identity those derive — and a discharge proposal reaching this
    /// seam does not have to be as large as a kill's demonstration to get here.
    fn store<Document: ProposalDocument>(
        &mut self,
        proposal: &Document,
    ) -> Result<StoredProposalRef, SinkRefusal>;
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

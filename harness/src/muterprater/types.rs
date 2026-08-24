//! Every public type of the proof-pressure engine.
//!
//! Declarations only.
//! The constructors and readers that reach a private field are this file's own child, `type_guard.rs`; trait implementations are in `type_contract.rs`; the lanes are the role-named modules beside them.
//!
//! # Borrowed vocabularies
//!
//! Rows, staged views, and namespaced references belong to [`crate::descriptor`].
//! Trial identities, findings, fingerprints, capsules, and reports belong to [`crate::report`].
//! Selections and invocations belong to [`crate::runner`], and operator families to [`crate::depot`].
//! This home binds those values; what each of them means is written where it is declared.

use crate::depot::capsules::{ReplayCapsuleEntry, ReplayDepotRefusal, StoredReplayEntryRef};
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
// The verdict chain.
// ---------------------------------------------------------------------------

/// What the unmutated subject's own suite did before any damage was inflicted.
///
/// An unchanged passing baseline is the precondition every kill stands on, so this axis is read before any other.
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
    /// The damage does not typecheck, or the site admits no such alternative.
    Unviable,
    /// The backend failed while materializing the damage, so nothing was established about it.
    ToolFailed,
}

/// What the backend or evaluation callable reported about one planted damage firing.
///
/// A damage nothing reached says nothing about the suite that did not catch it, which is why this axis stands between materialization and the verdict.
/// [`ActivationAxis::UnobservableUnderBackend`] is a fact about the backend and never about the damage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationAxis {
    /// An execution channel reported a positive firing count for the damage.
    Observed,
    /// The backend exposes an activation channel and supplied no positive observation.
    NotObserved,
    /// The backend offers no activation channel, so firing is unobservable under it.
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
    /// The harness or the backend failed around the witness, so nothing was learned.
    InfrastructureFailed,
}

/// What one mutant earned, at axis width.
///
/// The record carries [`MutationOutcome`], whose arms carry the evidence each one requires; this is the word a census counts.
/// A mutant unobservable under its backend can never earn [`MutationVerdict::Survived`], and that is a refusal in the record's constructors rather than a rule somebody follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationVerdict {
    /// The suite rejected the damaged subject.
    Killed,
    /// The suite accepted a damage whose firing was observed under the exact selection and witness.
    Survived,
    /// Nothing was learned about the suite from this mutant.
    Inconclusive,
}

/// What was established about the damaged subject meaning what the lawful one means.
///
/// [`EquivalenceAxis::ProvenInScope`] claims equivalence over the scope the proof was taken in and never in general.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquivalenceAxis {
    /// No equivalence question was put.
    NotAssessed,
    /// The damaged subject was proven equivalent, in the scope the proof was taken in.
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
/// Owned text rather than a [`NamespacedName`], because the spelling is a tool's output and not a name anybody authored.
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

/// One external mutant's identity, over the coordinate and damage text the backend reported.
///
/// Two runs of one backend over one unchanged tree name the same mutant, and a moved line names a different one — which is what a coordinate affords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MutantId(ContentAddress);

/// How one damaged thing is identified, by the lane that damaged it.
///
/// The two selected-projection arms name the same authored point and alternative under different execution roads, and their report provenance stays separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationIdentity {
    /// An external backend's mutant, addressed by its reported coordinate and damage.
    External(MutantId),
    /// A point on an evaluation surface, addressed by the reference its producer authored.
    Interpreted {
        /// The stable point the producer discovered.
        point: MutationPointRef,
        /// The stable mutation meaning selected at that point.
        alternative: AlternativeId,
    },
    /// A separately materialized production-shaped selected projection.
    CompiledProjection {
        /// The stable point the producer discovered.
        point: MutationPointRef,
        /// The stable mutation meaning baked into the compiled artifact.
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
/// [`OperatorFamilyRef::of_slug`] resolves against the bank's own roster, so a reference can never name a family the bank does not declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorFamilyRef(OperatorFamily);

/// Whether one damage is one the operator-family bank names.
///
/// A backend applies its own operators, and attributing one of them to a family the bank never declared would be this lane inventing a fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyAttribution {
    /// The damage realizes a family the bank declares.
    Declared(OperatorFamilyRef),
    /// The damage is not one the bank names, so no family is claimed for it.
    OutsideTheBank,
}

/// Whether the origin reading named the claim that owns one damage's site.
///
/// The owning claim rides the mapped arm, so a mapped target without a claim is not a value anybody can hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappingPosture {
    /// The reading named this claim as the owner of the damage's site.
    Mapped(ClaimRef),
    /// No mapping was available for the site.
    OwnerUnmapped,
}

/// One damaged thing a lane pressed: its identity, the family it realizes, where it lives, and whether its owning claim is known.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationTarget {
    identity: MutationIdentity,
    family: FamilyAttribution,
    site: MutationSite,
    owner: MappingPosture,
}

// ---------------------------------------------------------------------------
// Activation evidence.
// ---------------------------------------------------------------------------

/// A positive firing count reported for one exact selection and witness trial.
///
/// A zero count produces [`DudPlant`] instead, so a silent plant cannot enter the observed arm.
/// The count is callback output: this value binds it to a selection and a witness and does not instrument the callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActivationEvidence {
    selection: ActiveSelection,
    witness: TrialId,
    firings: u32,
}

/// A plant whose evaluation callback reported zero firings for its exact selection and witness trial.
#[must_use = "a dud plant is a finding, never a silent pass"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DudPlant {
    selection: ActiveSelection,
    witness: TrialId,
}

/// The activation axis with the evidence its observed arm requires.
///
/// The bare axis is [`ActivationAxis`], and the projection between the two is declared once in `type_contract.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationDisposition {
    /// A positive activation observation, with the selection, witness, and count it was bound to.
    Observed(ActivationEvidence),
    /// The backend exposes an activation channel and supplied no positive observation.
    NotObserved,
    /// The backend offers no activation channel at all.
    UnobservableUnderBackend,
}

// ---------------------------------------------------------------------------
// The rejection a kill stands on.
// ---------------------------------------------------------------------------

/// One rejection this harness's own engine demonstrated: the trial that refused, and the finding it refused with.
///
/// The finding is carried whole, so [`DemonstratedRejection::fingerprint`] derives the failure identity rather than remembering it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DemonstratedRejection {
    trial: TrialId,
    finding: TrialFinding,
}

/// The rejection one witness execution answered a damaged subject with.
///
/// [`IntendedRejection::Demonstrated`] names a trial and a finding, so it carries a failure fingerprint.
/// [`IntendedRejection::ReportedByBackend`] is an external tool's word and names neither, so a kill standing on it claims what the backend stated and nothing more.
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
/// [`RejectionIdentity::Unfingerprinted`] states that the rejection named neither a trial nor a cause, not that a value is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RejectionIdentity {
    /// The rejection names a fingerprint, derived from its trial and finding.
    Fingerprinted(Fingerprint),
    /// The rejection carries no trial and no cause, so it names no failure.
    Unfingerprinted,
}

// ---------------------------------------------------------------------------
// The per-mutant record, and the run over it.
// ---------------------------------------------------------------------------

/// Why nothing was learned about the suite from one mutant.
///
/// Every arm names a link of the verdict chain that did not hold, and none of them is a softer survivor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InconclusiveCause {
    /// The unchanged baseline did not qualify, so no rejection under it would prove anything.
    BaselineNotQualified,
    /// The damage never became a thing that could be executed.
    NotMaterialized,
    /// The backend can observe firing and reported none, so the suite was never asked.
    NotActivated,
    /// The witness execution did not complete.
    WitnessIncomplete,
    /// The backend cannot observe firing and the witness did not reject.
    UnobservableAndUnrejected,
    /// The damaged subject was proven equivalent in scope, so no suite could have rejected it.
    ProvenEquivalentInScope,
}

/// The verdict one mutant earned, with the evidence each arm requires.
///
/// The killed arm carries the rejection that killed it, so a kill asserted without one is unrepresentable.
/// The survived arm carries nothing, because it is the absence of a rejection after a positive firing count was bound to the selection and witness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationOutcome {
    /// The witness rejected the damaged subject, and this is the rejection.
    Killed(IntendedRejection),
    /// A damage with an observed firing was accepted by its witness.
    Survived,
    /// Nothing was learned about the suite from this mutant.
    Inconclusive(InconclusiveCause),
}

/// One mutant's complete record: the target, every axis of the verdict chain, and the outcome the chain earned.
///
/// There is no loose public constructor: the wrapped-backend adapter, the interpreted receiver, and the compiled-projection road each mint only the records their own evidence affords.
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
/// Dependent checks in a declared order — baseline, materialization, activation, execution — so exactly one cause is true of any refused kill.
#[must_use = "a refusal is the reason a kill was not minted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KillRefusal {
    /// The unchanged baseline did not qualify, so a rejection under it proves nothing.
    BaselineNotQualified(BaselineAxis),
    /// The damage never materialized, so there was nothing for a witness to reject.
    NotMaterialized(MaterializationAxis),
    /// The backend can observe firing and supplied no positive observation.
    ActivationNotObserved,
    /// The witness execution did not complete, so its rejection is not the suite's answer.
    WitnessDidNotComplete(ExecutionAxis),
}

/// The accounting over one pressure run's mutants.
///
/// One seat per arm of [`MutationVerdict`], and [`MutationCensus::pressed`] is their sum rather than a total that could disagree with its parts.
/// It counts mutants under one run, and it is not the trial, generation, or bench-sample census.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MutationCensus {
    killed: u32,
    survived: u32,
    inconclusive: u32,
}

/// That the unchanged subject's own suite ran and passed.
///
/// The precondition every kill stands on, carried as a value so that "was the baseline good" is not a question anywhere downstream.
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

/// One pressure run's complete record: the baseline it stood on, every mutant's report, and the census over them.
///
/// The baseline is a [`BaselineQualification`] rather than an axis reading, so a run whose baseline did not qualify is not a run this value describes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationRun {
    baseline: BaselineQualification,
    reports: Vec<MutationReport>,
    census: MutationCensus,
}

// ---------------------------------------------------------------------------
// What a wrapped backend's output is read into.
// ---------------------------------------------------------------------------

/// Which external mutation backend one reading was taken from.
///
/// One backend because one line grammar: a second backend is a second grammar and a second arm beside the line laws that read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrappedBackend {
    /// The `cargo-mutants` backend, which mutates real source and invokes the test command itself.
    CargoMutants,
}

/// One backend's version, as the party that ran it states it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendVersion(String);

/// Why one backend version was refused.
#[must_use = "a refusal is the reason a backend version was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendVersionRefusal {
    /// The spelling is empty, so it states no version.
    EmptySpelling,
}

/// Whether the party that ran a backend stated which version produced the text a reading was taken from.
///
/// The version is declared and never observed: this lane reads text a caller already holds and invokes nothing.
/// [`BackendVersionPosture::Stated`] records that party's word, and is not a verification that the text matches that version's rendering.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BackendVersionPosture {
    /// The party that ran the backend stated this version.
    Stated(BackendVersion),
    /// No party has stated a version, so the grammar assumption stands unbound.
    Unstated,
}

/// Which of a backend's outputs one reading was taken from.
///
/// A console stream is a rendering a tool writes for a person, so the shapes it carries are the ones the adapter's own page states.
/// A machine-readable output is a second arm beside the grammar that reads it, carrying whatever ceiling that output affords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadingSource {
    /// The line-oriented console stream the backend writes as it runs.
    ConsoleStream,
}

/// Which version of an adapter's stated line grammar one reading was taken under.
///
/// It moves when and only when those line shapes move, and it is neither the backend's version nor an encoding version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GrammarVersion(u32);

/// The most one reading's evidence can establish, in the verdict vocabulary.
///
/// A ceiling follows from what the reading's source carries, and a run carrying a verdict outside it is refused rather than believed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClaimCeiling {
    /// The strongest verdict is a kill that asserts witness rejection and states nothing about activation.
    ///
    /// The source carries no channel that could observe a damage firing, so no mutant read under it earns [`MutationVerdict::Survived`].
    WitnessRejection,
}

/// What one reading is stated under: the backend, its version posture, the output read, and the adapter grammar that read it.
///
/// A [`WrapReading`] cannot be built without one, so "which grammar was this read under, and what may it claim" is answered at the reading rather than remembered around it.
/// [`AdapterProfile::ceiling`] reads the ceiling from the source, so no profile grants its reading more than the source affords.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdapterProfile {
    backend: WrappedBackend,
    version: BackendVersionPosture,
    source: ReadingSource,
    grammar: GrammarVersion,
}

/// The caller-supplied reading from one source coordinate to the claim that owns it.
///
/// A function pointer rather than a closure, so the seam carries no captured state.
/// Answering `None` says no mapping was available and produces [`MappingPosture::OwnerUnmapped`], never a claim this lane picked.
pub type OwnerLookup = fn(&SourceCoordinate) -> Option<ClaimRef>;

/// The caller-supplied reading from one backend's damage text to the operator family it realizes.
///
/// Answering `None` produces [`FamilyAttribution::OutsideTheBank`], never a family this lane picked.
pub type FamilyLookup = fn(&SourceCoordinate, &[u8]) -> Option<OperatorFamilyRef>;

/// The outcome word one line of a backend's output states about one mutant.
///
/// A line whose leading word is none of these becomes an [`UnparsedLine`] and is never guessed at.
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
/// Never dropped: a parser that discarded what it did not understand would shrink the denominator with nobody able to read that it had.
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

/// What one reading recovered: the profile it was read under, the run, the roster the backend announced, and every line left unread.
///
/// The announced roster and the run's census answer different questions, so a difference between them is a finding for a reader and never a number this value reconciles on its own.
/// A reading claims exactly what its profile's ceiling affords, and no road anywhere widens it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrapReading {
    profile: AdapterProfile,
    run: MutationRun,
    announced: AnnouncedRoster,
    unparsed: Vec<UnparsedLine>,
}

/// Why one reading of a backend's output was refused.
///
/// Dependent checks in a declared order: the baseline is read before any mutant line, and the run is weighed against the profile's ceiling before a reading stands over it.
#[must_use = "a refusal is the reason a wrap reading was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapRefusal {
    /// The output states no unmutated-baseline line, so the kill precondition was never established.
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
    /// One record carries a verdict the profile's ceiling does not admit.
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
// Qualification, and the generic suite bite.
// ---------------------------------------------------------------------------

/// Whether the wrapped-backend pressure has reported, and what it reported.
///
/// The whole profiled reading rides here rather than a bare run, because the backend, the version posture, the output, the grammar, and the ceiling are exactly the facts the trust-opening road weighs.
/// A pass with no kill is not evidence that the properties bite, and [`CompiledSuitePressure::demonstrated`] reads it as the absence it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapStanding<'reading> {
    /// The wrapped-backend pressure reported, and this is the reading it reported.
    Reported(&'reading WrapReading),
    /// The wrapped-backend pressure has not reported.
    NotReported,
}

/// Whether anybody has checked one adapter's stated line grammar against output the backend itself wrote.
///
/// The bare arm is the bootstrap posture rather than a value somebody forgot to fill in.
/// [`GrammarStanding::Checked`] is the checking party's own word: nothing here invokes a backend, and nothing here reads a backend's output to discover what that backend renders.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrammarStanding {
    /// A party checked the adapter's line shapes against output this version of the backend wrote.
    Checked(BackendVersion),
    /// Nobody has checked them, so nothing is qualified and [`AdapterQualification::of`] refuses.
    Unchecked,
}

/// One adapter profile qualified for every reading taken under that exact profile.
///
/// [`AdapterQualification::of`] is the only road, and exactly one pairing travels it: the reading's profile states a backend version, and the standing is a check against that same version.
/// The qualification is reusable across readings carrying that profile, and it identifies no single reading instance.
///
/// # Nonclaims
///
/// Parser correctness is not suite bite: this says the adapter is fit to be read under, and nothing about whether a property rejected anything.
/// The claim ceiling rides through unchanged, because qualifying an adapter never widens what its source affords.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdapterQualification {
    profile: AdapterProfile,
    standing: GrammarStanding,
}

/// Why one reading's profile was not qualified.
///
/// Dependent checks in a declared order: whether anybody checked the shapes at all, whether the profile states a version a check could name, then whether the check and the reading name one version.
#[must_use = "a refusal is the reason a reading's profile was not qualified"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QualificationRefusal {
    /// Nobody has checked the adapter's stated shapes against output the backend wrote.
    GrammarUnchecked,
    /// The reading's profile states no backend version, so a check names nothing it stands under.
    BackendVersionUnstated,
    /// The reading was taken under one backend version and the shapes were checked against another.
    CheckedAgainstAnotherVersion {
        /// The version the reading's own profile states wrote the text.
        stated: BackendVersion,
        /// The version the standing states the shapes were checked against.
        checked: BackendVersion,
    },
}

/// At least one lawful backend-reported kill, read out of a reading whose adapter profile stands qualified.
///
/// The qualification rides inside, so suite pressure over an unqualified profile is not a value anybody can hold.
///
/// # Nonclaims
///
/// Suite bite is not campaign accounting: how many mutants a run pressed and how they divide is [`MutationCensus`]'s question.
/// Neither is it the no-mutation parity ([`NoMutationParityQualification`]), and it cannot open any pair's interpreted trust by itself.
/// It retains no source-tree revision: a reported coordinate is the backend text's coordinate, not a statement that the same line still names the current checkout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSuitePressure {
    qualification: AdapterQualification,
    kill: MutationReport,
}

/// Why one wrap standing demonstrated no generic compiled suite pressure.
///
/// Dependent checks in a declared order: whether the pressure reported, whether the qualification carries the reading's exact profile, then whether what it reported carries a kill.
#[must_use = "a refusal is the reason no compiled suite pressure was demonstrated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SuitePressureRefusal {
    /// The wrapped-backend pressure has not reported, so there is no reading to stand on.
    WrapNotReported,
    /// The qualification names another adapter profile and stands behind nothing here.
    QualificationUnderAnotherProfile,
    /// The reading's run demonstrated no lawful kill.
    NoKillDemonstrated,
}

// ---------------------------------------------------------------------------
// Owner policy and producer discovery.
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

/// The domain tag of one complete producer discovery reading.
pub const MUTATION_DISCOVERY_TAG: DomainTag =
    DomainTag::declared("mutation-discovery", IdentityProfileVersion::declared(1));

/// The domain tag of the exact source bytes one compiled specimen host consumes.
pub const ARTIFACT_CONTENT_TAG: DomainTag = DomainTag::declared(
    "compiled-artifact-content",
    IdentityProfileVersion::declared(1),
);

/// The owner-declared family that binds one production road, evaluation callable, policy, and evidence chain.
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
/// A file move must rename nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActivationSite(NamespacedName);

/// One operator family and canonical mutation meaning a producer found at a site, before owner policy admits it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlternativeDeclaration {
    family: OperatorFamilyRef,
    operation: Vec<u8>,
}

/// Whether the producer's origin reading maps one discovered site to an owner claim.
///
/// The unmapped arm stays a first-class discovery fact and can acquire no policy membership or executable point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwnerClaimMapping {
    /// The origin reading mapped this site to the exact owner claim.
    Mapped(ClaimRef),
    /// The origin reading established no owner claim for this site.
    OwnerUnmapped,
}

/// One producer-discovered mutation site, before owner policy admits it.
///
/// A discovery states the site, its unchanged operation, its candidate alternative meanings, its activation site, and its owner mapping.
/// It grants no permission and is not executable: [`lower_discoveries`](super::discover::lower_discoveries) is the only road from a discovery roster to executable points.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscoveredMutationSite {
    identity: MutationPointRef,
    mapping: OwnerClaimMapping,
    original_operation: Vec<u8>,
    alternatives: Vec<AlternativeDeclaration>,
    activation_site: ActivationSite,
}

/// Why one producer-discovered mutation site was not structurally readable.
#[must_use = "a refusal is the reason one discovered mutation site was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiscoveryRefusal {
    /// The site states no unchanged operation.
    EmptyOriginalOperation,
    /// The site carries no candidate alternative meaning.
    NoAlternative,
    /// One candidate alternative states no mutation meaning.
    EmptyAlternative {
        /// The alternative's position in producer order.
        at: usize,
    },
    /// One candidate alternative is byte-identical to the unchanged operation.
    AlternativeIsOriginal {
        /// The alternative's position in producer order.
        at: usize,
    },
    /// Two candidate alternatives state one operator family and mutation meaning.
    DuplicateAlternativeMeaning {
        /// The duplicate alternative's position in producer order.
        at: usize,
    },
}

/// Why one owner-mapped discovered site did not become executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappedUnpermittedCause {
    /// The policy carries no permission row for the mapped claim.
    Claim(ClaimRef),
    /// One candidate alternative uses a family outside the mapped claim's permission.
    Family {
        /// The alternative's position in producer order.
        at: usize,
        /// The family outside the mapped claim's permission.
        family: OperatorFamilyRef,
    },
}

/// Whether one discovered site was mapped and executable, owner-unmapped, or mapped but unpermitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiscoveryDisposition {
    /// Owner mapping and policy permission admitted this exact executable point.
    Mapped {
        /// The executable point issued from this discovery.
        point: MutationPointRef,
    },
    /// The producer found the site and its origin reading named no owner claim.
    OwnerUnmapped,
    /// The producer mapped the site, and owner policy did not admit it.
    MappedUnpermitted {
        /// The exact policy cause that withheld executable admission.
        cause: MappedUnpermittedCause,
    },
}

/// One complete producer discovery row and its owner-policy admission disposition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscoveryEntry {
    site: DiscoveredMutationSite,
    disposition: DiscoveryDisposition,
}

/// The content identity of one complete producer discovery reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationDiscoveryId(ContentAddress);

/// The complete producer discovery denominator, after owner-policy admission was read over it.
///
/// Every offered site appears exactly once in producer order with its disposition, so unmapped and unpermitted sites stay visible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationDiscoveryReading {
    family: EvaluationFamilyRef,
    policy: MutationPolicyId,
    identity: MutationDiscoveryId,
    entries: Vec<DiscoveryEntry>,
}

/// Why one complete discovery roster could not be lowered.
#[must_use = "a refusal is the reason no complete mutation discovery reading was lowered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiscoveryLoweringRefusal {
    /// Two discovered sites state one point identity.
    DuplicateSite {
        /// The duplicate site's position in producer order.
        at: usize,
        /// The repeated point identity.
        point: MutationPointRef,
    },
}

/// One closed lowering: the complete discovery denominator beside the executable subset drawn from it.
pub struct MutationSurfaceLowering {
    discovery: MutationDiscoveryReading,
    surface: EvaluationSurface,
}

// ---------------------------------------------------------------------------
// The evaluation surface.
// ---------------------------------------------------------------------------

/// The stable identity of one point's admitted mutation meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlternativeId(ContentAddress);

/// One executable operator family and canonical mutation meaning admitted under a point's policy membership.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdmittedAlternative {
    identity: AlternativeId,
    family: OperatorFamilyRef,
    operation: Vec<u8>,
}

/// One owner-admitted executable mutation point on an evaluation surface.
///
/// Only [`lower_discoveries`](super::discover::lower_discoveries) mints this value, after retaining the complete discovery and checking owner mapping and policy permission.
/// A roster of admitted alternatives says which damages the point admits, and never that any of them was materialized, activated, or killed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPoint {
    identity: MutationPointRef,
    membership: PolicyMembership,
    original_operation: Vec<u8>,
    admitted_alternatives: Vec<AdmittedAlternative>,
    activation_site: ActivationSite,
}

/// The content identity of one complete evaluation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluationSurfaceId(ContentAddress);

/// One evaluation surface's complete point table.
///
/// A hand author may supply discovery candidates and owner policy to the same closed lowering a producer targets, and only that lowering mints this surface.
/// Runtime is selection among these points, never interpretation of arbitrary source, which would mint a second meaning authority.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvaluationSurface {
    family: EvaluationFamilyRef,
    policy: MutationPolicyId,
    identity: EvaluationSurfaceId,
    points: Vec<MutationPoint>,
}

/// Whether a complete evaluation surface admits executable points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointCatalogPosture {
    /// The surface is lawful and admits no active directive.
    NoAdmittedPoints,
    /// The surface admits at least one executable mutation point.
    Mutable,
}

/// One point selected into one of the damages it admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActiveSelection {
    surface: EvaluationSurfaceId,
    point: MutationPointRef,
    alternative: AlternativeId,
}

/// One surface-resolved mutation handed to an evaluation callable.
///
/// The value keeps the surface-issued selection and borrows the exact point and alternative it resolved to, so an evaluation callable never reconstructs an identity or consults a positional registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedMutation<'surface> {
    selection: ActiveSelection,
    point: &'surface MutationPoint,
    alternative: &'surface AdmittedAlternative,
}

/// What one evaluation call reads once the surface has resolved its authority.
///
/// The no-mutation posture is directly constructible through [`EvaluationDirective::no_mutation`]; an active directive is minted privately, and only from a selection its exact surface issued.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationDirective<'surface> {
    resolved: Option<ResolvedMutation<'surface>>,
}

/// Why an evaluation callable could not execute one otherwise-lawful directive.
#[must_use = "a refusal is the reason an evaluation callable produced no observation"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationCallRefusal {
    /// The evaluation callable contains no no-mutation branch.
    NoMutationNotImplemented,
    /// The surface admitted a selection the evaluation callable has no branch for.
    ActiveSelectionNotImplemented(ActiveSelection),
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

// ---------------------------------------------------------------------------
// The pair, its witness, and the no-mutation parity.
// ---------------------------------------------------------------------------

/// Why the mandatory no-mutation parity suite could not be declared.
///
/// Dependent checks in a declared order: the substrate names are parsed, then the roster they are declared into.
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

/// The evaluation callable of one evaluation family.
pub type EvaluationCall<Input, Meaning> =
    for<'surface> fn(
        &Input,
        EvaluationDirective<'surface>,
    ) -> Result<EvaluationObservation<Meaning>, EvaluationCallRefusal>;

/// The check that judges one meaning under the trial binding it is joined to.
pub type MeaningCheck<Meaning> = fn(&Meaning) -> TrialConclusion;

/// Raw output from one evaluation call.
///
/// Caller output rather than admitted evidence: the receiver validates the directive, firing count, trial binding, report, and trust facts before any mutation evidence exists.
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

/// One production and evaluation binding under a shared owner declaration and equivalence.
///
/// Matching family references prove the declared relationship and not behavioral agreement; only an executed no-mutation parity reading establishes that, for its exact input.
pub struct EvaluationPair<Input, Meaning> {
    production: ProductionBinding<Input, Meaning>,
    evaluation: EvaluationBinding<Input, Meaning>,
    same: Equivalence<Meaning>,
}

/// Why one production and evaluation binding could not be paired.
#[must_use = "a refusal is the reason an evaluation pair was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationPairRefusal {
    /// The two bindings name different owner families.
    FamilyMismatch {
        /// The production binding's family.
        production: EvaluationFamilyRef,
        /// The evaluation binding's family.
        evaluation: EvaluationFamilyRef,
    },
}

/// The identity and revision facts every reading over one evaluation pair retains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationPairStanding {
    family: EvaluationFamilyRef,
    production_revision: RevisionBinding,
    evaluation_revision: RevisionBinding,
    surface: EvaluationSurfaceId,
}

/// The exact member by which two evaluation-pair standings disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationPairStandingMismatch {
    /// The standings name different evaluation families.
    Family {
        /// The required family.
        expected: EvaluationFamilyRef,
        /// The offered family.
        found: EvaluationFamilyRef,
    },
    /// The standings name different production revisions.
    ProductionRevision {
        /// The required production revision.
        expected: RevisionBinding,
        /// The offered production revision.
        found: RevisionBinding,
    },
    /// The standings name different evaluation revisions.
    EvaluationRevision {
        /// The required evaluation revision.
        expected: RevisionBinding,
        /// The offered evaluation revision.
        found: RevisionBinding,
    },
    /// The standings name different evaluation surfaces.
    Surface {
        /// The required surface.
        expected: EvaluationSurfaceId,
        /// The offered surface.
        found: EvaluationSurfaceId,
    },
    /// The standings differ beyond the individually projected members.
    StandingChanged,
}

/// One trial binding joined to the check identity and callable that judge mutation executions through it.
pub struct MutationWitness<Meaning> {
    binding: TrialBinding,
    check: MeaningCheck<Meaning>,
}

/// Why one mutation witness could not bind its check identity to its trial.
#[must_use = "a refusal is the reason a mutation witness was not bound"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MutationWitnessRefusal {
    /// The offered check identity is not the one the trial row retains.
    CheckMismatch {
        /// The check identity retained by the row.
        expected: CheckRef,
        /// The check identity offered with the callable.
        found: CheckRef,
    },
}

/// The three facts one no-mutation observation compares.
pub struct NoMutationResults<Meaning> {
    production: Meaning,
    evaluation: Meaning,
    evaluation_firings: u32,
}

/// The production and evaluation reports of one no-mutation comparison, kept in their semantic roles.
pub(in crate::muterprater) struct NoMutationReports {
    production: TrialReport,
    evaluation: TrialReport,
}

/// The exact input, results, substrate, conclusion, and reports of one no-mutation comparison.
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
    /// The evaluation callable refused the no-mutation directive.
    EvaluationCall(EvaluationCallRefusal),
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
    /// The no-mutation directive reported an activation.
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

/// One complete no-mutation reading that did not earn qualification, kept whole beside the reason.
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

// ---------------------------------------------------------------------------
// Exact compiled selected-projection pressure.
// ---------------------------------------------------------------------------

/// The bytes handed unchanged to one compiled-specimen host, under their bytes-only content identity.
///
/// The identity commits to no pair, selection, target, toolchain, or caller label; those relationships live in [`CompiledSpecimenStanding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactContent {
    identity: ArtifactContentId,
    bytes: Vec<u8>,
}

/// The bytes-only content identity of one compiler-source artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactContentId(ContentAddress);

/// Whether one compiled specimen is the unchanged baseline or one exact selected mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompiledSpecimenRole {
    /// The production-shaped artifact under no mutation.
    Baseline,
    /// The production-shaped artifact with this surface-issued selection baked in.
    Selected(ActiveSelection),
}

/// Why one specimen source could not be rendered.
#[must_use = "a refusal is the reason one specimen source was not rendered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecimenMaterializerRefusal {
    /// The materializer contains no unchanged-production branch.
    NoMutationNotImplemented,
    /// The surface admitted a selection the materializer has no branch for.
    ActiveSelectionNotImplemented(ActiveSelection),
}

/// A capture-free source materializer over a surface-bound directive.
pub type SpecimenMaterializerCall =
    for<'surface> fn(EvaluationDirective<'surface>) -> Result<Vec<u8>, SpecimenMaterializerRefusal>;

/// One materializer bound, before execution, to the exact pair whose source it renders.
///
/// The compiled-specimen road validates that pair before calling it, resolves the active directive itself, and derives content identity from the bytes returned; nothing here inspects the callable's implementation.
pub struct SpecimenMaterializerBinding {
    pair: EvaluationPairStanding,
    call: SpecimenMaterializerCall,
}

/// One immutable request handed to a compiled-specimen host.
///
/// The request binds the exact content, operation, parity-qualified input, semantic role, execution key, and check identity before any caller code runs.
pub struct CompiledSpecimenRequest<'content, 'input, Input> {
    content: &'content ArtifactContent,
    role: CompiledSpecimenRole,
    operation: &'content [u8],
    input: &'input Input,
    execution: &'content ExecutionKey,
    check: CheckRef,
}

/// A host's typed report that it compiled and executed one exact request and recovered this meaning.
///
/// The constructor copies the content, role, execution, and check facts off the request, so a host cannot supply sibling identity labels.
/// This is still caller output: it records what the host reported and does not prove that a compiler process ran or that the host used those inputs faithfully.
pub struct CompiledSpecimenObservation<Meaning> {
    content: ArtifactContentId,
    role: CompiledSpecimenRole,
    execution: ExecutionKey,
    check: CheckRef,
    meaning: Meaning,
}

/// Which request member made one host observation foreign to the request being judged.
#[must_use = "a mismatch is the exact request member a host observation did not reproduce"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompiledSpecimenObservationMismatch {
    /// The observation names compiler-source content other than the requested content.
    Content {
        /// The requested compiler-source content identity.
        expected: ArtifactContentId,
        /// The compiler-source content identity retained by the observation.
        found: ArtifactContentId,
    },
    /// The observation names another baseline or selected role.
    Role,
    /// The observation retains another execution key.
    Execution,
    /// The observation names another check contract.
    Check,
}

/// Why a compiled-specimen host produced no execution observation.
#[must_use = "a refusal is the reason one compiled specimen produced no observation"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompiledSpecimenHostRefusal {
    /// The host reported that its compiler produced no executable artifact.
    Compilation(ForeignText),
    /// The host reported that the compiled artifact did not complete execution.
    Execution(ForeignText),
    /// The host recovered no meaning tied to the requested operation.
    Meaning(ForeignText),
}

/// A capture-free host adapter that compiles and executes one exact specimen request.
pub type CompiledSpecimenHost<Input, Meaning> = for<'content, 'input> fn(
    CompiledSpecimenRequest<'content, 'input, Input>,
) -> Result<
    CompiledSpecimenObservation<Meaning>,
    CompiledSpecimenHostRefusal,
>;

/// What one selected projection's compiled specimen stands on: its artifact, pair, selection, execution, and check.
///
/// The artifact identity names compiler-source bytes alone, and this standing carries their relationship to the host-reported execution without rehashing caller labels into that identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSpecimenStanding {
    artifact: ArtifactContentId,
    pair: EvaluationPairStanding,
    selection: ActiveSelection,
    execution: ExecutionKey,
    check: CheckRef,
}

/// Exact compiled pressure for one selected projection and one surface-issued selection.
///
/// Building one takes the retained no-mutation qualification, separately rendered baseline and selected artifacts, host-reported executions of those exact bytes, a passing baseline report, and a rejecting selected report.
/// It cannot be minted from an external backend's output or from labels attached after execution.
pub struct CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning> {
    parity: &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning>,
    baseline_artifact: ArtifactContentId,
    standing: CompiledSpecimenStanding,
    baseline_report: TrialReport,
    selected_report: TrialReport,
    mutation: MutationReport,
}

/// Why exact compiled selected-projection pressure could not be established.
#[must_use = "a refusal is the reason exact compiled projection pressure was not established"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompiledProjectionRefusal {
    /// The no-mutation qualification belongs to another evaluation surface.
    ParityForAnotherSurface {
        /// The surface offered for compiled projection pressure.
        expected: EvaluationSurfaceId,
        /// The surface retained by the qualified pair.
        found: EvaluationSurfaceId,
    },
    /// The source materializer is bound to another production and evaluation pair.
    MaterializerForAnotherPair(EvaluationPairStandingMismatch),
    /// The active selection does not belong to the qualified surface.
    Selection(SelectionRefusal),
    /// The retained witness trial belongs to another owner claim.
    WitnessForAnotherClaim {
        /// The claim that owns the selected point.
        expected: ClaimRef,
        /// The claim carried by the retained witness binding.
        found: ClaimRef,
    },
    /// The supplied invocation does not reproduce the qualification's execution key.
    InvocationForAnotherExecution,
    /// The materializer refused the unchanged source.
    BaselineMaterialization(SpecimenMaterializerRefusal),
    /// The materializer refused the exact selected source.
    SelectedMaterialization(SpecimenMaterializerRefusal),
    /// The selected rendering has the same bytes as the unchanged rendering.
    ArtifactDidNotChange(ArtifactContentId),
    /// The host refused compilation or execution of the unchanged artifact.
    BaselineHost(CompiledSpecimenHostRefusal),
    /// The unchanged host observation belongs to another request.
    BaselineObservation(CompiledSpecimenObservationMismatch),
    /// The unchanged host observation could not join the retained trial binding.
    BaselineReport(ReportRecordingRefusal),
    /// The separately compiled unchanged artifact did not pass its witness.
    BaselineDidNotQualify,
    /// The host refused compilation or execution of the selected artifact.
    SelectedHost(CompiledSpecimenHostRefusal),
    /// The selected host observation belongs to another request.
    SelectedObservation(CompiledSpecimenObservationMismatch),
    /// The selected host observation could not join the retained trial binding.
    SelectedReport(ReportRecordingRefusal),
    /// The exact witness did not reject the selected compiled artifact.
    ProjectionDidNotReject,
}

// ---------------------------------------------------------------------------
// The interpreted lane's trust boundary.
// ---------------------------------------------------------------------------

/// Which of the trust order's facts the interpreted lane is still owed.
///
/// Generic suite pressure proves the external suite bit somewhere under its adapter profile and carries no evaluation pair; exact projection pressure owns one qualified pair and one surface-issued selection.
/// Every arm names an absent or mismatched strict value rather than a weak one the gate tries to upgrade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MissingTrustEvidence {
    /// No generic compiled suite pressure demonstrated that the suite bites.
    CompiledSuitePressure,
    /// No exact compiled selected-projection pressure exists for a selection.
    CompiledProjectionPressure,
    /// The exact projection pressure belongs to another evaluation surface.
    ProjectionPressureForAnotherSurface,
}

/// The generic suite bite and exact selection pressure that open interpreted execution for one surface.
pub struct InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning> {
    surface: &'surface EvaluationSurface,
    suite: &'suite CompiledSuitePressure,
    projection: &'projection CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning>,
}

/// The availability of interpreted evidence for one evaluation surface.
///
/// A surface alone earns no trust: availability takes a generic compiled suite bite plus exact projection pressure whose qualification, pair standing, and selection all belong to this surface.
pub enum InterpreterAvailability<
    'surface,
    'suite,
    'projection,
    'parity,
    'pair,
    'input,
    Input,
    Meaning,
> {
    /// A conforming evaluation surface exists and trust has opened.
    Available(
        InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>,
    ),
    /// No conforming evaluation surface exists, producer-authored or hand-authored.
    NoConformingSurface,
    /// A surface exists and the trust order still owes this evidence.
    TrustNotOpened {
        /// What the staging is still owed.
        missing: MissingTrustEvidence,
    },
}

/// The admitted interpreted result of the one active selection an opened trust boundary retains.
pub struct InterpretedMutationEvidence<
    'surface,
    'suite,
    'projection,
    'parity,
    'pair,
    'input,
    Input,
    Meaning,
> {
    trust: InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>,
    meaning: Meaning,
    report: TrialReport,
    mutation: MutationReport,
}

/// Why one interpreted execution produced no admitted mutation evidence.
#[must_use = "a refusal is the reason interpreted mutation evidence was not built"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretedExecutionRefusal {
    /// The supplied invocation does not reproduce the compiled projection's execution key.
    InvocationForAnotherExecution,
    /// The active selection does not belong to the opened surface.
    Selection(SelectionRefusal),
    /// The witness trial belongs to a claim other than the selected point's owner.
    WitnessForAnotherClaim {
        /// The claim that owns the selected point.
        expected: ClaimRef,
        /// The claim carried by the offered trial binding.
        found: ClaimRef,
    },
    /// The evaluation callable omitted the exact surface-issued branch.
    EvaluationCall(EvaluationCallRefusal),
    /// The evaluation callback reported zero firings for the selected damage.
    ///
    /// Boxed because the dud retains the exact surface-issued selection and trial, and the refusing side of a `Result` should not widen its passing side.
    DudPlant(Box<DudPlant>),
    /// The observation could not join its exact trial binding.
    Report(ReportRecordingRefusal),
}

// ---------------------------------------------------------------------------
// The rewrite lane.
// ---------------------------------------------------------------------------

/// One rewrite-mutation descriptor: the shape a damage matches, the shape it rewrites to, and the operator family the pair realizes.
///
/// Data rows, never programs: a descriptor states a pattern and its rewrite as text a structural rewriter reads, and nothing here compiles, executes, or interprets either side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RewriteDescriptor {
    family: OperatorFamilyRef,
    pattern: &'static str,
    rewrite: &'static str,
}

/// Why one rewrite descriptor was refused.
///
/// Dependent checks in a declared order: the pattern, then the rewrite, then the pair.
#[must_use = "a refusal is the reason a rewrite descriptor was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteRefusal {
    /// The pattern is empty, so the descriptor matches nothing.
    EmptyPattern,
    /// The rewrite is empty, so the descriptor states no damage.
    EmptyRewrite,
    /// The pattern and the rewrite are one shape, so applying it damages nothing.
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
/// Rewrite-produced descriptors are admitted last, as candidates the harness audits and never as evidence on their own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteTrust {
    /// The descriptor awaits the harness's audit.
    AuditPending,
}

/// One rewrite descriptor planned for audit, with the scope it was planned under and the trust it stands under.
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
    /// The interpreted lane, which is what makes rewrite families cheap, is unavailable.
    InterpreterUnavailable,
    /// The trust order still owes this evidence.
    TrustNotOpened(MissingTrustEvidence),
}

/// Whether rewrite descriptors may enter the interpreted audit road.
///
/// Admission here is execution availability and not evidence: a descriptor stays [`RewriteTrust::AuditPending`] until an actual execution establishes what a later evidence owner requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteAdmission {
    /// The audit road is available under a generic suite bite and exact selection-scoped projection pressure.
    Admitted,
    /// The audit road is unavailable for a stated reason.
    Withheld(RewriteWithheld),
}

// ---------------------------------------------------------------------------
// The artifact-mutation seed roster.
// ---------------------------------------------------------------------------

/// One deliberate damage the artifact-mutation mode inflicts on a lawful rendered artifact.
///
/// Each arm is a lie a damaged rendering tells about the declaration it claims to project, and every one of them is this harness's own — a producer that writes its own exam is rehearsed only against the defects it already imagined.
/// The roster is seed material rather than a lane: the surgery that realizes one is authored where the anchors are, so a damage is cut against the anchors a generator emits rather than against spellings a hand restated beside them.
///
/// # Nonclaims
///
/// It says nothing about which reader catches a damage.
/// That ownership belongs to the readers that exist ([`crate::oracle`]) and is stated there, against a seat that can hold it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactMutation {
    /// The emitted members are written in reverse of the order the declaration states.
    OrderPermuted,
    /// Every emitted member is written under the first member's key, so members the declaration keeps distinct share one identity.
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
    /// A decoy carrying the anchored bytes is planted in a comment while the real constant is damaged.
    DecoyInComment,
    /// One planned member constant is emitted twice inside one implementation.
    ImplMemberDuplicated,
    /// A member nobody planned is added inside one implementation.
    ImplMemberUnexpected,
    /// A declared value is carried through a constructor the declaration did not name.
    ConstructorPathAltered,
    /// The implementation is written under a posture the declaration did not name.
    ImplPostureAltered,
    /// An attribute that decides something is added to an implementation.
    MeaningBearingAttributeAdded,
    /// The artifact stops being well-formed Rust.
    MalformedRust,
}

/// The artifact-mutation roster, in the order this home states it.
///
/// A declared table rather than a derived one, so a plan reads the damages in an order written down once here.
/// A slice rather than a sized array: a consumer whose artifacts break in ways this table does not name declares its own slice, and nothing here closes the width.
pub const ARTIFACT_MUTATIONS: &[ArtifactMutation] = &[
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
// Survivor explanation, and the check gap.
// ---------------------------------------------------------------------------

/// Which independence lane a survivor's explanation names as the missing judge.
///
/// The roster is the independence annex's own lanes ([`crate::oracle`]), named here so an explanation says which kind of judge is absent rather than that one is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OracleClass {
    /// Bytes a specification states for an input.
    GoldenVector,
    /// A published identity re-derived from its published specification.
    IndependentTranscript,
    /// What a rendered artifact declares.
    StructuralRead,
    /// What a compiled artifact hands back as values.
    CompiledReadBack,
}

/// One survivor, explained: the target, the claim that owns it, the oracle class no check supplies, and the check that would close the opening.
///
/// An explanation over an owner-unmapped target is refused rather than guessed, so no candidate is cut against a claim nobody established.
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
    /// The target's owning claim is unmapped, so the explanation would have to invent it.
    OwnerUnmapped,
}

/// The typed finding a synthesis raises instead of a candidate it cannot honestly build.
///
/// Synthesis is scoped to already-authored executable attachments, so where the named check has no attachment the opening is this finding rather than a candidate citing a callable nobody wrote.
#[must_use = "a check gap is a finding, never a candidate"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckGap {
    claim: ClaimRef,
    check: CheckRef,
    missing: OracleClass,
}

/// The row coordinates a synthesis cannot read off a survivor.
///
/// The explanation names the claim and the check; the suite, classification, subject route, and population are the caller's to state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSketch {
    suite: ExecutionSuite,
    classification: Classification,
    subject: SubjectRoute,
    population: PopulationRef,
}

/// Why one candidate row could not be synthesized.
///
/// Dependent checks in a declared order: the attachment roster, then the synthesis facts the origin arm needs, then the row itself.
#[must_use = "a refusal is the reason a candidate was not synthesized"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthesisRefusal {
    /// The named check has no authored executable attachment, so the opening is a check gap.
    CheckGapFound(CheckGap),
    /// The explained record names a coordinate rather than a mutation point.
    ///
    /// A guard on the identity shape rather than on a lane: earning the survived verdict takes observed activation, and the one wrapped backend offers no channel that could observe a firing.
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
/// Scope shapes are invocation parameters and never a second world: each narrows a run, and every report is still stated over the complete table.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeShape {
    /// One seam, entered through the subject route the run is narrowed to.
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
/// The mutant bound is this home's; the per-trial budgets are the invocation profile's, borrowed rather than restated.
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
    /// The interpreted route's rapid loop.
    InterpretedMutation,
    /// Structure-aware generation with budgeted minimization.
    Fuzz,
    /// Campaigns over the fault adapters.
    Chaos,
}

/// Which damage one planned run presses.
///
/// An external backend chooses its own damage and the mutant identity already names it; an interpreted run states which admitted alternative it selects, which is what keeps two runs over one point from reading as one run stated twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlannedDamage {
    /// The backend's own damage, already named by the mutant identity.
    BackendChosen,
    /// One admitted alternative of an interpreted point.
    Alternative(AlternativeId),
}

/// One intended run: the lane, the target, the damage, what the run selects, and what it may spend.
///
/// A planned run is a value and spends nothing, which is what makes planning inspectable before a budget is committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedRun {
    lane: PressureLane,
    target: MutationIdentity,
    damage: PlannedDamage,
    selection: Selection,
    budget: PressureBudget,
}

/// The complete statement of what one pressure pass intends to run, before any budget is spent.
///
/// Planning is a pure function and this is its image: a caller reads every intended run and its budget, and decides, before the first mutant is pressed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPlan {
    scope: ScopedInvocation,
    runs: Vec<PlannedRun>,
}

/// Why one proof plan was refused.
///
/// Dependent checks in a declared order: the roster is read before it is weighed against the budget.
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

/// A claim declared owed: its identity, and the opening condition its declaration named.
///
/// Owed is a posture on a claim and never a genus, and an obligation that never comes due is refused, so no value here is an obligation nobody can discharge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwedClaim {
    claim: ClaimRef,
    opening_condition: &'static str,
}

/// Why one owed-claim posture was refused.
#[must_use = "a refusal is the reason an owed claim was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwedClaimRefusal {
    /// The posture names no opening condition, so nothing states when the claim comes due.
    NoOpeningCondition,
}

/// What shape of proof one opening asks for.
///
/// Which lane discharges an obligation follows from this and nothing else, and that map is declared once in `type_contract.rs`.
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

/// One opening a coverage reading states: an owed claim the denominator names and no report exercised.
///
/// Where proof is missing is claim coverage over reports and never a structural scan, so this value is born from a coverage entry and carries the counts it was born from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InferredObligation {
    owed: OwedClaim,
    exercise: ClaimExercise,
    shape: ProofShape,
}

/// What discharged one owed claim: the lane it was routed to, the trial that discharged it, and the key that trial ran under.
///
/// A discharge authors no capsule, because the admitted row is its permanent record and rerunning it regenerates the behavioral evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DischargeEvidence {
    lane: ObligationLane,
    trial: TrialId,
    key: ExecutionKey,
}

// ---------------------------------------------------------------------------
// The proposal road.
// ---------------------------------------------------------------------------

/// One demonstrated kill: the report the staged run wrote, and the rejection read out of it.
///
/// A claimed kill is demonstrated on the evaluation surface with the mutant active, never asserted, and the mutant-killed ground cannot be built without one of these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Demonstration {
    report: RunReport,
    trial_report: TrialReport,
    rejection: DemonstratedRejection,
}

/// Why no kill was demonstrated.
///
/// Dependent checks in a declared order: the view's posture, then the census, then the candidate's own disposition.
#[must_use = "a refusal is the reason a kill was not demonstrated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofRefusal {
    /// The staged view could not be built.
    StagingRefused(StagedTableRefusal),
    /// The report stands over the authored world rather than a staged view.
    NotStaged,
    /// The report's census does not carry the candidate's trial at all.
    CandidateNotInCensus,
    /// The run's selection passed the candidate over, so it never executed.
    CandidateNotSelected,
    /// The candidate was selected and did not execute.
    CandidateDidNotExecute,
    /// The candidate executed and did not refuse, so the claimed kill is asserted rather than shown.
    CandidateDidNotRefuse,
}

/// How much proof one candidate adds to the claim it pins.
///
/// [`ProofDelta::between`] refuses a pair that does not move, because a pin that adds nothing is not a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProofDelta {
    before: usize,
    after: usize,
}

/// Why one proof delta was refused.
#[must_use = "a refusal is the reason a proof delta was not stated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofDeltaRefusal {
    /// The candidate leaves the claim's exercised count where it was.
    NoProofAdded {
        /// The count before.
        before: usize,
        /// The count after.
        after: usize,
    },
}

/// The ground a mutant-killed proposal stands on: a kill shown on the surface with the mutant active.
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
/// No capsule, and no seat for one: the admitted row is the discharge's permanent record, and the two grounds that do author a capsule each carry it as a field.
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
    /// Nothing comparable was kept: no previous fingerprint and no discharge roster.
    NoKnownMaterial,
}

/// The evidence a failure-bearing proposal is not a duplicate: the candidate's fingerprint, against every fingerprint already known.
///
/// The comparison happens where the value is built, so a duplicate is a refusal rather than a paragraph a reader has to check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureComparison {
    /// The fingerprint this candidate carries.
    candidate: Fingerprint,
    /// The fingerprints already known, in the order they were compared.
    known: Vec<Fingerprint>,
}

/// The evidence a discharge proposal is not a duplicate: the owed claim, compared against the discharges already recorded for it.
///
/// The comparison happens where the value is built and only an empty roster survives it, so holding one IS holding the evidence — no roster seat rides along.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationComparison {
    /// The owed claim.
    owed: ClaimRef,
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
    /// The candidate's fingerprint is one the known roster already carries.
    FingerprintAlreadyKnown(Fingerprint),
    /// The owed claim already carries a discharge, so this one discharges nothing new.
    ObligationAlreadyDischarged(TrialId),
}

/// Where an admitted row would land: a semantic owner and a suite, never a file path.
///
/// One field, because the suite's own namespace is the semantic owner, and a second owner field here would be a second authority answering one question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProposalDestination {
    suite: ExecutionSuite,
}

/// The domain tag every proposal identity is derived under.
pub const PROPOSAL_TAG: DomainTag =
    DomainTag::declared("proposal", IdentityProfileVersion::declared(1));

/// What every proposal is, whichever ground it stands on: a candidate row, a ground word, a destination, and the identity those three derive.
///
/// Open, so a consumer with an admission ground of its own implements it in its own crate and reaches every [`ProposalSink`] through the same seam.
/// A road that stores or reports a proposal takes one of these rather than a sum type every ground would have to fit inside, which is what keeps a discharge proposal from being as large as a kill's demonstration.
///
/// # Nonclaims
///
/// It reaches no ground's own contents: what a kill demonstrated and what a pin moved are read off the concrete proposal, because they are exactly the facts the implementations do not share.
pub trait ProposalDocument {
    /// The candidate row.
    fn candidate(&self) -> &Row;

    /// The ground at summary width — the word an admission act states.
    fn ground_summary(&self) -> AdmissionGround;

    /// Where it would land.
    fn destination(&self) -> ProposalDestination;

    /// The proposal's content identity, which is permanent provenance.
    ///
    /// # The specification
    ///
    /// Two primitives: `u32be(n)`, and `bytes(x)` — `u64be(len(x))` followed by the bytes of `x`.
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
    /// The evidence is deliberately absent: the capsule, the demonstration, and the duplicate comparison are what stands behind a proposal rather than what it proposes.
    /// Two offers of one row on one ground into one destination therefore share an identity, which is what makes an admitted origin's citation stable across a rerun.
    fn identity(&self) -> ProposalId;
}

/// The replay-bearing subset of the sealed proposal roster.
///
/// A discharge proposal cannot implement this trait and so cannot reach the replay admission operation.
pub trait ReplayBearingProposal: ProposalDocument {
    /// The run-bound capsule this proposal carries.
    fn replay_capsule(&self) -> &ReplayCapsule;

    /// The replay-bearing ground the human admission states.
    fn replay_ground(&self) -> crate::descriptor::ReplayBearingGround;
}

/// One proposal on the mutant-killed ground.
///
/// Process-local until a caller's own sink stores it, and constructing one asserts nothing about admission.
/// The comparison seat takes a [`FailureComparison`] and admits nothing else, so evidence that does not fit the ground is unwritable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantKilledProposal {
    candidate: Row,
    ground: MutantKilledGround,
    duplicate: FailureComparison,
    destination: ProposalDestination,
}

/// One proposal on the claim-pinned ground.
///
/// Its comparison seat takes a [`NoComparison`], because a pin carries no failure to fingerprint and discharges no obligation.
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
/// Dependent checks in a declared order: the row's origin, then — where the ground names a mutation point — the survivor point against the target.
/// There is no evidence-against-ground cause, because each proposal's comparison seat admits exactly the comparison its ground owes.
#[must_use = "a refusal is the reason a proposal was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProposalRefusal {
    /// The row does not carry the candidate origin arm.
    NotACandidate,
    /// The row's synthesis facts and the ground's target name different mutation points.
    SurvivorPointMismatch {
        /// The point the row's synthesis facts name.
        synthesis: MutationPointRef,
        /// The point the ground's target names.
        target: MutationPointRef,
    },
}

/// Why one mutant-killed proposal was not offered.
///
/// Dependent checks in a declared order: a harness-demonstrated rejection, agreement with the staged demonstration, replay execution and fingerprint binding, duplicate comparison, then proposal construction.
#[must_use = "a refusal is the reason a mutant-killed proposal was not offered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KillProposalRefusal {
    /// The mutation report does not carry a harness-demonstrated rejection.
    MutationNotDemonstrated {
        /// The verdict the report actually earned.
        verdict: MutationVerdict,
    },
    /// The mutation report and staged demonstration name different failures.
    DemonstrationMismatch {
        /// The content address of the failure the mutation report names.
        mutation: ContentAddress,
        /// The content address of the failure the staged demonstration names.
        demonstration: ContentAddress,
    },
    /// The replay capsule stands over another execution.
    ReplayExecutionMismatch {
        /// The execution address the capsule names.
        replay: ContentAddress,
        /// The execution address the demonstrating trial report names.
        demonstration: ContentAddress,
    },
    /// The replay capsule preserved another failure.
    ReplayFingerprintMismatch {
        /// The content address of the failure the capsule preserved.
        replay: ContentAddress,
        /// The content address of the failure the staged demonstration names.
        demonstration: ContentAddress,
    },
    /// The comparison found the candidate's failure already known.
    Duplicate(DuplicateRefusal),
    /// The proposal constructor refused the values that were assembled.
    Refused(ProposalRefusal),
}

/// Why one obligation-discharge proposal was not offered.
///
/// Dependent checks in a declared order: duplicate comparison, then proposal construction.
#[must_use = "a refusal is the reason a discharge proposal was not offered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DischargeProposalRefusal {
    /// The owed claim already carries a discharge.
    Duplicate(DuplicateRefusal),
    /// The proposal constructor refused the values that were assembled.
    Refused(ProposalRefusal),
}

/// The location one sink stored a proposal at.
///
/// Opaque and mortal: the review artifact may die after any ruling, which is why an admitted origin cites the proposal's [`ProposalId`] and never this token.
/// It is not an identity, not a path this crate can interpret, and not evidence that the destination is durable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredProposalRef {
    proposal: ProposalId,
    token: String,
}

/// Why one sink did not store a proposal.
///
/// The durability arm is the sink's own statement: this crate reaches no filesystem and can establish nothing about where a sink writes.
#[must_use = "a refusal is the reason a proposal was not stored"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinkRefusal {
    /// The sink is not accepting proposals.
    Unavailable,
    /// The sink already holds a proposal under this content identity.
    AlreadyStored(ProposalId),
    /// The location offered is empty, so it names nowhere.
    EmptyLocation,
    /// The destination lies inside the repository tree or a build cache.
    ///
    /// Undischarged review evidence may never live there: deleting a cache must change only cost, never truth.
    DestinationNotDurable,
}

/// The caller-supplied storage the proposal road ends at.
///
/// The only storage seam anywhere in this crate: no realization is declared here, no filesystem is reached, and no scratch directory exists.
/// Storing is not admitting — a stored proposal is review material a human rules on, and the ruling is what discharges it.
pub trait ProposalSink {
    /// Store one proposal, and hand back the location custody begins at.
    ///
    /// Generic over the sealed roster rather than one sum type, so a discharge proposal does not have to be as large as a kill's demonstration to reach this seam.
    ///
    /// # Errors
    ///
    /// The sink's own refusal: unavailable, already stored under this identity, an empty location, or a destination that is not durable.
    fn store<Document: ProposalDocument>(
        &mut self,
        proposal: &Document,
    ) -> Result<StoredProposalRef, SinkRefusal>;
}

/// A completed human admission on a replay-bearing proposal.
///
/// The admitted row, the depot entry, proposal custody, and depot custody ride together, and construction happens only after the caller's sink reports the exact entry stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayAdmissionReceipt {
    row: Row,
    entry: ReplayCapsuleEntry,
    proposal_custody: StoredProposalRef,
    replay_custody: StoredReplayEntryRef,
}

/// A completed human admission on an obligation-discharge proposal.
///
/// The discharge authors no replay entry, because the admitted row is its durable behavioral record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DischargeAdmissionReceipt {
    row: Row,
    proposal_custody: StoredProposalRef,
}

/// Why an explicit human admission did not complete.
///
/// Checks precede caller storage: proposal custody, then row construction, then the replay depot's result and its exact-reference binding.
#[must_use = "a refusal is the reason human admission did not complete"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HumanAdmissionRefusal {
    /// The supplied review custody belongs to another proposal.
    ProposalCustodyMismatch {
        /// The proposal being admitted.
        expected: ProposalId,
        /// The proposal the storage reference names.
        found: ProposalId,
    },
    /// The admitted row could not be encoded.
    RowRefused(RowRefusal),
    /// The caller's replay depot refused storage.
    ReplayDepotRefused(ReplayDepotRefusal),
    /// The sink reported a location bound to another replay entry.
    ReplayCustodyMismatch {
        /// The content-derived replay reference being admitted.
        expected: crate::descriptor::ReplayRef,
        /// The replay reference the sink's location names.
        found: crate::descriptor::ReplayRef,
    },
}

// ---------------------------------------------------------------------------
// The names this home spells.
// ---------------------------------------------------------------------------

/// The owner every name this home spells is declared under.
pub const MUTERPRATER_NAMESPACE: &str = "muterprater";

/// The spelling of the declaration both no-mutation parity roads are projected from.
pub const PARITY_DECLARATION_SUBSTRATE: &str = "one-declaration";

/// The spelling of the rendering engine both no-mutation parity roads stand on.
pub const PARITY_RENDERING_SUBSTRATE: &str = "rendering-engine";

/// The spelling the no-mutation parity's road pairing is declared under.
pub const NO_MUTATION_PAIRING: &str = "no-mutation-parity";

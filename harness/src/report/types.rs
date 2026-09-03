//! Every public type of the record vocabulary.
//!
//! Declarations only.
//! The roads that reach a private field are in `type_guard.rs` and its three subject files, the canonical preimages are in `encode.rs`, and the readings are their own pure-function files.

use crate::clock::MeasurementReading;
use crate::descriptor::{
    AuthoredTableName, ClaimRef, GeneratedSupportSchemaId, TablePosture, TrialKey,
};
use crate::identity::{DomainTag, IdentityProfileVersion};

#[path = "type_guard.rs"]
mod guard;

// The semantic rail: what a trial means.

/// The feature profile a trial's identity is stated under.
///
/// The one value says what is true of every trial the harness runs: nothing splits trials by feature profile, so no trial carries one.
/// A later split adds a variant beside this one without moving its encoded slot, so identities already derived keep their names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TrialProfile {
    /// The trial is not profiled.
    Unprofiled,
}

/// The complete preimage of one [`TrialId`]: a trial's compact key, and the profile coordinate this home adds to it.
///
/// The claim, the subject, the check contract, and the population reach the identity through the key, which the descriptor home derived over them where the row was born.
/// Encoding those four again here would be a second implementation of one framing, and two implementations agree until one of them is edited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProfiledTrial {
    key: TrialKey,
    profile: TrialProfile,
}

/// The domain tag every trial identity is derived under.
pub const TRIAL_IDENTITY_TAG: DomainTag =
    DomainTag::declared("trial-identity", IdentityProfileVersion::declared(1));

crate::identity::content_address_reference! {
    /// One trial's semantic identity: what the trial means, independent of where it is written.
    ///
    /// The same key under the same profile derives the same thirty-two bytes anywhere, so two rows sharing an identity are two spellings of one measurement and the table constructor refuses the pair.
    /// Where a trial lives is a [`TrialSite`], and the two rails never mix.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct TrialId;
}

/// Where one trial is written: the rail a person filters on and jumps to.
///
/// A refactor that moves the trial changes every field here and changes no [`TrialId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrialSite {
    module_path: &'static str,
    file: &'static str,
    line: u32,
    name: &'static str,
}

// The revision rail: which code a run stood on.

/// The domain tag every row revision identity is derived under.
pub const ROW_REVISION_TAG: DomainTag =
    DomainTag::declared("row-revision", IdentityProfileVersion::declared(1));

crate::identity::content_address_reference! {
    /// The identity of one complete authored row, derived from the canonical bytes that row committed to.
    ///
    /// It owns bookkeeping — census, aggregation, report diff.
    /// A tag or origin edit moves it and owes no execution, because nothing about what the row runs has changed.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct RowRevisionId;
}

crate::identity::content_address_reference! {
    /// The exact subject revision one attachment bound.
    ///
    /// The binding is the authority on what was committed to and under which posture, so nothing is derived a second time here.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct SubjectRevisionId;
}

crate::identity::content_address_reference! {
    /// The exact check revision one attachment bound.
    ///
    /// The check's contract is a coordinate of [`TrialId`]; this is the implementation standing behind that contract, and the two move independently.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct CheckRevisionId;
}

/// The exact subject and check revisions one trial binding stands on.
///
/// Kept as one relationship because an execution never has one half without the other, while the differently typed fields keep the two roles from being exchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionRevisions {
    subject: SubjectRevisionId,
    check: CheckRevisionId,
}

// The execution rail: what one run was keyed by.

/// The compilation target a run stood on, by its declared triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TargetTriple(String);

/// The toolchain a run stood on, by its declared identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToolchainIdentity(String);

/// The target and toolchain one execution actually ran under.
///
/// Both are declared at the invocation, and they enter [`ExecutionKey`] rather than any semantic identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetBinding {
    target: TargetTriple,
    toolchain: ToolchainIdentity,
}

/// How many generated cases one invocation admits for one trial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CaseBudget(u32);

/// How many input bytes one invocation admits for one trial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ByteBudget(u64);

/// How long one invocation admits one trial to run, in nanoseconds.
///
/// A declared bound rather than a measurement; what a run actually observed is a [`MeasurementReading`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeBudget(u64);

/// The facts of one invocation that can change what a trial concludes.
///
/// A result reached under a smaller budget is not evidence for a larger one, which is why these ride [`ExecutionKey`].
/// The set is closed: a fact that can move a conclusion is a field here, and adding one is a change to the law rather than a new argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvocationProfile {
    cases: CaseBudget,
    bytes: ByteBudget,
    time: TimeBudget,
}

/// The domain tag every execution key is derived under.
pub const EXECUTION_KEY_TAG: DomainTag =
    DomainTag::declared("execution-key", IdentityProfileVersion::declared(1));

/// What one execution of one trial was keyed by.
///
/// The target binding is a member unconditionally, so a key derived on one target cannot equal a key derived on another.
/// The address is derived from the parts on demand rather than stored beside them, so no second value can disagree with what the key is made of.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecutionKey {
    trial: TrialId,
    revisions: ExecutionRevisions,
    invocation: InvocationProfile,
    target: TargetBinding,
}

// What an attachment's revision bindings buy.

/// Whether a rerun cache may stand in for executing an attachment again.
///
/// This vocabulary is the one owning statement about cache eligibility, and every other mention of eligibility in the harness points here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheEligibility {
    /// Both revisions were derived by the harness operation from the canonical material their owners supplied, so the key carries their exact addresses.
    Eligible,
    /// The attachment carries no harness-derived commitment strong enough to let a prior execution stand in for this one, so every run executes.
    NeverEligible,
}

/// What a reproduction of one execution can claim.
///
/// The runner opens the posture at the attachment's revision meet, and every later participant — the probe adapter, each semantic reducer actually invoked — can only narrow it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplayPosture {
    /// The one posture that earns the phrase "replay exactly": the reproduction is the same execution, not a similar one.
    ExactDerived,
    /// The reproduction stands on revisions a hand committed to, so the claim is exactly as good as that commitment.
    DeclaredByAuthor,
    /// Exact reproduction is unavailable: the historical run and its input are still evidence, the attachment always reruns, and every rendering says so.
    UnavailableBecauseUntracked,
}

// The replay account.

/// Which generation profile produced a capsule's input, and at which version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenerationProfile {
    name: &'static str,
    version: u32,
}

/// Which minimization profile reduced a capsule's input, and at which version.
///
/// A reduction that preserved the fingerprint under one profile says nothing about another, so the profile rides the capsule instead of being remembered by whoever reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinimizationProfile {
    name: &'static str,
    version: u32,
}

/// The domain tag every replay capsule identity is derived under.
pub const REPLAY_CAPSULE_TAG: DomainTag =
    DomainTag::declared("replay-capsule", IdentityProfileVersion::declared(2));

/// The closed shape of one run-bound reproduction account.
///
/// Its one mint consumes completed [`crate::generate::ReductionEvidence`] bound to a real refused report, so no caller assembles these seats independently.
/// Holding a capsule is not holding an exact-replay claim: only [`ReplayPosture::ExactDerived`] earns that phrase, and the other two postures state their own ceilings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCapsule {
    key: ExecutionKey,
    input: Vec<u8>,
    fingerprint: Fingerprint,
    generation: GenerationProfile,
    minimization: MinimizationProfile,
    schema: GeneratedSupportSchemaId,
    posture: ReplayPosture,
}

// Findings.

/// The normalized shape of a failure.
///
/// It names the kind of disagreement rather than any of its particulars, which is what turns many finds into few defects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FailureClass {
    /// The check returned a typed refusal about the subject.
    RefusedByCheck,
    /// An algebraic property's law disagreed with the subject.
    PropertyDisagreement,
    /// The independent oracle disagreed with the subject.
    OracleDisagreement,
    /// A panic from the subject, caught at the trial boundary and recorded as the finding it is.
    SubjectPanic,
    /// The subject exceeded a declared budget while the check was still undecided.
    BudgetExhausted,
}

/// The cause a finding names, as the pair of declared names its owner wrote down.
///
/// The family and the local key are the caller's own spelling.
/// The harness stores them, hashes them into every [`Fingerprint`], and never reads inside them; free text about a cause is [`ForeignText`] and decides nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FindingCause {
    family: &'static str,
    local: &'static str,
}

/// Where a refusal was raised.
///
/// Not where the trial lives: a property suite refuses inside itself while the trial sits in a table somewhere else, and a reader needs both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FindingLocation {
    file: &'static str,
    line: u32,
}

/// The greatest number of foreign-text bytes one finding admits.
pub const FOREIGN_TEXT_MAX_BYTES: usize = 4096;

/// Whether a foreign-text field carries everything it was offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Truncation {
    /// Everything offered was admitted.
    Complete,
    /// The material exceeded the bound and was cut, with both counts kept so no reader is shown a shortened rendering that looks whole.
    TruncatedAt {
        /// How many bytes were admitted.
        admitted: usize,
        /// How many bytes were offered.
        offered: usize,
    },
}

/// Whether rendering a foreign-text field as text loses anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextFidelity {
    /// The admitted bytes are valid UTF-8 and render unchanged.
    Exact,
    /// The admitted bytes are not valid UTF-8, so rendering substitutes replacement characters.
    LossyReplacement,
}

/// Text that came from outside this crate's own vocabulary, bounded and marked.
///
/// A subject's panic payload, an external tool's output, a decoder's message.
/// It travels one way: nothing in the harness reads it back, matches on it, or builds a summary from it, so a finding is a typed value first and prose second.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForeignText {
    bytes: Vec<u8>,
    truncation: Truncation,
    fidelity: TextFidelity,
}

/// One typed refusal: what disagreed, the cause it names, where it was raised, and any foreign text it carried in.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrialFinding {
    class: FailureClass,
    cause: FindingCause,
    located: FindingLocation,
    foreign: Option<ForeignText>,
}

/// What one executed trial concluded.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrialConclusion {
    /// The check was satisfied.
    Passed,
    /// The check refused, and the refusal carries its evidence.
    Refused(TrialFinding),
}

/// The domain tag every failure fingerprint is derived under.
pub const FINGERPRINT_TAG: DomainTag =
    DomainTag::declared("failure-fingerprint", IdentityProfileVersion::declared(1));

/// One failure's identity: the trial's semantic identity, the typed cause, and the normalized class.
///
/// Naming a failure this way is what lets two runs deduplicate one find, a minimizer shrink an input without wandering to a different bug, rerun selection survive a refactor, and many finds group into few defects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fingerprint {
    trial: TrialId,
    cause: FindingCause,
    class: FailureClass,
}

// One execution's record.

/// Why a selected trial did not execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkipReason {
    /// The invocation's budget was spent before this trial ran.
    BudgetExhausted,
    /// The subject is not exercisable on the running target.
    TargetUnsupported,
    /// Material the trial requires — generated support, a corpus, a fault adapter — was not present.
    PrerequisiteAbsent,
    /// A cached execution under this trial's execution key stood in, which is lawful exactly when [`CacheEligibility`] admitted it.
    SatisfiedByCachedExecution,
}

/// What failed in the harness rather than in the subject.
///
/// Kept apart from every conclusion, because folding one into a refusal would put a verdict on a subject nobody exercised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfrastructureFault {
    /// The population could not supply inputs.
    GenerationUnavailable,
    /// Generated support the trial binds was absent or unreadable.
    SupportAbsent,
    /// The harness could not record the execution's own evidence.
    CaptureFailed,
    /// A requested backend is not implemented for the compilation target.
    BackendUnavailable,
    /// A backend could not establish its configured execution environment.
    BackendInitializationFailed,
    /// A backend began its work but did not return evidence that distinguishes a subject result from an infrastructure interruption.
    BackendExecutionUnresolved,
}

/// One harness-side failure: its typed class and any bounded material the failing boundary carried in.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfrastructureFailure {
    fault: InfrastructureFault,
    foreign: Option<ForeignText>,
}

/// What became of one selected trial.
///
/// The conclusion rides the executed arm, so a conclusion without an execution and an execution without a conclusion are both unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunAttempt {
    /// It ran, and this is what it concluded.
    Executed(TrialConclusion),
    /// It did not run, for a stated reason.
    SkippedWithReason(SkipReason),
    /// It ran past the invocation's time budget.
    ///
    /// The exact bound lives on the report's execution key, so a host cannot attach a second budget that disagrees with the invocation.
    TimedOut,
    /// The harness failed around it, so nothing was learned about the subject.
    InfrastructureFailed(InfrastructureFailure),
}

/// One external host's typed input about a selected trial.
///
/// A host can establish an attempt and a wall-measurement reading the in-process runner cannot, and that is the whole of what it may state.
/// The semantic standing, the site, the census, the selection outcome, and the table posture are derived at the join that admits this value, never accepted from it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostTrialRecord {
    trial: TrialId,
    attempt: RunAttempt,
    measurement: MeasurementReading,
}

/// The exact semantic and revision standing one admitted trial report ran under.
///
/// The runner derives both members: the key from the bound row, attachment, invocation profile and target, and the replay ceiling from the attachment's two revision postures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrialRunStanding {
    key: ExecutionKey,
    replay: ReplayPosture,
}

/// One execution's record: the two rails joined, what became of the attempt, and the wall-measurement posture recorded around it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrialReport {
    standing: TrialRunStanding,
    site: TrialSite,
    attempt: RunAttempt,
    measurement: MeasurementReading,
}

// One run's complete-table accounting.

/// Why a trial in the denominator was not selected by this invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotSelectedReason {
    /// The invocation's selection did not name this trial.
    OutsideSelection,
    /// The row's execution suite was not part of this invocation.
    SuiteNotRun,
}

/// What one invocation did about one row of the denominator.
///
/// A census is mostly rows nobody selected, so the selected arm boxes its record rather than making every unselected row as large as the largest report.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SelectionDisposition {
    /// Selected, and here is what happened.
    Selected(Box<TrialReport>),
    /// Not selected, for a stated reason.
    NotSelected {
        /// The table-derived semantic identity of the row that was not selected.
        trial: TrialId,
        /// Why the selection passed it over.
        reason: NotSelectedReason,
    },
}

/// Whether one row of the denominator was actually exercised.
///
/// Selection is not exercise: a trial the invocation selected and then skipped was not exercised, and counting it would claim evidence nobody produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exercise {
    /// The trial executed and reached a conclusion.
    Exercised,
    /// The trial did not execute.
    Unexercised,
}

/// One row of the denominator, and what this invocation did about it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrialAccounting {
    row: RowRevisionId,
    revisions: ExecutionRevisions,
    claim: ClaimRef,
    disposition: SelectionDisposition,
}

/// Why a caller stated in advance that a selection matching nothing is a lawful answer.
///
/// Typed and closed, because a reason nobody can enumerate is a reason nobody can review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmptySelectionReason {
    /// The selection was carried over from a previous run, so a trial the world no longer holds is a lawful absence rather than a failure.
    CarriedOverFromAPreviousRun,
    /// The run asks what the world holds under this selection, and a claim with no row serving it is the reading's own finding.
    AskingWhatTheWorldHolds,
}

/// What one invocation expects its selection to match.
///
/// [`SelectionExpectation::AtLeastOne`] is what a caller gets without saying anything, because a run that exercised nothing is not a run that passed.
/// Admitting zero is declared in advance, with the reason attached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionExpectation {
    /// The selection is expected to name at least one row of the denominator.
    AtLeastOne,
    /// The selection may name no row at all, for the stated reason.
    AllowEmpty(EmptySelectionReason),
}

/// What one run's selection matched, read against what that run expected.
///
/// An empty selection is not a trial that failed and not a harness that broke, so it is stated once here rather than as a census entry nobody ran.
/// No arm spells "passed": a run that exercised nothing has nothing to pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionOutcome {
    /// The selection named at least one row of the denominator, which every expectation admits.
    Satisfied,
    /// The selection named no row, and the run expected at least one: it exercised nothing it meant to exercise.
    UnsatisfiedByEmptySelection,
    /// The selection named no row, and the caller stated in advance that zero is a lawful answer, for this reason.
    EmptyAsStated(EmptySelectionReason),
}

/// One run's complete-table accounting: the denominator, what happened to every row of it, the table posture, the selection's own outcome, the invocation profile, and the target binding.
///
/// The denominator is the descriptor table itself, one entry per row whether selected or not, which is what makes claim coverage a computation rather than a hand count.
/// Recording the posture is what lets coverage admit authored reports only and lets the comparison refuse a cross-posture pair.
/// Recording the selection outcome is what lets a run that selected nothing still be a complete report rather than an absent one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReport {
    census: Vec<TrialAccounting>,
    posture: TablePosture,
    selection: SelectionOutcome,
    invocation: InvocationProfile,
    target: TargetBinding,
}

// The comparison.

/// Why no previous report is available to compare against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoBaselineReason {
    /// No previous run's report was kept.
    NotRecorded,
    /// A previous report exists but could not be read back.
    Unreadable,
}

/// The typed baseline a comparison is taken against.
///
/// Typed rather than optional, because an optional input cannot tell a first run from a lost report, and a result may not claim knowledge absent from its input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Baseline<'previous> {
    /// A previous report, borrowed for the comparison.
    Previous(&'previous RunReport),
    /// There is no previous report because this is the first run.
    FirstRun,
    /// There is no previous report, for a stated reason.
    Unavailable(NoBaselineReason),
}

/// Why a comparison was not taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotComparedReason {
    /// This was the first run, so there was nothing to compare against.
    FirstRun,
    /// No baseline was available, for a stated reason.
    Unavailable(NoBaselineReason),
    /// The two reports stand over different table postures, and comparing them would let a staged view's numbers pass as an authored world's.
    PostureMismatch {
        /// The baseline's posture.
        left: TablePosture,
        /// The current report's posture.
        right: TablePosture,
    },
}

/// Which direction the denominator moved between two runs.
///
/// A shrinking census is a typed fact rather than a smaller number a reader might not notice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CensusDirection {
    /// The denominator grew.
    Grew,
    /// The denominator held.
    Unchanged,
    /// The denominator shrank.
    Shrank,
}

/// How the denominator moved between two runs, with both counts kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CensusDelta {
    before: usize,
    after: usize,
    direction: CensusDirection,
}

crate::report::declare_change_pair! {
    /// One trial present in both runs whose authored row was edited between them.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RowRevisionChange {
        context { trial: TrialId, }
        value: RowRevisionId,
    }
}

crate::report::declare_change_pair! {
    /// One trial present in both runs whose subject or check revision standing moved.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ExecutionRevisionChange {
        context { trial: TrialId, }
        value: ExecutionRevisions,
    }
}

crate::report::declare_change_pair! {
    /// How the conclusion-relevant invocation profile moved between two runs.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct InvocationProfileChange {
        context {}
        value: InvocationProfile,
    }
}

crate::report::declare_change_pair! {
    /// How the exact target and toolchain pair moved between two runs.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TargetBindingChange {
        context {}
        value: TargetBinding,
    }
}

/// The normalized outcome of one row of the denominator.
///
/// Enough to say that something flipped, and deliberately not enough to be mistaken for the record itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutcomeClass {
    /// Executed, and the check was satisfied.
    Passed,
    /// Executed, and the check refused with this normalized class.
    Refused(FailureClass),
    /// Selected but not executed, for this reason.
    Skipped(SkipReason),
    /// Selected and stopped at its time budget.
    TimedOut,
    /// Selected, and the harness failed around it.
    InfrastructureFailed(InfrastructureFault),
    /// Not selected, for this reason.
    NotSelected(NotSelectedReason),
}

crate::report::declare_change_pair! {
    /// One trial whose outcome differs between the two runs.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ConclusionFlip {
        context { trial: TrialId, }
        value: OutcomeClass,
    }
}

/// The table-population half of a report difference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReportPopulationDiff {
    added: Vec<TrialId>,
    removed: Vec<TrialId>,
    revised: Vec<RowRevisionChange>,
    census: CensusDelta,
}

/// The execution-standing half of a report difference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReportExecutionDiff {
    revisions: Vec<ExecutionRevisionChange>,
    flips: Vec<ConclusionFlip>,
    invocation: Option<InvocationProfileChange>,
    target: Option<Box<TargetBindingChange>>,
}

/// The declared population and execution-standing comparison reading between two reports.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReportDiff {
    population: ReportPopulationDiff,
    execution: ReportExecutionDiff,
}

/// The outcome of one comparison: a difference, or an honest refusal to compare.
///
/// A census never reads "no change" merely because there was nothing to compare against; that absence is the second arm, with its reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportComparison {
    /// The two reports were compared, and this is the difference.
    Compared(ReportDiff),
    /// No comparison was taken, for a stated reason.
    NotCompared(NotComparedReason),
}

// The coverage reading.

/// Why a coverage reading refused its input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoverageRefusal {
    /// The report stands over a staged view, and coverage admits authored-posture reports only.
    StagedPosture {
        /// The authored parent the staged view named.
        parent: AuthoredTableName,
    },
}

/// One claim's exercise count over the denominator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClaimExercise {
    claim: ClaimRef,
    exercised: usize,
    unexercised: usize,
}

/// What a run exercised, per claim, over the denominator it recorded.
///
/// It counts exercise and never correctness: a claim every one of whose trials ran and refused is fully exercised, and what those trials concluded is the report's to state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClaimCoverage {
    entries: Vec<ClaimExercise>,
}

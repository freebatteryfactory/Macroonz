//! The record vocabulary's public types: the identity rails, the execution
//! key, the replay account, the findings, the reports, and the comparison.
//!
//! Declarations only. Every road that reaches a private field is in this
//! module's own child `type_guard.rs`; the canonical preimages are in
//! `encode.rs`; the posture readings, the comparison, and the coverage reading
//! are their own pure-function modules.

use crate::descriptor::{
    AuthoredTableName, CanonicalRowBytes, CheckRef, ClaimRef, GeneratedSupportSchemaId,
    PopulationRef, SubjectRoute, TablePosture,
};
use crate::identity::{ContentAddress, DomainTag};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The semantic rail.
// ---------------------------------------------------------------------------

/// The profile coordinate of a trial's semantic identity.
///
/// The sole lawful value states what is true of every trial the harness runs:
/// nothing splits trials by feature profile, so no trial carries one. It is an
/// honest present-tense value rather than a default standing in for a choice
/// nobody has made — a fictional `Default` here would mint identities under a
/// coordinate the harness cannot vary and could not honour.
///
/// # Bounds
///
/// The first real feature split adds variants beside this one. Adding a variant
/// leaves every identity derived under [`TrialProfile::Unprofiled`] with its
/// name, because the variant's encoded slot does not move.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TrialProfile {
    /// The trial is not profiled.
    Unprofiled,
}

/// The COMPLETE preimage one [`TrialId`] is derived from: the five semantic
/// coordinates of one trial.
///
/// A trial's identity is what it MEANS — the claim it serves, the subject it
/// exercises, the check contract that judges it, the population that supplies
/// its inputs, and its profile. Nothing about where it lives is here, which is
/// why the identity survives a file move, a module move, and a rename.
///
/// The mechanism coordinate is the check CONTRACT the check reference names,
/// which is what a [`CheckRef`] is — a typed selection of a judging contract,
/// never a function pointer and never a path. The check's REVISION is a
/// different fact and rides [`ExecutionKey`].
///
/// # Construction
///
/// [`TrialCoordinates::of_key`] is the road from a descriptor row: the
/// descriptor-side trial key carries the four semantic references, and the
/// profile is the coordinate this home adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrialCoordinates {
    claim: ClaimRef,
    subject: SubjectRoute,
    mechanism: CheckRef,
    population: PopulationRef,
    profile: TrialProfile,
}

/// The domain tag every trial identity is derived under.
pub const TRIAL_IDENTITY_TAG: DomainTag = DomainTag::declared("trial-identity");

/// One trial's semantic identity.
///
/// # Authority
///
/// Holding one means the harness derived these thirty-two bytes from a complete
/// [`TrialCoordinates`] under [`TRIAL_IDENTITY_TAG`], and would derive the same
/// ones again from the same coordinates anywhere. Two rows with one identity
/// are two spellings of one measurement, which is why the table constructor
/// refuses the pair.
///
/// # Nonclaims
///
/// It says nothing about where the trial is written. A path-spelled name is a
/// [`TrialSite`], and a report joins both rails without ever mixing them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrialId(ContentAddress);

/// Where one trial is written: the diagnostic rail.
///
/// This is what a person filters on, jumps to, and reads in a failure — and it
/// is deliberately not identity. A refactor that moves the trial changes every
/// field here and changes no [`TrialId`], which is the whole point of keeping
/// the two rails apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrialSite {
    module_path: &'static str,
    file: &'static str,
    line: u32,
    name: &'static str,
}

// ---------------------------------------------------------------------------
// The revision rail.
// ---------------------------------------------------------------------------

/// The domain tag every row revision identity is derived under.
pub const ROW_REVISION_TAG: DomainTag = DomainTag::declared("row-revision");

/// The identity of one complete authored row.
///
/// It owns bookkeeping — census, aggregation, report diff. A suite-tag or
/// origin edit moves it, aggregation recomputes over the new value, and no
/// execution is owed by the move: nothing about what the row EXECUTES has
/// changed.
///
/// # Construction
///
/// Deriving it is TOTAL. The preimage is the row's own
/// [`CanonicalRowBytes`](crate::descriptor::CanonicalRowBytes), written where
/// the row was born, so by the time a report names a row the bytes exist and
/// hashing them cannot fail. A row whose bytes could not be written is a row
/// that was never constructed, and no census entry can be stated over one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RowRevisionId(ContentAddress);

/// The identity of the exact subject revision one attachment binds.
///
/// # Authority
///
/// The value is the attachment's own revision binding, given a name that cannot
/// be confused with the check's. Nothing is derived here: the binding is the
/// authority on what revision was committed to and under which posture, and a
/// second derivation in this home would be a second answer to that question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubjectRevisionId(ContentAddress);

/// The identity of the exact check revision one attachment binds.
///
/// The check's CONTRACT is a coordinate of [`TrialId`]; this is the
/// implementation standing behind that contract, and the two move
/// independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CheckRevisionId(ContentAddress);

// ---------------------------------------------------------------------------
// The execution rail.
// ---------------------------------------------------------------------------

/// The compilation target a run stood on, by its declared triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TargetTriple(String);

/// The toolchain a run stood on, by its declared identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToolchainIdentity(String);

/// The target and toolchain one execution actually ran under.
///
/// # Authority
///
/// These are host facts, read by the harness because it needs them to run, and
/// they enter [`ExecutionKey`] — a cache key — rather than any semantic
/// identity. A cross-target cache hit is a claim nothing verifies, and refusing
/// it costs reruns: cost, never truth.
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
/// A declared bound, not a measurement: what a run actually spent is a
/// [`RecordedDuration`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeBudget(u64);

/// The invocation's conclusion-relevant facts.
///
/// # Authority
///
/// This is the declared typed subset of an invocation whose values can change
/// what a trial CONCLUDES, which is why it rides [`ExecutionKey`]: a result
/// reached under a smaller budget is not evidence for a larger one. The set is
/// closed — a fact that can move a conclusion is a field here, and adding one
/// is a law change rather than a new argument.
///
/// # Nonclaims
///
/// It is NOT the profile coordinate of [`TrialId`]. That coordinate says which
/// feature profile a trial MEANS; this one says what one invocation permitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvocationProfile {
    cases: CaseBudget,
    bytes: ByteBudget,
    time: TimeBudget,
}

/// The domain tag every execution key is derived under.
pub const EXECUTION_KEY_TAG: DomainTag = DomainTag::declared("execution-key");

/// What one execution of one trial was actually keyed by.
///
/// The trial's semantic identity, the subject and check revisions, the
/// invocation profile, and the target and toolchain binding — the last
/// UNCONDITIONALLY, because a hit across targets asserts something nothing
/// verified.
///
/// # Construction
///
/// [`ExecutionKey::over`] is the only road, and the key's address is derived
/// from the parts on demand rather than stored beside them, so there is no
/// second value that could disagree with what the key is made of.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecutionKey {
    trial: TrialId,
    subject: SubjectRevisionId,
    check: CheckRevisionId,
    invocation: InvocationProfile,
    target: TargetBinding,
}

// ---------------------------------------------------------------------------
// The postures: what an attachment's revision bindings buy.
// ---------------------------------------------------------------------------

/// Whether a rerun cache may stand in for executing an attachment again.
///
/// # Authority
///
/// This vocabulary and the reading that produces it are the one owning
/// statement about cache eligibility. Every other mention of eligibility,
/// anywhere in the harness, points here rather than restating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheEligibility {
    /// Fully eligible: the revisions are derived from owned declarations, so a
    /// hit stands on the same ground a fresh execution would.
    Eligible,
    /// Eligible only while the author's declared revisions are unchanged.
    ///
    /// The ceiling is the author's word: nothing mechanical establishes that
    /// the declared revision still describes the code, so a hit inherits
    /// exactly the confidence the declaration carries and no more.
    EligibleWhileDeclaredRevisionsUnchanged,
    /// Never eligible: no stable commitment exists, so every run executes.
    NeverEligible,
}

/// What a reproduction of one execution can claim.
///
/// # Authority
///
/// The posture is the MEET's image — the weakest of the attachment's two
/// revision bindings decides it — so a mixed attachment can never mint an
/// exact-replay claim over an author's-word check revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplayPosture {
    /// The one posture that earns the phrase "replay exactly": both revisions
    /// are derived from owned declarations, so the reproduction is the same
    /// execution and not a similar one.
    ExactDerived,
    /// Reproduction inherits the author's-word ceiling: the run is reproduced
    /// against revisions a hand committed to, and the claim is exactly as good
    /// as that commitment.
    DeclaredByAuthor,
    /// No exact reproduction is available. The historical run and its input are
    /// real evidence, the attachment always reruns, no cache hit is permitted,
    /// and every rendering states that reproduction is non-exact.
    UnavailableBecauseUntracked,
}

// ---------------------------------------------------------------------------
// The replay account.
// ---------------------------------------------------------------------------

/// Which generation profile produced a capsule's input, and at which version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenerationProfile {
    name: &'static str,
    version: u32,
}

/// Which minimization profile reduced a capsule's input, and at which version.
///
/// A find is minimized under a profile, and a reduction that preserves the
/// fingerprint under one profile says nothing about another — so the profile
/// and its version ride the capsule rather than being remembered by whoever
/// reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MinimizationProfile {
    name: &'static str,
    version: u32,
}

/// The domain tag every replay capsule identity is derived under.
pub const REPLAY_CAPSULE_TAG: DomainTag = DomainTag::declared("replay-capsule");

/// One reproduction account: everything a second run needs, and the ceiling of
/// what reproducing it proves.
///
/// It binds the execution key, the exact input bytes, the generation and
/// minimization profiles with their versions, the generated-support schema
/// identity, and the replay posture the attachment's meet produced.
///
/// # Nonclaims
///
/// Holding a capsule is not holding an exact-replay claim. Only
/// [`ReplayPosture::ExactDerived`] earns that phrase; the other two postures
/// state their own ceilings, and a rendering that omits them is renaming
/// evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCapsule {
    key: ExecutionKey,
    input: Vec<u8>,
    generation: GenerationProfile,
    minimization: MinimizationProfile,
    schema: GeneratedSupportSchemaId,
    posture: ReplayPosture,
}

// ---------------------------------------------------------------------------
// Findings.
// ---------------------------------------------------------------------------

/// The normalized shape of a failure.
///
/// Normalized on purpose: it is what turns many finds into few defects, so it
/// names the KIND of disagreement rather than any of its particulars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FailureClass {
    /// The check returned a typed refusal about the subject.
    RefusedByCheck,
    /// An algebraic property's law disagreed with the subject.
    PropertyDisagreement,
    /// The independent oracle disagreed with the subject.
    OracleDisagreement,
    /// A panic from the subject, caught at the trial boundary and recorded as
    /// the finding it is.
    SubjectPanic,
    /// The subject exceeded a declared budget while the check was still
    /// undecided.
    BudgetExhausted,
}

/// The typed cause a finding names, cited by the declared identity pair its
/// home wrote down.
///
/// A finding cites a cause the way the machine spells one — a family and a
/// local key — rather than carrying prose. Free text about a cause is
/// [`ForeignText`] and decides nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FindingCause {
    family: &'static str,
    local: &'static str,
}

/// Where a refusal was raised.
///
/// This is the finding's own location, which is not the trial's: a property
/// suite refuses inside itself while the trial lives in a table somewhere else,
/// and a reader needs both.
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
    /// The material exceeded the bound and was cut, with both counts kept so a
    /// reader is never shown a cut rendering that looks whole.
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
    /// The admitted bytes are not valid UTF-8; rendering substitutes
    /// replacement characters.
    LossyReplacement,
}

/// Text that came from outside this crate's own vocabulary.
///
/// A subject's panic payload, an external tool's output, a decoder's message:
/// bounded, marked when it was cut, marked when rendering it loses bytes, and
/// carried on its own field so that a finding is a typed value first and prose
/// second.
///
/// # Nonclaims
///
/// Nothing in the harness reads this back, matches on it, or decides from it,
/// and no composed summary is built from it. A reading that needs a fact takes
/// the typed field that carries the fact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForeignText {
    bytes: Vec<u8>,
    truncation: Truncation,
    fidelity: TextFidelity,
}

/// One typed refusal: what disagreed, which cause it names, where it was
/// raised, and any foreign text it carried in.
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
pub const FINGERPRINT_TAG: DomainTag = DomainTag::declared("failure-fingerprint");

/// One failure's identity: the trial's semantic identity joined with the typed
/// cause and the normalized failure class.
///
/// Four dividends fall out of naming a failure this way: the same failure found
/// in two runs deduplicates to one; a minimizer can reduce an input while
/// requiring the fingerprint to hold, so it shrinks the input without wandering
/// to a different bug; rerun selection stays stable across refactors, because
/// the semantic half of the name survives file and module moves; and many finds
/// group into few defects, because one defect keeps one name however many
/// inputs reach it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fingerprint {
    trial: TrialId,
    cause: FindingCause,
    class: FailureClass,
}

// ---------------------------------------------------------------------------
// One execution's record.
// ---------------------------------------------------------------------------

/// Why a selected trial did not execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkipReason {
    /// The invocation's budget was spent before this trial ran.
    BudgetExhausted,
    /// The subject is not exercisable on the running target.
    TargetUnsupported,
    /// Material the trial requires — generated support, a corpus, a fault
    /// adapter — was not present.
    PrerequisiteAbsent,
    /// A cached execution under this trial's execution key stood in, which is
    /// lawful exactly when [`CacheEligibility`] admitted it.
    SatisfiedByCachedExecution,
}

/// What failed in the HARNESS rather than in the subject.
///
/// Kept apart from every conclusion on purpose: an infrastructure failure is
/// not evidence about the subject, and folding one into a refusal would put a
/// verdict on a subject nobody exercised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfrastructureFault {
    /// The population could not supply inputs.
    GenerationUnavailable,
    /// Generated support the trial binds was absent or unreadable.
    SupportAbsent,
    /// The harness could not record the execution's own evidence.
    CaptureFailed,
}

/// How long an execution took, in nanoseconds, as the caller recorded it.
///
/// # Authority
///
/// A recorded fact and never a reading: nothing in this crate consults a clock,
/// so a duration arrives from the caller that measured it or does not arrive at
/// all. Timing is reported, never concluded from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordedDuration(u64);

/// What became of one selected trial.
///
/// The conclusion rides the executed arm, so a conclusion without an execution
/// and an execution without a conclusion are both unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunAttempt {
    /// It ran, and this is what it concluded.
    Executed(TrialConclusion),
    /// It did not run, for a stated reason.
    SkippedWithReason(SkipReason),
    /// It ran past the budget it was given, which is carried so a reader knows
    /// which bound was reached.
    TimedOut(TimeBudget),
    /// The harness failed around it, so nothing was learned about the subject.
    InfrastructureFailed(InfrastructureFault),
}

/// One execution's record: the two identity rails joined, what became of the
/// attempt, and how long it took.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrialReport {
    trial: TrialId,
    site: TrialSite,
    attempt: RunAttempt,
    elapsed: RecordedDuration,
}

// ---------------------------------------------------------------------------
// One run's complete-table accounting.
// ---------------------------------------------------------------------------

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
/// # Construction
///
/// The selected arm carries the whole execution record behind a box: a census
/// is mostly rows one invocation did not select, and an unboxed arm would make
/// every one of them as large as the largest report.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SelectionDisposition {
    /// Selected, and here is what happened.
    Selected(Box<TrialReport>),
    /// Not selected, for a stated reason.
    NotSelected {
        /// Why the selection passed it over.
        reason: NotSelectedReason,
    },
}

/// Whether one row of the denominator was actually exercised.
///
/// Selection is not exercise: a trial the invocation selected and then skipped
/// was not exercised, and a coverage reading that counted it would be claiming
/// evidence nobody produced.
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
    trial: TrialId,
    row: RowRevisionId,
    claim: ClaimRef,
    disposition: SelectionDisposition,
}

/// Why a caller stated in advance that a selection matching nothing is a lawful
/// answer.
///
/// # Authority
///
/// Typed and closed, because an escape from the anti-vacuity law has to be
/// readable: a reason nobody can enumerate is a reason nobody can review. Free
/// text has no seat here — a rendering of one is [`ForeignText`], and it decides
/// nothing.
///
/// # Nonclaims
///
/// A reason states why zero was admissible. It states nothing about whether the
/// run was worth taking, and it never claims that anything was exercised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmptySelectionReason {
    /// The selection was carried over from a previous run's report, so a trial
    /// the world no longer holds is a lawful absence rather than a failure.
    CarriedOverFromAPreviousRun,
    /// The run asks what the world holds under this selection. A claim with no
    /// row serving it is the reading's own finding — the strongest opening a
    /// coverage reading has — rather than a seat that could not run.
    AskingWhatTheWorldHolds,
}

/// What one invocation expects its selection to match.
///
/// # Authority
///
/// [`SelectionExpectation::AtLeastOne`] is the standing expectation of every
/// run, and it is what a caller gets without saying anything: a run that
/// exercised nothing is not a run that passed, and a selection that named no row
/// is the vacuity a harness exists to catch. The escape is DECLARED rather than
/// discovered — a caller that means to admit zero says so in advance and says
/// why.
///
/// It is this home's rather than the engine's for the reason
/// [`InvocationProfile`] is: it is a declared input whose value the record has to
/// carry, and the record vocabulary sits below the engine that reads it.
///
/// # Nonclaims
///
/// An expectation is not an outcome. What a run's selection actually matched,
/// read against this, is [`SelectionOutcome`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionExpectation {
    /// The selection is expected to name at least one row of the denominator.
    AtLeastOne,
    /// The selection may name no row at all, for the stated reason.
    AllowEmpty(EmptySelectionReason),
}

/// What one run's selection matched, read against what that run expected.
///
/// # Authority
///
/// A run-level fact, recorded on the report itself. An empty selection is not a
/// trial that failed and not a harness that broke — nothing was exercised, and
/// there is no row to hang either verdict on — so it is stated here, once, where
/// a reading can find it without inventing a census entry nobody ran.
///
/// # Nonclaims
///
/// It says nothing about what any trial concluded. It is also not
/// [`OutcomeClass`], which normalizes what happened to ONE row; this is what
/// happened to the selection as a whole. No arm of it spells "passed": a run
/// that exercised nothing has nothing to pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionOutcome {
    /// The selection named at least one row of the denominator, which is what
    /// every expectation admits.
    Satisfied,
    /// The selection named no row of the denominator, and the run expected at
    /// least one: it exercised nothing it meant to exercise.
    UnsatisfiedByEmptySelection,
    /// The selection named no row of the denominator, and the caller stated in
    /// advance that zero is a lawful answer, for this reason. Zero work was
    /// done, and this arm is how a reading says so without saying anything ran.
    EmptyAsStated(EmptySelectionReason),
}

/// One run's complete-table accounting: the denominator, what happened to every
/// row of it, the table posture, the selection's own outcome, and the invocation
/// profile it ran under.
///
/// # Authority
///
/// The denominator is the descriptor table itself — one entry per row, always,
/// selected or not — which is what makes claim coverage a computation rather
/// than a hand count. The posture is the view's own
/// ([`TablePosture`]), recorded rather than restated: coverage admits the
/// authored arm only, and the comparison refuses a cross-posture pair.
///
/// The selection outcome is the run-level fact a census cannot carry: whether
/// the selection matched anything, read against what the run expected of it.
/// Recording it here is what lets a run that selected nothing still be a
/// COMPLETE report rather than an absent one.
///
/// # Nonclaims
///
/// Trial uniqueness across the census is the table constructor's refusal, not
/// this seat's. Restating it here would keep passing after the stronger seat
/// was removed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReport {
    census: Vec<TrialAccounting>,
    posture: TablePosture,
    selection: SelectionOutcome,
    invocation: InvocationProfile,
}

// ---------------------------------------------------------------------------
// The comparison.
// ---------------------------------------------------------------------------

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
/// # Authority
///
/// Typed rather than optional, because an optional input cannot distinguish two
/// absences: a first run and a lost report are different facts, and a result
/// may never claim knowledge absent from its input.
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
    /// The two reports stand over different table postures. Comparing them
    /// would let a staged run's numbers pass as an authored world's.
    PostureMismatch {
        /// The baseline's posture.
        left: TablePosture,
        /// The current report's posture.
        right: TablePosture,
    },
}

/// Which direction the denominator moved between two runs.
///
/// A shrinking census is a typed fact rather than a smaller number a reader
/// might not notice: rows leaving the world is exactly the change a comparison
/// exists to state.
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

/// One trial present in both runs whose authored row was edited between them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowRevisionChange {
    trial: TrialId,
    before: RowRevisionId,
    after: RowRevisionId,
}

/// The normalized outcome of one row of the denominator.
///
/// The comparable projection of a [`SelectionDisposition`]: enough to say that
/// something flipped, and deliberately not enough to be mistaken for the
/// record itself.
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

/// One trial whose outcome differs between the two runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConclusionFlip {
    trial: TrialId,
    before: OutcomeClass,
    after: OutcomeClass,
}

/// The pure difference between two reports.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReportDiff {
    added: Vec<TrialId>,
    removed: Vec<TrialId>,
    revised: Vec<RowRevisionChange>,
    flips: Vec<ConclusionFlip>,
    census: CensusDelta,
}

/// The outcome of one comparison: a difference, or an honest refusal to
/// compare.
///
/// The census never reads "no change" merely because there was nothing to
/// compare against — that absence is the second arm, with its reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportComparison {
    /// The two reports were compared, and this is the difference.
    Compared(ReportDiff),
    /// No comparison was taken, for a stated reason.
    NotCompared(NotComparedReason),
}

// ---------------------------------------------------------------------------
// The coverage reading.
// ---------------------------------------------------------------------------

/// Why a coverage reading refused its input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoverageRefusal {
    /// The report stands over a staged view. Coverage admits authored-posture
    /// reports only, so a candidate run never enters the numbers a gate reads.
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
/// # Nonclaims
///
/// It counts exercise, never correctness: a claim every one of whose trials ran
/// and refused is fully exercised. What those trials concluded is the report's
/// to state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClaimCoverage {
    entries: Vec<ClaimExercise>,
}

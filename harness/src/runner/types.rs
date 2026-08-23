//! The runner's declarations: the typed invocation both engine calls take, what
//! one invocation chooses from the complete world, the concrete spellings of
//! the generic seam this home instantiates, and the one refusal a seat answers
//! with.
//!
//! Declarations only. The roads that build an invocation and hand its seats back
//! are this file's own child, `type_guard.rs`, so an invocation is born in one
//! place. The selection itself has no nucleus at all: every arm is a set over a
//! shape the rows already carry, so there is no invariant a constructor could
//! establish that the set itself does not. What a run EXPECTS of that selection
//! does have one, because the standing expectation is the one a caller gets for
//! saying nothing and the escape from it is a statement somebody makes on
//! purpose.
//!
//! # The generic seam
//!
//! The descriptor home declares its attachment over two type parameters because
//! it sits below the record vocabulary and may not import a record type. This
//! home sees both vocabularies, so this is where the parameters are
//! instantiated: the invocation facts are [`Invocation`] and the conclusion is
//! [`TrialConclusion`]. The aliases below are that instantiation written once,
//! so a caller building a table never spells the parameters by hand.
//!
//! # The seat's vocabulary
//!
//! A stamped seat is a test function returning a `Result`, so it needs one type
//! to refuse with. [`SeatRefusal`] is it, [`SeatOutcome`] is what the reading
//! answers with when it does not refuse, and both readings are `verdict.rs`'s:
//! the fold from a report to a verdict lives in this home once, rather than
//! being written into every expansion that wants one.

use crate::clock::HarnessClock;
use crate::descriptor::{
    AuthoredTable, Binding, ClaimRef, ExecutionSuite, SubjectRoute, TableView, TrialTableRefusal,
};
use crate::report::{
    EmptySelectionReason, FindingCause, InfrastructureFault, InvocationProfile,
    SelectionExpectation, SkipReason, TargetBinding, TimeBudget, TrialConclusion, TrialFinding,
    TrialId, TrialSite,
};
use std::collections::BTreeSet;

#[path = "type_guard.rs"]
mod guard;

/// The typed invocation both engine calls take: the budgets a check reads, the
/// host facts the run stands on, the site its reports are written at, and the
/// caller's clock.
///
/// # Authority
///
/// The engine reads this value and its other parameters and nothing else: there
/// is no argument vector, no environment, no clock of its own, and no output
/// stream anywhere in this home. A hosting fact reaches a run because a caller
/// declared it here.
///
/// # Nonclaims
///
/// The site states where the INVOCATION was written. On the stamped road that
/// is the row's own named test function and the two coincide; from an aggregate
/// seat every report this invocation produces carries the seat's site, because
/// a descriptor row carries no site of its own for the engine to read.
///
/// The budgets are the check's to honour. This value hands them to the callable
/// and reads nothing back from them: a bound that was exceeded is a conclusion
/// the check states, never one the engine infers from a measurement.
#[derive(Debug, Clone)]
pub struct Invocation {
    profile: InvocationProfile,
    target: TargetBinding,
    site: TrialSite,
    clock: HarnessClock,
}

/// What one invocation chooses FROM the complete world.
///
/// # The arms
///
/// Every arm is a set over a shape a row already carries, so a selection joins
/// on the table itself and no second index exists to disagree with it.
/// [`Selection::All`] is the world as the view presents it — the aggregate
/// seat's ordinary run. [`Selection::ByClaim`] is the claim-scoped run a
/// coverage reading or a proof gap asks for. [`Selection::ByExecutionSuite`] is
/// the seat's own arm: a row runs under exactly one suite, and a row outside
/// the invocation's suites is passed over as
/// [`NotSelectedReason::SuiteNotRun`](crate::report::NotSelectedReason::SuiteNotRun).
/// [`Selection::ByTrialIds`] names trials by semantic identity — the named
/// lens, the rerun subset, the batch a caller carried over from a previous
/// report. [`Selection::BySubjectRoute`] is the mutant-scoped shape: a mutation
/// target maps to the route it lives on, and the rows that exercise that route
/// are the ones worth running.
///
/// # Nonclaims
///
/// A selection narrows a RUN and never the denominator: the report is stated
/// over every row of the world however few of them this invocation named. An
/// empty roster is lawful and selects nothing, and the census still carries
/// every row with its stated not-selected reason.
///
/// What a run EXPECTS its selection to match is not here. That is
/// [`SelectionPlan`]'s, because it is a fact about the run rather than about
/// which rows a roster names — two runs can choose identically and expect
/// differently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// Every trial the view presents.
    All,
    /// The trials whose row serves one of these claims.
    ByClaim(BTreeSet<ClaimRef>),
    /// The trials whose row runs under one of these aggregate seats.
    ByExecutionSuite(BTreeSet<ExecutionSuite>),
    /// The trials these semantic identities name.
    ByTrialIds(BTreeSet<TrialId>),
    /// The trials whose row exercises one of these subject routes.
    BySubjectRoute(BTreeSet<SubjectRoute>),
}

/// What one invocation chooses from the complete world, and what it expects
/// that choice to match.
///
/// # Authority
///
/// The engine takes this rather than a bare [`Selection`], so every run states
/// its anti-vacuity posture and no run is missing one. The expectation itself is
/// the record vocabulary's ([`SelectionExpectation`]), because the report has to
/// carry the answer it is read against.
///
/// # Construction
///
/// [`SelectionPlan::of`] is the ordinary road and it asks for nothing beyond the
/// selection: a run expects at least one match unless somebody says otherwise,
/// so the standing expectation costs a caller no ceremony at all.
/// [`SelectionPlan::allowing_empty`] is the escape, and it is the only road that
/// admits zero — a caller taking it states the reason in the same call.
///
/// # Nonclaims
///
/// A plan states what a run means to do. It never narrows the denominator, and
/// admitting an empty selection admits exactly that and nothing more: no arm of
/// it says a trial passed, because no trial ran.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionPlan {
    chooses: Selection,
    expects: SelectionExpectation,
}

/// Why a host-authored trial record was not admitted as report evidence.
///
/// # Authority
///
/// The runner joins host input to the complete table and the selection plan. Each cause names the first relation that did not hold; facts the runner derives rather than accepts have no refusal arm here.
#[must_use = "a refusal is the reason a host record was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportRecordingRefusal {
    /// The one-binding road was handed a record naming another trial.
    TrialMismatch {
        /// The trial the binding declares.
        expected: TrialId,
        /// The trial the host record names.
        recorded: TrialId,
    },
    /// Two host records name one trial.
    DuplicateHostRecord(TrialId),
    /// A host record names no trial in the complete table view.
    TrialOutsideTable(TrialId),
    /// A host record names a table row this selection did not select.
    RecordForUnselectedTrial(TrialId),
    /// The selection named a trial for which the host supplied no record.
    MissingSelectedRecord(TrialId),
}

/// The callable one executable attachment carries at the types this engine
/// runs.
///
/// A capture-free function pointer: invocation facts in, one conclusion out.
///
/// The type excludes captured state. Rust's function-pointer type does not establish semantic purity or termination, so callers do not acquire either claim from this alias.
pub type TrialCall = fn(&Invocation) -> TrialConclusion;

/// One row married to its callable, at the types this engine runs.
pub type TrialBinding = Binding<Invocation, TrialConclusion>;

/// The complete authored world, at the types this engine runs.
pub type TrialTable = AuthoredTable<Invocation, TrialConclusion>;

/// The sealed read surface an authored table and a staged view both present, at
/// the types this engine runs.
pub type TrialTableView<'view> = TableView<'view, Invocation, TrialConclusion>;

/// The typed cause every caught subject panic is cited under.
///
/// The pair is this home's declaration, so a fingerprint over a subject panic
/// names the boundary that caught it rather than any of the panic's own words.
pub const SUBJECT_PANIC_CAUSE: FindingCause = FindingCause::named("runner", "subject-panic");

/// What one selected trial did instead of concluding lawfully.
///
/// # Authority
///
/// A satisfied check has no arm here, so "this is why the seat refused" is
/// unsayable about a trial that passed. Every arm carries a typed value lifted
/// straight out of the record the run wrote — the check's own finding, the
/// stated skip reason, the declared budget that was reached, the harness fault
/// — so a seat describes a failure by CARRYING it. Nothing here reads a
/// rendered sentence, and nothing here is matched on prose.
///
/// # Nonclaims
///
/// It says nothing about the subject beyond what the record already said. The
/// harness-fault arm in particular is not evidence about the subject at all: it
/// states that nothing was learned, which is why it refuses a seat rather than
/// passing one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeatFailure {
    /// The check refused, and the refusal carries its own evidence.
    CheckRefused(TrialFinding),
    /// The trial was selected and did not run, for a stated reason.
    ///
    /// Every skip reason lands here, [`SkipReason::SatisfiedByCachedExecution`]
    /// included: the conclusion a cached execution stood in for is not in the
    /// report being read, so a seat that passed on it would be passing on a
    /// verdict it never saw.
    NotRun(SkipReason),
    /// The trial ran past the budget it was given, which is carried so a reader
    /// knows which bound was reached.
    PastTimeBudget(TimeBudget),
    /// The harness failed around the trial, so nothing was learned about the
    /// subject.
    HarnessFailed(InfrastructureFault),
}

/// One selected trial that did not conclude lawfully: both identity rails, and
/// what it did instead.
///
/// # Authority
///
/// The two rails ride together because a reader needs both and neither stands
/// in for the other — the semantic identity is the name the failure keeps
/// across a refactor, and the site is the spelling a person filters on and
/// jumps to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailedTrial {
    trial: TrialId,
    site: TrialSite,
    failure: SeatFailure,
}

/// What one aggregate seat's reading found when it did not refuse.
///
/// # Authority
///
/// The two arms are two different facts and neither stands in for the other: a
/// run that exercised every trial it named, and a run that deliberately
/// exercised nothing. Naming them apart is what keeps the second from being read
/// as the first — nothing here spells "passed", because a run under an admitted
/// empty selection has no conclusion to pass on, and the reason it was admitted
/// rides with it so a reader is never shown a silent zero.
///
/// # Nonclaims
///
/// The satisfied arm says every SELECTED trial concluded lawfully. It says
/// nothing about the rows the selection passed over — the census states those,
/// with their reasons, and narrowing a run has never been an outcome.
#[must_use = "a seat's outcome states what the run did, and a run that did nothing says so"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeatOutcome {
    /// Every trial the selection named concluded lawfully.
    EveryTrialConcluded {
        /// How many rows of the denominator the selection named.
        selected: usize,
        /// How many rows the run was stated over.
        denominator: usize,
    },
    /// The selection named no row, exactly as the caller stated in advance that
    /// it might. Nothing was exercised, and the stated reason is why that was
    /// admissible.
    NoWorkAsStated {
        /// The reason the caller stated for admitting an empty selection.
        reason: EmptySelectionReason,
        /// How many rows the run was stated over.
        denominator: usize,
    },
}

/// The seats' one refusal type: everything a stamped test function answers with
/// instead of passing.
///
/// # Authority
///
/// This is the typed failure channel the stamped road itself writes. A construction refusal enters unchanged through this type's [`From`] road over [`TrialTableRefusal`], and the run's own verdict supplies the other arms. Caller-authored row expressions and caller-supplied target and subject functions retain their own effect and unwind ceilings; harness-clock source unwind is owned separately by the clock reading and never enters this refusal family.
///
/// # Bounds
///
/// `Debug` is the entire rendering surface, deliberately. The test harness that
/// hosts a seat prints the returned value with `Debug`, and a `Display` written
/// here would be a second vocabulary for facts the typed fields already carry.
#[must_use = "a refusal is the reason a seat did not pass"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeatRefusal {
    /// The world could not be built, and this is the construction that refused.
    ///
    /// A row whose canonical bytes could not be written arrives here too: the
    /// bytes are written where the row is born, so a row that cannot be encoded
    /// is a row the table was never able to hold.
    TableNotBuilt(TrialTableRefusal),
    /// The selection named no row of the denominator, and the run expected at
    /// least one, so it exercised nothing it meant to exercise.
    ///
    /// A run that exercised nothing is not a run that passed. The fact is the
    /// run's own — the report records it as
    /// [`SelectionOutcome::UnsatisfiedByEmptySelection`](crate::report::SelectionOutcome::UnsatisfiedByEmptySelection)
    /// — and this arm is where that fact reaches the channel a test function
    /// answers in. On the stamped road it is exactly the pairing a stamp cannot
    /// check: a suite group whose declared suite is no row's own.
    NothingSelected {
        /// How many rows the run was stated over.
        denominator: usize,
    },
    /// One trial did not conclude lawfully: the reading a named lens takes.
    ///
    /// The record rides behind a box because it is the largest thing this
    /// family carries, and an unboxed arm would make every refusal — a name
    /// that would not parse included — as large as the largest failure.
    TrialFailed(Box<FailedTrial>),
    /// Selected trials did not conclude lawfully: the reading an aggregate seat
    /// takes.
    RunFailed {
        /// Every selected trial that did not conclude lawfully, in census
        /// order.
        failed: Vec<FailedTrial>,
        /// How many rows of the denominator the selection named.
        selected: usize,
        /// How many rows the run was stated over.
        denominator: usize,
    },
}

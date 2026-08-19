//! The runner's declarations: the typed invocation both engine calls take, what
//! one invocation chooses from the complete world, the concrete spellings of
//! the generic seam this home instantiates, and the one refusal a seat answers
//! with.
//!
//! Declarations only. The roads that build an invocation and hand its seats back
//! are this file's own child, `type_guard.rs`, so an invocation is born in one
//! place. A selection has no nucleus at all: every arm is a set over a shape the
//! rows already carry, so there is no invariant a constructor could establish
//! that the set itself does not.
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
//! to refuse with. [`SeatRefusal`] is it, and the readings that produce it are
//! `verdict.rs`'s: the fold from a report to a verdict lives in this home once,
//! rather than being written into every expansion that wants one.

use crate::descriptor::{
    AuthoredTable, Binding, ClaimRef, EncodeRefusal, ExecutionSuite, SubjectRoute, TableView,
    TrialTableRefusal,
};
use crate::report::{
    FindingCause, InfrastructureFault, InvocationProfile, SkipReason, TargetBinding, TimeBudget,
    TrialConclusion, TrialFinding, TrialId, TrialSite,
};
use std::collections::BTreeSet;

#[path = "type_guard.rs"]
mod guard;

/// The caller's own road to a nanosecond reading.
///
/// # Authority
///
/// This engine consults no clock. A duration in a report is the difference of
/// two readings this function returned and nothing else, so the measurement is
/// the caller's fact rather than an ambient one the engine went and took.
///
/// # Bounds
///
/// A function pointer rather than a closure, so a clock carries no captured
/// state. The origin a reading counts from is the caller's own; only
/// differences are read, never the value itself. A caller with no measurement
/// to offer hands a reading that does not move, and every duration then reads
/// zero rather than a number nobody measured.
#[derive(Debug, Clone, Copy)]
pub struct HostClock(fn() -> u64);

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
    clock: HostClock,
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

/// The callable one executable attachment carries at the types this engine
/// runs.
///
/// A pure map: invocation facts in, one conclusion out.
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

/// The seats' one refusal type: everything a stamped test function answers with
/// instead of passing.
///
/// # Authority
///
/// A seat refuses; it does not panic. Because a seat is a test function
/// returning a `Result`, its failure is a returned typed value carrying its own
/// evidence, and this is the one family the whole stamped road ends in: a
/// construction refusal enters unchanged through this type's [`From`] road over
/// [`TrialTableRefusal`], and the run's own verdict is the other arms.
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
    TableNotBuilt(TrialTableRefusal),
    /// A row's canonical bytes could not be written, so the run's census could
    /// not name that row's revision and no report was stated.
    ///
    /// The cause is the descriptor home's own and is carried unchanged. It is
    /// unreachable on every target this crate is built for — the row encoder
    /// states its widths rather than guessing at one — and it is a refusal
    /// rather than a silence because a census entry under an identity derived
    /// from bytes nobody wrote would be two rows' bookkeeping under one name.
    RowNotEncoded(EncodeRefusal),
    /// The selection named no row of the denominator, so the run exercised
    /// nothing.
    ///
    /// A run that exercised nothing is not a run that passed. On the stamped
    /// road this is exactly the pairing a stamp cannot check — a suite group
    /// whose declared suite is no row's own — answered at run time from the
    /// census the run wrote.
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

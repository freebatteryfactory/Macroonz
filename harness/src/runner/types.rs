//! Every public type of the runner.
//!
//! Constructors and readers are this file's own child, `type_guard.rs`, so an invocation is born in one place and nothing reaches in afterwards.
//!
//! The descriptor home declares its table and its attachment over two type parameters, because it sits below the record vocabulary and may not import a record type.
//! This home sees both, so the parameters are pinned here once: the facts are [`Invocation`] and the conclusion is [`TrialConclusion`].

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

/// What one run stands on: the budgets a check reads, the host facts it was told, the site its reports are written at, and the caller's clock.
///
/// The engine reads this value and its other parameters and nothing else: no argument vector, no environment, no clock of its own, no output stream.
/// The site states where the invocation was written, not where a row was authored.
/// The budgets are the check's to honour, so a bound that was exceeded is a conclusion the check states rather than one the engine infers from a measurement.
#[derive(Debug, Clone)]
pub struct Invocation {
    profile: InvocationProfile,
    target: TargetBinding,
    site: TrialSite,
    clock: HarnessClock,
}

/// What one invocation chooses from the complete world.
///
/// Every arm is a set over a shape a row already carries, so a selection joins on the table itself and no second index exists to disagree with it.
/// A selection narrows a run and never the denominator: the report is stated over every row of the world, however few of them this invocation named.
/// What a run expects that choice to match is [`SelectionPlan`]'s, because two runs can choose identically and expect differently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// Every trial the view presents.
    All,
    /// The trials whose row serves one of these claims.
    ByClaim(BTreeSet<ClaimRef>),
    /// The trials whose row runs under one of these aggregate seats.
    ///
    /// A row outside them is passed over as [`NotSelectedReason::SuiteNotRun`](crate::report::NotSelectedReason::SuiteNotRun), which is a different fact from a row the selection simply did not name.
    ByExecutionSuite(BTreeSet<ExecutionSuite>),
    /// The trials these semantic identities name.
    ByTrialIds(BTreeSet<TrialId>),
    /// The trials whose row exercises one of these subject routes.
    BySubjectRoute(BTreeSet<SubjectRoute>),
}

/// A selection joined to what the run expects that selection to match.
///
/// The engine takes this rather than a bare [`Selection`], so every run states its anti-vacuity posture and no run is missing one.
/// [`SelectionPlan::allowing_empty`] is the only road that admits zero, and admitting it admits exactly that: no arm of it says a trial passed, because no trial ran.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionPlan {
    chooses: Selection,
    expects: SelectionExpectation,
}

/// Why a host-authored trial record was not admitted as evidence.
///
/// Each arm names the first relation that did not hold between the record and the binding, table, and selection it was joined to.
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

/// The callable one executable attachment carries, at the types this engine runs.
///
/// A capture-free function pointer, which excludes captured state and establishes neither semantic purity nor termination.
pub type TrialCall = fn(&Invocation) -> TrialConclusion;

/// One row married to its callable, at the types this engine runs.
pub type TrialBinding = Binding<Invocation, TrialConclusion>;

/// The complete authored world, at the types this engine runs.
pub type TrialTable = AuthoredTable<Invocation, TrialConclusion>;

/// The sealed read surface an authored table and a staged view both present, at the types this engine runs.
pub type TrialTableView<'view> = TableView<'view, Invocation, TrialConclusion>;

/// The typed cause every caught subject panic is cited under.
///
/// The pair is this home's own, so a fingerprint over a subject panic names the boundary that caught it rather than any of the panic's own words.
/// The family is qualified with the harness's own name, like every sibling family, so a consumer's bare `runner` family cannot alias it.
pub const SUBJECT_PANIC_CAUSE: FindingCause =
    FindingCause::named("macroonz.runner", "subject-panic");

/// What one selected trial did instead of concluding lawfully.
///
/// A satisfied check has no arm here, so "this is why the seat refused" is unsayable about a trial that passed.
/// Every arm carries a typed value lifted out of the record the run wrote, so a seat describes a failure by carrying it rather than by rendering it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeatFailure {
    /// The check refused, and the refusal carries its own evidence.
    CheckRefused(TrialFinding),
    /// The trial was selected and did not run, for a stated reason.
    ///
    /// [`SkipReason::SatisfiedByCachedExecution`] lands here with the rest: the conclusion a cached execution stood in for is not in the report being read.
    NotRun(SkipReason),
    /// The trial ran past the budget it was given, which rides along so a reader knows which bound was reached.
    PastTimeBudget(TimeBudget),
    /// The harness failed around the trial, so nothing was learned about the subject.
    HarnessFailed(InfrastructureFault),
}

/// One selected trial that did not conclude lawfully: both identity rails, and what it did instead.
///
/// Neither rail stands in for the other — the semantic identity is the name this failure keeps across a refactor, and the site is the spelling a person filters on and jumps to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FailedTrial {
    trial: TrialId,
    site: TrialSite,
    failure: SeatFailure,
}

/// What one aggregate seat's reading found when it did not refuse.
///
/// The two arms are two different facts, and naming them apart is what keeps a run that deliberately exercised nothing from being read as a run that exercised everything.
/// Nothing here spells "passed": the satisfied arm says every selected trial concluded lawfully, and says nothing at all about the rows the selection passed over.
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
    /// The selection named no row, exactly as the caller stated in advance that it might.
    NoWorkAsStated {
        /// The reason the caller stated for admitting an empty selection.
        reason: EmptySelectionReason,
        /// How many rows the run was stated over.
        denominator: usize,
    },
}

/// The seats' one refusal type: everything a stamped test function answers with instead of passing.
///
/// A construction refusal enters unchanged through this type's [`From`] road over [`TrialTableRefusal`], and the run's own verdict supplies the other arms.
/// That is the whole road in, which is what makes `?` the entire ceremony at a seat.
/// `Debug` is the rendering surface, deliberately: a `Display` written here would be a second vocabulary for facts the typed fields already carry.
#[must_use = "a refusal is the reason a seat did not pass"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeatRefusal {
    /// The world could not be built, and this is the construction that refused.
    TableNotBuilt(TrialTableRefusal),
    /// The selection named no row of the denominator, and the run expected at least one.
    ///
    /// The run records the fact as [`SelectionOutcome::UnsatisfiedByEmptySelection`](crate::report::SelectionOutcome::UnsatisfiedByEmptySelection), and this arm is where it reaches the channel a test function answers in.
    NothingSelected {
        /// How many rows the run was stated over.
        denominator: usize,
    },
    /// One trial did not conclude lawfully: the reading a named lens takes.
    TrialFailed(Box<FailedTrial>),
    /// Selected trials did not conclude lawfully: the reading an aggregate seat takes.
    RunFailed {
        /// Every selected trial that did not conclude lawfully, in census order.
        failed: Vec<FailedTrial>,
        /// How many rows of the denominator the selection named.
        selected: usize,
        /// How many rows the run was stated over.
        denominator: usize,
    },
}

//! The runner's declarations: the typed invocation both engine calls take, what
//! one invocation chooses from the complete world, and the concrete spellings of
//! the generic seam this home instantiates.
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

use crate::descriptor::{Binding, ClaimRef, ExecutionSuite, SubjectRoute, TableView};
use crate::report::{
    FindingCause, InvocationProfile, TargetBinding, TrialConclusion, TrialId, TrialSite,
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

/// The sealed read surface an authored table and a staged view both present, at
/// the types this engine runs.
pub type TrialTableView<'view> = TableView<'view, Invocation, TrialConclusion>;

/// The typed cause every caught subject panic is cited under.
///
/// The pair is this home's declaration, so a fingerprint over a subject panic
/// names the boundary that caught it rather than any of the panic's own words.
pub const SUBJECT_PANIC_CAUSE: FindingCause = FindingCause::named("runner", "subject-panic");

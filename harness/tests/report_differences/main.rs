//! Report comparison retains every established execution-standing axis and keeps absence distinct from no change.

use macroonz_harness::clock::{HarnessClock, MeasurementReading};
use macroonz_harness::descriptor::{
    AuthoredTableName, Binding, CheckRef, ClaimRef, Classification, DerivedRevision,
    ExecutableAttachment, ExecutionSuite, Origin, PopulationRef, Provenance, RevisionBinding, Role,
    Row, SubjectRoute, Tag, TrialTableRefusal,
};
use macroonz_harness::report::{
    Baseline, ByteBudget, CaseBudget, CensusDirection, EmptySelectionReason, FailureClass,
    FindingCause, FindingLocation, HostTrialRecord, InvocationProfile, NoBaselineReason,
    NotComparedReason, ReportComparison, ReportDiff, RunAttempt, RunReport, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding, TrialSite,
};
use macroonz_harness::runner::{
    Invocation, SeatFailure, SeatRefusal, Selection, SelectionPlan, TrialBinding, TrialTable,
    lens_verdict, record_one, run_all, trial_identity,
};
use std::collections::BTreeSet;
use std::fmt;

const OWNER: &str = "report-differences";
const REFUSAL: FindingCause = FindingCause::named(OWNER, "budget-selected-refusal");

enum LaneFailure {
    Table(TrialTableRefusal),
    Missing(&'static str),
}

impl fmt::Debug for LaneFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Table(refusal) => formatter.debug_tuple("Table").field(refusal).finish(),
            Self::Missing(standing) => formatter.debug_tuple("Missing").field(standing).finish(),
        }
    }
}

impl From<TrialTableRefusal> for LaneFailure {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Table(refusal)
    }
}

fn concluded(invocation: &Invocation) -> TrialConclusion {
    if invocation.profile().cases().cases() == 1u32 {
        TrialConclusion::Passed
    } else {
        TrialConclusion::Refused(TrialFinding::established(
            FailureClass::RefusedByCheck,
            REFUSAL,
            FindingLocation::at(file!(), line!()),
            None,
        ))
    }
}

fn binding(
    stem: &'static str,
    tag: &'static str,
    subject_revision: &'static [u8],
    check_revision: &'static [u8],
) -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(OWNER, stem)?;
    let check = CheckRef::named(OWNER, "conclusion")?;
    let row = Row::declared(
        ClaimRef::named(OWNER, "reports-retain-standing")?,
        ExecutionSuite::named(OWNER, "comparison")?,
        Classification::authored(
            vec![Role::named(OWNER, "report")?],
            vec![Tag::named(OWNER, tag)?],
        )?,
        subject,
        check,
        PopulationRef::named(OWNER, "one-invocation")?,
        Origin::HandWritten,
    )?;
    Binding::bound(
        row,
        ExecutableAttachment::attached(
            subject,
            check,
            RevisionBinding::derived(DerivedRevision::from_material(subject_revision)),
            RevisionBinding::derived(DerivedRevision::from_material(check_revision)),
            concluded,
        ),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

fn world(bindings: Vec<TrialBinding>) -> Result<TrialTable, TrialTableRefusal> {
    TrialTable::authored(
        AuthoredTableName::named(OWNER, "comparison-world")?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(TrialTableRefusal::TableNotAuthored)
}

fn invocation(cases: u32, bytes: u64, time: u64, target: &str, toolchain: &str) -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(cases),
            ByteBudget::declared(bytes),
            TimeBudget::declared(time),
        ),
        TargetBinding::bound(
            TargetTriple::declared(target),
            ToolchainIdentity::declared(toolchain),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "report-difference"),
        HarnessClock::unavailable(),
    )
}

fn all(world: &TrialTable, invocation: &Invocation) -> RunReport {
    run_all(
        &world.view(),
        &SelectionPlan::of(Selection::All),
        invocation,
    )
}

fn difference(baseline: &RunReport, current: &RunReport) -> Result<ReportDiff, LaneFailure> {
    match macroonz_harness::report::compare(Baseline::Previous(baseline), current) {
        ReportComparison::Compared(diff) => Ok(diff),
        ReportComparison::NotCompared(_) => Err(LaneFailure::Missing("authored comparison")),
    }
}

/// Subject, check, budget, target, and toolchain movement remain separately readable even where census membership and row meaning do not move.
#[test]
fn execution_standing_changes_are_not_flattened() -> Result<(), LaneFailure> {
    let baseline_world = world(vec![binding(
        "held-trial",
        "stable-row",
        b"subject-v1",
        b"check-v1",
    )?])?;
    let current_world = world(vec![binding(
        "held-trial",
        "stable-row",
        b"subject-v2",
        b"check-v2",
    )?])?;
    let baseline = all(
        &baseline_world,
        &invocation(1u32, 8u64, 13u64, "target-a", "rustc-1.98.0"),
    );
    let current = all(
        &current_world,
        &invocation(2u32, 16u64, 21u64, "target-b", "rustc-1.99.0"),
    );

    let diff = difference(&baseline, &current)?;
    assert!(diff.population().added().is_empty());
    assert!(diff.population().removed().is_empty());
    assert!(diff.population().revised().is_empty());
    assert_eq!(
        diff.population().census().direction(),
        CensusDirection::Unchanged
    );

    let [revisions] = diff.execution().revisions() else {
        return Err(LaneFailure::Missing("execution revision change"));
    };
    let baseline_trial = baseline
        .census()
        .first()
        .ok_or(LaneFailure::Missing("baseline accounting"))?
        .disposition()
        .trial();
    assert_eq!(revisions.trial(), baseline_trial);
    assert_ne!(revisions.before().subject(), revisions.after().subject());
    assert_ne!(revisions.before().check(), revisions.after().check());

    let invocation = diff
        .execution()
        .invocation()
        .ok_or(LaneFailure::Missing("invocation profile change"))?;
    assert_eq!(invocation.before().cases().cases(), 1u32);
    assert_eq!(invocation.after().cases().cases(), 2u32);
    assert_eq!(invocation.before().bytes().bytes(), 8u64);
    assert_eq!(invocation.after().bytes().bytes(), 16u64);
    assert_eq!(invocation.before().time().nanoseconds(), 13u64);
    assert_eq!(invocation.after().time().nanoseconds(), 21u64);

    let target = diff
        .execution()
        .target()
        .ok_or(LaneFailure::Missing("target binding change"))?;
    assert_eq!(target.before().target().spelling(), "target-a");
    assert_eq!(target.after().target().spelling(), "target-b");
    assert_eq!(target.before().toolchain().spelling(), "rustc-1.98.0");
    assert_eq!(target.after().toolchain().spelling(), "rustc-1.99.0");
    assert_eq!(diff.execution().flips().len(), 1usize);
    Ok(())
}

/// A run that selects nothing still reports run-level budget and target movement, while an identical rerun has empty change axes.
#[test]
fn empty_selection_does_not_hide_run_standing() -> Result<(), LaneFailure> {
    let baseline_world = world(vec![binding(
        "unselected-trial",
        "stable-row",
        b"subject-v1",
        b"check-v1",
    )?])?;
    let selection = SelectionPlan::allowing_empty(
        Selection::ByTrialIds(BTreeSet::new()),
        EmptySelectionReason::AskingWhatTheWorldHolds,
    );
    let baseline_invocation = invocation(1u32, 8u64, 13u64, "target-a", "rustc-1.98.0");
    let baseline = run_all(&baseline_world.view(), &selection, &baseline_invocation);
    let repeated = run_all(&baseline_world.view(), &selection, &baseline_invocation);
    let revised_world = world(vec![binding(
        "unselected-trial",
        "stable-row",
        b"subject-v2",
        b"check-v2",
    )?])?;
    let revised = run_all(&revised_world.view(), &selection, &baseline_invocation);
    let unchanged = difference(&baseline, &repeated)?;
    assert!(unchanged.execution().revisions().is_empty());
    assert!(unchanged.execution().flips().is_empty());
    assert!(unchanged.execution().invocation().is_none());
    assert!(unchanged.execution().target().is_none());

    let revision_change = difference(&baseline, &revised)?;
    assert_eq!(revision_change.execution().revisions().len(), 1usize);
    assert!(revision_change.execution().flips().is_empty());
    assert!(revision_change.execution().invocation().is_none());
    assert!(revision_change.execution().target().is_none());

    for moved_invocation in [
        invocation(2u32, 8u64, 13u64, "target-a", "rustc-1.98.0"),
        invocation(1u32, 9u64, 13u64, "target-a", "rustc-1.98.0"),
        invocation(1u32, 8u64, 14u64, "target-a", "rustc-1.98.0"),
    ] {
        let moved = run_all(&baseline_world.view(), &selection, &moved_invocation);
        let changed = difference(&baseline, &moved)?;
        assert!(changed.execution().revisions().is_empty());
        assert!(changed.execution().flips().is_empty());
        assert!(changed.execution().invocation().is_some());
        assert!(changed.execution().target().is_none());
    }

    for moved_invocation in [
        invocation(1u32, 8u64, 13u64, "target-b", "rustc-1.98.0"),
        invocation(1u32, 8u64, 13u64, "target-a", "rustc-1.99.0"),
    ] {
        let moved = run_all(&baseline_world.view(), &selection, &moved_invocation);
        let changed = difference(&baseline, &moved)?;
        assert!(changed.execution().invocation().is_none());
        assert!(changed.execution().target().is_some());
    }

    assert_eq!(
        macroonz_harness::report::compare(Baseline::FirstRun, &baseline),
        ReportComparison::NotCompared(NotComparedReason::FirstRun)
    );
    assert_eq!(
        macroonz_harness::report::compare(
            Baseline::Unavailable(NoBaselineReason::NotRecorded),
            &baseline,
        ),
        ReportComparison::NotCompared(NotComparedReason::Unavailable(
            NoBaselineReason::NotRecorded,
        ))
    );
    Ok(())
}

/// Membership and authored-row movement remain visible beside the added execution-standing axes.
#[test]
fn membership_and_row_revision_axes_still_bite() -> Result<(), LaneFailure> {
    let baseline_world = world(vec![
        binding("removed-trial", "stable-row", b"removed", b"check")?,
        binding("shared-trial", "row-v1", b"shared", b"check")?,
    ])?;
    let current_world = world(vec![
        binding("shared-trial", "row-v2", b"shared", b"check")?,
        binding("added-trial", "stable-row", b"added", b"check")?,
    ])?;
    let invocation = invocation(1u32, 8u64, 13u64, "target-a", "rustc-1.98.0");
    let baseline = all(&baseline_world, &invocation);
    let current = all(&current_world, &invocation);
    let diff = difference(&baseline, &current)?;

    assert_eq!(diff.population().added().len(), 1usize);
    assert_eq!(diff.population().removed().len(), 1usize);
    let [revision] = diff.population().revised() else {
        return Err(LaneFailure::Missing("row revision change"));
    };
    assert_ne!(revision.before(), revision.after());
    assert!(diff.execution().revisions().is_empty());
    assert!(diff.execution().flips().is_empty());
    assert_eq!(
        diff.population().census().direction(),
        CensusDirection::Unchanged
    );
    Ok(())
}

/// A host states that the attempt timed out, while the admitted report's own invocation remains the only authority on which bound was reached.
#[test]
fn timeout_failure_reads_the_invocation_budget() -> Result<(), LaneFailure> {
    let binding = binding("timed-trial", "stable-row", b"subject-v1", b"check-v1")?;
    let invocation = invocation(1u32, 8u64, 55u64, "target-a", "rustc-1.98.0");
    let report = record_one(
        &binding,
        &invocation,
        HostTrialRecord::recorded(
            trial_identity(binding.row()),
            RunAttempt::TimedOut,
            MeasurementReading::Unavailable,
        ),
    )
    .map_err(|_| LaneFailure::Missing("admitted timeout record"))?;
    let Err(SeatRefusal::TrialFailed(failed)) = lens_verdict(&report) else {
        return Err(LaneFailure::Missing("timeout seat failure"));
    };
    let SeatFailure::PastTimeBudget(budget) = failed.failure() else {
        return Err(LaneFailure::Missing("invocation-owned timeout budget"));
    };
    assert_eq!(budget.nanoseconds(), 55u64);
    Ok(())
}

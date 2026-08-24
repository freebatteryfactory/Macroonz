//! The harness's wall-measurement boundary: distinct postures, checked tick order, and runner retention.

use macroonz_harness::clock::{
    ClockFailure, ClockReadRefusal, HarnessClock, MeasurementReading, RecordedDuration,
};
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, Tag, TrialTableRefusal,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, RunAttempt, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialConclusion, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, run_one};
use std::sync::atomic::{AtomicUsize, Ordering};

const OWNER: &str = "clock-measurements";
const REVISION_TAG: DomainTag = DomainTag::declared(
    "clock-measurements-revision",
    IdentityProfileVersion::declared(1),
);

static ZERO_CALLS: AtomicUsize = AtomicUsize::new(0usize);
static REGRESSION_CALLS: AtomicUsize = AtomicUsize::new(0usize);
static CLOSING_REFUSAL_CALLS: AtomicUsize = AtomicUsize::new(0usize);
static CLOSING_UNWIND_CALLS: AtomicUsize = AtomicUsize::new(0usize);
static SUBJECT_CALLS: AtomicUsize = AtomicUsize::new(0usize);

fn observed_zero() -> u64 {
    ZERO_CALLS.fetch_add(1usize, Ordering::SeqCst);
    0u64
}

fn regressing() -> u64 {
    if REGRESSION_CALLS.fetch_add(1usize, Ordering::SeqCst) == 0usize {
        11u64
    } else {
        7u64
    }
}

fn opening_refuses() -> Result<u64, ClockReadRefusal> {
    Err(ClockReadRefusal::Refused)
}

fn closing_refuses() -> Result<u64, ClockReadRefusal> {
    if CLOSING_REFUSAL_CALLS.fetch_add(1usize, Ordering::SeqCst) == 0usize {
        Ok(5u64)
    } else {
        Err(ClockReadRefusal::Refused)
    }
}

fn opening_unwinds() -> u64 {
    std::panic::resume_unwind(Box::new("opening clock unwind"))
}

fn closing_unwinds() -> u64 {
    if CLOSING_UNWIND_CALLS.fetch_add(1usize, Ordering::SeqCst) == 0usize {
        13u64
    } else {
        std::panic::resume_unwind(Box::new("closing clock unwind"))
    }
}

fn subject_passes(_invocation: &Invocation) -> TrialConclusion {
    SUBJECT_CALLS.fetch_add(1usize, Ordering::SeqCst);
    TrialConclusion::Passed
}

fn binding() -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(OWNER, "subject")?;
    let check = CheckRef::named(OWNER, "check")?;
    let row = Row::declared(
        ClaimRef::named(OWNER, "measurement-does-not-decide")?,
        ExecutionSuite::named(OWNER, "clock")?,
        Classification::authored(
            vec![Role::named(OWNER, "measurement")?],
            vec![Tag::named(OWNER, "failure")?],
        )?,
        subject,
        check,
        PopulationRef::named(OWNER, "one-run")?,
        Origin::HandWritten,
    )?;
    let revision = RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"clock-trial"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, subject_passes),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

fn invocation(clock: HarnessClock) -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1u32),
            ByteBudget::declared(64u64),
            TimeBudget::declared(1_000_000_000u64),
        ),
        TargetBinding::bound(
            TargetTriple::declared("x86_64-pc-windows-msvc"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "clock-measurement"),
        clock,
    )
}

/// Declared unavailability cannot be confused with a source that really observed zero elapsed time.
#[test]
fn unavailable_and_observed_zero_are_distinct() {
    ZERO_CALLS.store(0usize, Ordering::SeqCst);
    assert_eq!(
        HarnessClock::unavailable().begin().finish(),
        MeasurementReading::Unavailable
    );
    assert_eq!(ZERO_CALLS.load(Ordering::SeqCst), 0usize);
    assert_eq!(
        HarnessClock::reading(observed_zero).begin().finish(),
        MeasurementReading::Observed(RecordedDuration::recorded(0u64))
    );
    assert_eq!(ZERO_CALLS.load(Ordering::SeqCst), 2usize);
}

/// Checked elapsed time retains both reversed ticks instead of saturating their difference to zero.
#[test]
fn a_regression_is_not_a_zero_duration() {
    REGRESSION_CALLS.store(0usize, Ordering::SeqCst);
    assert!(matches!(
        HarnessClock::reading(regressing).begin().finish(),
        MeasurementReading::Failed(ClockFailure::Regressed { opened, closed })
            if opened.nanoseconds() == 11u64 && closed.nanoseconds() == 7u64
    ));
}

/// Source refusal retains whether the opening or closing read failed.
#[test]
fn typed_source_refusal_retains_its_boundary() {
    CLOSING_REFUSAL_CALLS.store(0usize, Ordering::SeqCst);
    assert_eq!(
        HarnessClock::fallible(opening_refuses).begin().finish(),
        MeasurementReading::Failed(ClockFailure::OpeningRefused)
    );
    assert_eq!(
        HarnessClock::fallible(closing_refuses).begin().finish(),
        MeasurementReading::Failed(ClockFailure::ClosingRefused)
    );
}

/// Ordinary source unwind becomes infrastructure-shaped measurement failure at its exact boundary.
#[test]
fn source_unwind_retains_its_boundary() {
    CLOSING_UNWIND_CALLS.store(0usize, Ordering::SeqCst);
    assert_eq!(
        HarnessClock::reading(opening_unwinds).begin().finish(),
        MeasurementReading::Failed(ClockFailure::OpeningUnwound)
    );
    assert_eq!(
        HarnessClock::reading(closing_unwinds).begin().finish(),
        MeasurementReading::Failed(ClockFailure::ClosingUnwound)
    );
}

/// A failed opening remains report evidence while the subject still executes and decides its own conclusion.
#[test]
fn runner_retains_clock_failure_without_changing_the_subject() -> Result<(), TrialTableRefusal> {
    SUBJECT_CALLS.store(0usize, Ordering::SeqCst);
    let report = run_one(
        &binding()?,
        &invocation(HarnessClock::fallible(opening_refuses)),
    );
    assert_eq!(SUBJECT_CALLS.load(Ordering::SeqCst), 1usize);
    assert_eq!(
        report.measurement(),
        MeasurementReading::Failed(ClockFailure::OpeningRefused)
    );
    assert!(matches!(
        report.attempt(),
        RunAttempt::Executed(TrialConclusion::Passed)
    ));
    Ok(())
}

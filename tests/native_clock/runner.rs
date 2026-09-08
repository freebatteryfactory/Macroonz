//! Native and nonnative measurement postures around one independently authored trial.

use macroonz::harness::clock::{ClockReadRefusal, HarnessClock, MeasurementReading};
use macroonz::harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, TrialTableRefusal,
};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, RunAttempt, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialConclusion, TrialSite,
};
use macroonz::harness::runner::{Invocation, TrialBinding, run_one};
use std::time::Duration;

fn passes(_invocation: &Invocation) -> TrialConclusion {
    std::thread::sleep(Duration::from_millis(2));
    TrialConclusion::Passed
}

fn refuses() -> Result<u64, ClockReadRefusal> {
    Err(ClockReadRefusal::Refused)
}

fn binding() -> Result<TrialBinding, TrialTableRefusal> {
    let owner = "native-clock-control";
    let subject = SubjectRoute::named(owner, "subject")?;
    let check = CheckRef::named(owner, "sleep-then-pass")?;
    let row = Row::declared(
        ClaimRef::named(owner, "timing-does-not-decide")?,
        ExecutionSuite::named(owner, "measurement")?,
        Classification::authored(vec![Role::named(owner, "control")?], vec![])?,
        subject,
        check,
        PopulationRef::named(owner, "single")?,
        Origin::HandWritten,
    )?;
    let tag = DomainTag::declared("native-clock-control", IdentityProfileVersion::declared(1));
    let revision = RevisionBinding::declared(ContentAddress::derived(tag, b"sleep-then-pass"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, passes),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

fn invocation(clock: HarnessClock) -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(0),
            TimeBudget::declared(0),
        ),
        TargetBinding::bound(
            TargetTriple::declared("native-clock-control-target"),
            ToolchainIdentity::declared("native-clock-control-toolchain"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "clock-control"),
        clock,
    )
}

#[test]
fn native_measurement_changes_neither_execution_standing_nor_conclusion()
-> Result<(), TrialTableRefusal> {
    let trial = binding()?;
    let unmeasured = run_one(&trial, &invocation(HarnessClock::unavailable()));
    assert_eq!(unmeasured.measurement(), MeasurementReading::Unavailable);
    for clock in [
        macroonz::native_clock::source(),
        HarnessClock::reading(|| 0),
        HarnessClock::reading(|| u64::MAX),
        HarnessClock::fallible(refuses),
    ] {
        let report = run_one(&trial, &invocation(clock));
        assert_eq!(report.standing(), unmeasured.standing());
        assert_eq!(report.trial(), unmeasured.trial());
        assert_eq!(
            report.attempt(),
            &RunAttempt::Executed(TrialConclusion::Passed)
        );
    }
    let native = run_one(&trial, &invocation(macroonz::native_clock::source()));
    assert!(native.measurement().duration().is_some());
    assert_eq!(
        native.clock_attribution(),
        macroonz::harness::clock::ClockAttribution::Monotonic
    );
    Ok(())
}

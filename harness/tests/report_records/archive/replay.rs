//! Saved witnesses execute once and cannot borrow the result of another input.

use super::{LIMITS, fixture};
use arbitrary::Unstructured;
use macroonz_harness::clock::{ClockReadRefusal, HarnessClock, MeasurementReading};
use macroonz_harness::descriptor::{DerivedRevision, RevisionBinding};
use macroonz_harness::input::{BoundInput, InputBinding, InputLimits, InputRefusal, pack};
use macroonz_harness::report::archive::{ArchivedCapsule, retain_capsule};
use macroonz_harness::report::replay::{
    HistoricalReplayStanding, ReplayCoordinate, ReplayJoinRefusal, ReplayNonReproduction,
    ReplayOutcome, ReplayReading, WitnessLineage, compare,
};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FindingCause, FindingLocation, InvocationProfile, ReplayPosture,
    RunAttempt, SkipReason, TimeBudget, TrialConclusion, TrialFinding,
};
use macroonz_harness::runner::{Invocation, ReplayedTrial, TrialBinding, replay, run_one};
use std::cell::{Cell, RefCell};

pub(super) const INPUT_LIMITS: InputLimits = InputLimits::declared(4096, 64);

std::thread_local! {
    static TRACE: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
    static CLOCK_READS: Cell<u8> = const { Cell::new(0) };
}

pub(super) fn reset() {
    fixture::reset_decodes();
    TRACE.with_borrow_mut(Vec::clear);
    CLOCK_READS.set(0);
}

pub(super) fn observations() -> (u8, Vec<Vec<u8>>, u8) {
    (
        fixture::decode_count(),
        TRACE.with_borrow(Clone::clone),
        CLOCK_READS.get(),
    )
}

pub(super) fn defective(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    TRACE.with_borrow_mut(|trace| trace.push(invocation.input().value().clone()));
    fixture::subject(invocation)
}

pub(super) fn fixed(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    TRACE.with_borrow_mut(|trace| trace.push(invocation.input().value().clone()));
    assert_eq!(invocation.input().value(), &[1]);
    TrialConclusion::Passed
}

pub(super) fn moved_revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(include_bytes!("replay.rs")))
}

pub(super) fn historical() -> Result<ArchivedCapsule, ()> {
    retain_capsule(&fixture::capsule()?, LIMITS).map_err(|_| ())
}

pub(super) fn binding(
    subject: RevisionBinding,
    call: fn(&Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion,
) -> Result<TrialBinding<BoundInput<Vec<u8>>>, ()> {
    fixture::binding_under(super::super::row()?, subject, fixture::revision(), call)
}

pub(super) fn execute(
    capsule: &ArchivedCapsule,
    binding: &TrialBinding<BoundInput<Vec<u8>>>,
) -> Result<ReplayedTrial, ()> {
    replay(
        capsule,
        binding,
        &fixture::decoder()?,
        fixture::invocation(),
        INPUT_LIMITS,
    )
    .map_err(|_| ())
}

pub(super) fn reading(replayed: &ReplayedTrial) -> Result<ReplayReading, ()> {
    replayed.comparison().as_ref().copied().map_err(|_| ())
}

#[test]
fn reduced_witness_reproduces_and_fixed_subject_earns_its_own_result() -> Result<(), ()> {
    let capsule = historical()?;
    let before = execute(&capsule, &binding(fixture::revision(), fixture::subject)?)?;
    let reproduced = reading(&before)?;
    assert_eq!(reproduced.outcome(), ReplayOutcome::DefectReproduced);
    assert_eq!(reproduced.lineage(), WitnessLineage::ReachedWitness);
    assert_eq!(
        reproduced.standing(),
        HistoricalReplayStanding::Comparable(ReplayPosture::ExactDerived)
    );
    assert_eq!(reproduced.movement().subject(), ReplayCoordinate::Same);
    assert_eq!(before.historical(), capsule.address());
    assert_eq!(before.witness().payload(), &[1]);
    assert_ne!(
        before.witness().case().address().as_bytes(),
        capsule.key().input().ok_or(())?.case().as_bytes()
    );
    assert_ne!(
        before.report().standing().key().address(),
        capsule.key().address()
    );

    let after = execute(&capsule, &binding(moved_revision(), fixed)?)?;
    assert_eq!(reading(&after)?.outcome(), ReplayOutcome::FixedOnWitness);
    assert_eq!(
        reading(&after)?.movement().subject(),
        ReplayCoordinate::Moved
    );
    assert_eq!(
        after.report().attempt(),
        &RunAttempt::Executed(TrialConclusion::Passed)
    );
    assert_eq!(before.witness(), after.witness());
    assert_ne!(
        before.report().standing().key().address(),
        after.report().standing().key().address()
    );
    let minimal = retain_capsule(&fixture::capsule_for(&[1])?, LIMITS).map_err(|_| ())?;
    assert_eq!(
        reading(&execute(
            &minimal,
            &binding(fixture::revision(), fixture::subject)?
        )?)?
        .lineage(),
        WitnessLineage::OriginalCase
    );
    Ok(())
}

#[test]
fn different_saved_bytes_are_decoded_and_executed_once_each() -> Result<(), ()> {
    for byte in [1, 2] {
        let mut vector = super::vector::Vector::declared(super::vector::InputKind::Bound);
        *vector.capsule.get_mut(48).ok_or(())? = byte;
        let capsule = macroonz_harness::report::archive::read_capsule(&vector.encoded(), LIMITS)
            .map_err(|_| ())?;
        reset();
        let original = fixture::decoder()?;
        let decoder = InputBinding::declared(original.profile(), moved_revision(), fixture::bytes);
        let result = replay(
            &capsule,
            &binding(moved_revision(), defective)?,
            &decoder,
            fixture::invocation(),
            INPUT_LIMITS,
        )
        .map_err(|_| ())?;
        assert_eq!(observations(), (1, vec![vec![byte]], 0));
        assert_eq!(result.witness().payload(), &[byte]);
        match byte {
            1 => assert!(matches!(
                result.report().attempt(),
                RunAttempt::Executed(TrialConclusion::Refused(_))
            )),
            2 => assert_eq!(
                result.report().attempt(),
                &RunAttempt::Executed(TrialConclusion::Passed)
            ),
            _ => return Err(()),
        }
        assert_eq!(
            reading(&result)?.outcome(),
            ReplayOutcome::NotReproduced(ReplayNonReproduction::TrialMoved)
        );
    }
    Ok(())
}

#[test]
fn unrelated_passing_reports_and_conventions_cannot_discharge_a_witness() -> Result<(), ()> {
    let capsule = historical()?;
    let decoder = fixture::decoder()?;
    let witness = pack(decoder.profile(), capsule.input(), INPUT_LIMITS).map_err(|_| ())?;
    let unrelated = fixture::report(&[2])?;
    assert_eq!(
        unrelated.attempt(),
        &RunAttempt::Executed(TrialConclusion::Passed)
    );
    assert_eq!(
        compare(&capsule, &unrelated, &witness),
        Err(ReplayJoinRefusal::CurrentCaseDiffers)
    );
    let other = pack(decoder.profile(), &[2], INPUT_LIMITS).map_err(|_| ())?;
    assert_eq!(
        compare(&capsule, &unrelated, &other),
        Err(ReplayJoinRefusal::WitnessBytesDiffer)
    );
    let unit = fixture::host_report(
        RunAttempt::Executed(TrialConclusion::Passed),
        MeasurementReading::Unavailable,
    )?;
    assert_eq!(
        compare(&capsule, &unit, &witness),
        Err(ReplayJoinRefusal::CurrentInputUnrecorded)
    );
    let profile = macroonz_harness::input::InputProfile::declared(
        decoder.profile().name(),
        2,
        decoder.profile().schema(),
    );
    let changed = InputBinding::declared(profile, fixture::revision(), fixture::bytes);
    let input = changed
        .decode(pack(profile, capsule.input(), INPUT_LIMITS).map_err(|_| ())?)
        .map_err(|_| ())?;
    let current = run_one(
        &binding(moved_revision(), fixed)?,
        &fixture::invocation().with_input(input),
    );
    assert_eq!(
        compare(&capsule, &current, &witness),
        Err(ReplayJoinRefusal::CurrentProfileDiffers)
    );
    Ok(())
}

fn refuses(source: &mut Unstructured<'_>) -> arbitrary::Result<Vec<u8>> {
    let _bytes = fixture::bytes(source)?;
    Err(arbitrary::Error::IncorrectFormat)
}

fn incomplete(source: &mut Unstructured<'_>) -> arbitrary::Result<Vec<u8>> {
    fixture::record_decode();
    source.bytes(0).map(<[u8]>::to_vec)
}

fn clock_refuses() -> Result<u64, ClockReadRefusal> {
    CLOCK_READS.set(CLOCK_READS.get().saturating_add(1));
    Err(ClockReadRefusal::Refused)
}

#[test]
fn input_limits_and_decoder_refusals_never_execute_a_subject() -> Result<(), ()> {
    let capsule = historical()?;
    let bound = binding(moved_revision(), defective)?;
    let original = fixture::decoder()?;
    let decoder = InputBinding::declared(original.profile(), moved_revision(), fixture::bytes);
    for (limits, expected) in [
        (
            InputLimits::declared(4096, 0),
            InputRefusal::PayloadTooLarge,
        ),
        (InputLimits::declared(0, 64), InputRefusal::EnvelopeTooLarge),
    ] {
        reset();
        assert_eq!(
            replay(&capsule, &bound, &decoder, fixture::invocation(), limits),
            Err(expected)
        );
        assert_eq!(observations(), (0, Vec::new(), 0));
    }
    let refusing: fn(&mut Unstructured<'_>) -> arbitrary::Result<Vec<u8>> = refuses;
    for (call, expected) in [
        (
            refusing,
            InputRefusal::DecoderRefused(arbitrary::Error::IncorrectFormat),
        ),
        (incomplete, InputRefusal::TrailingInputBytes { count: 1 }),
    ] {
        reset();
        let refused = InputBinding::declared(original.profile(), moved_revision(), call);
        assert_eq!(
            replay(
                &capsule,
                &bound,
                &refused,
                fixture::invocation(),
                INPUT_LIMITS
            ),
            Err(expected)
        );
        assert_eq!(observations(), (1, Vec::new(), 0));
    }
    Ok(())
}

#[test]
fn replay_budget_skips_before_clock_and_subject_without_disguising_a_pass() -> Result<(), ()> {
    let capsule = historical()?;
    let original = fixture::decoder()?;
    let decoder = InputBinding::declared(original.profile(), moved_revision(), fixture::bytes);
    for (cases, bytes) in [(0, 64), (1, 0)] {
        reset();
        let base = fixture::invocation();
        let invocation = Invocation::declared(
            InvocationProfile::declared(
                CaseBudget::declared(cases),
                ByteBudget::declared(bytes),
                TimeBudget::declared(1000),
            ),
            base.target().clone(),
            base.site(),
            HarnessClock::fallible(clock_refuses),
        );
        let result = replay(
            &capsule,
            &binding(moved_revision(), defective)?,
            &decoder,
            invocation,
            INPUT_LIMITS,
        )
        .map_err(|_| ())?;
        assert_eq!(observations(), (1, Vec::new(), 0));
        assert_eq!(
            result.report().attempt(),
            &RunAttempt::SkippedWithReason(SkipReason::BudgetExhausted)
        );
        assert_eq!(
            result.report().measurement(),
            MeasurementReading::Unavailable
        );
        assert_eq!(
            reading(&result)?.outcome(),
            ReplayOutcome::NotReproduced(ReplayNonReproduction::DidNotConclude)
        );
        assert_eq!(
            reading(&result)?.movement().invocation(),
            ReplayCoordinate::Moved
        );
    }
    Ok(())
}

#[test]
fn current_clock_failure_and_observed_zero_remain_separate_from_reproduction() -> Result<(), ()> {
    let capsule = historical()?;
    let original = fixture::invocation();
    for (clock, expected) in [
        (HarnessClock::unavailable(), MeasurementReading::Unavailable),
        (
            HarnessClock::reading(|| 7),
            MeasurementReading::Observed(macroonz_harness::clock::RecordedDuration::recorded(0)),
        ),
        (
            HarnessClock::fallible(clock_refuses),
            MeasurementReading::Failed(macroonz_harness::clock::ClockFailure::OpeningRefused),
        ),
    ] {
        reset();
        let invocation = Invocation::declared(
            original.profile(),
            original.target().clone(),
            original.site(),
            clock,
        );
        let result = replay(
            &capsule,
            &binding(fixture::revision(), defective)?,
            &fixture::decoder()?,
            invocation,
            INPUT_LIMITS,
        )
        .map_err(|_| ())?;
        assert_eq!(result.report().measurement(), expected);
        assert_eq!(reading(&result)?.outcome(), ReplayOutcome::DefectReproduced);
        assert_eq!(observations().0, 1);
        assert_eq!(observations().1, vec![vec![1]]);
    }
    Ok(())
}

#[test]
fn a_pass_without_source_movement_and_a_different_failure_are_not_repairs() -> Result<(), ()> {
    let capsule = historical()?;
    let passing = execute(&capsule, &binding(fixture::revision(), fixed)?)?;
    assert_eq!(
        reading(&passing)?.outcome(),
        ReplayOutcome::NotReproduced(ReplayNonReproduction::PassedWithoutRepairStanding)
    );
    let different = execute(
        &capsule,
        &binding(moved_revision(), |invocation| {
            assert_eq!(invocation.input().value(), &[1]);
            TrialConclusion::Refused(TrialFinding::established(
                macroonz_harness::report::FailureClass::PropertyDisagreement,
                FindingCause::named("replay", "another-failure"),
                FindingLocation::at(file!(), line!()),
                None,
            ))
        })?,
    )?;
    assert_eq!(
        reading(&different)?.outcome(),
        ReplayOutcome::NotReproduced(ReplayNonReproduction::FingerprintMoved)
    );
    let panic = execute(
        &capsule,
        &binding(moved_revision(), |_| {
            std::panic::resume_unwind(Box::new("replay panic control"))
        })?,
    )?;
    let RunAttempt::Executed(TrialConclusion::Refused(finding)) = panic.report().attempt() else {
        return Err(());
    };
    assert_eq!(
        finding.cause(),
        macroonz_harness::runner::SUBJECT_PANIC_CAUSE
    );
    assert_eq!(
        reading(&panic)?.outcome(),
        ReplayOutcome::NotReproduced(ReplayNonReproduction::FingerprintMoved)
    );
    Ok(())
}

//! A counting subject and independent correctness and work expectations.

use macroonz::harness::bench::{
    WorkConclusion, WorkCurve, WorkGapStanding, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef, WorkRecorder, WorkRecordingRefusal,
};
use macroonz::harness::properties::{Holding, concluded};
use macroonz::harness::report::{FailureClass, FindingCause, TrialConclusion};
use macroonz::harness::runner::Invocation;

pub(crate) const OWNER: &str = "example.benchmark";

pub(crate) fn mapped<T>(value: Result<T, impl std::fmt::Debug>) -> Result<T, String> {
    value.map_err(|error| format!("{error:?}"))
}

pub(crate) fn observation() -> Result<WorkObservationRef, WorkRecordingRefusal> {
    WorkObservationRef::named(OWNER, "visited-element")
        .map_err(WorkRecordingRefusal::ObservationName)
}

fn count_nonzero(
    values: impl IntoIterator<Item = u64>,
    mut visit: impl FnMut() -> Result<(), WorkRecordingRefusal>,
) -> Result<u64, WorkRecordingRefusal> {
    let mut count = 0u64;
    for value in values {
        visit()?;
        count = count.saturating_add(u64::from(value != 0));
    }
    Ok(count)
}

pub(crate) fn measured(size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    let observation = observation()?;
    let count = count_nonzero(std::hint::black_box(0..size), || {
        recorder.record(observation, 1)
    })?;
    std::hint::black_box(count);
    Ok(())
}

pub(crate) fn worse(size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    for _repeat in 0..size {
        measured(size, recorder)?;
    }
    Ok(())
}

pub(crate) fn preflight(_invocation: &Invocation) -> TrialConclusion {
    let cases: &[(&[u64], u64)] = &[(&[], 0), (&[0, 0], 0), (&[1, 0, 2], 2), (&[7, 8, 9, 10], 4)];
    let holds = cases.iter().all(|(input, expected)| {
        count_nonzero(input.iter().copied(), || Ok(())).is_ok_and(|actual| actual == *expected)
    });
    concluded(
        if holds {
            Holding::Holds
        } else {
            Holding::Fails
        },
        FailureClass::PropertyDisagreement,
        FindingCause::named(OWNER, "nonzero-count-disagrees"),
    )
}

fn linear(curve: &WorkCurve, samples: u32) -> bool {
    curve.points().iter().all(|point| {
        let [count] = point.counts() else {
            return false;
        };
        Some(count.count()) == point.input_size().checked_mul(u64::from(samples))
    })
}

pub(crate) fn judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    let measured_conclusion = if linear(input.measured(), input.budgets().samples()) {
        WorkConclusion::Satisfied
    } else {
        WorkConclusion::Refused(FindingCause::named(OWNER, "measured-not-linear"))
    };
    let control_conclusion = if linear(input.planted_worse(), input.budgets().samples()) {
        WorkConclusion::Satisfied
    } else {
        WorkConclusion::Refused(FindingCause::named(OWNER, "control-not-linear"))
    };
    let ratio = input.budgets().ratio();
    let gap = input.measured().points().iter().zip(input.planted_worse().points()).all(|(left, right)| {
        let ([measured_count], [control_count]) = (left.counts(), right.counts()) else { return false; };
        matches!(
            (control_count.count().checked_mul(ratio.denominator()), measured_count.count().checked_mul(ratio.numerator())),
            (Some(control_bound), Some(measured_bound)) if control_bound >= measured_bound
        )
    });
    WorkJudgment::stated(
        measured_conclusion,
        control_conclusion,
        if gap {
            WorkGapStanding::Distinguished
        } else {
            WorkGapStanding::NotDistinguished(FindingCause::named(OWNER, "gap-not-reached"))
        },
    )
}

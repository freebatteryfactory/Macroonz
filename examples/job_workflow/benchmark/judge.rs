//! Independent literal curves and exact arithmetic distinguish the planted control.

use macroonz::harness::bench::{
    WorkConclusion, WorkCurve, WorkGapStanding, WorkJudgment, WorkJudgmentInput,
};
use macroonz::harness::report::FindingCause;

pub(super) fn counts(curve: &WorkCurve) -> Option<Vec<(u64, u64)>> {
    curve
        .points()
        .iter()
        .map(|point| {
            let [count] = point.counts() else {
                return None;
            };
            Some((point.input_size(), count.count()))
        })
        .collect()
}

pub(super) fn fixed_axis_judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    let measured = counts(input.measured());
    let control = counts(input.planted_worse());
    let measured_holds = measured.as_deref() == Some(&[(2, 8), (4, 16), (8, 32)]);
    let control_is_hostile = control.as_deref() == Some(&[(2, 16), (4, 64), (8, 256)]);
    let ratio = input.budgets().ratio();
    let gap = match (&measured, &control) {
        (Some(measured), Some(control)) if measured.len() == 3 && control.len() == 3 => measured
            .iter()
            .zip(control)
            .all(|((size, work), (other_size, extra))| {
                size == other_size
                    && match (
                        extra.checked_mul(ratio.denominator()),
                        work.checked_mul(ratio.numerator()),
                    ) {
                        (Some(left), Some(right)) => left >= right,
                        _ => false,
                    }
            }),
        _ => false,
    };
    WorkJudgment::stated(
        if measured_holds {
            WorkConclusion::Satisfied
        } else {
            WorkConclusion::Refused(FindingCause::named("neutral-job", "linear-curve-disagrees"))
        },
        if control_is_hostile {
            WorkConclusion::Refused(FindingCause::named("neutral-job", "quadratic-damage"))
        } else {
            WorkConclusion::Satisfied
        },
        if gap {
            WorkGapStanding::Distinguished
        } else {
            WorkGapStanding::NotDistinguished(FindingCause::named("neutral-job", "gap-missing"))
        },
    )
}

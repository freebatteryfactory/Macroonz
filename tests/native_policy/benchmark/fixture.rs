//! Instrumented public example calls and independently damaged preflight.

use super::specimen;
use macroonz::harness::bench::{
    WorkJudgment, WorkJudgmentInput, WorkRecorder, WorkRecordingRefusal,
};
use macroonz::harness::properties::{Holding, concluded};
use macroonz::harness::report::{FailureClass, FindingCause, TrialConclusion};
use macroonz::harness::runner::Invocation;
use std::cell::Cell;

std::thread_local! {
    pub(super) static MEASURED: Cell<u32> = const { Cell::new(0) };
    static WORSE: Cell<u32> = const { Cell::new(0) };
    static JUDGES: Cell<u32> = const { Cell::new(0) };
    static PREFLIGHT: Cell<u32> = const { Cell::new(0) };
}

pub(super) fn reset() {
    MEASURED.set(0);
    WORSE.set(0);
    JUDGES.set(0);
    PREFLIGHT.set(0);
}
pub(super) fn calls() -> (u32, u32, u32, u32) {
    (MEASURED.get(), WORSE.get(), JUDGES.get(), PREFLIGHT.get())
}

pub(super) fn measured(size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    MEASURED.set(MEASURED.get().saturating_add(1));
    specimen::measured(size, recorder)
}
pub(super) fn worse(size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    WORSE.set(WORSE.get().saturating_add(1));
    specimen::worse(size, recorder)
}
pub(super) fn judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    JUDGES.set(JUDGES.get().saturating_add(1));
    specimen::judge(input)
}
pub(super) fn preflight(invocation: &Invocation) -> TrialConclusion {
    PREFLIGHT.set(PREFLIGHT.get().saturating_add(1));
    specimen::preflight(invocation)
}

pub(super) fn refused_preflight(_invocation: &Invocation) -> TrialConclusion {
    concluded(
        Holding::Fails,
        FailureClass::PropertyDisagreement,
        FindingCause::named("outside.benchmark", "wrong-answer"),
    )
}

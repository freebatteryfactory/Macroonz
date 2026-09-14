//! The completion check supplies the benchmark's correctness preflight.

use crate::checks;
use macroonz::harness::bench::{BenchStampRefusal, PreflightRef, PreflightTrial};
use macroonz::harness::clock::HarnessClock;
use macroonz::harness::report::TrialSite;
use macroonz::harness::runner::Invocation;

pub(super) fn completion() -> Result<PreflightTrial, BenchStampRefusal> {
    let invocation = Invocation::declared(
        checks::BUDGETS,
        checks::target(),
        TrialSite::located(module_path!(), file!(), line!(), "neutral-benchmark"),
        HarnessClock::unavailable(),
    );
    Ok(PreflightTrial::bound(
        PreflightRef::named("neutral-job", "completes")?,
        crate::trials::row::completion()?,
        invocation,
    ))
}

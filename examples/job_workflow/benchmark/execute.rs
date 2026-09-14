//! The harness runs both declared bindings before their report readers check the outcomes.

use crate::{checks, debug};
use macroonz::harness::bench::{
    BenchInvocation, BenchVerdictRefusal, ContentionPosture, bench_verdict, run_all,
};
use macroonz::harness::clock::HarnessClock;
use std::io::Write;

pub(crate) fn execute() -> Result<(), String> {
    let invocation = BenchInvocation::declared(
        checks::target(),
        HarnessClock::unavailable(),
        ContentionPosture::NoDeclaredContention,
    );
    let table = super::table().map_err(debug)?;
    let report = run_all(&table, &invocation).map_err(debug)?;
    (super::reporter::REPORT)(&report);
    writeln!(std::io::stdout(), "benchmark: {:?}", bench_verdict(&report)).map_err(debug)?;
    writeln!(
        std::io::stdout(),
        "benchmark report: {}",
        macroonz::presentation::benchmark(&report).json()
    )
    .map_err(debug)?;

    let control = super::control_table().map_err(debug)?;
    let control_report = run_all(&control, &invocation).map_err(debug)?;
    (super::control_reporter::REPORT)(&control_report);
    writeln!(
        std::io::stdout(),
        "same-work control: {:?}",
        bench_verdict(&control_report)
            .err()
            .map(BenchVerdictRefusal::stage)
    )
    .map_err(debug)?;
    writeln!(
        std::io::stdout(),
        "same-work report: {}",
        macroonz::presentation::benchmark(&control_report).json()
    )
    .map_err(debug)
}

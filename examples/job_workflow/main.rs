//! One job declaration supplies structural methods and a table of independent unit-input checks.

mod checks;
mod acceptance;
#[macro_use]
mod declaration;

mod benchmark;
mod ordinary;
mod display;

pub use declaration::job;

use macroonz::harness::report::TrialSite;
use macroonz::harness::runner::{
    Invocation, SeatOutcome, Selection, SelectionPlan, run_all, seat_verdict,
};
use macroonz::workflow::{SuiteSelectionRefusal, select_suites};
use std::io::Write;

macroonz::support! { job_support {
    declaring: crate,
    consumer: crate,
    invocation: crate::checks::BUDGETS,
    target: crate::checks::target(),
    clock: macroonz::harness::clock::HarnessClock::unavailable(),
} }

fn main() -> Result<(), String> {
    ordinary::lifecycle();
    ordinary::record_bytes();
    display::declaration()?;
    let table = trials::table().map_err(debug)?;
    let invocation = Invocation::declared(
        checks::BUDGETS,
        checks::target(),
        TrialSite::located(module_path!(), file!(), line!(), "neutral-workflow"),
        trials::CLOCK,
    );
    let report = run_all(
        &table.view(),
        &SelectionPlan::of(Selection::All),
        &invocation,
    );
    let outcome = seat_verdict(&report);
    assert_eq!(
        outcome,
        Ok(SeatOutcome::EveryTrialConcluded {
            selected: 2,
            denominator: 2,
        })
    );
    writeln!(std::io::stdout(), "all: {outcome:?}").map_err(debug)?;
    acceptance::all_declared(&report)?;
    display::run("all", &report)?;

    let selection = select_suites(&table.view(), &[("neutral-job", "codec")]).map_err(debug)?;
    let selected = run_all(&table.view(), &selection, &invocation);
    let selected_outcome = seat_verdict(&selected);
    assert_eq!(
        selected_outcome,
        Ok(SeatOutcome::EveryTrialConcluded {
            selected: 1,
            denominator: 2,
        })
    );
    writeln!(std::io::stdout(), "codec: {selected_outcome:?}").map_err(debug)?;
    let omitted = acceptance::all_declared(&selected);
    assert_eq!(
        omitted,
        Err("required declared trials were not selected".to_owned())
    );
    writeln!(std::io::stdout(), "required-lane control: {omitted:?}").map_err(debug)?;
    display::run("codec", &selected)?;

    let absent = select_suites(&table.view(), &[("neutral-job", "absent")]).err();
    assert_eq!(absent, Some(SuiteSelectionRefusal::Unknown { position: 0 }));
    writeln!(std::io::stdout(), "absent: {absent:?}").map_err(debug)?;
    benchmark::execute()
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

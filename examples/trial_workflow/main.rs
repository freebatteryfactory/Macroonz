//! Co-located trial declarations, typed input and table-derived selection through the facade.

mod checks;
mod presentation;

use macroonz::configuration::v1;
#[cfg(test)]
use macroonz::harness::input::{BoundInput, InputRefusal};
use macroonz::harness::report::{TrialConclusion, TrialSite};
use macroonz::harness::runner::{self, Invocation, Selection, SelectionPlan};

macroonz::recipe! {
    /// Ordinary caller-owned arithmetic accompanied by two independent trial bindings.
    pub mod arithmetic {
        /// Totals the supplied byte operands without narrowing their sum.
        #[must_use]
        pub fn total(values: &[u8]) -> u64 {
            values.iter().map(|value| u64::from(*value)).sum()
        }

        bake! {
            evidence {
                trials {
                    support = arithmetic_support,
                    module = trials,
                    table = named("arithmetic", "byte-operations"),
                    input = { ::std::vec::Vec<u8> },
                    suite totals = named("arithmetic", "totals") {
                        sum {
                            claim = named("arithmetic", "sum"),
                            subject = named("arithmetic", "total"),
                            check = named("arithmetic", "six"),
                            population = named("arithmetic", "one-two-three"),
                            binding = {
                                subject_revision = { $consumer::checks::revision(b"byte-total-v1") },
                                check_revision = { $consumer::checks::revision(b"one-plus-two-plus-three-is-six") },
                                call = { $consumer::checks::total },
                            },
                        },
                    },
                    suite counts = named("arithmetic", "counts") {
                        length {
                            claim = named("arithmetic", "length"),
                            subject = named("arithmetic", "slice-length"),
                            check = named("arithmetic", "three"),
                            population = named("arithmetic", "one-two-three"),
                            binding = {
                                subject_revision = { $consumer::checks::revision(b"slice-length-v1") },
                                check_revision = { $consumer::checks::revision(b"three-operands") },
                                call = { $consumer::checks::length },
                            },
                        },
                    },
                };
            };
        }
    }
}

macroonz::support! { arithmetic_support {
    declaring: crate,
    consumer: crate,
    invocation: crate::checks::BUDGETS,
    target: crate::checks::target(),
    clock: macroonz::harness::clock::HarnessClock::unavailable(),
    specimen: crate::specimen(),
} }

#[cfg(test)]
fn specimen() -> Result<BoundInput<Vec<u8>>, InputRefusal> {
    let decoder = checks::decoder()?;
    decoder.decode(macroonz::harness::input::pack(
        decoder.profile(),
        &[1, 2, 3],
        v1::input_limits(),
    )?)
}

fn main() -> Result<(), String> {
    let table = trials::table().map_err(debug)?;
    let decoder = checks::decoder().map_err(debug)?;
    let selection = macroonz::workflow::select_suites(&table.view(), &[("arithmetic", "totals")])
        .map_err(debug)?;
    let invoke = || {
        Invocation::declared(
            checks::BUDGETS,
            checks::target(),
            TrialSite::located(module_path!(), file!(), line!(), "selected arithmetic"),
            trials::CLOCK,
        )
    };
    let selected =
        v1::run(&table.view(), &selection, &decoder, &[1, 2, 3], invoke()).map_err(debug)?;
    assert_eq!(
        runner::seat_verdict(selected.report()),
        Ok(runner::SeatOutcome::EveryTrialConcluded {
            selected: 1,
            denominator: 2,
        })
    );
    let all = v1::run(
        &table.view(),
        &SelectionPlan::of(Selection::All),
        &decoder,
        &[1, 2, 3],
        invoke(),
    )
    .map_err(debug)?;
    assert_eq!(
        runner::seat_verdict(all.report()),
        Ok(runner::SeatOutcome::EveryTrialConcluded {
            selected: 2,
            denominator: 2,
        })
    );
    let wrong =
        v1::run(&table.view(), &selection, &decoder, &[1, 2, 4], invoke()).map_err(debug)?;
    assert!(
        wrong
            .report()
            .census()
            .iter()
            .filter_map(|row| row.disposition().report())
            .any(|report| matches!(
                report.attempt(),
                macroonz::harness::report::RunAttempt::Executed(TrialConclusion::Refused(_))
            ))
    );
    assert_eq!(wrong.input().payload(), &[1, 2, 4]);
    presentation::write("selected", &selected)?;
    presentation::write("all", &all)?;
    presentation::write("refused", &wrong)?;
    Ok(())
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

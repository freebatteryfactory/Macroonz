//! One declaration binds each independent callable without a second row roster.

#[path = "typed_catalogue.rs"]
mod input;

use bakery::harness::clock::HarnessClock;
use bakery::harness::descriptor::{DerivedRevision, RevisionBinding};
use bakery::harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialConclusion, TrialSite,
};
use bakery::harness::runner::{self, Invocation, Selection, SelectionPlan};
use macroonz as bakery;
use std::cell::Cell;

std::thread_local! {
    static CALLED: Cell<usize> = const { Cell::new(0) };
}

fn revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(material))
}

fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("caller-selected-target"),
        ToolchainIdentity::declared("caller-selected-toolchain"),
    )
}

fn addition(_invocation: &Invocation) -> TrialConclusion {
    CALLED.set(CALLED.get().saturating_add(1));
    assert_eq!(2_u32.checked_add(3), Some(5));
    TrialConclusion::Passed
}

fn subtraction(_invocation: &Invocation) -> TrialConclusion {
    CALLED.set(CALLED.get().saturating_add(1));
    assert_eq!(2_u32.checked_sub(3), None);
    TrialConclusion::Passed
}

bakery::recipe! {
    /// Arithmetic evidence whose independent checks belong to this consuming test target.
    pub(crate) mod arithmetic {
        bake! {
            evidence {
                trials {
                    support = co_located_trial_support,
                    module = declared_trials,
                    table = named("arithmetic", "checked-operations"),
                    suite operations = named("arithmetic", "checked") {
                        addition_holds {
                            claim = named("arithmetic", "addition"),
                            subject = named("arithmetic", "checked-add"),
                            check = named("arithmetic", "sum"),
                            population = named("arithmetic", "small-operands"),
                            binding = {
                                subject_revision = { $consumer::catalogue::revision(b"u32-add") },
                                check_revision = { $consumer::catalogue::revision(b"sum-five") },
                                call = { $consumer::catalogue::addition },
                            },
                        },
                        subtraction_refuses {
                            claim = named("arithmetic", "subtraction"),
                            subject = named("arithmetic", "checked-sub"),
                            check = named("arithmetic", "underflow"),
                            population = named("arithmetic", "small-operands"),
                            binding = {
                                subject_revision = { $consumer::catalogue::revision(b"u32-sub") },
                                check_revision = { $consumer::catalogue::revision(b"underflow-none") },
                                call = { $consumer::catalogue::subtraction },
                            },
                        },
                    },
                };
            };
        }
    }
}

co_located_trial_support! {
    declaring: crate,
    harness: crate::bakery::harness,
    consumer: crate,
    invocation: crate::bakery::harness::report::InvocationProfile::declared(
        crate::bakery::harness::report::CaseBudget::declared(1),
        crate::bakery::harness::report::ByteBudget::declared(64),
        crate::bakery::harness::report::TimeBudget::declared(1),
    ),
    target: crate::catalogue::target(),
    clock: crate::bakery::harness::clock::HarnessClock::unavailable(),
}

#[test]
fn one_declaration_exposes_the_complete_table_and_executes_its_independent_callables()
-> Result<(), String> {
    let table = declared_trials::table().map_err(|error| format!("{error:?}"))?;
    let names = table
        .bindings()
        .iter()
        .map(|binding| binding.row().subject().name().stem().written())
        .collect::<Vec<_>>();
    assert_eq!(names, ["checked-add", "checked-sub"]);
    let invocation = Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1),
        ),
        target(),
        TrialSite::located(module_path!(), file!(), line!(), "table-reference"),
        HarnessClock::unavailable(),
    );
    let report = runner::run_all(
        &table.view(),
        &SelectionPlan::of(Selection::All),
        &invocation,
    );
    assert_eq!(
        runner::seat_verdict(&report),
        Ok(runner::SeatOutcome::EveryTrialConcluded {
            selected: 2,
            denominator: 2
        })
    );
    assert_eq!(CALLED.get(), 2);
    Ok(())
}

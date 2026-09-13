//! The declared input type reaches the same admitted-input runner from suites and table references.

use super::{revision, target};
use bakery::harness::descriptor::{NamespacedName, TrialTableRefusal};
use bakery::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use bakery::harness::input::{
    self, BoundInput, InputBinding, InputLimits, InputProfile, InputRefusal,
};
use bakery::harness::report::{TrialConclusion, TrialSite};
use bakery::harness::runner::{self, Invocation, Selection, SelectionPlan, TrialTable};
use macroonz as bakery;
use std::cell::Cell;
use std::sync::LazyLock;

type Bytes = Vec<u8>;
type ByteTable = TrialTable<BoundInput<Bytes>>;
static TABLE: LazyLock<Result<ByteTable, TrialTableRefusal>> = LazyLock::new(typed_trials::table);
std::thread_local! {
    static DECODES: Cell<usize> = const { Cell::new(0) };
    static CALLS: Cell<usize> = const { Cell::new(0) };
}
const LIMITS: InputLimits = InputLimits::declared(4096, 64);

fn decoder() -> Result<InputBinding<Bytes>, InputRefusal> {
    let name = NamespacedName::named("typed-catalogue", "bytes")
        .map_err(|_| InputRefusal::ProfileMismatch)?;
    let schema = ContentAddress::derived(
        DomainTag::declared("typed-catalogue", IdentityProfileVersion::declared(1)),
        b"each byte is one operand",
    );
    Ok(InputBinding::declared(
        InputProfile::declared(name, 1, schema),
        revision(b"consume-all-bytes-v1"),
        |source| {
            DECODES.set(DECODES.get().saturating_add(1));
            source.bytes(source.len()).map(<[u8]>::to_vec)
        },
    ))
}

fn specimen() -> Result<BoundInput<Bytes>, InputRefusal> {
    let decoder = decoder()?;
    decoder.decode(input::pack(decoder.profile(), &[2, 3], LIMITS)?)
}

fn check_sum(invocation: &Invocation<BoundInput<Bytes>>) -> TrialConclusion {
    CALLS.set(CALLS.get().saturating_add(1));
    let sum = invocation
        .input()
        .value()
        .iter()
        .map(|value| u16::from(*value))
        .sum::<u16>();
    assert_eq!(sum, 5);
    TrialConclusion::Passed
}

fn check_length(invocation: &Invocation<BoundInput<Bytes>>) -> TrialConclusion {
    CALLS.set(CALLS.get().saturating_add(1));
    assert_eq!(invocation.input().value().len(), 2);
    TrialConclusion::Passed
}

bakery::recipe! {
    /// Typed byte-operand evidence whose input convention and independent checks are explicit.
    pub(crate) mod byte_operations {
        /// An ordinary const reference; Rust initializes the table on first access.
        pub(crate) const TABLE: &::std::sync::LazyLock<
            Result<crate::catalogue::input::ByteTable, crate::bakery::harness::descriptor::TrialTableRefusal>
        > = &crate::catalogue::input::TABLE;

        bake! {
            evidence {
                trials {
                    support = typed_trial_support,
                    module = typed_trials,
                    table = named("typed-catalogue", "operations"),
                    input = { $consumer::catalogue::input::Bytes },
                    suite checks = named("typed-catalogue", "totals") {
                        sum {
                            claim = named("typed-catalogue", "sum"),
                            subject = named("typed-catalogue", "sum"),
                            check = named("typed-catalogue", "five"),
                            population = named("typed-catalogue", "two-three"),
                            binding = {
                                subject_revision = { $consumer::catalogue::revision(b"sum-v1") },
                                check_revision = { $consumer::catalogue::revision(b"five") },
                                call = { $consumer::catalogue::input::check_sum },
                            },
                        },
                    },
                    suite lengths = named("typed-catalogue", "counts") {
                        length {
                            claim = named("typed-catalogue", "length"),
                            subject = named("typed-catalogue", "length"),
                            check = named("typed-catalogue", "two"),
                            population = named("typed-catalogue", "two-three"),
                            binding = {
                                subject_revision = { $consumer::catalogue::revision(b"length-v1") },
                                check_revision = { $consumer::catalogue::revision(b"two") },
                                call = { $consumer::catalogue::input::check_length },
                            },
                        },
                    },
                };
            };
        }
    }
}

typed_trial_support! {
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
    specimen: crate::catalogue::input::specimen(),
}

#[test]
fn complete_typed_table_reference_decodes_once_and_runs_each_check_once() -> Result<(), String> {
    let table = byte_operations::TABLE.as_ref().map_err(debug)?;
    let invocation = Invocation::declared(
        typed_trials::INVOCATION,
        target(),
        TrialSite::located(module_path!(), file!(), line!(), "typed-table"),
        typed_trials::CLOCK,
    );
    let run = bakery::workflow::run(
        &table.view(),
        &SelectionPlan::of(Selection::All),
        &decoder().map_err(debug)?,
        &[2, 3],
        invocation,
        LIMITS,
    )
    .map_err(debug)?;
    assert_eq!(
        runner::seat_verdict(run.report()),
        Ok(runner::SeatOutcome::EveryTrialConcluded {
            selected: 2,
            denominator: 2
        })
    );
    assert_eq!(run.input().payload(), &[2, 3]);
    assert_eq!(DECODES.get(), 1);
    assert_eq!(CALLS.get(), 2);
    Ok(())
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

#[test]
fn suite_text_selects_existing_names_and_preserves_the_complete_denominator() -> Result<(), String>
{
    let table = byte_operations::TABLE.as_ref().map_err(debug)?;
    let namespace = String::from("typed-catalogue");
    let totals = String::from("totals");
    let selection =
        bakery::workflow::select_suites(&table.view(), &[(namespace.as_str(), totals.as_str())])
            .map_err(debug)?;
    let available = bakery::workflow::suites(&table.view())
        .into_iter()
        .map(|suite| {
            (
                suite.name().namespace().written(),
                suite.name().stem().written(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        available,
        [("typed-catalogue", "counts"), ("typed-catalogue", "totals")]
    );
    let invocation = Invocation::declared(
        typed_trials::INVOCATION,
        target(),
        TrialSite::located(module_path!(), file!(), line!(), "text-selection"),
        typed_trials::CLOCK,
    );
    let run = bakery::workflow::run(
        &table.view(),
        &selection,
        &decoder().map_err(debug)?,
        &[2, 3],
        invocation,
        LIMITS,
    )
    .map_err(debug)?;
    assert_eq!(
        runner::seat_verdict(run.report()),
        Ok(runner::SeatOutcome::EveryTrialConcluded {
            selected: 1,
            denominator: 2
        })
    );
    assert_eq!(CALLS.get(), 1);
    assert_eq!(DECODES.get(), 1);
    Ok(())
}

#[test]
fn text_selection_refuses_unknown_and_duplicate_names_without_an_interner() -> Result<(), String> {
    use bakery::workflow::{SuiteSelectionRefusal, select_suites};
    let table = byte_operations::TABLE.as_ref().map_err(debug)?;
    assert_eq!(
        select_suites(&table.view(), &[]),
        Err(SuiteSelectionRefusal::Empty)
    );
    assert_eq!(
        select_suites(
            &table.view(),
            &[("typed-catalogue", "totals"), ("unknown", "counts")]
        ),
        Err(SuiteSelectionRefusal::Unknown { position: 1 })
    );
    assert_eq!(
        select_suites(
            &table.view(),
            &[("typed-catalogue", "counts"), ("typed-catalogue", "counts")]
        ),
        Err(SuiteSelectionRefusal::Duplicate { position: 1 })
    );
    let first_namespace = String::from("typed-catalogue");
    let second_namespace = String::from("typed-catalogue");
    let first = select_suites(
        &table.view(),
        &[
            (first_namespace.as_str(), "counts"),
            ("typed-catalogue", "totals"),
        ],
    )
    .map_err(debug)?;
    let second = select_suites(
        &table.view(),
        &[
            (second_namespace.as_str(), "totals"),
            ("typed-catalogue", "counts"),
        ],
    )
    .map_err(debug)?;
    assert_eq!(first, second);
    assert_eq!(CALLS.get(), 0);
    assert_eq!(DECODES.get(), 0);
    Ok(())
}

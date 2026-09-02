//! Renamed-facade producer and consumer specimen.

pub(super) const PRODUCER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

use core::sync::atomic::{AtomicUsize, Ordering};

static OPENED: AtomicUsize = AtomicUsize::new(0);

fn record_open() {
    OPENED.fetch_add(1, Ordering::Relaxed);
}

fn preserve_misdeclared_row() {}

bakery::recipe! {
    /// A package-shaped adopter recipe.
    pub mod door {
        /// The caller-owned state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The closed state.
            Closed,
            /// The open state.
            Open,
        }

        /// The caller-owned event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// The request to open.
            OpenDoor,
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                compile_contract;
                declaration_conformance;
            };
            evidence {
                trials {
                    support = recipe_trials_support,
                    module = recipe_trials,
                    table = named("recipe", "trial-table"),
                    suite checks = named("recipe", "unit") {
                        transition_answers {
                            claim = named("recipe", "transition-answers"),
                            subject = named("recipe", "dispatch"),
                            check = named("recipe", "exact"),
                            population = named("recipe", "declared-rows"),
                        },
                    },
                };
                mutation(State) {
                    module = recipe_mutations,
                    refusal = RecipeMutationRefusal,
                    support = recipe_mutation_support,
                    family = named("recipe", "refusals"),
                    point = named("recipe", "state-order"),
                    fact = named("recipe", "state-order"),
                    map named("recipe", "state-order") = named("recipe", "order-held"),
                    permit named("recipe", "order-held") = ["declared-order-permutation"],
                };
                benchmarks {
                    support = recipe_bench_support,
                    table_function = recipe_bench_table,
                    table = named("recipe", "bench-table"),
                    reporter = recipe_bench_reporter,
                    dispatch_pace {
                        workload = named("recipe", "dispatch"),
                        preflight = named("recipe", "dispatch-correct"),
                        planted_worse = named("recipe", "dispatch-worse"),
                        complexity = named("recipe", "linear"),
                        axis = [2, 4, 8],
                        samples = 16,
                        warmups = 4,
                        ratio_numerator = 3,
                        ratio_denominator = 1,
                        observe = [named("recipe", "rows-touched")],
                    },
                };
                network {
                    harness = bakery::harness,
                    module = recipe_network,
                    namespace = "recipe",
                    nodes = [client, server],
                    link forward = client to server,
                    schedule quiet = [],
                };
                concurrency {
                    harness = bakery::harness,
                    module = recipe_concurrency,
                    namespace = "recipe",
                    transitions_hold {
                        population = "transition-orders",
                        interleavings = 16,
                        samples = 32,
                        seed = 11,
                    },
                };
            };
            support(door_recipe_support);
        }
    }
}

bakery::recipe! {
    /// A deliberately wrong declaration that still conforms to its generated dispatcher.
    pub mod misdeclared {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            Closed,
            Open,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            OpenDoor,
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Closed with(crate::preserve_misdeclared_row);
            };
            absence(refused);
            projections {
                dispatch(apply);
                declaration_conformance;
            };
            support(misdeclared_recipe_support);
        }
    }
}

bakery::recipe! {
    /// Caller-owned language evidence classified through the generic trial carrier.
    pub mod language_evidence {
        bake! {
            evidence {
                trials {
                    support = language_trials_support,
                    module = language_trials,
                    table = named("recipe", "language-trials"),
                    suite checks = named("recipe", "catalog") {
                        language_contract {
                            claim = named("recipe", "language-contract"),
                            roles = [
                                named("recipe", "compile-contract"),
                                named("recipe", "property"),
                                named("recipe", "temporal"),
                            ],
                            tags = [named("recipe", "catalog")],
                            subject = named("recipe", "compile-contract"),
                            check = named("recipe", "callable"),
                            population = named("recipe", "declared-signature"),
                        },
                    },
                };
            };
        }
    }
}

bakery::recipe! {
    /// Caller-owned search evidence classified through the generic trial carrier.
    pub mod search_evidence {
        bake! {
            evidence {
                trials {
                    support = search_trials_support,
                    module = search_trials,
                    table = named("recipe", "search-trials"),
                    suite checks = named("recipe", "catalog") {
                        adversarial_search {
                            claim = named("recipe", "adversarial-search"),
                            roles = [
                                named("recipe", "generation"),
                                named("recipe", "fuzz"),
                                named("recipe", "fault"),
                                named("recipe", "schedule"),
                            ],
                            tags = [named("recipe", "catalog")],
                            subject = named("recipe", "search-frontier"),
                            check = named("recipe", "bounded-and-repeatable"),
                            population = named("recipe", "declared-populations"),
                        },
                    },
                };
            };
        }
    }
}

bakery::recipe! {
    /// Caller-owned delivery evidence classified through the generic trial carrier.
    pub mod delivery_evidence {
        bake! {
            evidence {
                trials {
                    support = delivery_trials_support,
                    module = delivery_trials,
                    table = named("recipe", "delivery-trials"),
                    suite checks = named("recipe", "catalog") {
                        delivery_challenge {
                            claim = named("recipe", "delivery-challenge"),
                            roles = [
                                named("recipe", "package"),
                                named("recipe", "publication"),
                            ],
                            tags = [named("recipe", "catalog")],
                            subject = named("recipe", "package-surface"),
                            check = named("recipe", "registry-shaped"),
                            population = named("recipe", "package-only"),
                        },
                    },
                };
            };
        }
    }
}

/// Reads how many admitted transitions invoked their caller-owned effect.
pub fn opened() -> usize {
    OPENED.load(Ordering::Relaxed)
}
"#;

pub(super) const CONSUMER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

use bakery::harness::bench::{
    BenchInvocation, BenchReport, BenchStampRefusal, ContentionPosture, PreflightRef,
    PreflightTrial, WorkConclusion, WorkGapStanding, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef, WorkRecorder, WorkRecordingRefusal,
};
use bakery::harness::clock::HarnessClock;
use bakery::harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, NamespacedName, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row,
    SubjectRoute, Tag,
};
use bakery::harness::interleave::{InterleavingSpace, Strand, StrandSet};
use bakery::harness::properties::{Holding, TemporalClaim, TemporalDemand, TransitionContract};
use bakery::harness::report::{
    ByteBudget, CaseBudget, FindingCause, InvocationProfile, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity, TrialConclusion, TrialSite,
};
use bakery::harness::runner::{Invocation, TrialBinding};
use renamed_recipe_adopter::{
    delivery_trials_support, door_recipe_support, language_trials_support, recipe_bench_support,
    recipe_mutation_support, recipe_trials_support, search_trials_support,
};
use std::sync::atomic::{AtomicU64, Ordering};

const TEMPORAL_HOLDS: FindingCause = FindingCause::named("recipe", "temporal-holds");
const MEASURED_REFUSED: FindingCause = FindingCause::named("recipe", "measured-refused");
const WORSE_REFUSED: FindingCause = FindingCause::named("recipe", "worse-refused");
const GAP_REFUSED: FindingCause = FindingCause::named("recipe", "gap-refused");

static BENCH_CLOCK: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Command(i8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Model(i8);

fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("x86_64-pc-windows-msvc"),
        ToolchainIdentity::declared("1.98.0"),
    )
}

fn revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(material))
}

fn catalog_trial(_invocation: &Invocation) -> TrialConclusion {
    if renamed_recipe_adopter::door::baked::apply(
        renamed_recipe_adopter::door::State::Closed,
        renamed_recipe_adopter::door::Event::OpenDoor,
    ) == Ok(renamed_recipe_adopter::door::State::Open)
    {
        TrialConclusion::Passed
    } else {
        TrialConclusion::Refused(bakery::harness::report::TrialFinding::established(
            bakery::harness::report::FailureClass::PropertyDisagreement,
            TEMPORAL_HOLDS,
            bakery::harness::properties::raised_here(),
            None,
        ))
    }
}

fn bench_clock() -> u64 {
    BENCH_CLOCK.fetch_add(10, Ordering::SeqCst)
}

fn observation() -> Result<WorkObservationRef, WorkRecordingRefusal> {
    WorkObservationRef::named("recipe", "rows-touched")
        .map_err(WorkRecordingRefusal::ObservationName)
}

fn measured(input_size: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    recorder.record(observation()?, input_size)
}

fn planted_worse(
    input_size: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let named = observation()?;
    let amount = input_size
        .checked_mul(4)
        .ok_or(WorkRecordingRefusal::AmountOverflow {
            observation: named,
            input_size,
        })?;
    recorder.record(named, amount)
}

fn judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    let samples = u64::from(input.budgets().samples());
    let measured_holds = input.measured().points().iter().all(|point| {
        let [count] = point.counts() else {
            return false;
        };
        point
            .input_size()
            .checked_mul(samples)
            .is_some_and(|expected| count.count() == expected)
    });
    let worse_holds = input.planted_worse().points().iter().all(|point| {
        let [count] = point.counts() else {
            return false;
        };
        point
            .input_size()
            .checked_mul(samples)
            .and_then(|expected| expected.checked_mul(4))
            .is_some_and(|expected| count.count() == expected)
    });
    WorkJudgment::stated(
        if measured_holds {
            WorkConclusion::Satisfied
        } else {
            WorkConclusion::Refused(MEASURED_REFUSED)
        },
        if worse_holds {
            WorkConclusion::Refused(WORSE_REFUSED)
        } else {
            WorkConclusion::Satisfied
        },
        if measured_holds && worse_holds {
            WorkGapStanding::Distinguished
        } else {
            WorkGapStanding::NotDistinguished(GAP_REFUSED)
        },
    )
}

fn preflight() -> Result<PreflightTrial, BenchStampRefusal> {
    let reference = PreflightRef::named("recipe", "dispatch-correct")?;
    let subject = SubjectRoute::named("recipe", "dispatch")?;
    let check = CheckRef::named("recipe", "exact")?;
    let row = Row::declared(
        ClaimRef::named("recipe", "benchmark-preflight")?,
        ExecutionSuite::named("recipe", "benchmark")?,
        Classification::authored(
            vec![Role::named("recipe", "benchmark")?],
            vec![Tag::named("recipe", "catalog")?],
        )
        .map_err(bakery::harness::descriptor::TrialTableRefusal::ClassificationNotAuthored)?,
        subject,
        check,
        PopulationRef::named("recipe", "declared-rows")?,
        Origin::HandWritten,
    )
    .map_err(bakery::harness::descriptor::TrialTableRefusal::RowNotDeclared)?;
    let binding: TrialBinding = Binding::bound(
        row,
        ExecutableAttachment::attached(
            subject,
            check,
            revision(b"benchmark-subject"),
            revision(b"benchmark-check"),
            catalog_trial,
        ),
        Provenance::Unproduced,
    )
    .map_err(bakery::harness::descriptor::TrialTableRefusal::BindingNotBound)?;
    let invocation = Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1),
        ),
        target(),
        TrialSite::located(module_path!(), file!(), line!(), "recipe-benchmark-preflight"),
        HarnessClock::unavailable(),
    );
    Ok(PreflightTrial::bound(reference, binding, invocation))
}

fn report(report: &BenchReport) {
    assert!(bakery::harness::bench::bench_verdict(report).is_ok());
}

fn strands() -> Result<StrandSet<Command>, String> {
    let left = Strand::declared(
        NamespacedName::named("recipe", "left").map_err(debug)?,
        vec![Command(1)],
    )
    .map_err(debug)?;
    let right = Strand::declared(
        NamespacedName::named("recipe", "right").map_err(debug)?,
        vec![Command(-1)],
    )
    .map_err(debug)?;
    StrandSet::declared(vec![left, right]).map_err(debug)
}

const fn opening() -> Model {
    Model(0)
}

fn applied(model: &Model, command: &Command) -> Model {
    Model(model.0.saturating_add(command.0))
}

const fn lawful(_model: &Model) -> Holding {
    Holding::Holds
}

fn contract() -> Result<TransitionContract<Model, Command>, String> {
    TransitionContract::declared(
        opening,
        applied,
        vec![TemporalClaim::declared(
            TEMPORAL_HOLDS,
            TemporalDemand::Always(lawful),
        )],
    )
    .map_err(debug)
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

door_recipe_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
}

mod misdeclared_conformance {
    renamed_recipe_adopter::misdeclared_recipe_support! {
        declaring: renamed_recipe_adopter,
        harness: bakery::harness,
    }
}

recipe_trials_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
    invocation: bakery::harness::report::InvocationProfile::declared(
        bakery::harness::report::CaseBudget::declared(16),
        bakery::harness::report::ByteBudget::declared(4_096),
        bakery::harness::report::TimeBudget::declared(1),
    ),
    target: crate::target(),
    clock: bakery::harness::clock::HarnessClock::unavailable(),
    transition_answers_subject_revision: crate::revision(b"transition-answers-subject"),
    transition_answers_check_revision: crate::revision(b"transition-answers-check"),
    transition_answers_call: crate::catalog_trial,
}

language_trials_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
    invocation: bakery::harness::report::InvocationProfile::declared(
        bakery::harness::report::CaseBudget::declared(16),
        bakery::harness::report::ByteBudget::declared(4_096),
        bakery::harness::report::TimeBudget::declared(1),
    ),
    target: crate::target(),
    clock: bakery::harness::clock::HarnessClock::unavailable(),
    language_contract_subject_revision: crate::revision(b"language-contract-subject"),
    language_contract_check_revision: crate::revision(b"language-contract-check"),
    language_contract_call: crate::catalog_trial,
}

search_trials_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
    invocation: bakery::harness::report::InvocationProfile::declared(
        bakery::harness::report::CaseBudget::declared(16),
        bakery::harness::report::ByteBudget::declared(4_096),
        bakery::harness::report::TimeBudget::declared(1),
    ),
    target: crate::target(),
    clock: bakery::harness::clock::HarnessClock::unavailable(),
    adversarial_search_subject_revision: crate::revision(b"adversarial-search-subject"),
    adversarial_search_check_revision: crate::revision(b"adversarial-search-check"),
    adversarial_search_call: crate::catalog_trial,
}

delivery_trials_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
    invocation: bakery::harness::report::InvocationProfile::declared(
        bakery::harness::report::CaseBudget::declared(16),
        bakery::harness::report::ByteBudget::declared(4_096),
        bakery::harness::report::TimeBudget::declared(1),
    ),
    target: crate::target(),
    clock: bakery::harness::clock::HarnessClock::unavailable(),
    delivery_challenge_subject_revision: crate::revision(b"delivery-challenge-subject"),
    delivery_challenge_check_revision: crate::revision(b"delivery-challenge-check"),
    delivery_challenge_call: crate::catalog_trial,
}

recipe_mutation_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
}

recipe_bench_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
    reporter: crate::report,
    dispatch_pace_measured: crate::measured,
    dispatch_pace_planted_worse: crate::planted_worse,
    dispatch_pace_judge: bakery::harness::bench::WorkJudgeBinding::bound(
        bakery::harness::bench::ComplexityClaimRef::named("recipe", "linear")?,
        crate::judge,
    ),
    dispatch_pace_preflight: crate::preflight()?,
}

#[test]
fn the_generated_recipe_and_independent_carriers_are_callable() -> Result<(), String> {
    assert_eq!(
        renamed_recipe_adopter::door::baked::apply(
            renamed_recipe_adopter::door::State::Closed,
            renamed_recipe_adopter::door::Event::OpenDoor,
        ),
        Ok(renamed_recipe_adopter::door::State::Open)
    );
    assert!(renamed_recipe_adopter::opened() > 0);
    assert_eq!(recipe_mutations::production(&()), ["Closed", "Open"]);
    assert_eq!(
        recipe_mutations::candidate_orders(),
        [["Open", "Closed"]]
    );
    assert!(recipe_mutations::lowering().is_ok());
    assert_ne!(recipe_mutations::production(&()), ["Open", "Closed"]);
    let language = language_trials::table().map_err(debug)?;
    let search = search_trials::table().map_err(debug)?;
    let delivery = delivery_trials::table().map_err(debug)?;
    let roles = language
        .bindings()
        .iter()
        .chain(search.bindings())
        .chain(delivery.bindings())
        .flat_map(|binding| binding.row().roles().iter())
        .map(|role| role.name().stem().written())
        .collect::<Vec<_>>();
    assert_eq!(
        roles,
        [
            "compile-contract",
            "property",
            "temporal",
            "fault",
            "fuzz",
            "generation",
            "schedule",
            "package",
            "publication",
        ]
    );
    assert!(
        language
            .bindings()
            .iter()
            .chain(search.bindings())
            .chain(delivery.bindings())
            .all(|binding| {
                binding
                    .row()
                    .tags()
                    .iter()
                    .any(|tag| tag.name().stem().written() == "catalog")
            })
    );
    let benchmark = recipe_bench_table().map_err(debug)?;
    let report = bakery::harness::bench::run_all(
        &benchmark,
        &BenchInvocation::declared(
            target(),
            HarnessClock::reading(bench_clock),
            ContentionPosture::NoDeclaredContention,
        ),
    )
    .map_err(debug)?;
    (recipe_bench_reporter::REPORT)(&report);
    bakery::harness::bench::bench_verdict(&report).map_err(debug)?;
    let topology = renamed_recipe_adopter::door::baked::recipe_network::topology()
        .map_err(debug)?;
    assert_eq!(topology.nodes().len(), 2);
    assert_eq!(topology.links().len(), 1);
    let quiet = renamed_recipe_adopter::door::baked::recipe_network::quiet().map_err(debug)?;
    assert!(quiet.disciplines().is_empty());
    let (reading, conclusion) =
        renamed_recipe_adopter::door::baked::recipe_concurrency::transitions_hold(
            &strands()?,
            &contract()?,
        )
        .map_err(debug)?;
    assert_eq!(reading.space(), InterleavingSpace::Counted(2));
    assert_eq!(conclusion, TrialConclusion::Passed);
    Ok(())
}

#[test]
fn generated_declaration_conformance_cannot_establish_independent_intent() {
    use renamed_recipe_adopter::misdeclared::{Event, State, baked};

    let generated = baked::apply(State::Closed, Event::OpenDoor);
    let independent_intent = Ok(State::Open);
    assert_eq!(generated, Ok(State::Closed));
    assert_ne!(generated, independent_intent);
}
"#;

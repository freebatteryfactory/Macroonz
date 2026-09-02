//! Renamed-facade producer and consumer specimen.

pub(super) const PRODUCER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

use core::sync::atomic::{AtomicUsize, Ordering};

static OPENED: AtomicUsize = AtomicUsize::new(0);

fn record_open() {
    OPENED.fetch_add(1, Ordering::Relaxed);
}

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
                property;
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

/// Reads how many admitted transitions invoked their caller-owned effect.
pub fn opened() -> usize {
    OPENED.load(Ordering::Relaxed)
}
"#;

pub(super) const CONSUMER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

use renamed_recipe_adopter::{door_recipe_support, recipe_mutation_support};

door_recipe_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
}

recipe_mutation_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
}

#[test]
fn the_generated_recipe_and_independent_carrier_are_callable() {
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
}
"#;

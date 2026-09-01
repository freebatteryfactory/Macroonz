//! Shared recipe text and its callable door.

use macroonz_compiler::{CrateBinding, Door, Producer};

pub(crate) const DOOR: Door = Door::declared(
    "recipe-crossing",
    "recipe-crossing.grammar",
    "recipe-crossing::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-crossing",
        name: "recipe",
    },
);

pub(crate) const COMPLETE_RECIPE: &str = r"
pub mod door {
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
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
            dispatch(apply);
            compile_contract;
            property;
            typestate;
        };
        support(door_recipe_support);
    }
}
";

pub(crate) const COMPANION_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
        };
    }
}
";

pub(crate) const EXACT_DISPATCH_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            dispatch {
                #[inline]
                pub fn advance<'a>(
                    current: State,
                    stimulus: Event,
                ) -> Result<State, TransitionRefusal>
                where
                    State: 'a;
            };
        };
    }
}
";

pub(crate) const EVIDENCE_RECIPE: &str = r#"
pub mod door {
    pub enum State {
        Closed,
        Open,
        Locked,
    }

    pub enum Event {
        OpenDoor,
        CloseDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
            (Open, CloseDoor) => Closed with(crate::effects::close);
        };
        absence(refused);
        projections {
            companions;
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
            mutation(states) {
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
                harness = renamed_facade::harness,
                module = recipe_network,
                namespace = "recipe",
                nodes = [client, server],
                link forward = client to server,
                schedule quiet = [],
            };
            concurrency {
                harness = renamed_facade::harness,
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
    }
}
"#;

pub(crate) const TARGET_UNAVAILABLE_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
        };
        evidence {
            trials unavailable;
        };
    }
}
";

pub(crate) const CALLER_OWNED_TRIAL_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
        };
        evidence {
            trials {
                this is intentionally not the descriptor trial grammar
            };
        };
    }
}
";

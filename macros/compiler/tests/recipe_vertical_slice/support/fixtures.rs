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
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
            dispatch(apply);
            compile_contract;
            property;
            typestate(State);
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
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
        };
    }
}
";

pub(crate) const CODEC_RECIPE: &str = r"
pub mod ledger {
    pub enum Seat {
        Primary,
    }

    pub struct Payload;

    pub enum Choice {
        First,
        Second,
    }

    pub struct Nested;

    pub struct Ledger {
        pub count: u16,
        pub payload: Payload,
        pub label: Option<String>,
        pub modes: Vec<Choice>,
        pub child: Nested,
    }

    impl Ledger {
        pub fn assembled(
            count: u16,
            payload: Payload,
            label: Option<String>,
            modes: Vec<Choice>,
            child: Nested,
        ) -> Self {
            Self {
                count,
                payload,
                label,
                modes,
                child,
            }
        }
    }

    bake! {
        vocabularies {
            Seat;
        };
        relations {
        };
        codecs {
            ledger(Ledger) {
                direction(round_trip);
                refusal(LedgerDecodeError);
                assembly(assembled, total);
                members {
                    count: u16 => count(required);
                    payload: Payload => bytes(required);
                    label: String => text(optional);
                    modes: Choice => closed_choice(repeated);
                    child: Nested => nested(required);
                };
            };
        };
        projections {
            codec;
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
        vocabularies { State; Event; };
        transitions(State, Event) {
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

pub(crate) const EXACT_EFFECT_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    pub struct Context {
        pub calls: usize,
    }

    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(target) {
                context.calls = context.calls.saturating_add(1);
                Ok(target)
            };
        };
        absence(refused);
        projections {
            dispatch(current, event) {
                pub fn advance(
                    context: &mut super::Context,
                    current: State,
                    event: Event,
                ) -> Result<State, TransitionRefusal>;
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
        vocabularies { State; Event; };
        transitions(State, Event) {
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
        vocabularies { State; Event; };
        transitions(State, Event) {
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
        vocabularies { State; Event; };
        transitions(State, Event) {
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

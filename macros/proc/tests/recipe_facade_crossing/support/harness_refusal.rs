//! Harness-feature refusal producer and empty consumer specimen.

pub(super) const HARNESS_REFUSAL_PRODUCER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

fn record_open() {}

bakery::recipe! {
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
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                dispatch(apply);
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
            };
        }
    }
}
"#;

pub(super) const EMPTY_CONSUMER: &str = "#![forbid(unsafe_code)]\n#![deny(warnings)]\n";

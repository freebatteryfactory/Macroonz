macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod missing_exact_type {
            pub enum State { Closed, Open }
            pub enum Event { OpenDoor }

            bake! {
                vocabularies { State; Event; };
                transitions(State, Event) {
                    (Closed, OpenDoor) => Open with(crate::record_open);
                };
                absence(refused);
                projections {
                    dispatch {
                        pub fn apply(
                            state: State,
                            event: Event,
                        ) -> Result<MissingState, TransitionRefusal>;
                    };
                };
            }
        }
    }
}

fn record_open() {}

fn main() {}

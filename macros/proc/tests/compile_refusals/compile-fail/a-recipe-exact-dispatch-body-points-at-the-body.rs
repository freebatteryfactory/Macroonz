macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod exact_body {
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
                        ) -> Result<State, TransitionRefusal> {
                            caller_body
                        }
                    };
                };
            }
        }
    }
}

fn main() {}

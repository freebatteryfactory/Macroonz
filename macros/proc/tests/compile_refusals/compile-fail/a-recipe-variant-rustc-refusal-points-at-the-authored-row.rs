macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod missing_variant {
            pub enum State {
                #[cfg(any())]
                Closed,
                Open,
            }
            pub enum Event { OpenDoor }

            bake! {
                vocabularies { State; Event; };
                transitions(State, Event) {
                    (Closed, OpenDoor) => Open with(crate::record_open);
                };
                absence(refused);
                projections { companions; };
            }
        }
    }
}

fn record_open() {}

fn main() {}

macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod missing_effect {
            pub enum State { Closed, Open }
            pub enum Event { OpenDoor }

            bake! {
                vocabularies { State; Event; };
                transitions(State, Event) {
                    (Closed, OpenDoor) => Open with(crate::missing_effect);
                };
                absence(refused);
                projections { dispatch; };
            }
        }
    }
}

fn main() {}

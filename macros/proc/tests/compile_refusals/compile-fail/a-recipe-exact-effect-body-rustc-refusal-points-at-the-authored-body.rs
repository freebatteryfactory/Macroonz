macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod exact_effect_body {
            pub enum State { Closed, Open }
            pub enum Event { OpenDoor }

            bake! {
                vocabularies { State; Event; };
                transitions(State, Event) {
                    (Closed, OpenDoor) => Open with(target) {
                        let _: crate::MissingEffectBody = target;
                        Ok(target)
                    };
                };
                absence(refused);
                projections { dispatch; };
            }
        }
    }
}

fn main() {}

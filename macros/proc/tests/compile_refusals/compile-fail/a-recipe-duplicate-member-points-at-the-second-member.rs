fn record_open() {}

macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod door {
            pub enum State {
                Closed,
                Closed,
            }

            pub enum Event {
                OpenDoor,
            }

            bake! {
                vocabularies(State, Event);
                transitions {
                    (Closed, OpenDoor) => Closed with(crate::record_open);
                };
                absence(refused);
                projections {
                    dispatch(apply);
                };
            }
        }
    }
}

fn main() {}

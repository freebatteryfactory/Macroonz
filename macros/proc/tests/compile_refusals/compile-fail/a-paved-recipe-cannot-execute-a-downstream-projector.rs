struct DownstreamProjector;

macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod inventory {
            pub enum Left {
                First,
                Second,
            }

            pub enum Right {
                Alpha,
            }

            bake! {
                vocabularies { Left; Right; };
                transitions(Left, Right) {
                    (First, Alpha) => Second with(crate::observe);
                };
                absence(refused);
                projections {
                    custom(crate::DownstreamProjector);
                };
            }
        }
    }
}

fn observe() {}

fn main() {}

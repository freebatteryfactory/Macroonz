macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod policy {
            pub enum Stage { Draft }
            pub enum Capability { Read }

            bake! {
                vocabularies { Stage; Capability; };
                relations {
                    policy(Stage, Capability) {
                        (Draft, Missing);
                    };
                };
                projections { companions; };
            }
        }
    }
}

fn main() {}

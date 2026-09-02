macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod empty_vocabulary {
            pub enum Empty {}

            bake! {
                vocabularies { Empty; };
                projections { companions; };
            }
        }
    }
}

fn main() {}

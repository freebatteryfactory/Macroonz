macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod missing_vocabulary {
            #[cfg(any())]
            pub enum State { Closed }

            bake! {
                vocabularies { State; };
                projections { companions; };
            }
        }
    }
}

fn main() {}

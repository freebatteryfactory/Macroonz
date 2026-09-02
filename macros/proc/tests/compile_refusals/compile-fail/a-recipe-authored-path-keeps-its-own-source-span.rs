macroonz_macros::__macroonz_recipe_carrier! {
    { macroonz_macros }
    __macroonz_test_carrier_available
    {
        pub mod authored_path {
            pub fn unresolved() {
                crate::Later::missing();
            }

            pub enum Later { One }

            bake! {
                vocabularies { Later; };
                projections { companions; };
            }
        }
    }
}

fn main() {}

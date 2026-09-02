//! Recipe emission observed across the real proc-macro span boundary.

const MODULE_BODY_SPAN_RESTORED: bool = macroonz_capture_observer::recipe_module_span! {
    pub mod pantry {
        pub enum Flavor {
            Plain,
            Chocolate,
        }

        bake! {
            vocabularies { Flavor; };
            projections { companions; };
        }
    }
};

/// The emitted module group retains the exact body span authored at the recipe entrance.
#[test]
fn recipe_emission_restores_the_authored_module_body_span() {
    assert!(core::hint::black_box(MODULE_BODY_SPAN_RESTORED));
}

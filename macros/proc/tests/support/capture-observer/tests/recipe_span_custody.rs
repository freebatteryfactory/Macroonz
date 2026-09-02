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

const RECIPE_REFERENCE_SPANS_RESTORED: bool =
    macroonz_capture_observer::recipe_reference_spans! {
        pub mod custody_recipe {
            pub enum CustodyState {
                CustodyClosed,
                CustodyOpen,
            }

            pub enum CustodyEvent {
                CustodyOpenDoor,
            }

            fn custody_effect() {}

            bake! {
                vocabularies {
                    CustodyState;
                    CustodyEvent;
                };
                transitions(CustodyState, CustodyEvent) {
                    (CustodyClosed, CustodyOpenDoor) => CustodyOpen with(crate::custody_effect);
                };
                relations {
                    CustodyPolicy(CustodyState, CustodyEvent) {
                        (CustodyClosed, CustodyOpenDoor);
                    };
                    CustodyExact(CustodyState, CustodyEvent) {
                        (CustodyClosed, CustodyOpenDoor);
                    };
                };
                absence(refused);
                projections {
                    relation_tables {
                        CustodyPolicy(custody_lookup);
                        CustodyExact {
                            pub fn custody_exact(
                                custody_left: &CustodyState,
                                custody_right: &CustodyEvent,
                            ) -> bool;
                        };
                    };
                    dispatch(custody_apply);
                };
            }
        }
    };

/// The emitted module group retains the exact body span authored at the recipe entrance.
#[test]
fn recipe_emission_restores_the_authored_module_body_span() {
    assert!(core::hint::black_box(MODULE_BODY_SPAN_RESTORED));
}

/// Every recipe reference the generated module repeats retains its caller-authored compiler span.
#[test]
fn recipe_emission_restores_every_authored_reference_kind() {
    assert!(core::hint::black_box(RECIPE_REFERENCE_SPANS_RESTORED));
}

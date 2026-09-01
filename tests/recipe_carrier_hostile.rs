//! The nameable proc carrier remains mechanically callable but outside the supported compatibility entrance.

fn record_direct() {}

macroonz::macros::__macroonz_recipe_carrier! {
    { macroonz }
    __macroonz_test_carrier_available
    {
        /// The direct-invocation control module.
        pub mod direct {
            /// The direct-control state vocabulary.
            pub enum State {
                /// The opening state.
                Closed,
                /// The target state.
                Open,
            }

            /// The direct-control event vocabulary.
            pub enum Event {
                /// The admitted event.
                OpenDoor,
            }

            bake! {
                vocabularies(State, Event);
                transitions {
                    (Closed, OpenDoor) => Open with(crate::record_direct);
                };
                absence(refused);
                projections {
                    dispatch(apply);
                };
            }
        }
    }
}

#[test]
fn direct_invocation_is_only_a_mechanical_hostile_control() {
    assert!(matches!(
        direct::baked::apply(direct::State::Closed, direct::Event::OpenDoor),
        Ok(direct::State::Open)
    ));
}

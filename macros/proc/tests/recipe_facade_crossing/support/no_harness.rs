//! No-harness producer and consumer specimen.

pub(super) const NO_HARNESS_PRODUCER: &str = r"#![forbid(unsafe_code)]
#![deny(warnings)]

const fn record_open() {}

bakery::recipe! {
    /// A no-harness facade recipe.
    pub mod door {
        /// The caller-owned state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The closed state.
            Closed,
            /// The open state.
            Open,
        }

        /// The caller-owned event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// The request to open.
            OpenDoor,
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch {
                    #[inline]
                    pub const fn advance<'a>(
                        current: crate::door::State,
                        event: crate::door::Event,
                    ) -> Result<crate::door::State, TransitionRefusal>
                    where
                        crate::door::State: 'a;
                };
            };
            evidence {
                trials unavailable;
                mutation unavailable;
                benchmarks unavailable;
                network unavailable;
                concurrency unavailable;
            };
        }
    }
}
";

pub(super) const NO_HARNESS_CONSUMER: &str = r"#![forbid(unsafe_code)]
#![deny(warnings)]

#[test]
fn the_no_harness_recipe_is_callable() {
    assert_eq!(
        renamed_recipe_adopter::door::baked::advance(
            renamed_recipe_adopter::door::State::Closed,
            renamed_recipe_adopter::door::Event::OpenDoor,
        ),
        Ok(renamed_recipe_adopter::door::State::Open)
    );
}
";

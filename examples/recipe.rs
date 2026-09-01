//! One recipe through conventional, configured, and exact projection levels.

#![forbid(unsafe_code)]

fn record_open() {}

macroonz::recipe! {
    /// A recipe using a configured dispatch name.
    pub mod configured {
        /// The states Macroonz must enumerate.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The door is closed.
            Closed,
            /// The door is open.
            Open,
        }

        /// The events Macroonz must enumerate.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Open the door.
            OpenDoor,
        }

        bake! {
            vocabularies(State, Event);
            transitions {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
            };
        }
    }
}

macroonz::recipe! {
    /// The same structural request with an exact dispatch signature.
    pub mod exact {
        /// The states Macroonz must enumerate.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The door is closed.
            Closed,
            /// The door is open.
            Open,
        }

        /// The events Macroonz must enumerate.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Open the door.
            OpenDoor,
        }

        bake! {
            vocabularies(State, Event);
            transitions {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                dispatch {
                    /// Applies one caller-declared transition or returns typed absence.
                    pub fn advance(
                        current: State,
                        event: Event,
                    ) -> Result<State, TransitionRefusal>;
                };
            };
        }
    }
}

fn main() {
    assert_eq!(
        configured::baked::apply(configured::State::Closed, configured::Event::OpenDoor),
        Ok(configured::State::Open),
    );
    assert_eq!(
        exact::baked::advance(exact::State::Closed, exact::Event::OpenDoor),
        Ok(exact::State::Open),
    );
}

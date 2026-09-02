//! The root recipe entrance remains usable without the optional harness dependency.

fn advance() {}

macroonz::recipe! {
    /// A recipe whose requested projections are compiler-owned.
    pub mod no_harness {
        /// The caller-owned state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Waiting,
            /// The admitted target state.
            Ready,
        }

        /// The caller-owned event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// The admitted transition event.
            Advance,
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Waiting, Advance) => Ready with(crate::advance);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
            };
        }
    }
}

#[test]
fn recipe_bakes_through_the_facade_without_the_harness_feature() {
    assert_eq!(
        no_harness::baked::STATE_VARIANTS,
        &[no_harness::State::Waiting, no_harness::State::Ready]
    );
    assert_eq!(
        no_harness::baked::apply(no_harness::State::Waiting, no_harness::Event::Advance),
        Ok(no_harness::State::Ready)
    );
}

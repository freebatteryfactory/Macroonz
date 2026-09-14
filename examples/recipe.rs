//! Generic structure, codec reuse, and transition dispatch through one recipe entrance.

#![forbid(unsafe_code)]

fn record_open() {}

const fn allow() -> bool {
    true
}

macroonz::recipe! {
    /// Structural markers selected from the sole declared vocabulary.
    pub mod single {
        /// The caller-owned positions.
        pub enum Position {
            /// The first position.
            Before,
            /// The second position.
            After,
        }

        bake! {
            vocabularies { Position; };
            projections { typestate; };
        }
    }
}

macroonz::recipe! {
    /// A generic cross-roster relation with no transition semantics.
    pub mod generic {
        /// One caller-owned lifecycle vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Stage {
            /// Work remains editable.
            Draft,
            /// Work is externally visible.
            Published,
        }

        /// One caller-owned capability vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Capability {
            /// Read access.
            Read,
            /// Write access.
            Write,
        }

        bake! {
            vocabularies {
                Stage;
                Capability;
            };
            relations {
                evolution(Stage, Stage) {
                    (Draft, Published);
                };
                readable(Stage, Capability) {
                    (Published, Read);
                    (Draft, Read);
                };
                policy(Stage, Capability) {
                    (Draft, Read) with(crate::allow);
                    (Published, Read) with(crate::allow);
                };
                labels(Stage, Capability) {
                    (Published, Write) with { 7u8 };
                };
            };
            postures {
                evolution {
                    empty(refused);
                    repetition(refused);
                    membership(closed, closed);
                    completeness(partial, partial);
                    density(sparse);
                    absence(allowed);
                    self_relation(refused);
                    cycle(refused);
                };
                policy {
                    repetition(refused);
                };
            };
            projections {
                companions;
                relation_tables {
                    evolution;
                    readable(allows);
                    policy {
                        /// Reads the caller-owned decision occupying one declared policy row.
                        pub fn decision(
                            stage: &Stage,
                            capability: &Capability,
                        ) -> Option<fn() -> bool>;
                    };
                    labels {
                        /// Reads an exact caller-owned payload without interpreting it.
                        pub fn label(stage: &Stage, capability: &Capability) -> Option<u8>;
                    };
                };
                typestate(Stage);
            };
        }
    }
}

macroonz::recipe! {
    /// A record using the existing compiler codec owner.
    pub mod codec {
        /// One caller-owned record.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Ledger {
            /// The recorded count.
            pub count: u16,
        }

        impl Ledger {
            /// Assemble one decoded ledger.
            #[must_use]
            pub const fn assembled(count: u16) -> Self {
                Self { count }
            }
        }

        bake! {
            codecs {
                ledger(Ledger) {
                    direction(round_trip);
                    refusal(LedgerDecodeError);
                    assembly(assembled, total);
                    members {
                        count: u16 => count(required);
                    };
                };
            };
            projections {
                codec;
            };
        }
    }
}

macroonz::recipe! {
    /// A transition using the conventional dispatch name.
    pub mod conventional {
        /// The caller-owned states.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The door is closed.
            Closed,
            /// The door is open.
            Open,
        }

        /// The caller-owned events.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Open the door.
            OpenDoor,
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections { dispatch; };
        }
    }
}

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
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(open);
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
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Open with(target) {
                    *calls = calls.saturating_add(1);
                    Ok(target)
                };
            };
            absence(refused);
            projections {
                dispatch(current, event) {
                    /// Applies one caller-declared transition or returns typed absence.
                    pub fn advance(
                        calls: &mut u32,
                        current: State,
                        event: Event,
                    ) -> Result<State, TransitionRefusal>;
                };
            };
        }
    }
}

fn observe_relations() {
    use generic::{Capability, Stage};

    assert_eq!(
        generic::baked::STAGE_VARIANTS,
        &[Stage::Draft, Stage::Published],
    );
    assert_eq!(
        generic::baked::POLICY_ROWS,
        &[
            (Stage::Draft, Capability::Read),
            (Stage::Published, Capability::Read),
        ],
    );
    assert_eq!(
        generic::baked::policy::decision(&Stage::Draft, &Capability::Read,)
            .map(|decision| decision()),
        Some(true),
    );
    assert!(generic::baked::policy::decision(&Stage::Draft, &Capability::Write,).is_none());
    let _draft = generic::baked::typestate::Stage::<generic::baked::typestate::Draft>::new();
    assert!(allow());

    assert!(generic::baked::evolution::contains(
        &Stage::Draft,
        &Stage::Published
    ));
    assert!(!generic::baked::evolution::contains(
        &Stage::Published,
        &Stage::Draft
    ));
    for (stage, capability, readable, label) in [
        (Stage::Draft, Capability::Read, true, None),
        (Stage::Draft, Capability::Write, false, None),
        (Stage::Published, Capability::Read, true, None),
        (Stage::Published, Capability::Write, false, Some(7u8)),
    ] {
        assert_eq!(
            generic::baked::readable::allows(&stage, &capability),
            readable
        );
        assert_eq!(generic::baked::labels::label(&stage, &capability), label);
    }
    assert_eq!(
        generic::baked::READABLE_ROWS,
        &[
            (Stage::Published, Capability::Read),
            (Stage::Draft, Capability::Read)
        ],
    );
}

fn observe_markers() {
    use single::baked::typestate::{After, Before, RecipeStage, Stage};

    assert_eq!(<Before as RecipeStage>::NAME, "Before");
    assert_eq!(<After as RecipeStage>::NAME, "After");
    assert_eq!(Stage::<Before>::new(), Stage(core::marker::PhantomData));
    assert_eq!(Stage::<After>::default(), Stage(core::marker::PhantomData));
}

fn observe_codec() {
    let ledger = codec::Ledger { count: 7 };
    let mut bytes = Vec::new();
    ledger.encode_canonical(&mut bytes);
    assert_eq!(bytes, [0u8, 0, 0, 0, 0, 0, 0, 7]);
    assert_eq!(codec::Ledger::decode_canonical(&bytes), Ok(ledger));
    assert!(codec::Ledger::decode_canonical(&[7u8]).is_err());
}

fn observe_dispatch() {
    assert_eq!(
        conventional::baked::apply(conventional::State::Closed, conventional::Event::OpenDoor),
        Ok(conventional::State::Open),
    );
    assert_eq!(
        conventional::baked::apply(conventional::State::Open, conventional::Event::OpenDoor),
        Err(conventional::baked::TransitionRefusal::Absent),
    );
    assert_eq!(
        configured::baked::open(configured::State::Closed, configured::Event::OpenDoor),
        Ok(configured::State::Open),
    );
    assert_eq!(
        configured::baked::open(configured::State::Open, configured::Event::OpenDoor),
        Err(configured::baked::TransitionRefusal::Absent),
    );
    let mut calls = 0u32;
    assert_eq!(
        exact::baked::advance(&mut calls, exact::State::Closed, exact::Event::OpenDoor),
        Ok(exact::State::Open),
    );
    assert_eq!(calls, 1);
    assert_eq!(
        exact::baked::advance(&mut calls, exact::State::Open, exact::Event::OpenDoor),
        Err(exact::baked::TransitionRefusal::Absent),
    );
    assert_eq!(calls, 1);
}

fn main() {
    observe_markers();
    observe_relations();
    observe_codec();
    observe_dispatch();
}

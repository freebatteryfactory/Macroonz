//! Generic structure, codec reuse, and transition dispatch through one recipe entrance.

#![forbid(unsafe_code)]

fn record_open() {}

const fn allow() -> bool {
    true
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
                policy(Stage, Capability) {
                    (Draft, Read) with(crate::allow);
                    (Published, Read) with(crate::allow);
                };
            };
            postures {
                policy {
                    repetition(refused);
                };
            };
            projections {
                companions;
                relation_tables {
                    policy {
                        /// Reads the caller-owned decision occupying one declared policy row.
                        pub fn decision(
                            stage: &Stage,
                            capability: &Capability,
                        ) -> Option<fn() -> bool>;
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
            vocabularies { State; Event; };
            transitions(State, Event) {
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
        generic::baked::STAGE_VARIANTS,
        &[generic::Stage::Draft, generic::Stage::Published],
    );
    assert_eq!(
        generic::baked::POLICY_ROWS,
        &[
            (generic::Stage::Draft, generic::Capability::Read),
            (generic::Stage::Published, generic::Capability::Read),
        ],
    );
    assert_eq!(
        generic::baked::policy::decision(&generic::Stage::Draft, &generic::Capability::Read,)
            .map(|decision| decision()),
        Some(true),
    );
    assert!(
        generic::baked::policy::decision(&generic::Stage::Draft, &generic::Capability::Write,)
            .is_none()
    );
    let _draft = generic::baked::typestate::Stage::<generic::baked::typestate::Draft>::new();
    assert!(allow());

    let ledger = codec::Ledger { count: 7 };
    let mut bytes = Vec::new();
    ledger.encode_canonical(&mut bytes);
    assert_eq!(codec::Ledger::decode_canonical(&bytes), Ok(ledger));

    assert_eq!(
        configured::baked::apply(configured::State::Closed, configured::Event::OpenDoor),
        Ok(configured::State::Open),
    );
    assert_eq!(
        exact::baked::advance(exact::State::Closed, exact::Event::OpenDoor),
        Ok(exact::State::Open),
    );
}

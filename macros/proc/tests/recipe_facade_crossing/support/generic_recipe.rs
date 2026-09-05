//! Independent generic recipe crossings through one renamed facade package.

pub(super) const GENERIC_PRODUCER: &str = r"#![forbid(unsafe_code)]
#![deny(warnings)]

bakery::recipe! {
    /// Ordinary Rust whose empty companion account emits no generated item.
    pub mod authored_only {
        /// One caller-owned newtype.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Revision(pub u64);

        /// One caller-owned marker.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Audited;

        bake! {
            projections {
                companions;
            };
        }
    }
}

bakery::recipe! {
    /// One enumerable vocabulary beside ordinary non-enumerated Rust items.
    pub mod vocabulary_shapes {
        /// One closed vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Stage {
            /// Work remains editable.
            Draft,
            /// Work is externally visible.
            Published,
        }

        /// One ordinary authored newtype.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Revision(pub u64);

        /// One ordinary authored marker.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Audited;

        bake! {
            vocabularies {
                Stage;
            };
            projections {
                companions;
            };
        }
    }
}

bakery::recipe! {
    /// One same-roster relation with explicit structural posture.
    pub mod evolution {
        /// One lifecycle vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Stage {
            /// Work remains editable.
            Draft,
            /// Work is externally visible.
            Published,
        }

        bake! {
            vocabularies {
                Stage;
            };
            relations {
                evolution(Stage, Stage) {
                    (Draft, Published);
                };
            };
            postures {
                evolution {
                    empty(refused);
                    repetition(refused);
                    membership(closed, closed);
                    completeness(partial, partial);
                    density(sparse);
                    self_relation(allowed);
                    cycle(allowed);
                };
            };
            projections {
                companions;
                relation_tables {
                    evolution;
                };
            };
        }
    }
}

bakery::recipe! {
    /// One cross-roster relation carrying ordinary Rust paths.
    pub mod policy {
        /// One lifecycle vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Stage {
            /// Editable work.
            Draft,
            /// Visible work.
            Published,
        }

        /// One capability vocabulary.
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
                    (Draft, Read) with(crate::policy_data::ALLOW);
                    (Published, Read) with(crate::policy_data::ALLOW);
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
                        pub fn lookup(
                            stage: &Stage,
                            capability: &Capability,
                        ) -> Option<crate::Decision>;
                    };
                };
            };
        }
    }
}

bakery::recipe! {
    /// One labeled many-to-many relation carrying exact Rust payloads.
    pub mod labels {
        /// One lifecycle vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Stage {
            /// Editable work.
            Draft,
            /// Visible work.
            Published,
        }

        /// One capability vocabulary.
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
                labels(Stage, Capability) {
                    (Draft, Write) with { crate::Decision::Review };
                    (Published, Read) with { crate::Decision::Audit };
                    (Published, Write) with { crate::Decision::Review };
                };
            };
            postures {
                labels {
                    repetition(refused);
                };
            };
            projections {
                companions;
                relation_tables {
                    labels {
                        pub fn lookup(
                            stage: &Stage,
                            capability: &Capability,
                        ) -> Option<crate::Decision>;
                    };
                };
            };
        }
    }
}

bakery::recipe! {
    /// One record projected through the existing codec owner.
    pub mod codec_record {
        /// Bytes admitted by one caller-owned conversion.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct EvenBytes(pub Vec<u8>);

        impl AsRef<[u8]> for EvenBytes {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl TryFrom<Vec<u8>> for EvenBytes {
            type Error = ();

            fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
                if bytes.first() == Some(&0xff) {
                    Err(())
                } else {
                    Ok(Self(bytes))
                }
            }
        }

        /// One closed caller-owned choice.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Choice {
            /// The first choice.
            First,
            /// The second choice.
            Second,
        }

        impl Choice {
            const ALL: [Self; 2] = [Self::First, Self::Second];

            const fn slot(self) -> u8 {
                match self {
                    Self::First => 0,
                    Self::Second => 1,
                }
            }
        }

        /// One caller-owned nested value.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Nested(pub u8);

        /// Why one nested value was not decoded.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct NestedRefusal;

        impl Nested {
            fn encode_canonical(&self, into: &mut Vec<u8>) {
                into.push(self.0);
            }

            fn decode_canonical(material: &[u8]) -> Result<Self, NestedRefusal> {
                match material {
                    [value] if *value != 0 => Ok(Self(*value)),
                    _ => Err(NestedRefusal),
                }
            }
        }

        /// One ordinary authored record spanning every codec member shape and cardinality.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Ledger {
            /// The recorded count.
            pub count: u16,
            /// Caller-validated bytes.
            pub payload: EvenBytes,
            /// One optional label.
            pub label: Option<String>,
            /// A repeated closed choice.
            pub modes: Vec<Choice>,
            /// One nested value.
            pub child: Nested,
        }

        impl Ledger {
            /// Assemble one decoded ledger.
            pub fn assembled(
                count: u16,
                payload: EvenBytes,
                label: Option<String>,
                modes: Vec<Choice>,
                child: Nested,
            ) -> Self {
                Self {
                    count,
                    payload,
                    label,
                    modes,
                    child,
                }
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
                        payload: crate::codec_record::EvenBytes => bytes(required);
                        label: String => text(optional);
                        modes: crate::codec_record::Choice => closed_choice(repeated);
                        child: crate::codec_record::Nested => nested(required);
                    };
                };
            };
            projections {
                codec;
            };
        }
    }
}

bakery::recipe! {
    /// The ergonomic transition spelling lowered through the generic account.
    pub mod transition {
        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The closed state.
            Closed,
            /// The open state.
            Open,
        }

        /// One event vocabulary.
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
                dispatch(apply);
            };
        }
    }
}

/// Caller-owned values referenced by ordinary Rust paths.
pub mod policy_data {
    /// The admitted policy decision.
    pub const ALLOW: super::Decision = super::Decision::Allow;
}

/// Caller-owned exact relation labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// The operation is admitted.
    Allow,
    /// Independent review.
    Review,
    /// Independent audit.
    Audit,
}

/// One caller-owned transition effect.
pub const fn record_open() {}
";

pub(super) const GENERIC_CONSUMER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

#[test]
fn authored_items_need_no_structural_projection() {
    use renamed_recipe_adopter::authored_only::{Audited, Revision};

    assert_eq!(Revision(7), Revision(7));
    assert_eq!(Audited, Audited);
}

#[test]
fn one_vocabulary_is_enumerated_without_absorbing_newtypes_or_markers() {
    use renamed_recipe_adopter::vocabulary_shapes::{Audited, Revision, Stage, baked};

    assert_eq!(baked::STAGE_VARIANTS, &[Stage::Draft, Stage::Published]);
    assert_eq!(Revision(7), Revision(7));
    assert_eq!(Audited, Audited);
}

#[test]
fn a_same_roster_evolution_relation_keeps_its_declared_rows() {
    use renamed_recipe_adopter::evolution::{Stage, baked};

    assert_eq!(baked::STAGE_VARIANTS, &[Stage::Draft, Stage::Published]);
    assert_eq!(baked::EVOLUTION_ROWS, &[(Stage::Draft, Stage::Published)]);
    assert!(baked::evolution::contains(&Stage::Draft, &Stage::Published));
    assert!(!baked::evolution::contains(&Stage::Published, &Stage::Draft));
}

#[test]
fn a_cross_roster_policy_keeps_right_members_and_path_payloads_distinct() {
    use renamed_recipe_adopter::Decision;
    use renamed_recipe_adopter::policy::{Capability, Stage, baked};

    assert_eq!(
        baked::POLICY_ROWS,
        &[
            (Stage::Draft, Capability::Read),
            (Stage::Published, Capability::Read),
        ]
    );
    assert_eq!(
        baked::policy::lookup(&Stage::Draft, &Capability::Read),
        Some(Decision::Allow)
    );
    assert_eq!(
        baked::policy::lookup(&Stage::Draft, &Capability::Write),
        None
    );
}

#[test]
fn a_labeled_many_to_many_relation_keeps_exact_payload_cardinality() {
    use renamed_recipe_adopter::Decision;
    use renamed_recipe_adopter::labels::{Capability, Stage, baked};

    assert_eq!(
        baked::LABELS_ROWS,
        &[
            (Stage::Draft, Capability::Write),
            (Stage::Published, Capability::Read),
            (Stage::Published, Capability::Write),
        ]
    );
    assert_eq!(
        baked::labels::lookup(&Stage::Published, &Capability::Read),
        Some(Decision::Audit)
    );
    assert_eq!(
        baked::labels::lookup(&Stage::Draft, &Capability::Read),
        None
    );
}

#[test]
fn a_record_uses_the_existing_codec_owner_through_the_facade() {
    use renamed_recipe_adopter::codec_record::{Choice, EvenBytes, Ledger, Nested};

    let ledger = Ledger {
        count: 513,
        payload: EvenBytes(vec![3, 4]),
        label: Some(String::from("hi")),
        modes: vec![Choice::First, Choice::Second],
        child: Nested(7),
    };
    let mut bytes = Vec::new();
    ledger.encode_canonical(&mut bytes);
    let mut expected = Vec::new();
    expected.extend_from_slice(&513_u64.to_be_bytes());
    bakery::compiler::encode_bytes(&[3, 4], &mut expected);
    expected.push(u8::from(true));
    bakery::compiler::encode_bytes(b"hi", &mut expected);
    expected.extend_from_slice(&2_u64.to_be_bytes());
    expected.extend_from_slice(&[0, 1]);
    bakery::compiler::encode_bytes(&[7], &mut expected);
    assert_eq!(bytes, expected);
    assert_eq!(Ledger::decode_canonical(&bytes), Ok(ledger));
}

#[test]
fn transitions_lower_to_callable_generated_rust() {
    use renamed_recipe_adopter::transition::{Event, State, baked};

    assert_eq!(baked::STATE_VARIANTS, &[State::Closed, State::Open]);
    assert_eq!(baked::EVENT_VARIANTS, &[Event::OpenDoor]);
    assert_eq!(
        baked::apply(State::Closed, Event::OpenDoor),
        Ok(State::Open)
    );
}
"#;

pub(super) const GENERIC_REFUSALS: [(&str, &str, &str); 8] = [
    (
        "foreign relation member",
        r"bakery::recipe! {
            pub mod foreign {
                pub enum Stage { Draft }
                pub enum Capability { Read }
                bake! {
                    vocabularies { Stage; Capability; };
                    relations { policy(Stage, Capability) { (Missing, Read); }; };
                    projections { companions; };
                }
            }
        }",
        "a relation row names undeclared `Stage` member `Missing`",
    ),
    (
        "duplicate relation row",
        r"bakery::recipe! {
            pub mod duplicate_row {
                pub enum Stage { Draft }
                pub enum Capability { Read }
                bake! {
                    vocabularies { Stage; Capability; };
                    relations {
                        policy(Stage, Capability) {
                            (Draft, Read);
                            (Draft, Read);
                        };
                    };
                    postures { policy { repetition(refused); }; };
                    projections { companions; };
                }
            }
        }",
        "relation `policy` states endpoint pair `Draft` and `Read` more than once",
    ),
    (
        "mixed relation payloads",
        r"bakery::recipe! {
            pub mod mixed_payloads {
                pub enum Stage { Draft }
                pub enum Capability { Read, Write }
                bake! {
                    vocabularies { Stage; Capability; };
                    relations {
                        policy(Stage, Capability) {
                            (Draft, Read) with(crate::allow);
                            (Draft, Write) with { crate::Decision::Review };
                        };
                    };
                    projections { companions; };
                }
            }
        }",
        "relation `policy` mixes `path` and `exact-rust` row payload contracts",
    ),
    (
        "posture without relation",
        r"bakery::recipe! {
            pub mod missing_relation {
                pub enum Stage { Draft }
                bake! {
                    vocabularies { Stage; };
                    postures { absent { empty(allowed); }; };
                    projections { companions; };
                }
            }
        }",
        "the recipe names no relation `absent`",
    ),
    (
        "relation without vocabularies",
        r"bakery::recipe! {
            pub mod missing_vocabularies {
                bake! {
                    relations { policy(Stage, Capability) { }; };
                    projections { companions; };
                }
            }
        }",
        "the recipe names no authored enum `Stage`",
    ),
    (
        "codec owner is not a record",
        r"bakery::recipe! {
            pub mod non_record_codec {
                pub enum Ledger { Empty }
                bake! {
                    codecs {
                        ledger(Ledger) {
                            direction(round_trip);
                            refusal(LedgerDecodeError);
                            assembly(assembled, total);
                            members { count: u16 => count(required); };
                        };
                    };
                    projections { codec; };
                }
            }
        }",
        "recipe codec `ledger` owner `Ledger` is not an authored record struct",
    ),
    (
        "duplicate codec name",
        r"bakery::recipe! {
            pub mod duplicate_codec {
                pub struct Ledger { pub count: u16 }
                impl Ledger { pub const fn assembled(count: u16) -> Self { Self { count } } }
                bake! {
                    codecs {
                        ledger(Ledger) {
                            direction(encode);
                            refusal(FirstDecodeError);
                            assembly(assembled, total);
                            members { count: u16 => count(required); };
                        };
                        ledger(Ledger) {
                            direction(encode);
                            refusal(SecondDecodeError);
                            assembly(assembled, total);
                            members { count: u16 => count(required); };
                        };
                    };
                    projections { codec; };
                }
            }
        }",
        "recipe codec `ledger` is declared more than once",
    ),
    (
        "ambiguous typestate subject",
        r"bakery::recipe! {
            pub mod ambiguous_typestate {
                pub enum Stage { Draft }
                pub enum Capability { Read }
                bake! {
                    vocabularies { Stage; Capability; };
                    projections { typestate; };
                }
            }
        }",
        "projection `typestate` requires one named vocabulary when several are declared",
    ),
];

//! Caller-owned record semantics and codec declarations through the root recipe entrance.

macroonz::recipe! {
    /// A record whose wire choices belong to its author.
    pub mod wire {
        /// One caller-owned closed choice.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Mode {
            /// Read the record.
            Read,
            /// Write the record.
            Write,
        }

        impl Mode {
            const ALL: [Self; 2] = [Self::Read, Self::Write];

            const fn slot(self) -> u8 {
                match self {
                    Self::Read => 0,
                    Self::Write => 1,
                }
            }
        }

        /// The caller's byte container.
        pub type Payload = Vec<u8>;

        /// A nested count with its own generated codec.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Child {
            /// The nested value.
            pub value: u16,
        }

        impl Child {
            /// Assemble a decoded nested count.
            #[must_use]
            pub const fn assembled(value: u16) -> Self {
                Self { value }
            }
        }

        /// A caller-declared record spanning the codec member shapes.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Record {
            /// The count checked by the caller's assembly.
            pub count: u16,
            /// Opaque caller bytes.
            pub payload: Payload,
            /// An optional UTF-8 label.
            pub label: Option<String>,
            /// An ordered sequence of caller choices.
            pub modes: Vec<Mode>,
            /// One framed nested value.
            pub child: Child,
        }

        /// Why the caller refused decoded fields.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum AssemblyRefusal {
            /// The record count is zero.
            ZeroCount,
        }

        impl Record {
            /// Admit decoded fields under the caller's positive-count rule.
            ///
            /// # Errors
            /// Refuses a zero count with `AssemblyRefusal::ZeroCount`.
            pub fn assembled(
                count: u16,
                payload: Payload,
                label: Option<String>,
                modes: Vec<Mode>,
                child: Child,
            ) -> Result<Self, AssemblyRefusal> {
                if count == 0 {
                    Err(AssemblyRefusal::ZeroCount)
                } else {
                    Ok(Self { count, payload, label, modes, child })
                }
            }
        }

        bake! {
            codecs {
                child(Child) {
                    direction(round_trip);
                    refusal(ChildDecodeError);
                    assembly(assembled, total);
                    members { value: u16 => count(required); };
                };
                record_write(Record) {
                    direction(encode);
                    refusal(RecordWriteError);
                    assembly(assembled, checked(crate::wire::AssemblyRefusal));
                    members {
                        count: u16 => count(required);
                        payload: crate::wire::Payload => bytes(required);
                        label: String => text(optional);
                        modes: crate::wire::Mode => closed_choice(repeated);
                        child: crate::wire::Child => nested(required);
                    };
                };
                record_read(Record) {
                    direction(decode);
                    refusal(RecordDecodeError);
                    assembly(assembled, checked(crate::wire::AssemblyRefusal));
                    members {
                        count: u16 => count(required);
                        payload: crate::wire::Payload => bytes(required);
                        label: String => text(optional);
                        modes: crate::wire::Mode => closed_choice(repeated);
                        child: crate::wire::Child => nested(required);
                    };
                };
            };
            projections { codec; };
        }
    }
}

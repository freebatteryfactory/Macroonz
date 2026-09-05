//! Construction for a lane that composes exploration with its own extra refusals.

macro_rules! declare_exploration_lane_failure {
    (
        $visibility:vis enum $name:ident {
            $($variant:ident($refusal:ty),)*
        }
    ) => {
        /// Everything this lane can refuse, carried as itself.
        $visibility enum $name {
            Name(NameRefusal),
            Strand(StrandRefusal),
            Set(StrandSetRefusal),
            Bound(ExplorationBoundRefusal),
            Exploration(ExplorationRefusal),
            Contract(ContractRefusal),
            $($variant($refusal),)*
            /// A reading did not carry the shape the claim demanded.
            Standing,
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
                    Self::Strand(refusal) => {
                        formatter.debug_tuple("Strand").field(refusal).finish()
                    }
                    Self::Set(refusal) => formatter.debug_tuple("Set").field(refusal).finish(),
                    Self::Bound(refusal) => formatter.debug_tuple("Bound").field(refusal).finish(),
                    Self::Exploration(refusal) => {
                        formatter.debug_tuple("Exploration").field(refusal).finish()
                    }
                    Self::Contract(refusal) => {
                        formatter.debug_tuple("Contract").field(refusal).finish()
                    }
                    $(Self::$variant(refusal) => formatter
                        .debug_tuple(stringify!($variant))
                        .field(refusal)
                        .finish(),)*
                    Self::Standing => formatter.write_str("Standing"),
                }
            }
        }

        impl From<NameRefusal> for $name {
            fn from(refusal: NameRefusal) -> Self {
                Self::Name(refusal)
            }
        }

        impl From<StrandRefusal> for $name {
            fn from(refusal: StrandRefusal) -> Self {
                Self::Strand(refusal)
            }
        }

        impl From<StrandSetRefusal> for $name {
            fn from(refusal: StrandSetRefusal) -> Self {
                Self::Set(refusal)
            }
        }

        impl From<ExplorationBoundRefusal> for $name {
            fn from(refusal: ExplorationBoundRefusal) -> Self {
                Self::Bound(refusal)
            }
        }

        impl From<ExplorationRefusal> for $name {
            fn from(refusal: ExplorationRefusal) -> Self {
                Self::Exploration(refusal)
            }
        }

        impl From<ContractRefusal> for $name {
            fn from(refusal: ContractRefusal) -> Self {
                Self::Contract(refusal)
            }
        }

        #[test]
        fn lane_failure_debug_shape_is_preserved() {
            assert_eq!(
                format!("{:?}", $name::Name(NameRefusal::EmptyNamespace)),
                "Name(EmptyNamespace)"
            );
            assert_eq!(format!("{:?}", $name::Standing), "Standing");
        }
    };
}

pub(crate) use declare_exploration_lane_failure;

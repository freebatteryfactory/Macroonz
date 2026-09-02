//! Negative-space producer and consumer specimens shared by the supported Rust editions.

pub(super) const NEGATIVE_SPACE_PRODUCER: &str = r"#![forbid(unsafe_code)]
#![deny(warnings)]

bakery::recipe! {
    /// A recipe whose structural and projection accounts are intentionally empty.
    pub mod empty_recipe {
        bake! {
            projections {
                companions;
            };
        }
    }
}

bakery::recipe! {
    /// Caller-authored Rust that asks Macroonz to add no production item.
    pub mod authored_only {
        /// One caller-owned value.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Revision(pub u64);

        /// Reads the caller-owned value without generated mediation.
        pub const fn revision(value: Revision) -> u64 {
            value.0
        }

        bakery::recipe! {
            /// One nested recipe with its own module-local generated namespace.
            pub mod nested {
                /// One nested caller-owned vocabulary.
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum Mode {
                    /// The only declared mode.
                    Ready,
                }

                bake! {
                    vocabularies {
                        Mode;
                    };
                    projections {
                        companions;
                    };
                }
            }
        }

        bake! {
            projections {
                companions;
            };
        }
    }
}

bakery::recipe! {
    /// One vocabulary with no relation or behavioral projection.
    pub mod vocabulary_only {
        /// One closed caller-owned vocabulary.
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
            projections {
                companions;
            };
        }
    }
}

bakery::recipe! {
    /// Handwritten production behavior beside explicitly unavailable evidence roads.
    pub mod handwritten_with_unavailable_evidence {
        /// Doubles one caller-owned value.
        pub const fn double(value: u8) -> u8 {
            value.saturating_mul(2)
        }

        bake! {
            projections {
                companions;
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

pub(super) const NEGATIVE_SPACE_CONSUMER: &str = r"#![forbid(unsafe_code)]
#![deny(warnings)]

#[test]
fn authored_rust_and_empty_accounts_need_no_generated_production_item() {
    use renamed_recipe_adopter::authored_only::{Revision, revision};

    assert_eq!(revision(Revision(7)), 7);
    assert_eq!(
        renamed_recipe_adopter::handwritten_with_unavailable_evidence::double(9),
        18
    );
}

#[test]
fn one_vocabulary_needs_no_relation() {
    use renamed_recipe_adopter::vocabulary_only::{Stage, baked};

    assert_eq!(baked::STAGE_VARIANTS, &[Stage::Draft, Stage::Published]);
    assert_eq!(
        renamed_recipe_adopter::authored_only::nested::baked::MODE_VARIANTS,
        &[renamed_recipe_adopter::authored_only::nested::Mode::Ready]
    );
}
";

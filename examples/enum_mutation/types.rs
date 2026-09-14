//! The caller's two declared vocabularies and their independent permissions.

/// The caller's priority vocabulary with explicit representation and discriminants.
#[macroonz::macros::mutations(
    module = priorities,
    refusal = PriorityRefusal,
    support = priority_support,
    family = named("priority", "order"),
    point = named("priority", "declared-order"),
    fact = named("priority", "urgency"),
    map named("priority", "urgency") = named("priority", "urgent-first"),
    permit named("priority", "urgent-first") = ["declared-order-permutation"],
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Priority {
    /// Work that must be handled first.
    Urgent = 7,
    /// Ordinary pending work.
    Normal = 11,
    /// Work the caller may defer.
    Deferred = 23,
}

macroonz::recipe! {
    /// A separate recipe whose discovered order has no permission to execute mutations.
    pub mod pipeline {
        /// The caller's pipeline vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Phase {
            /// A declared input has arrived.
            Received,
            /// The caller has checked that input.
            Checked,
            /// The caller has released its result.
            Released,
        }

        bake! {
            vocabularies { Phase; };
            evidence {
                mutation(Phase) {
                    module = phases,
                    refusal = PhaseRefusal,
                    support = pipeline_support,
                    family = named("pipeline", "order"),
                    point = named("pipeline", "declared-order"),
                    fact = named("pipeline", "processing-order"),
                    map named("pipeline", "processing-order") = named("pipeline", "checked-before-release"),
                    permit named("pipeline", "unrelated-claim") = ["declared-order-permutation"],
                };
            };
        }
    }
}

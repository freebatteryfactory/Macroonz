//! The deliberate artifact damages the harness can seed without claiming an observation.

macro_rules! artifact_mutation_bank {
    ($callback:ident) => {
        $callback! {
            /// The emitted members are written in reverse of the order the declaration states.
            OrderPermuted => "the emitted members are written in reverse of declared order",
            /// Every emitted member is written under the first member's key, so members the declaration keeps distinct share one identity.
            IdentityRecycled => "every emitted member is written under the first member's key",
            /// One planned output is deleted from the artifact.
            PlannedOutputOmitted => "a planned output is deleted",
            /// An output nobody planned is appended.
            UnplannedOutputAdded => "an unplanned output is appended",
            /// The implementation targets a different type than the one declared.
            ImplTargetAltered => "the implementation targets a different type",
            /// The declared body shape is changed.
            ShapeAltered => "the declared body shape is changed",
            /// A planned output is emitted twice.
            OutputDuplicated => "a planned output is emitted twice",
            /// The trait path names a contract the declaration did not realize.
            TraitPathWrong => "the trait path names a different contract",
            /// A decoy carrying the anchored bytes is planted in a comment while the real constant is damaged.
            DecoyInComment => "the anchored bytes are planted in a comment",
            /// One planned member constant is emitted twice inside one implementation.
            ImplMemberDuplicated => "one member constant is emitted twice",
            /// A member nobody planned is added inside one implementation.
            ImplMemberUnexpected => "a member nobody planned joins the implementation",
            /// A declared value is carried through a constructor the declaration did not name.
            ConstructorPathAltered => "a row is built through another constructor",
            /// The implementation is written under a posture the declaration did not name.
            ImplPostureAltered => "the implementation is written under another posture",
            /// An attribute that decides something is added to an implementation.
            MeaningBearingAttributeAdded => "an attribute that decides something is added",
            /// The artifact stops being well-formed Rust.
            MalformedRust => "the artifact stops being well-formed Rust",
        }
    };
}

pub(crate) use artifact_mutation_bank;

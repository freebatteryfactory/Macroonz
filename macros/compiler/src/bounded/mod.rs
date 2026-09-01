#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    AbsencePosture, Bounded, Capped, Capping, CompletenessPosture, CompletenessStanding,
    CyclePosture, CycleStanding, DensityPosture, DensityStanding, DuplicateKey, Empty,
    EmptyPosture, ForeignRosterReference, KeyedRoster, KeyedRosterAssignment,
    KeyedRosterAssignmentError, KeyedRosterError, KeyedRosterRelation, KeyedRosterRows,
    KeyedRosterRowsError, MembershipPosture, NonEmpty, NonEmptyError, OccupancyStanding, Overflow,
    Reachability, ReachabilityError, RepeatedRelationPair, RepeatedRelationPairs,
    RepetitionPosture, RepetitionStanding, RosterRelationStanding, RowOrder, SameRosterRequired,
    SelfRelationPosture, SelfRelationStanding, StructuralMismatch, StructuralRequirement,
    UnassignedRosterMember,
};

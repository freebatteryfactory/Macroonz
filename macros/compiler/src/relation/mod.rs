#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    AbsencePosture, CompletenessPosture, CompletenessStanding, CyclePosture, CycleStanding,
    DensityPosture, DensityStanding, EmptyPosture, KeyedRosterRelation, KeyedRosterRows,
    KeyedRosterRowsError, MembershipPosture, OccupancyStanding, Reachability, ReachabilityError,
    RepeatedRelationPair, RepeatedRelationPairs, RepetitionPosture, RepetitionStanding,
    RosterRelationStanding, RowOrder, SameRosterRequired, SelfRelationPosture,
    SelfRelationStanding, StructuralMismatch, StructuralRequirement,
};

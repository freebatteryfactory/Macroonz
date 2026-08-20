#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::{plan_scope_guard_stamp, stamped_unit_plan};
pub use render::{
    ADMITTED_PREFIX, BODY_SEAT, BOUND_PARAMETER, BOUNDED_CLAUSE, BY_CLAUSE, CALLER_REACH_PARAMETER,
    CARRIED_ROAD, COMPLETION_POSTURE, COMPLETION_ROAD, DECLARED_ISSUE_BOUND, ESTABLISHED_CLAUSE,
    ESTABLISHED_ROAD, ESTABLISHED_SENTENCE, EXAMINED_ROAD, FAMILY_PARAMETER, FAMILY_SHAPE,
    HOME_PARAMETER, IDENT_FRAGMENT, IN_CLAUSE, INHABITED_ROAD, INTERNAL_REACH_PARAMETER,
    ISSUE_COLLECTION, ISSUE_PARAMETER, ISSUES_ROAD, ISSUES_SENTENCE, META_FRAGMENT,
    NON_EMPTY_BOUNDED, NOTE_PARAMETER, OPAQUE_REACH_PARAMETER, OPAQUE_REACH_REFUSAL, OVER_CLAUSE,
    POSITIVE_LIMIT, POSTURE_ROAD, POSTURE_SENTENCE, PROFILE_PARAMETER, REFUSAL_FAMILY,
    REFUSAL_HOME, SEATED_CLAUSE, SELECTION_ORDER_SEAT, SHAPE_SEAT, STAMP_SENTENCE, STOP_BOUND,
    TRANSCRIBE_ARM, TRANSCRIBE_MINTING_ARM, TYPE_FRAGMENT, TYPES_HOME, UNDER_CLAUSE, VIS_FRAGMENT,
    attribute, declaration_forward, declaration_matcher, declared_reach, derive_attribute,
    documentation, fragment, front_arm, group, machine_path, metavariable, note_forward,
    note_matcher, obligation, refusing_arm, rule, seat_invocation, seat_module, seat_path,
    stamp_definition, transcribe_arm, transported_reach, unbounded,
};
pub use types::{
    CoupledSeatDeclaration, InsufficiencyGround, PublishedSeatStamp, ScopeGuardOwnerFacts,
    ScopeGuardStampAnchors, SeatDeclarationLimit, SeatDeclarationRefusal, SeatMint, SeatMintForm,
    SeatNames, SeatPath, SeatPathSegmentLimit, SeatProse, SeatSeating, SeatVisibility,
    StampCoverage, StampName, StampPublicationRecord, StampRenderIssue, StampedSeat,
    StampedUnitPlan, StampedUnitPlanIssue, TransportedReach,
};

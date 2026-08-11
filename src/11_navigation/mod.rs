//! Band 11 — navigation: the semantic address space, routes, `Fix<T>`, the
//! positioning refusal, bounded traversal, typed paths, paging, cursors, and
//! logical time-travel inspection.

pub mod types;

pub use types::{
    Address, AddressRole, AddressSpace, AdmittedRoute, AlternativeLimit, Axis, AxisCapability,
    AxisCapabilityLimit, CHECKPOINT_NON_ADVANCERS, CLOSURE_REQUIRED_CLAIMS, Cursor,
    CursorDirection, CursorTransplantation, DestinationKind, DomainPosture, ExactnessPosture,
    FUSIBLE_FOLD_OUTPUTS, Fix, FixShape, FrameTransformation, FrameVersion,
    HistoricalReconstruction, INCOMPARABLE_ROUTE_DIMENSIONS, JournalView,
    MultiAuthorityRelationship, MultiplicityPosture, NavigationRequest, PATH_CONTRACT_FACETS,
    PROHIBITED_SILENT_MERGERS, PageDowngradeTrigger, PathSelector, PointRole, PositioningRefusal,
    RECONSTRUCTABLE_FACETS, ReferenceFrameId, RegionRole, RelationRole, ResolvedRoute,
    ReversibilityPosture, RouteClosureEvidence, SemanticPathProgram, SupportDistinction,
    TraversalForm,
};

//! A compiler for declared code: one complete request goes in, one sealed expansion comes out, or one diagnostic that says exactly why not.
//!
//! The crate README is the specification — what a kind is, what each step of the road settles, and which half of the work is the caller's.
//! Every home below carries its own README as its module documentation.
//! The module list is a reading order: homes cite their semantic owners directly, including the request-owned door whose diagnostic projections are consumed by the diagnostic home, and the crate depends on nothing else in this workspace.

#[cfg(feature = "host")]
extern crate proc_macro;

pub mod bounded;
pub mod identity;
pub mod token;
pub mod kind;
pub mod origin;
pub mod diagnostic;
pub mod plan;
pub mod render;
pub mod closure;
pub mod explanation;
pub mod expansion;
pub mod support;
pub mod descriptor;
pub mod codec;
pub mod stamp;
pub mod request;

#[cfg(feature = "host")]
pub mod host;

pub use bounded::{
    Bounded, Capped, Capping, DuplicateKey, Empty, ForeignRosterReference, KeyedRoster,
    KeyedRosterAssignment, KeyedRosterAssignmentError, KeyedRosterError, NonEmpty, NonEmptyError,
    Overflow, UnassignedRosterMember,
};
pub use closure::{
    CLOSURE_ISSUE_LIMIT, CarriedTokens, Closure, ClosureError, ClosureIssue, PartitionCargo,
    PartitionedEmission,
};
pub use diagnostic::{
    ASSEMBLY_FAMILY, BENCH_HELPER_FAMILY, BINDING_FAMILY, CAPTURE_FAMILY, CLOSURE_FAMILY,
    CODEC_DECLARATION_FAMILY, CONCURRENCY_HELPER_FAMILY, DECLARATION_FAMILY, Diagnostic,
    DiagnosticName, DiagnosticNameRefusal, EXPLANATION_FAMILY, FIRST_HELPER_FAMILY, Family, Line,
    LineBody, LineSite, NETWORK_HELPER_FAMILY, Observed, PLANNING_FAMILY, Phase, Placement,
    RELATED_ISSUE_LIMIT, RENDERING_FAMILY, REPAIR_LIMIT, RefusalClass, Refused, RelatedIdentity,
    RelatedSet, RenderedMagnitude, Repair, Route, SECOND_HELPER_FAMILY, SHADOW_HELPER_FAMILY,
    SHELL_FAMILY, SUPPORT_DECLARATION_FAMILY, Site, SiteCoordinate, composed,
};
pub use expansion::{Accounted, BINDING_FACT, BindError, Expansion};
pub use explanation::{
    ASSUMPTION_LIMIT, AnsweredOutput, DECLARED_QUESTION_LIMIT, EXPLANATION_ISSUE_LIMIT,
    ExplanationError, ExplanationIssue, RELATED_KIND_LIMIT, RelatedDisposition,
    UNIVERSAL_QUESTION_COUNT, UniversalAnswer, UniversalQuestion, View,
};
pub use identity::{
    Anchoring, BUNDLE_PROFILE, CAPTURED_DECLARATION_PROFILE, CAPTURED_HELPER_PROFILE,
    CLOSED_EXPANSION_PROFILE, CLOSURE_PROFILE, ClosedExpansionId, ClosureId,
    DECLARATION_DOCUMENTATION_PROFILE, DECLARED_NAME_PROFILE, DIAGNOSTIC_RELATION_PROFILE,
    EXPLANATION_PROFILE, ExplanationId, GENERATED_UNIT_PROFILE, GENERATOR,
    GENERATOR_VERSION_PROFILE, GeneratorIdentity, HUMAN_TEXT_LIMIT, HumanProjection, Identity,
    MACROONZ_STEM, ORIGIN_NODE_PROFILE, OwnerFact, OwnerIdentity, PLAN_PROFILE,
    PROJECTION_CONTENT_PROFILE, PROJECTION_INTENT_PROFILE, PROJECTION_KIND_PROFILE, PlanId,
    Profile, Provenance, RENDERED_UNIT_PROFILE, ShapeVersion, Subject, Transcript, Version,
    encode_bytes, encode_length, names_are_separating,
};
pub use kind::{
    Answer, CanonicalContent, Destination, Disposition, DispositionRecord, DispositionSet,
    DispositionSetError, Kind, KindSet, NoQuestions, Question, Role, SoleRole,
};
pub use origin::{
    DecisionTrace, Nonclaim, ORIGIN_EDGE_LIMIT, OriginEdge, OriginRelation, OriginTrail,
    TRACE_ENTRY_LIMIT, TraceDecision, TraceEntry, TrailError,
};
pub use plan::{
    Account, BoundAxis, ContentBinding, Context, ContradictionPair, DEPENDENCY_LIMIT,
    DigestContract, Intent, InvalidationSet, InvalidationTrigger, MEMBERSHIP_LIMIT, Membership,
    NONCLAIM_LIMIT, PLAN_ISSUE_LIMIT, Plan, PlanDecisions, PlanError, PlanIssue, PlannedMember,
    PlannedOutput, TRIGGER_LIMIT,
};
pub use render::{Output, RENDERED_BYTE_LIMIT, RenderError, RenderedProjection, RenderedUnit};
pub use request::{
    CrateBinding, Door, Producer, RUST_DECLARATION_PROFILE, Request, SELECTION_FACT, bound_content,
};
pub use token::{
    CAPTURE_WORK_LIMIT, CAPTURED_TOKEN_LIMIT, CAPTURED_TREE_TOKEN_LIMIT, CaptureBound,
    CaptureBuildRefusal, CaptureBuilder, CaptureLevel, CaptureWalk, CapturedAtom,
    CapturedDelimiter, CapturedInput, CapturedPayload, CapturedTokenTree, CoordinateRole,
    GENERATED_TOKEN_LIMIT, GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree,
    LiteralReadCause, SourceCoordinate, SpanHandle, SpanResolutionRefusal, SpanTable,
    TEXT_SOURCE_BYTE_LIMIT, TOKEN_PATH_DEPTH_LIMIT, TextCapture, TextLexicalCause, TextReadCause,
    TextReadRefusal, TokenPath, absolute_path, and_all, attribute, bound_local, bound_path, call,
    capture_literal, comma, comma_many, constant, documentation, equality, function, group,
    keyed_assignment_slice, keyed_roster_slice, metavariable, method_call, method_chain,
    rendered_identifier, rendered_name, result_type, roster, rust_keyword, text_pair, twin_path,
};

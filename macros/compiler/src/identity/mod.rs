#![doc = include_str!("README.md")]

mod bank;
mod encode;
mod stamp;
mod transcript;
mod type_contract;
mod types;

pub use bank::{
    BUNDLE_PROFILE, CAPTURED_DECLARATION_PROFILE, CAPTURED_HELPER_PROFILE,
    CLOSED_EXPANSION_PROFILE, CLOSURE_PROFILE, DECLARATION_DOCUMENTATION_PROFILE,
    DECLARED_NAME_PROFILE, DIAGNOSTIC_RELATION_PROFILE, EXPLANATION_PROFILE,
    GENERATED_UNIT_PROFILE, GENERATOR_VERSION_PROFILE, ORIGIN_NODE_PROFILE, PLAN_PROFILE,
    PROJECTION_CONTENT_PROFILE, PROJECTION_INTENT_PROFILE, PROJECTION_KIND_PROFILE,
    RENDERED_UNIT_PROFILE,
};
pub use encode::{encode_bytes, encode_length};
pub use types::{
    Anchoring, CapturedDeclaration, CapturedHelper, ClosedExpansion, ClosedExpansionId, Closure,
    ClosureId, Contract, DeclaredName, Explanation, ExplanationId, GENERATOR, GeneratedUnit,
    GeneratorIdentity, GeneratorVersion, HUMAN_TEXT_LIMIT, HumanProjection, Identity,
    MACROONZ_STEM, Nonclaim, OriginNode, OutputBytes, OwnerFact, OwnerIdentity, Plan, PlanId,
    Profile, ProjectionContent, ProjectionIntent, ProjectionKind, ProjectionProfile, Provenance,
    RelatedBody, RelatedIssue, RenderedUnit, Role, ServiceEntry, ShapeVersion, Subject, Traced,
    Transcript, Version, names_are_separating,
};
pub(crate) use types::{human_projection, name_is_grammatical, static_bytes};

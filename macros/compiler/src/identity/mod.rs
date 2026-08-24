#![doc = include_str!("README.md")]

mod encode;
mod transcript;
mod type_contract;
mod types;

pub use encode::{encode_bytes, encode_length};
pub use types::{
    Anchoring, BUNDLE_PROFILE, CAPTURED_DECLARATION_PROFILE, CAPTURED_HELPER_PROFILE,
    CLOSED_EXPANSION_PROFILE, CLOSURE_PROFILE, CapturedDeclaration, ClosedExpansion,
    ClosedExpansionId, Closure, ClosureId, Contract, DECLARATION_DOCUMENTATION_PROFILE,
    DECLARED_NAME_PROFILE, DIAGNOSTIC_RELATION_PROFILE, DeclaredName, EXPLANATION_PROFILE,
    Explanation, ExplanationId, GENERATED_UNIT_PROFILE, GENERATOR, GENERATOR_VERSION_PROFILE,
    GeneratedUnit, GeneratorIdentity, GeneratorVersion, HUMAN_TEXT_LIMIT, HumanProjection,
    Identity, MACROONZ_STEM, Nonclaim, ORIGIN_NODE_PROFILE, OriginNode, OutputBytes, OwnerFact,
    OwnerIdentity, PLAN_PROFILE, PROJECTION_INTENT_PROFILE, Plan, PlanId, Profile,
    ProjectionIntent, ProjectionKind, ProjectionProfile, Provenance, RENDERED_UNIT_PROFILE,
    RelatedBody, RelatedIssue, RenderedUnit, Role, ServiceEntry, ShapeVersion, Subject, Traced,
    Transcript, Version, names_are_separating,
};
pub(crate) use types::{human_projection, static_bytes};

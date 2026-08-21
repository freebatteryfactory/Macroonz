#![doc = include_str!("README.md")]

mod encode;
mod transcript;
mod type_contract;
mod types;

pub use encode::{encode_bytes, encode_length};
pub use types::{
    ApplicationDistinctnessSubject, AssumptionLimit, AuthoringLimitProfile,
    BUNDLE_IDENTITY_PROFILE, BoundFormulaSubject, BundleSubject, ByteRoleSubject,
    CAPTURED_DECLARATION_IDENTITY_PROFILE, CLOSED_EXPANSION_IDENTITY_PROFILE,
    CLOSURE_IDENTITY_PROFILE, CapturedDeclarationSubject, CapturedTokenLimit, ClosedExpansionId,
    ClosedExpansionSubject, ClosureId, ClosureSubject, ContractSubject,
    DECLARATION_DOCUMENTATION_IDENTITY_PROFILE, DECLARED_NAME_IDENTITY_PROFILE,
    DIAGNOSTIC_RELATION_IDENTITY_PROFILE, DerivedTypeSubject, DescriptorProviderSubject,
    DocumentedSubject, EXPLANATION_IDENTITY_PROFILE, ExpansionSurfaceSubject, ExplanationId,
    ExplanationSubject, FacetLimit, FixturePopulationSubject, GENERATED_UNIT_IDENTITY_PROFILE,
    GENERATOR_VERSION_IDENTITY_PROFILE, GeneratedTokenLimit, GeneratedUnitSubject,
    GeneratorIdentity, GeneratorProfileId, GeneratorSchemaVersion, GeneratorVersionSubject,
    HumanProjection, HumanTextLimit, IDENTITY_PROFILE_STEM, IdentityProfile,
    IdentityProfileVersion, IdentitySubject, ImplementedContractSubject, InputDescriptorSubject,
    LanguageProfileSubject, MACROC_GENERATOR, MeasuredSubject, MechanismProfileSubject,
    MembershipLimit, MetaProfileSubject, NonclaimLimit, NonclaimSubject,
    ORIGIN_NODE_IDENTITY_PROFILE, ObligationSubject, OriginNodeSubject, OutputBytesSubject,
    OwnerFactName, OwnerFactRef, OwnerFactSubject, OwnerHomeSubject, OwnerIdentityRef,
    PLAN_IDENTITY_PROFILE, PROJECTION_INTENT_IDENTITY_PROFILE, PatternArgumentLimit,
    PatternArgumentSubject, PatternInstanceSubject, PatternSubject, PlanId, PlanSubject,
    PortSubject, PreimageFamily, ProfileVersion, ProjectionIdentity, ProjectionIntentSubject,
    ProjectionKindSubject, ProjectionProfileSubject, ProjectionProvenance, ProjectionRole,
    ProjectionTranscript, RENDERED_UNIT_IDENTITY_PROFILE, RefusalFamilySubject, RefusalReason,
    RelatedBodySubject, RelatedIssueSubject, RenderedByteLimit, RenderedRole, RenderedRoleSeal,
    RenderedUnitSubject, RepairLimit, RuntimeTraceSubject, SchemaSubject, ServiceEntrySubject,
    SoleRenderedUnit, SourceSnapshotSubject, SubjectSeal, TRIAL_DECLARATION_IDENTITY_PROFILE,
    TemplateArgumentSubject, TemplateParameterSubject, TemplateSubject, TraceEntryLimit,
    TracedSubject, TranscriptAnchoring, WireContractSubject, WorkCurrencySubject,
    WorkFormulaSubject, WrapperComponentLimit,
};
pub(crate) use types::{human_projection, limits, names_are_separating, static_bytes};

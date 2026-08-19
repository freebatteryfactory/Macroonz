#![doc = include_str!("README.md")]

mod encode;
mod transcript;
mod type_contract;
mod types;

pub use encode::{encode_bytes, encode_length};
pub use types::{
    ApplicationDistinctnessSubject, AssumptionLimit, AuthoringLimitProfile, BUNDLE_IDENTITY_PROFILE,
    BoundFormulaSubject, BundleMemberLimit, BundleSubject, ByteRoleSubject,
    CAPTURED_DECLARATION_IDENTITY_PROFILE, CLOSED_EXPANSION_IDENTITY_PROFILE,
    CLOSURE_IDENTITY_PROFILE, CaptureWorkLimit, CapturedDeclarationSubject, CapturedTokenLimit,
    CapturedTreeTokenLimit, ClosedExpansionId, ClosedExpansionSubject, ClosureId,
    ClosureIssueLimit, ClosureSubject, CompositionIssueLimit, ContractSubject,
    DECLARATION_DOCUMENTATION_IDENTITY_PROFILE, DECLARED_NAME_IDENTITY_PROFILE,
    DIAGNOSTIC_RELATION_IDENTITY_PROFILE, DeriveCauseLimit,
    DeriveSourceLimit, DerivedTypeSubject, DescriptorProviderLimit, DescriptorProviderSubject,
    DocumentedSubject, EXPLANATION_IDENTITY_PROFILE, ExpansionSurfaceSubject,
    ExplanationId, ExplanationIssueLimit, ExplanationSeatLimit, ExplanationSubject, FacetLimit,
    FixturePopulationSubject,
    FragmentDependencyLimit, GENERATED_UNIT_IDENTITY_PROFILE, GENERATOR_VERSION_IDENTITY_PROFILE,
    GeneratedTokenLimit,
    GeneratedUnitSubject, GeneratorIdentity, GeneratorProfileId, GeneratorSchemaVersion,
    GeneratorVersionSubject, HumanProjection, HumanTextLimit, IDENTITY_PROFILE_STEM,
    IdentityProfile, IdentityProfileVersion, IdentitySubject, ImplementedContractSubject,
    InputDescriptorLimit, InputDescriptorSubject, InvalidationLimit, LanguageProfileSubject,
    MACROC_GENERATOR, MeasuredSubject, MechanismProfileSubject, MembershipLimit, MetaBoundAxisLimit,
    MetaProfileSubject, MutationAlternativeLimit, MutationPointLimit, NonclaimLimit,
    NonclaimSubject, ORIGIN_NODE_IDENTITY_PROFILE, ObligationSubject, OriginEdgeLimit,
    OriginNodeSubject, OutputBytesSubject, OwnerFactName, OwnerFactRef, OwnerFactSubject,
    OwnerHomeSubject, OwnerIdentityRef, PLAN_IDENTITY_PROFILE, PROJECTION_INTENT_IDENTITY_PROFILE,
    PatternArgumentLimit, PatternArgumentSubject, PatternInstanceSubject, PatternSubject, PlanId,
    PlanSubject, PlanningIssueLimit, PortSubject, PreimageFamily, ProfileVersion,
    ProjectionIdentity, ProjectionIntentSubject, ProjectionKindSubject, ProjectionProfileSubject,
    ProjectionProvenance, ProjectionRole, ProjectionTranscript, RENDERED_UNIT_IDENTITY_PROFILE,
    RefusalFamilySubject, RefusalReason, RelatedBodySubject, RelatedIssueLimit, RelatedIssueSubject,
    RenderedByteLimit, RenderedRole, RenderedRoleSeal, RenderedUnitSubject, RepairLimit,
    RuntimeTraceSubject, SchemaSubject, SelectionCitationLimit, ServiceEntrySubject,
    SoleRenderedUnit, SourceDeclarationLimit, SourceSnapshotSubject, SubjectSeal, SurfaceIssueLimit,
    TemplateArgumentSubject, TemplateIssueLimit, TemplateParameterLimit, TemplateParameterSubject,
    TemplateSubject, TokenPathDepthLimit, TraceEntryLimit, TracedSubject, TranscriptAnchoring,
    TriggerViewIssueLimit, WireContractSubject, WorkCurrencySubject, WorkFormulaSubject,
    WrapperComponentLimit,
};
pub(crate) use types::{human_projection, limits, static_bytes};

#[cfg(test)]
pub(crate) use types::{DECLARED_LIMITS, SUBJECT_NAMES, for_laws};

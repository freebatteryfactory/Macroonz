#![doc = include_str!("README.md")]

mod encode;
mod transcript;
mod type_contract;
mod types;

pub use encode::{encode_bytes, encode_length};
pub use types::{
    ApplicationDistinctnessSubject, AssumptionLimit, AuthoringLimitProfile, BoundFormulaSubject,
    BundleMemberLimit, BundleSubject, ByteRoleSubject, CapturedDeclarationSubject,
    CapturedTokenLimit, CapturedTreeTokenLimit, ClosedExpansionId, ClosedExpansionSubject,
    ClosureId, ClosureIssueLimit, ClosureSubject, CompositionIssueLimit, ContractSubject,
    DeriveCauseLimit, DeriveSourceLimit, DerivedTypeSubject, DescriptorProviderLimit,
    DescriptorProviderSubject, DocumentedSubject, ExpansionSurfaceSubject, ExplanationIssueLimit,
    ExplanationSeatLimit, FacetLimit, FixturePopulationSubject, FragmentDependencyLimit,
    GeneratedTokenLimit, GeneratedUnitSubject, GeneratorIdentity, GeneratorProfileId,
    GeneratorSchemaVersion, GeneratorVersionSubject, HumanProjection, HumanTextLimit,
    IdentityProfile, IdentityProfileVersion, IdentitySubject, ImplementedContractSubject,
    InputDescriptorLimit, InputDescriptorSubject, InvalidationLimit, LanguageProfileSubject,
    MACROC_GENERATOR, MeasuredSubject, MechanismProfileSubject, MembershipLimit,
    MetaBoundAxisLimit, MetaProfileSubject, NonclaimLimit, NonclaimSubject, ObligationSubject,
    OriginEdgeLimit, OriginNodeSubject, OutputBytesSubject, OwnerFactName, OwnerFactRef,
    OwnerFactSubject, OwnerHomeSubject, OwnerIdentityRef, PROJECTION_IDENTITY_PROFILE,
    PatternArgumentLimit, PatternArgumentSubject, PatternInstanceSubject, PatternSubject, PlanId,
    PlanSubject, PlanningIssueLimit, PortSubject, ProfileVersion, ProjectionIdentity,
    ProjectionKindSubject, ProjectionProfileSubject, ProjectionProvenance, ProjectionRole,
    ProjectionTranscript, RefusalFamilySubject, RefusalReason, RelatedIssueLimit,
    RelatedIssueSubject, RenderedByteLimit, RenderedRole, RenderedRoleSeal, RenderedUnitSubject,
    RepairLimit, RuntimeTraceSubject, SchemaSubject, SelectionCitationLimit, ServiceEntrySubject,
    SoleRenderedUnit, SourceDeclarationLimit, SourceSnapshotSubject, SubjectSeal,
    TemplateArgumentSubject, TemplateIssueLimit, TemplateParameterLimit, TemplateParameterSubject,
    TemplateSubject, TokenPathDepthLimit, TraceEntryLimit, TracedSubject, TranscriptAnchoring,
    TriggerViewIssueLimit, WireContractSubject, WorkCurrencySubject, WorkFormulaSubject,
    WrapperComponentLimit,
};
pub(crate) use types::{human_projection, static_bytes};

#[cfg(test)]
pub(crate) use types::{DECLARED_LIMITS, SUBJECT_NAMES, for_laws};

#![doc = include_str!("README.md")]

mod anchor;
mod encode;
mod type_contract;
mod types;

pub use types::{
    BenchmarkDescriptorContent, BenchmarkDescriptorProjection, BundleMemberLimit,
    CapturedDependencies, CauseAnchoring, CodecContent, CodecDirection, CodecProjection,
    ContentAddressing, DeclaredBootstrap, DeriveImplContent, DeriveImplProjection, DigestContract,
    DocumentationContent, DocumentationProjection, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
    EmissionPartition, ExpectedGeneratedSupportSchemaId,
    GENERATED_SUPPORT_SCHEMA_DECLARED_BOOTSTRAP, GraphAnchoring, HostWrapperContent,
    HostWrapperProjection, InvalidationLimit, InvalidationSet, InvalidationTrigger, KindSeal,
    MemberDestination, ObligationAnchoring, OwnerContentAccount, PatternStampContent,
    PatternStampProjection, PlanDecisions, PlanDerivation, PlannedMember, PlannedMembership,
    PlannedOutput, ProjectionBundlePlan, ProjectionContext, ProjectionDisposition,
    ProjectionIntentId, ProjectionKind, ProjectionPlan, RemoteSurfaceContent,
    RemoteSurfaceProjection, RenderedImplementation, RowMaterialPosture, SourceDeclarationLimit,
    SourceDeclarations, SurfaceDirection, TargetBinding, TargetRequirement, TestDescriptorContent,
    TestDescriptorProjection, UNIVERSAL_QUESTIONS, VerifiedDerived, WRAPPER_COMPONENTS,
    WrapperComponent,
};

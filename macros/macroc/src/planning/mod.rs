#![doc = include_str!("README.md")]

mod anchor;
mod encode;
mod type_contract;
mod types;

pub use types::{
    BenchmarkDescriptorContent, BenchmarkDescriptorProjection, CapturedDependencies, CauseAnchoring,
    CodecContent, CodecDirection, CodecProjection, ContentAddressing, DeclaredBootstrap,
    DeriveImplContent, DeriveImplProjection, DigestContract, DocumentationContent,
    DocumentationProjection, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
    ExpectedGeneratedSupportSchemaId, GENERATED_SUPPORT_SCHEMA_DECLARED_BOOTSTRAP, GraphAnchoring,
    HostWrapperContent, HostWrapperProjection, InvalidationSet, InvalidationTrigger, KindSeal,
    MemberDestination, OwnerContentAccount, PatternStampContent, PatternStampProjection,
    PlanDecisions, PlanDerivation, PlannedMember, PlannedMembership, PlannedOutput,
    ProjectionBundlePlan, ProjectionContext, ProjectionDisposition, ProjectionIntentId,
    ProjectionKind, ProjectionPlan, RemoteSurfaceContent, RemoteSurfaceProjection,
    RenderedImplementation, SourceDeclarations, SurfaceDirection, TargetBinding,
    TargetRequirement, TestDescriptorContent, TestDescriptorProjection, UNIVERSAL_QUESTIONS,
    VerifiedDerived, WRAPPER_COMPONENTS, WrapperComponent,
};

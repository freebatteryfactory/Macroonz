#![doc = include_str!("README.md")]

mod anchor;
mod encode;
mod type_contract;
mod types;

pub use types::{
    BenchmarkDescriptorContent, BenchmarkDescriptorProjection, BundleMemberLimit,
    CapturedDependencies, CauseAnchoring, CodecContent, CodecDirection, CodecProjection,
    ContentAddressing, DeclaredBootstrap, DigestContract, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
    EmissionPartition, ExpectedGeneratedSupportSchemaId, InvalidationLimit, InvalidationSet,
    InvalidationTrigger, KindDispositions, KindSeal, MemberDestination, ObligationAnchoring,
    OwnerContentAccount, PatternStampContent, PatternStampProjection, PlanDecisions,
    PlanDerivation, PlannedMember, PlannedMembership, PlannedOutput, ProjectionBundlePlan,
    ProjectionContext, ProjectionDisposition, ProjectionIntentId, ProjectionKind,
    ProjectionKindRow, ProjectionPlan, RefusalFamilyImplementationContent,
    RefusalFamilyImplementationProjection, RenderedImplementation, RowMaterialPosture,
    SourceDeclarationLimit, TestDescriptorContent, TestDescriptorProjection, UNIVERSAL_QUESTIONS,
    VerifiedDerived,
};

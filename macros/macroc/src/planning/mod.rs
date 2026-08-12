#![doc = include_str!("README.md")]

mod anchor;
mod encode;
mod type_contract;
mod types;

pub use types::{
    BenchmarkDescriptorContent, BenchmarkDescriptorProjection, CauseAnchoring, CodecContent,
    CodecDirection, CodecProjection, DeriveImplContent, DeriveImplProjection, DigestContract,
    DocumentationContent, DocumentationProjection, GraphAnchoring, HostWrapperContent,
    HostWrapperProjection, InvalidationSet, InvalidationTrigger, KindSeal, MemberDestination,
    PatternStampContent, PatternStampProjection, PlanDerivation, PlannedMember, PlannedMembership,
    PlannedOutput, ProjectionBundlePlan, ProjectionContext, ProjectionDisposition, ProjectionKind,
    ProjectionPlan, RemoteSurfaceContent, RemoteSurfaceProjection, RenderedImplementation,
    SourceDeclarations, SurfaceDirection, TargetBinding, TargetRequirement, TestDescriptorContent,
    TestDescriptorProjection, UNIVERSAL_QUESTIONS, WRAPPER_COMPONENTS, WrapperComponent,
};

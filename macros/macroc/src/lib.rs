//! `threadpak-macroc`: the metaprogramming services.
//!
//! The services are ordinary callable Rust — planning, rendering, inspection,
//! explanation — reached the same way by any caller.
//! They depend inward on the machine and never back outward: nothing here knows
//! a proc-macro exists, under test as well as in a build.
//! The Rust-facing expansion shell (`threadpak-macros`) is one thin surface over
//! this crate; a language frontend would be another.
//!
//! Because this crate carries no dev edge to the shell, it has no composition
//! test of its own: composition is proven from outside the participants, by a
//! consumer that depends on neither's internals.
//!
//! # The charter
//!
//! **Derivers, not legislators.** The services project contracts the machine's
//! homes already own.
//! They decide no meaning, own no semantic noun, and are never their own oracle.
//! Every roster they speak — identity classes, refusal shapes, verification
//! methods, semantic facets — is the machine's, imported rather than restated.
//!
//! **Plan before render, and close before emit.** Nothing is rendered that was
//! not planned: a plan names its complete output set LOGICALLY — role, semantic
//! key, destination, origin, expected renderer, digest contract — before a byte
//! of target syntax exists, and never carries a digest of bytes nobody has
//! produced.
//! Nothing is emitted that did not close: the membership is rebuilt out of the
//! rendered units and proven equal to the plan's, role by role, and the token
//! tree is reachable only off the value that proof produced.
//!
//! **No partial output.** A declared output set is materialized whole or not at
//! all.
//! A refusal is a refusal; it is never a smaller success.
//!
//! **Callable without a proc-macro.** Every service is reachable as an ordinary
//! function.
//! The expansion shell is one caller among possible callers, and a diagnostic
//! names the callable route as a first-class way to reproduce it.
//!
//! # The plane's spine
//!
//! ```text
//! captured tokens → plan (context + kind content + LOGICAL membership +
//! invalidation + trace + origin + nonclaims) → rendered units (trees, bytes,
//! digests) → proved closure → explanation → closed expansion → emitted tokens
//! ```
//!
//! Expansion is deterministic from its declared input: no network, no
//! filesystem scans, no environment reads, no clock, no entropy.
//! There is no seat where one could enter.
//!
//! # Declaration order
//!
//! The machine states its dependency bands with numbered directories.
//! This crate carries no numbers, so it states the same fact with the only
//! ordering an unnumbered module list has: **the order the `pub mod`
//! declarations appear below.**
//! A module imports only modules declared EARLIER than itself, never a later
//! one and never itself-by-way-of-another, so reading top to bottom is reading
//! the dependency graph.
//! A cycle cannot hide in that order, because a cycle always contains at least
//! one backward-pointing edge.
//!
//! `reorder_modules = false` in `rustfmt.toml` keeps a formatter from
//! re-alphabetizing the list and erasing the law.
//!
//! The per-edge map is drawn once, as a diagram in the crate README; this list
//! is not a second copy of it, because the order IS the graph and the `use`
//! lines under each home are the edges themselves.
//!
//! A directory module's edges are the union of every file under it:
//! `derive_refusal/` reaches what its capture, plan, render, explain, and
//! diagnose files reach, and a submodule pointing forward is its parent pointing
//! forward.
//! Inside a directory, `super::` names the parent and is not a crate-root route;
//! in a single-file module `super::` IS the crate root.
//!
//! Every module above is a directory home carrying the repository's file
//! grammar, and `mod.rs` is its door: the seat files behind the door are
//! private, and a home's public names are re-exported so `crate::plane::X` names
//! the owner exactly as it did when the home was one file.
//! A sibling is therefore always reached by its owner's path —
//! `crate::planning::PlannedMember` names an owner and `crate::PlannedMember`
//! names none.
//!
//! `question` is a leaf over nothing and could sit anywhere earlier; it is
//! seated exactly where both its readers need it, which is the honest place for
//! a vocabulary that exists to keep two machinery modules from importing each
//! other.
//!
//! Only `mod` declarations carry the rule.
//! The `#[cfg(test)]` proof surface (`laws`) is declared last and reaches every
//! module by design: it is what proves the order, not a participant in it.

pub mod plane;
pub mod token;
pub mod refusal;
pub mod diagnostics;
pub mod question;
pub mod origin_graph;
pub mod planning;
pub mod closure;
pub mod explanation_protocol;
pub mod template;
pub mod trigger_view;
pub mod composition;
pub mod pattern_stamp;
pub mod derive_refusal;

pub use closure::{
    ClosureIssue, ProjectionClosure, ProjectionClosureRefusal, RenderedProjection, RenderedUnit,
    RenderingRefusal,
};
pub use composition::{
    CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DESCRIPTOR_KINDS,
    DescriptorKind, DescriptorProvider,
};
pub use derive_refusal::{
    CapturedCause, CauseOrderStanding, ClosedExpansion, CrateBinding, DEFAULT_CRATE_BINDING,
    DerivedMembership, DerivedPlan, ExplanationBindingRefusal, ExplanationSeat,
    RefusalCompileContext, RefusalDerivationDraft, RefusalDeriveCapture, RefusalDeriveRefusal,
    RefusalDeriveSurface, RefusalOwnerFacts, RenderRefusal, TextCompileRefusal, captured,
    captured_text, compile_refusal, compile_refusal_text,
};
pub use diagnostics::{
    DiagnosticSite, MachineAnchoring, MachineAnchors, MacrocDiagnostic, MacrocPhase,
    ObservedClassification, RelatedIdentity, RelatedSet, RelatedSetCompletion,
    RelatedSetTruncation, ReleasePosture, RepairAction, ReproductionRoute, SiteCoordinate,
};
pub use explanation_protocol::{
    ExplanationAnswer, ExplanationCoverage, ExplanationCoverageIssue, ProjectionExplanation,
    ProjectionExplanationView, kind_admits,
};
pub use origin_graph::{
    DecisionTrace, Nonclaim, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
pub use pattern_stamp::{ScopeGuardOwnerFacts, ScopeGuardStampAnchors, plan_scope_guard_stamp};
pub use plane::{
    AuthoringLimitProfile, ClosedExpansionId, ClosureId, GeneratorIdentity, GeneratorProfileId,
    GeneratorSchemaVersion, HumanProjection, IdentityProfile, IdentityProfileVersion,
    IdentitySubject, MACROC_GENERATOR, OwnerFactName, OwnerFactRef, OwnerIdentityRef,
    PROJECTION_IDENTITY_PROFILE, PlanId, ProfileVersion, ProjectionIdentity, ProjectionProvenance,
    ProjectionRole, ProjectionTranscript, RenderedRole, RenderedRoleSeal, SoleRenderedUnit,
    SubjectSeal, TranscriptAnchoring, encode_bytes, encode_length,
};
pub use planning::{
    BenchmarkDescriptorContent, BenchmarkDescriptorProjection, CauseAnchoring, CodecContent,
    CodecDirection, CodecProjection, DeriveImplContent, DeriveImplProjection, DigestContract,
    DocumentationContent, DocumentationProjection, GraphAnchoring, HostWrapperContent,
    HostWrapperProjection, InvalidationSet, InvalidationTrigger, KindSeal, MemberDestination,
    PatternStampContent, PatternStampProjection, PlannedMember, PlannedMembership, PlannedOutput,
    ProjectionBundlePlan, ProjectionContext, ProjectionDisposition, ProjectionKind, ProjectionPlan,
    RemoteSurfaceContent, RemoteSurfaceProjection, RenderedImplementation, SourceDeclarations,
    SurfaceDirection, TargetBinding, TargetRequirement, TestDescriptorContent,
    TestDescriptorProjection, UNIVERSAL_QUESTIONS, WRAPPER_COMPONENTS, WrapperComponent,
};
pub use question::{EXPLANATION_PROTOCOL_VERSION, ExplanationQuestion, QuestionApplicability};
pub use refusal::{BoundAxis, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
pub use template::{
    ApplicativeDistinctness, AxisCeiling, CheckedMeterPosture, DeclarationTemplate,
    ForbiddenKeyFact, INVOCATION_KEY_NEVER, META_BOUND_AXES, MetaBoundAxis, ProfileCeiling,
    SPLICE_CATEGORIES, SpliceCategory, SymbolicBoundFormula, TemplateApplication, TemplateArgument,
    TemplateBinding, TemplateBindingIssue, TemplateConstruction, TemplateConstructionIssue,
    TemplateInvocationKey, TemplateParameter, TemplateSeat, VersionedProfile,
};
pub use token::{
    CaptureBound, CaptureWalk, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree,
    SpanHandle, SpanResolutionRefusal, SpanTable, TextCapture, TextReadCause, TextReadRefusal,
    TokenPath,
};
pub use trigger_view::{
    TriggerCitations, TriggerOmission, TriggerSelection, TriggerViewComposition, TriggerViewIssue,
    WrapperTriggerView,
};

#[cfg(test)]
mod laws;

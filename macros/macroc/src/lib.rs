//! `threadpak-macroc`: the metaprogramming services.
//!
//! The services are ordinary callable Rust — planning, rendering, inspection,
//! explanation — reached the same way by any caller. They depend inward on the
//! machine and never back outward: nothing here knows a proc-macro exists. The
//! Rust-facing expansion shell (`threadpak-macros`) is one thin surface over
//! this crate; a future language frontend would be another.
//!
//! That absence holds under test too. A compiler service never depends on its
//! frontend surfaces, even for tests, so this crate carries no dev edge to the
//! shell and no composition test of its own. Composition is proven from OUTSIDE
//! the participants, by the consumer fixture at `xtask/fixtures/macro-consumer`,
//! which depends on the machine and the shell and on neither of their internals.
//! The `no-core-tooling-edge` gate enforces the absence.
//!
//! # The charter
//!
//! **Derivers, not legislators.** The services project contracts the machine's
//! homes already own. They decide no meaning, own no semantic noun, and are
//! never their own oracle. Every roster they speak — identity classes, refusal
//! shapes, verification methods, semantic facets — is the machine's, imported
//! rather than restated.
//!
//! **Plan before render.** Nothing is rendered that was not planned. A plan
//! names its complete output set, what invalidates it, which decisions produced
//! it, and where it came from, before a byte of target syntax exists.
//!
//! **No partial output.** A declared output set is materialized whole or not at
//! all. A refusal is a refusal; it is never a smaller success.
//!
//! **Callable without a proc-macro.** Every service is reachable as an ordinary
//! function. The expansion shell is one caller among possible callers, and a
//! diagnostic names the callable route as a first-class way to reproduce it.
//!
//! # The plane's spine
//!
//! ```text
//! exact identities → plan (context + kind content + membership + invalidation
//! + trace + origin + nonclaims) → disposition or output → explanation
//! ```
//!
//! Expansion is deterministic from its declared input: no network, no
//! filesystem scans, no environment reads, no clock, no entropy. Nothing in
//! this crate reaches for any of them, and there is no seat where one could
//! enter.
//!
//! # Declaration order IS the dependency order
//!
//! The machine states its dependency bands with numbered directories. This
//! crate is flat, so it states the same fact with the only ordering a flat
//! module list has: **the order the `pub mod` declarations appear below.** A
//! module imports only modules declared EARLIER than itself, never a later one
//! and never itself-by-way-of-another. Read top to bottom and you have read the
//! dependency graph; there is no second place to look and nothing to keep in
//! sync by hand.
//!
//! The rule is machine-enforced, not a convention: `cargo xtask check` runs
//! `tooling-module-order`, which reads the declaration order out of this file,
//! reads each module's `crate::` references out of its own source, and refuses
//! any reference pointing later in the list. A cycle cannot survive that check,
//! because a cycle always contains at least one backward-pointing edge.
//!
//! The order is a straight line, not an accident of the alphabet. The formatter
//! is told so — `reorder_modules = false` in `rustfmt.toml` — because a tool
//! that re-alphabetizes this list would be erasing a law.
//!
//! ```text
//! plane                 the shared carriers; over nothing
//! refusal               over plane
//! diagnostics           over plane
//! question              the closed question roster; over nothing at all
//! origin_graph          over plane, refusal
//! planning              over plane, refusal, question, origin_graph
//! explanation_protocol  over plane, diagnostics, question, origin_graph, planning
//! template              over plane, origin_graph
//! trigger_view          over plane, refusal, planning
//! composition           over plane
//! pattern_stamp         over plane, refusal, origin_graph, planning
//! derive_refusal        over plane, refusal, diagnostics, origin_graph,
//!                       planning, explanation_protocol
//! ```
//!
//! `question` is a leaf over nothing and could sit anywhere earlier; it is
//! seated exactly where both its readers need it, which is the honest place for
//! a vocabulary that exists to keep two machinery modules from importing each
//! other.
//!
//! Only `mod` declarations carry the rule. The proof surface (`laws`) is
//! declared last, is not public, is excluded by its `#[cfg(test)]`, and reaches
//! every module by design: it is what proves the order, not a participant in it.
//!
//! The graph above is stated here once. The declarations below carry no
//! commentary of their own, because a second copy of a dependency map is a
//! second thing to keep true.

pub mod plane;
pub mod refusal;
pub mod diagnostics;
pub mod question;
pub mod origin_graph;
pub mod planning;
pub mod explanation_protocol;
pub mod template;
pub mod trigger_view;
pub mod composition;
pub mod pattern_stamp;
pub mod derive_refusal;

pub use composition::{
    CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DESCRIPTOR_KINDS,
    DescriptorKind, DescriptorProvider,
};
pub use derive_refusal::{
    CaptureDiagnosticAnchors, CapturedCause, CauseOrderStanding, DerivationAnchors, DerivedItem,
    DerivedMembership, DerivedProjection, PlantedDefect, RefusalDeriveCapture,
    RefusalDeriveDisposition, RefusalDeriveRefusal, RefusalDeriveSurface, RefusalFamilyDerivation,
    RefusalOwnerFacts, RenderedDerivation, captured, disposed,
};
pub use diagnostics::{
    MACROC_PHASES, MacrocDiagnostic, MacrocPhase, ObservedClassification, ReleasePosture,
    RepairAction, ReproductionRoute,
};
pub use explanation_protocol::{
    ExplanationAnswer, ExplanationCoverage, ExplanationCoverageIssue, ProjectionExplanation,
    ProjectionExplanationView, kind_admits,
};
pub use origin_graph::{
    DecisionTrace, Nonclaim, ORIGIN_RELATIONS, OriginEdge, OriginRelation, OriginTrail,
    TraceDecision, TraceEntry,
};
pub use pattern_stamp::{ScopeGuardOwnerFacts, ScopeGuardStampAnchors, plan_scope_guard_stamp};
pub use plane::{ExactIdentity, HumanProjection, OwnerFactRef, ProfileVersion};
pub use planning::{
    BenchmarkDescriptorContent, BenchmarkDescriptorProjection, CodecContent, CodecDirection,
    CodecProjection, DeriveImplContent, DeriveImplProjection, DocumentationContent,
    DocumentationProjection, HostWrapperContent, HostWrapperProjection, InvalidationSet,
    InvalidationTrigger, KindSeal, OutputIdentity, PatternStampContent, PatternStampProjection,
    PlannedMembership, ProjectionBundlePlan, ProjectionContext, ProjectionDisposition,
    ProjectionKind, ProjectionPlan, RemoteSurfaceContent, RemoteSurfaceProjection,
    SourceDeclarations, SurfaceDirection, TargetBinding, TargetRequirement, TestDescriptorContent,
    TestDescriptorProjection, UNIVERSAL_QUESTIONS, WRAPPER_COMPONENTS, WrapperComponent,
};
pub use question::{EXPLANATION_QUESTIONS, ExplanationQuestion, QuestionApplicability};
pub use refusal::{
    BOUND_AXES, BoundAxis, PlanIdentity, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue,
};
pub use template::{
    ApplicativeDistinctness, AxisCeiling, CheckedMeterPosture, DeclarationTemplate,
    ForbiddenKeyFact, INVOCATION_KEY_NEVER, META_BOUND_AXES, MetaBoundAxis, ProfileCeiling,
    SPLICE_CATEGORIES, SpliceCategory, SymbolicBoundFormula, TemplateApplication, TemplateArgument,
    TemplateBinding, TemplateBindingIssue, TemplateConstruction, TemplateConstructionIssue,
    TemplateInvocationKey, TemplateParameter, TemplateSeat, VersionedProfile,
};
pub use trigger_view::{
    TriggerCitations, TriggerOmission, TriggerSelection, TriggerViewComposition, TriggerViewIssue,
    WrapperTriggerView,
};

/// The machine's frontend-role type, re-exported rather than restated. The
/// expansion shell reaches the machine's vocabulary through the services, so
/// the shell needs no edge of its own to the machine and no copy of the type.
pub use threadpak::declaration::FrontendRole;

/// Names the front door a role stands for.
#[must_use]
pub const fn describe_frontend_role(role: FrontendRole) -> &'static str {
    match role {
        FrontendRole::RustDeclaration => "the live Rust-declaration front door",
        FrontendRole::ApplicationLanguage => "the pluggable application-language front door",
    }
}

#[cfg(test)]
mod laws;

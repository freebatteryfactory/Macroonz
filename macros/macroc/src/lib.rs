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
//! **Plan before render, and close before emit.** Nothing is rendered that was
//! not planned: a plan names its complete output set LOGICALLY — role, semantic
//! key, destination, origin, expected renderer, digest contract — before a byte
//! of target syntax exists, and never carries a digest of bytes nobody has
//! produced. Nothing is emitted that did not close: the membership is rebuilt
//! out of the rendered units and proven equal to the plan's, role by role, and
//! the token tree is reachable only off the value that proof produced.
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
//! captured tokens → plan (context + kind content + LOGICAL membership +
//! invalidation + trace + origin + nonclaims) → rendered units (trees, bytes,
//! digests) → proved closure → explanation → closed expansion → emitted tokens
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
//! crate carries no numbers, so it states the same fact with the only ordering
//! an unnumbered module list has: **the order the `pub mod` declarations appear
//! below.** A module imports only modules declared EARLIER than itself, never a
//! later one and never itself-by-way-of-another. Read top to bottom and you
//! have read the dependency graph; there is no second place to look and nothing
//! to keep in sync by hand.
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
//! plane                 the shared carriers and the two identity families
//! token                 the typed token seam, both directions; over plane
//! refusal               over plane
//! diagnostics           over plane, token
//! question              the closed question roster; over nothing at all
//! origin_graph          over plane, refusal
//! planning              over plane, refusal, question, origin_graph
//! closure               over plane, origin_graph, planning, token
//! explanation_protocol  over plane, diagnostics, question, origin_graph, planning
//! template              over plane, origin_graph
//! trigger_view          over plane, refusal, planning
//! composition           over plane
//! pattern_stamp         over plane, refusal, origin_graph, planning
//! derive_refusal        over every module above it
//! ```
//!
//! A directory module's edges are the union of every file under it:
//! `derive_refusal/` reaches what its capture, plan, render, explain, and
//! diagnose files reach, and a submodule pointing forward is its parent pointing
//! forward. Inside a directory, `super::` names the parent and is not a
//! crate-root route; in a single-file module `super::` IS the crate root, and
//! the checker reads it as one.
//!
//! Every module above is a directory home, and each carries the repository's
//! file grammar inside it: `README.md` states what the home owns and why, and is
//! the module's own documentation — `mod.rs` includes it rather than restating
//! it. `mod.rs` is the door and re-exports the home's public names, so
//! `crate::plane::X` names the owner exactly as it did when the home was one
//! file; `types.rs` declares; `types.rs`'s own child `type_guard.rs` holds every
//! road that reaches a private field, which is what makes each home's walls
//! structural; `type_contract.rs` states the declarative surface; and the
//! remaining files are role-named and pure. A seat exists only where it has
//! content: a home with no private field carries no `type_guard.rs`, and a home
//! that declares and computes nothing else is `mod.rs` and `types.rs` alone. No
//! home publishes a second path to its own contents: the seat files are private
//! and reached through the door.
//!
//! Reaching a sibling's content through a crate-root re-export is refused
//! outright, whatever the declaration order says: `crate::planning::PlannedMember`
//! names an owner and `crate::PlannedMember` names none, so only the first can
//! be checked at all.
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
    ObservedClassification, RelatedSetCompletion, ReleasePosture, RepairAction, ReproductionRoute,
    SiteCoordinate,
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

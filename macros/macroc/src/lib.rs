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
//! The road those functions compose is written out below as the builder door:
//! the entrance itself, made public, rather than a second surface over it.
//!
//! # The plane's spine
//!
//! ```text
//! owner content account → intent → plan (account + context + kind content +
//! LOGICAL membership + invalidation + trace + origin + nonclaims) → rendered
//! units (trees, bytes, digests) → proved closure → explanation → closed
//! expansion → emitted tokens
//! ```
//!
//! The account is where owner content walks in: the commitment the owner
//! supplied and what that commitment declares it stands on, carried under one
//! posture — the machine's declaration fragment where a caller holds the
//! linker's mint, and the captured token material one expansion was handed where
//! nothing has been linked.
//! It is stated once and read four ways, so no seat downstream holds a second
//! account of what a plan was planned over.
//!
//! Expansion is deterministic from its declared input: no network, no
//! filesystem scans, no environment reads, no clock, no entropy.
//! There is no seat where one could enter.
//!
//! # The builder door
//!
//! The entrance is public: the road below is the one the derive walks, called
//! directly, as ordinary functions in one order.
//! Each step hands the next a value the next one cannot forge, and nothing
//! stands between a caller and any of them — no station, no registry, no shell.
//!
//! 1. **The account.** [`OwnerContentAccount::captured`] at expansion time,
//!    where the content IS the token material one expansion was handed, and
//!    [`OwnerContentAccount::linked`] where a caller holds the identity the
//!    machine minted for a declaration fragment; `captured_over` and
//!    `linked_over` are those two roads for content that declares what it stands
//!    on. The linked pair takes identities the MACHINE minted, so it is walked
//!    exactly where a caller was handed them. What comes back is the ONE account
//!    of owner content: the intent, the watch set, the causing-declaration
//!    answer, and the origin edges are all read off this one value.
//! 2. **The intent.** [`OwnerContentAccount::intent`] hands back
//!    [`ProjectionIntentId`] — the kind's declared name and the content
//!    commitment — which is the pair door equivalence compares, since plan
//!    identities carry origin and cannot be compared for it.
//!    [`OwnerContentAccount::intent_bytes`] is that same pair as canonical bytes.
//! 3. **The context.** [`ProjectionContext`] is written as a literal: its seats
//!    are public, and the identities seated in them are the caller's own or the
//!    machine's, never minted here. [`ProjectionContext::graph_of`] reads a
//!    closed graph where a caller holds one;
//!    [`GraphAnchoring::CapturedDeclarationOnly`] states the expansion-time
//!    posture where there is none. The watch set is not listed at a call site —
//!    [`ProjectionContext::watch_set`] derives it from this context and that
//!    account, and refuses where the trigger roster cannot represent them.
//! 4. **The plan.** [`PlannedMembership::complete`] for a roster a shape fixes
//!    and [`PlannedMembership::declared`] for one decided at runtime;
//!    [`OriginTrail::from_edge`] for the walk back to authored material;
//!    [`DecisionTrace::from_entry`] for the decisions in selection order; then
//!    [`ProjectionPlan::planned`], which takes the account FIRST, moves it in,
//!    and derives the plan's own identity over it.
//! 5. **The rendering.** [`GeneratedToken`] spells target syntax as typed tokens
//!    and [`GeneratedTree::assembled`] carries them;
//!    [`RenderedUnit::materialized`] takes the digest over that tree's own
//!    canonical bytes, so no caller supplies one;
//!    [`RenderedProjection::complete`] gathers the units a shape fixes.
//! 6. **The closure.** [`ProjectionClosure::proved`], over the plan's identity,
//!    the plan's membership, and the rendering. It rebuilds the membership out
//!    of the rendered units, proves the rebuild equals the plan's role by role,
//!    joins the token tree, and keeps it: [`ProjectionClosure::emitted`] is the
//!    road to tokens and there is no other.
//! 7. **The explanation.** [`ProjectionExplanation::answered`] per seat — the
//!    question is taken from the answer — and
//!    [`ProjectionExplanationView::complete`] over the roster
//!    [`ProjectionPlan::applicable_questions`] states. It is written after the
//!    closure, because one seat carries a digest of bytes that must exist.
//! 8. **The closed expansion.** [`ClosedExpansion`] is the refusal-family
//!    derive's receipt over its own captured surface, and the road that binds
//!    one is crate-internal, so the public roads to a receipt are
//!    [`compile_refusal`] and [`compile_refusal_text`] — steps 1 through 7 for
//!    that family, in one call, with the capture in front. A caller that walked
//!    the steps itself holds the plan, the closure, and the explanation, and
//!    reaches its tokens through the closure; what it does not hold is a receipt
//!    binding those three under one identity, because that family's is the only
//!    receipt type there is.
//!
//! Every step refuses in its own vocabulary: [`ProjectionPlanning`] at the
//! account, the watch set, and the plan; [`RenderingRefusal`] at a rendered
//! unit; [`ProjectionClosureRefusal`] at the proof; [`ExplanationCoverage`] at
//! the view.
//! A caller either matches each one or projects it into the plane's diagnostic,
//! and those projections live where the derive door needed them —
//! [`derive_refusal::diagnose`], whose planning, rendering, closure, and
//! explanation roads are public and generic over the rendered role.
//! A token tree that outgrows its magnitude is the one refusal on this road with
//! no plane vocabulary of its own: it refuses with the machine's bounded
//! construction, and the only projection of that into a diagnostic is spelled
//! over the derive's [`RenderRefusal`].
//!
//! ```ignore
//! use threadpak::types::Bounded;
//! use threadpak_macroc::derive_refusal::diagnose;
//! use threadpak_macroc::{
//!     DecisionTrace, DeriveImplContent, DeriveImplProjection, DigestContract, ExplanationAnswer,
//!     ExplanationBindingRefusal, GeneratedDelimiter, GeneratedToken, GeneratedTree,
//!     GraphAnchoring, MacrocDiagnostic, MemberDestination, OriginEdge, OriginRelation,
//!     OriginTrail, OwnerContentAccount, OwnerFactRef, PlannedMember, PlannedMembership,
//!     PlannedOutput, ProfileVersion, ProjectionClosure, ProjectionContext, ProjectionDisposition,
//!     ProjectionExplanation, ProjectionExplanationView, ProjectionIdentity, ProjectionPlan,
//!     ProjectionRole, ProjectionTranscript, RenderRefusal, RenderedImplementation,
//!     RenderedProjection, RenderedRole, RenderedUnit, TargetBinding, TraceDecision, TraceEntry,
//! };
//!
//! /// Plan, render, close, and explain one implementation projection, with every
//! /// refusal on the road projected into the plane's own diagnostic.
//! fn expand() -> Result<ProjectionExplanationView<DeriveImplProjection>, MacrocDiagnostic> {
//!     let role = RenderedImplementation::RenderedFamilyImpl;
//!
//!     // 1. The account. At expansion time the content IS the captured material.
//!     let declaration = ProjectionIdentity::derived(ProjectionTranscript::rooted(
//!         ProjectionRole::CapturedDeclaration,
//!         b"enum ReadRefusal { NotNamed }",
//!         0,
//!     ));
//!     let account = OwnerContentAccount::<DeriveImplProjection>::captured(declaration);
//!
//!     // 2. The intent: the kind's declared name, and what it was meant over.
//!     assert_eq!(account.intent().kind(), "derive-impl-projection");
//!
//!     // 3. The context, and the watch set derived from it and the account.
//!     let profile = ProjectionIdentity::derived(ProjectionTranscript::rooted(
//!         ProjectionRole::Plan,
//!         b"example.profile.rust-declaration",
//!         0,
//!     ));
//!     let context = ProjectionContext {
//!         graph: GraphAnchoring::CapturedDeclarationOnly(declaration),
//!         profile,
//!         profile_version: ProfileVersion::declared(1),
//!         generator: ProjectionIdentity::derived(ProjectionTranscript::rooted(
//!             ProjectionRole::Plan,
//!             b"example.generator.schema-1",
//!             0,
//!         )),
//!         target: TargetBinding::TargetFree,
//!     };
//!     let invalidation = context
//!         .watch_set(&account)
//!         .map_err(|refusal| diagnose::planning_refused(&refusal))?;
//!
//!     // 4. The plan: the complete declared output set, where it came from, and why.
//!     let key = ProjectionIdentity::derived(ProjectionTranscript::under_projection(
//!         ProjectionRole::GeneratedUnit,
//!         &declaration,
//!         b"family-implementation",
//!         role.slot(),
//!     ));
//!     let origin = OriginTrail::from_edge(OriginEdge {
//!         from: ProjectionIdentity::derived(ProjectionTranscript::under_projection(
//!             ProjectionRole::OriginNode,
//!             &declaration,
//!             b"authored-declaration",
//!             0,
//!         )),
//!         relation: OriginRelation::SemanticDerivation,
//!         to: ProjectionIdentity::derived(ProjectionTranscript::under_projection(
//!             ProjectionRole::OriginNode,
//!             &declaration,
//!             b"family-implementation",
//!             role.slot(),
//!         )),
//!     });
//!     let assumed = OwnerFactRef::named("refusal", "family-shapes-are-three-and-closed");
//!     let plan = ProjectionPlan::<DeriveImplProjection>::planned(
//!         account,
//!         context,
//!         DeriveImplContent {
//!             derived_type: ProjectionIdentity::derived(ProjectionTranscript::under_projection(
//!                 ProjectionRole::GeneratedUnit,
//!                 &declaration,
//!                 b"ReadRefusal",
//!                 0,
//!             )),
//!             contract: ProjectionIdentity::derived(ProjectionTranscript::under_projection(
//!                 ProjectionRole::GeneratedUnit,
//!                 &declaration,
//!                 b"threadpak.refusal.RefusalFamily",
//!                 1,
//!             )),
//!             assumptions: Bounded::from_array([assumed]),
//!         },
//!         PlannedMembership::complete(
//!             PlannedMember {
//!                 role,
//!                 output: PlannedOutput {
//!                     semantic_key: key,
//!                     destination: MemberDestination::AtDeclarationSite,
//!                     origin: origin.clone(),
//!                     expected_profile: profile,
//!                     expected_profile_version: ProfileVersion::declared(1),
//!                     digest_contract: DigestContract::over(key),
//!                 },
//!             },
//!             [],
//!         ),
//!         invalidation,
//!         DecisionTrace::from_entry(TraceEntry {
//!             subject: ProjectionIdentity::derived(ProjectionTranscript::under_projection(
//!                 ProjectionRole::Plan,
//!                 &declaration,
//!                 b"implementation-derivation",
//!                 0,
//!             )),
//!             decision: TraceDecision::SelectedBecause(assumed),
//!         }),
//!         origin.clone(),
//!         Bounded::empty(),
//!     )
//!     .map_err(|refusal| diagnose::planning_refused(&refusal))?;
//!
//!     // 5. The rendering: typed tokens, and a digest taken over their bytes here.
//!     let tree = GeneratedTree::assembled(vec![
//!         GeneratedToken::word("impl"),
//!         GeneratedToken::word("RefusalFamily"),
//!         GeneratedToken::word("for"),
//!         GeneratedToken::word("ReadRefusal"),
//!         GeneratedToken::group(GeneratedDelimiter::Brace, Vec::new())
//!             .map_err(|_| diagnose::render_refused(RenderRefusal::Unbounded, role))?,
//!     ])
//!     .map_err(|_| diagnose::render_refused(RenderRefusal::Unbounded, role))?;
//!     let unit = RenderedUnit::materialized(
//!         role,
//!         key,
//!         MemberDestination::AtDeclarationSite,
//!         profile,
//!         ProfileVersion::declared(1),
//!         origin,
//!         tree,
//!     )
//!     .map_err(|refusal| diagnose::rendering_refused(refusal, role))?;
//!     let digest = unit.digest();
//!
//!     // 6. The closure: the rebuild equals the plan, and the tokens are inside the proof.
//!     let closure = ProjectionClosure::proved(
//!         plan.identity(),
//!         plan.membership(),
//!         RenderedProjection::complete(unit, []),
//!     )
//!     .map_err(|refusal| diagnose::closure_refused(&refusal))?;
//!     let _emitted = closure.emitted();
//!
//!     // 7. The explanation: every seat this kind owes, answered once.
//!     let kind = ProjectionIdentity::derived(ProjectionTranscript::rooted(
//!         ProjectionRole::Plan,
//!         b"example.kind.derive-impl-projection",
//!         0,
//!     ));
//!     ProjectionExplanationView::<DeriveImplProjection>::complete(vec![
//!         ProjectionExplanation::answered(ExplanationAnswer::Kind { kind }),
//!         ProjectionExplanation::answered(ExplanationAnswer::Owner { owner: assumed }),
//!         ProjectionExplanation::answered(ExplanationAnswer::CausingDeclarations {
//!             sources: plan.account().commitment(),
//!         }),
//!         ProjectionExplanation::answered(ExplanationAnswer::GraphAndProfile {
//!             graph: plan.context().graph,
//!             profile: plan.context().profile,
//!             version: plan.context().profile_version,
//!         }),
//!         ProjectionExplanation::answered(ExplanationAnswer::OutputAndDigest {
//!             output: Box::new(plan.membership().first().output.clone()),
//!             digest,
//!         }),
//!         ProjectionExplanation::answered(ExplanationAnswer::AssumptionsAndSpecializations {
//!             assumptions: plan.content().assumptions.clone(),
//!         }),
//!         ProjectionExplanation::answered(ExplanationAnswer::Invalidators {
//!             triggers: plan.invalidation().clone(),
//!         }),
//!         ProjectionExplanation::answered(ExplanationAnswer::RelatedProjectionDisposition {
//!             related: kind,
//!             disposition: ProjectionDisposition::NotApplicable { because: assumed },
//!         }),
//!         ProjectionExplanation::answered(ExplanationAnswer::Repairs {
//!             repairs: Bounded::empty(),
//!         }),
//!     ])
//!     .map_err(|coverage| {
//!         diagnose::explanation_refused(&ExplanationBindingRefusal::Coverage(coverage))
//!     })
//! }
//! ```
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
pub mod derive_impl;
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
    BenchmarkDescriptorContent, BenchmarkDescriptorProjection, CapturedDependencies, CauseAnchoring,
    CodecContent, CodecDirection, CodecProjection, ContentAddressing, DeclaredBootstrap,
    DeriveImplContent, DeriveImplProjection, DigestContract, DocumentationContent,
    DocumentationProjection, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID, ExpectedGeneratedSupportSchemaId,
    GraphAnchoring, HostWrapperContent, HostWrapperProjection, InvalidationSet, InvalidationTrigger,
    KindSeal, MemberDestination, OwnerContentAccount, PatternStampContent, PatternStampProjection,
    PlannedMember, PlannedMembership, PlannedOutput, ProjectionBundlePlan, ProjectionContext,
    ProjectionDisposition, ProjectionIntentId, ProjectionKind, ProjectionPlan, RemoteSurfaceContent,
    RemoteSurfaceProjection, RenderedImplementation, SourceDeclarations, SurfaceDirection,
    TargetBinding, TargetRequirement, TestDescriptorContent, TestDescriptorProjection,
    UNIVERSAL_QUESTIONS, VerifiedDerived, WRAPPER_COMPONENTS, WrapperComponent,
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

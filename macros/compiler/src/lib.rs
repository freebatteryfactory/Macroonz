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
//! rendered units and proven equal to the plan's, role by role, and the
//! emissions are built inside that proof — one per delivery, split by the
//! destination each member declared, so what the consumer's normal build
//! compiles is exactly what was planned into it and nothing else.
//! Nothing is handed out that did not bind: the emissions are reachable only
//! off the closed expansion that joins the plan, the proof, and the
//! explanation — and it joins them only where the three name one another.
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
//! units (trees, bytes, digests) → proved closure (membership rebuilt, emission
//! partitioned by delivery) → explanation (over that plan and that closure) →
//! closed expansion → the emission each build receives
//! ```
//!
//! The account is where captured owner content walks in: the capture identity the caller supplied and the captured dependencies it declares.
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
//! 1. **The account.** [`OwnerContentAccount::captured`] records content that stands on no other capture, while [`OwnerContentAccount::captured_over`] records the complete captured dependency set. What comes back is the one account from which intent, invalidation, explanation, and origin are read.
//! 2. **The intent.** [`OwnerContentAccount::intent`] hands back
//!    [`ProjectionIntentId`] — thirty-two bytes DERIVED over the kind's declared
//!    name and the content commitment, which is what door equivalence compares,
//!    since plan identities carry origin and cannot be compared for it.
//!    [`ProjectionIntentId::as_bytes`] is that identity's whole public surface;
//!    a digest hands back neither half of the pair it committed to, so a caller
//!    that needs the pair itself reads the ACCOUNT, whose
//!    [`OwnerContentAccount::intent_bytes`] is that pair as canonical bytes —
//!    the exact preimage the identity was derived over.
//! 3. **The context.** [`ProjectionContext`] carries the selected profile and generator identity. [`ProjectionContext::watch_set`] derives the complete shared watch set from this context and the account.
//! 4. **The plan.** [`PlannedMembership::complete`] for a roster a shape fixes
//!    and [`PlannedMembership::declared`] for one decided at runtime;
//!    [`OriginTrail::from_edge`] for the walk back to authored material;
//!    [`DecisionTrace::from_entry`] for the decisions in selection order. Those,
//!    the watch set from step 3, and the nonclaims are the five DECIDED seats,
//!    and they travel as one [`PlanDecisions`] value in the order a plan's
//!    transcript writes them — every field required and public, so a
//!    construction that leaves one out stops compiling exactly where a missing
//!    argument used to. Then [`ProjectionPlan::planned`], which takes the
//!    account FIRST, moves it in, and derives the plan's own identity over it.
//! 5. **The rendering.** [`GeneratedToken`] spells target syntax as typed tokens
//!    and [`GeneratedTree::assembled`] carries them;
//!    [`RenderedUnit::materialized`] takes the digest over that tree's own
//!    canonical bytes, so no caller supplies one;
//!    [`RenderedProjection::complete`] gathers the units a shape fixes.
//! 6. **The closure.** [`ProjectionClosure::proved`], over the plan's identity,
//!    the plan's membership, and the rendering. It rebuilds the membership out
//!    of the rendered units, proves the rebuild equals the plan's role by role,
//!    then SPLITS the rendering across the deliveries its members declared —
//!    one joined emission per build, each in role-roster order, each digest
//!    inside the closure's own identity — and keeps them. Those emissions are
//!    the proof's material, and the closure hands none of them out.
//! 7. **The explanation.** [`ProjectionExplanation::answered`] per seat — the
//!    question is taken from the answer — and
//!    [`ProjectionExplanationView::complete`] over the roster
//!    [`ProjectionPlan::applicable_questions`] states. It takes the PLAN and the
//!    PROVED CLOSURE themselves, reads their identities off them, stores the
//!    seats in the kind's declared question order, and mints the view's own
//!    [`ExplanationId`] over the three — so a complete view names the parentage
//!    it was answered over and cannot be handed one. It is written after the
//!    closure, because one seat carries a digest of bytes that must exist.
//! 8. **The closed expansion.** [`ClosedExpansion::bound`] takes the plan, the
//!    closure proved against it, and the explanation answered over the two, and
//!    binds them under one identity — after establishing that the three name one
//!    another, and committing to all three inside that identity. This road is
//!    public and it is where the door ends for EVERY projection kind: a caller
//!    that walked steps 1 through 7 arrives here with three unforgeable values
//!    and leaves holding the one account emission is reachable from. The
//!    emissions are read off it —
//!    [`ClosedExpansion::declaration_site`] for the tokens an expansion shell
//!    hands the compiler, [`ClosedExpansion::test_carrier`] and
//!    [`ClosedExpansion::bench_carrier`] for the cargo a consumption target
//!    invokes, [`ClosedExpansion::published`] for the units a publication
//!    writes to their own addresses. What an expansion does not have, it states:
//!    [`ClosedExpansion::addressing`] says no carrier has been named and
//!    nothing has been published at this seam, which is an absence the account
//!    carries rather than a reason to refuse. [`compile_refusal`] and
//!    [`compile_refusal_text`] are the refusal family's one-call road through
//!    the whole of it, with the capture in front, and
//!    [`RefusalFamilyExpansion`] is that family's own view over the closed
//!    expansion this step binds.
//!
//! # The joined road: the same steps, a second time, for the carrier
//!
//! A projection that plans cargo into a CARRIER has said where those tokens are
//! compiled and nothing about how they get there. The vehicle is a second
//! projection — the generated support shell — and a door that wants its
//! declaration delivered walks the same eight steps a second time for it.
//!
//! No new step, no station, and no lobby: the carrier's account is
//! [`OwnerContentAccount::captured`] over the SAME captured surface, its plan is
//! [`ProjectionPlan::planned`], its unit is [`RenderedUnit::materialized`], its
//! proof is [`ProjectionClosure::proved`], its explanation is
//! [`ProjectionExplanationView::complete`], and its terminal is
//! [`ClosedExpansion::bound`]. Every one of those is the road above, called
//! again.
//!
//! What sits between the two roads is the one thing neither of them is: the physical assembly.
//! The assembly home's crate-internal promotion road reads one terminal's own proved carrier partition and refuses anything that is not that partition's own — promotion belongs to the road that owns the source's rendering vocabulary, because the envelope the tokens ride in is that road's declaration and no terminal carries a copy of it to check against.
//! The [`SupportAssembly`] construction road verifies that the axes compose — one root, one published expectation, every carried unit consumed once, no unit reaching a second destination — and [`assembled_shell`] is the one road from a carrier plan and a verified assembly to a rendered carrier.
//! The carrier's own composition road is crate-internal, so there is no way to an exported shell that skips the verification.
//!
//! That road establishes the join the assembly cannot: the carrier PLAN's
//! declared root against the assembly's. An assembly proves its cargo is one
//! declaration's, and a plan for a second declaration agrees with every reading
//! downstream, because the rendered unit is born wearing that plan's own
//! metadata. So the comparison is made at the public seam, where both values
//! exist, and what comes back is [`ShellComposition`] — this home's composition
//! body where the pair is not one declaration's, the carrier's own rendering
//! body where the tokens pass their bound, each carried whole.
//!
//! [`compile_declaration`] is the refusal family's one-call road through both,
//! and inside what it hands back stands a [`JoinedExpansion`]: both terminals
//! and the assembly that joined them. Its TWO declaration-site cargos are
//! exactly the two terminals' declaration-site partitions — the implementation
//! members, and the shell definition — read off the terminals themselves rather
//! than joined into a third value nobody proved. An emitter writes both.
//!
//! [`compile_refusal`] is unchanged and its callers stand: a caller that wants
//! the implementation projection alone asks for exactly that. The difference
//! between the two roads is what is added, never a different first one.
//!
//! # The complete account: every kind of the sealed roster answers
//!
//! A door produces some kinds and produces none of the rest, and the value it
//! hands back says both. [`AccountedExpansion`] is that value: the joined road's
//! own product, and beside it a [`KindDispositions`] carrying one required seat
//! for every row of [`ProjectionKindRow`] — the enumerated kind roster, emitted
//! by the same declaration that declares the kinds, so it cannot be shorter than
//! the roster it stands for.
//!
//! The generated rows name the one output a disposition names, read off the
//! terminals that produced them. The rest carry the existing disposition
//! vocabulary: what a projection's absence IS, and on whose fact or under whose
//! profile. Nothing is silently absent and nothing is a seat generated to look
//! full, which is the whole of what the roster buys — a reader asks why an
//! implementation arrived, why no bench did, and where the evaluation support
//! went, and reads three answers off one value:
//! [`AccountedExpansion::disposition`] for what happened to a kind,
//! [`AccountedExpansion::landed`] for which delivery a generated kind's cargo
//! belongs to, and the assembly's own axes for what the carrier delivers.
//!
//! What a door CAN generate is decided at the door, never here. The refusal
//! family's door generates the implementation projection and the carrier, and
//! states the standing of the six it does not: each of those kinds' plans names
//! a fact the machine mints — a schema and a byte role, a measured unit and a
//! work currency, a host contract, a port and a wire contract, a documented
//! subject and its audience, an authored pattern and its arguments — and
//! [`OwnerIdentityRef`] has one production road, which takes a commitment the
//! machine already minted. An expansion holds none, so the honest answer is that
//! the profile does not offer the kind, at that profile and at its version.
//!
//! Every step refuses in its own vocabulary: [`ProjectionPlanning`] at the
//! account, the watch set, and the plan; [`RenderingRefusal`] at a rendered
//! unit; [`ProjectionClosureRefusal`] at the proof and at the emissions it
//! splits; [`ExplanationCoverage`] at the view; [`ExpansionBindingRefusal`] at
//! the binding, where three separately produced values can disagree about their
//! parentage in exactly three places and each of them is a different repair.
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
//! use macroonz::Bounded;
//! use threadpak_macroc::derive_refusal::diagnose;
//! use threadpak_macroc::{
//!     ClosedExpansion, DecisionTrace, DigestContract, RefusalFamilyImplementationContent,
//!     RefusalFamilyImplementationProjection,
//!     ExplanationAnswer, ExplanationBindingRefusal, GeneratedDelimiter, GeneratedToken,
//!     GeneratedTree, MacrocDiagnostic, MemberDestination, OriginEdge,
//!     OriginRelation, OriginTrail, OwnerContentAccount, OwnerFactRef, PlanDecisions, PlannedMember,
//!     PlannedMembership, PlannedOutput, ProfileVersion, ProjectionClosure, ProjectionContext,
//!     ProjectionDisposition, ProjectionExplanation, ProjectionExplanationView, ProjectionIdentity,
//!     ProjectionPlan, ProjectionRole, ProjectionTranscript, RenderRefusal,
//!     RenderedImplementation, RenderedProjection, RenderedRole, RenderedUnit,
//!     TraceDecision, TraceEntry,
//! };
//!
//! /// Plan, render, close, explain, and bind one implementation projection, with
//! /// every refusal on the road projected into the plane's own diagnostic.
//! fn expand() -> Result<ClosedExpansion<RefusalFamilyImplementationProjection>, MacrocDiagnostic> {
//!     let role = RenderedImplementation::RenderedFamilyImpl;
//!
//!     // 1. The account. At expansion time the content IS the captured material.
//!     let declaration = ProjectionIdentity::derived(ProjectionTranscript::rooted(
//!         ProjectionRole::CapturedDeclaration,
//!         b"enum ReadRefusal { NotNamed }",
//!         0,
//!     ));
//!     let account = OwnerContentAccount::<RefusalFamilyImplementationProjection>::captured(declaration);
//!
//!     // 2. The intent: thirty-two bytes over the kind's declared name and what
//!     //    it was meant over. Two doors that meant the same thing derive one.
//!     let same_intent = OwnerContentAccount::<RefusalFamilyImplementationProjection>::captured(declaration);
//!     assert_eq!(account.intent().as_bytes(), same_intent.intent().as_bytes());
//!
//!     // 3. The context, and the watch set derived from it and the account.
//!     let profile = ProjectionIdentity::derived(ProjectionTranscript::rooted(
//!         ProjectionRole::DeclaredName,
//!         b"example.profile.rust-declaration",
//!         0,
//!     ));
//!     let context = ProjectionContext {
//!         profile,
//!         profile_version: ProfileVersion::declared(1),
//!         generator: ProjectionIdentity::derived(ProjectionTranscript::rooted(
//!             ProjectionRole::GeneratorVersion,
//!             b"example.generator.schema-1",
//!             0,
//!         )),
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
//!     let plan = ProjectionPlan::<RefusalFamilyImplementationProjection>::planned(
//!         account,
//!         context,
//!         RefusalFamilyImplementationContent {
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
//!         // The five decided seats, in the order a plan's transcript writes
//!         // them. Every field is required, so none can be left unstated.
//!         PlanDecisions {
//!             membership: PlannedMembership::complete(
//!                 PlannedMember {
//!                     role,
//!                     output: PlannedOutput {
//!                         semantic_key: key,
//!                         destination: MemberDestination::AtDeclarationSite,
//!                         origin: origin.clone(),
//!                         expected_profile: profile,
//!                         expected_profile_version: ProfileVersion::declared(1),
//!                         digest_contract: DigestContract::over(key),
//!                     },
//!                 },
//!                 [],
//!             ),
//!             invalidation,
//!             trace: DecisionTrace::from_entry(TraceEntry {
//!                 subject: ProjectionIdentity::derived(ProjectionTranscript::under_projection(
//!                     ProjectionRole::Plan,
//!                     &declaration,
//!                     b"implementation-derivation",
//!                     0,
//!                 )),
//!                 decision: TraceDecision::SelectedBecause(assumed),
//!             }),
//!             origin: origin.clone(),
//!             nonclaims: Bounded::empty(),
//!         },
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
//!     // 6. The closure: the rebuild equals the plan, and the emissions are
//!     //    split by delivery and kept inside the proof.
//!     let closure = ProjectionClosure::proved(
//!         plan.identity(),
//!         plan.membership(),
//!         RenderedProjection::complete(unit, []),
//!     )
//!     .map_err(|refusal| diagnose::closure_refused(&refusal))?;
//!
//!     // 7. The explanation: every seat this kind owes, answered once, over the
//!     //    plan and the proof themselves — so the view names its own parentage.
//!     let kind = ProjectionIdentity::derived(ProjectionTranscript::rooted(
//!         ProjectionRole::DeclaredName,
//!         b"example.kind.refusal-family-implementation-projection",
//!         0,
//!     ));
//!     let explanation = ProjectionExplanationView::<RefusalFamilyImplementationProjection>::complete(
//!         &plan,
//!         &closure,
//!         vec![
//!         ProjectionExplanation::answered(ExplanationAnswer::Kind { kind }),
//!         ProjectionExplanation::answered(ExplanationAnswer::Owner { owner: assumed }),
//!         ProjectionExplanation::answered(ExplanationAnswer::CausingDeclarations {
//!             sources: plan.account().commitment(),
//!         }),
//!         ProjectionExplanation::answered(ExplanationAnswer::Profile {
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
//!         ],
//!     )
//!     .map_err(|coverage| {
//!         diagnose::explanation_refused(&ExplanationBindingRefusal::Coverage(coverage))
//!     })?;
//!
//!     // 8. The closed expansion: the plan, the proof, and the explanation under
//!     //    one identity — the one value the emissions are read off, bound only
//!     //    where the three name one another.
//!     ClosedExpansion::bound(plan, closure, explanation)
//!         .map_err(|refusal| diagnose::expansion_refused(&refusal))
//! }
//! ```
//!
//! # Declaration order
//!
//! The `pub mod` list below records this crate's intended one-way dependency order for review: a module imports only owners declared earlier in the list.
//! Rust does not enforce direction between modules in one crate, so this order and the informed-request seam from `derive_refusal` to `mutation_descriptor` are review-enforced.
//! Cargo enforces acyclicity only where a boundary is a crate dependency.
//!
//! `reorder_modules = false` in `rustfmt.toml` keeps a formatter from re-alphabetizing the review surface.
//!
//! The crate README draws the explanatory edge map, while each home's `use` declarations remain the actual module edges.
//!
//! A directory module's edges are the union of every file under it, so review follows submodule imports as part of the owning home.
//!
//! Every module above is a directory home carrying the repository's file grammar, and `mod.rs` is its door.
//! Private seat files stay behind that door, and public names are re-exported through the owning module path.
//!
//! `question` is a leaf vocabulary seated before both readers that would otherwise import one another.
//! Structural claims inside a module remain compiler-enforced where private fields, sealed rosters, constructors, or `const` checks make violations unwritable.

pub mod plane;
pub mod token;
pub mod refusal;
pub mod diagnostics;
pub mod question;
pub mod origin_graph;
pub mod planning;
pub mod codec;
pub mod test_descriptor;
pub mod mutation_descriptor;
pub mod benchmark_descriptor;
pub mod explanation_protocol;
pub mod closure;
pub mod composition;
pub mod pattern_stamp;
pub mod generated_support;
pub mod derive_refusal;

pub use closure::{
    CarriedTokens, ClosedExpansion, ClosureIssue, DeliveryAddressing, ExpansionBindingRefusal,
    PartitionCargo, PartitionedEmission, ProjectionClosure, ProjectionClosureRefusal,
    RenderedProjection, RenderedUnit, RenderingRefusal,
};
pub use composition::{
    CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DESCRIPTOR_KINDS,
    DescriptorKind, DescriptorProvider,
};
pub use derive_refusal::{
    CapturedCause, CapturedFamilyFacts, CauseOrderStanding, CrateBinding, DEFAULT_CRATE_BINDING,
    DeclaredMutations, DeclaredTrials, DerivedMembership, DerivedPlan, ExplanationBindingRefusal,
    ExplanationSeat, MutationDeclarationPosture, RefusalCompileContext, RefusalDerivationDraft,
    RefusalDeriveCapture, RefusalDeriveRefusal, RefusalDeriveSurface, RefusalFamilyExpansion,
    RefusalOwnerFacts, RenderRefusal, SurfaceCaptureRefusal, TextCompileRefusal,
    TrialDeclarationPosture, captured, captured_text, compile_declaration, compile_refusal,
    compile_refusal_text,
};
pub use diagnostics::{
    DiagnosticSite, MacrocDiagnostic, MacrocPhase, ObservedClassification, RelatedIdentity,
    RelatedSet, RelatedSetCompletion, RelatedSetTruncation, RepairAction, ReproductionRoute,
    SiteCoordinate,
};
pub use explanation_protocol::{
    ClosureProofSeal, ExplanationAnswer, ExplanationCoverage, ExplanationCoverageIssue,
    ProjectionExplanation, ProjectionExplanationView, ProvedClosure, kind_admits,
};
pub use generated_support::{
    AccountedExpansion, AssemblyIssue, AxisCargo, CargoAxis, CarrierAssembly, EvaluationCargo,
    JoinedExpansion, ProvedCargo, ShellComposition, SupportAssembly, assembled_shell,
};
pub use origin_graph::{
    DecisionTrace, Nonclaim, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
pub use plane::{
    AuthoringLimitProfile, BUNDLE_IDENTITY_PROFILE, CAPTURED_DECLARATION_IDENTITY_PROFILE,
    CLOSED_EXPANSION_IDENTITY_PROFILE, CLOSURE_IDENTITY_PROFILE, ClosedExpansionId, ClosureId,
    DECLARATION_DOCUMENTATION_IDENTITY_PROFILE, DECLARED_NAME_IDENTITY_PROFILE,
    DIAGNOSTIC_RELATION_IDENTITY_PROFILE, EXPLANATION_IDENTITY_PROFILE, ExplanationId,
    GENERATED_UNIT_IDENTITY_PROFILE, GENERATOR_VERSION_IDENTITY_PROFILE, GeneratorIdentity,
    GeneratorProfileId, GeneratorSchemaVersion, HumanProjection, IDENTITY_PROFILE_STEM,
    IdentityProfile, IdentityProfileVersion, IdentitySubject, MACROC_GENERATOR,
    MUTATION_DECLARATION_IDENTITY_PROFILE, ORIGIN_NODE_IDENTITY_PROFILE, OwnerFactName,
    OwnerFactRef, OwnerIdentityRef, PLAN_IDENTITY_PROFILE, PROJECTION_INTENT_IDENTITY_PROFILE,
    PlanId, PreimageFamily, ProfileVersion, ProjectionIdentity, ProjectionProvenance,
    ProjectionRole, ProjectionTranscript, RENDERED_UNIT_IDENTITY_PROFILE, RenderedRole,
    RenderedRoleSeal, SoleRenderedUnit, SubjectSeal, TRIAL_DECLARATION_IDENTITY_PROFILE,
    TranscriptAnchoring, encode_bytes, encode_length,
};
pub use planning::{
    BenchmarkDescriptorContent, BenchmarkDescriptorProjection, CapturedDependencies,
    CauseAnchoring, CodecContent, CodecDirection, CodecProjection, ContentAddressing,
    DeclaredBootstrap, DigestContract, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID, EmissionPartition,
    ExpectedGeneratedSupportSchemaId, InvalidationSet, InvalidationTrigger, KindDispositions,
    KindSeal, MemberDestination, ObligationAnchoring, OwnerContentAccount, PatternStampContent,
    PatternStampProjection, PlanDecisions, PlannedMember, PlannedMembership, PlannedOutput,
    ProjectionBundlePlan, ProjectionContext, ProjectionDisposition, ProjectionIntentId,
    ProjectionKind, ProjectionKindRow, ProjectionPlan, RefusalFamilyImplementationContent,
    RefusalFamilyImplementationProjection, RenderedImplementation, RowMaterialPosture,
    TestDescriptorContent, TestDescriptorProjection, UNIVERSAL_QUESTIONS, VerifiedDerived,
};
pub use question::{EXPLANATION_PROTOCOL_VERSION, ExplanationQuestion, QuestionApplicability};
pub use refusal::{BoundAxis, ProjectionPlanning, ProjectionPlanningIssue};
pub use token::{
    CaptureBound, CaptureWalk, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, CoordinateRole, GeneratedDelimiter, GeneratedSpacing, GeneratedToken,
    GeneratedTree, LiteralReadCause, SourceCoordinate, SpanHandle, SpanResolutionRefusal,
    SpanTable, TextCapture, TextReadCause, TextReadRefusal, TokenPath, capture_literal,
};

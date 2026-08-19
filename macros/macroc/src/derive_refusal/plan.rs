//! Planning one refusal-family derivation: the identities, the membership, the
//! origin graph, the decision trace, and the watch set — all of it before a
//! token of Rust exists.
//!
//! # Derived identities
//!
//! Nothing in this module is handed an identity by a caller.
//! Each one is derived from a complete transcript that names the profile and its
//! version, the role, the anchor it hangs off at full width, the material it
//! stands over at full length, and the generator that produced it — so the whole
//! plan is a deterministic function of the captured declaration, and two
//! captures of the same declaration produce the same plan on every machine.
//!
//! That is what makes the plan comparable to the rendering afterwards.
//! A plan whose identities were supplied could be made to agree with any
//! rendering by supplying different ones.

use super::render::contract_path_bytes;
use super::types::{
    CauseOrderStanding, DerivedMembership, RefusalDerivationDraft, RefusalOwnerFacts,
};
use crate::origin_graph::{
    DecisionTrace, Nonclaim, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
use crate::plane::{
    AssumptionLimit, DerivedTypeSubject, GeneratedUnitSubject, GeneratorVersionSubject,
    ImplementedContractSubject, MACROC_GENERATOR, NonclaimLimit, OriginNodeSubject, OwnerFactRef,
    ProfileVersion, ProjectionIdentity, ProjectionKindSubject, ProjectionProfileSubject,
    ProjectionRole, ProjectionTranscript, RenderedRole, TracedSubject, encode_bytes,
};
use crate::planning::{
    DeriveImplContent, DeriveImplProjection, DigestContract, GraphAnchoring, OwnerContentAccount,
    PlanDecisions, PlannedMember, PlannedMembership, PlannedOutput, ProjectionContext,
    ProjectionDisposition, ProjectionPlan, RenderedImplementation, TargetBinding,
};
use crate::refusal::ProjectionPlanning;
use threadpak::types::Bounded;

/// The projection profile a Rust-declaration expansion runs under.
#[must_use]
pub fn rust_declaration_profile() -> ProjectionIdentity<ProjectionProfileSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        b"macroc.profile.rust-declaration",
        0,
    ))
}

/// That profile's version, as the services declare it.
#[must_use]
pub const fn rust_declaration_profile_version() -> ProfileVersion {
    ProfileVersion::declared(1)
}

/// The generator identity: which generator, under which rendered shape,
/// produced a plan.
///
/// The content is the generator's two LOAD-BEARING facts — its declared name and
/// its schema version — and nothing else.
/// The schema version is the fact that moves when the rendered shape moves, so
/// it is the fact this identity turns on, and the package version is
/// deliberately absent from it.
#[must_use]
pub fn generator_version() -> ProjectionIdentity<GeneratorVersionSubject> {
    let mut content = Vec::new();
    encode_bytes(
        MACROC_GENERATOR.profile().spelling().as_bytes(),
        &mut content,
    );
    content.extend_from_slice(&MACROC_GENERATOR.schema().position().to_be_bytes());
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        &content,
        0,
    ))
}

/// This projection kind's identity.
#[must_use]
pub fn derive_impl_kind() -> ProjectionIdentity<ProjectionKindSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::Plan,
        b"macroc.kind.derive-impl-projection",
        0,
    ))
}

/// One planned derivation: the plan, the semantic keys its members carry, and
/// what happened to the cause-order projection.
#[must_use = "a planned derivation carries the plan and the cause-order disposition"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedPlan {
    plan: ProjectionPlan<DeriveImplProjection>,
    cause_order: ProjectionDisposition,
}

impl DerivedPlan {
    /// The identity-bearing plan.
    pub const fn plan(&self) -> &ProjectionPlan<DeriveImplProjection> {
        &self.plan
    }

    /// What happened to the typed cause-order projection.
    pub const fn cause_order(&self) -> &ProjectionDisposition {
        &self.cause_order
    }

    /// Take the plan out, for a closed expansion to bind.
    pub fn into_parts(self) -> (ProjectionPlan<DeriveImplProjection>, ProjectionDisposition) {
        (self.plan, self.cause_order)
    }
}

/// The semantic key one rendered role's member answers to.
///
/// Derived from the captured declaration and the ROLE, so the two members of one
/// derivation are different identities by construction, and the same role over
/// the same declaration is the same identity every time.
#[must_use]
pub fn semantic_key(
    draft: &RefusalDerivationDraft,
    role: RenderedImplementation,
) -> ProjectionIdentity<GeneratedUnitSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::GeneratedUnit,
        &draft.surface().identity(),
        role.described().as_bytes(),
        role.slot(),
    ))
}

/// The origin node one rendered role's member sits at.
#[must_use]
pub fn member_node(
    draft: &RefusalDerivationDraft,
    role: RenderedImplementation,
) -> ProjectionIdentity<OriginNodeSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::OriginNode,
        &draft.surface().identity(),
        role.described().as_bytes(),
        role.slot(),
    ))
}

/// The entry account one captured declaration walks in the door carrying.
///
/// # One account, and no second one
///
/// An expansion is handed token material and nothing else: nothing has been
/// linked, so the content's own address IS the capture identity the plane
/// derived for it, and the dependency set is empty — which is a STATED fact
/// about content that stands on nothing, not a set somebody forgot to supply.
///
/// It is derived here rather than supplied, on the same terms as every other
/// identity in this module: a caller that could hand a plan a different account
/// could make the plan's identity, its watch set, and its origin edges all agree
/// with a declaration it was not planned over.
pub fn content_account(
    draft: &RefusalDerivationDraft,
) -> OwnerContentAccount<DeriveImplProjection> {
    OwnerContentAccount::captured(draft.surface().identity())
}

/// The origin node the authored declaration sits at.
///
/// # Read, never re-derived
///
/// It is the ACCOUNT's node
/// ([`OwnerContentAccount::origin_node`](crate::planning::OwnerContentAccount::origin_node)).
/// A node derived here over a preimage of this module's own choosing would be a
/// second node for one piece of content: the same declaration would stand at one
/// node as the content this plan is over, and at another as the content some
/// other plan declares it stands on, and the origin graph would carry two
/// answers to one question. One content, one node.
#[must_use]
pub fn authored_node(draft: &RefusalDerivationDraft) -> ProjectionIdentity<OriginNodeSubject> {
    content_account(draft).origin_node()
}

/// The origin trail one member walks back along.
#[must_use]
pub fn member_origin(draft: &RefusalDerivationDraft, role: RenderedImplementation) -> OriginTrail {
    OriginTrail::from_edge(OriginEdge {
        from: authored_node(draft),
        relation: OriginRelation::SemanticDerivation,
        to: member_node(draft, role),
    })
}

/// The shared plan context one expansion is decided under.
///
/// # Bounds
///
/// What the plan was planned OVER is not here.
/// That is the entry account's fact ([`content_account`]), stated once: a
/// context that also named the content would be a second account of what the
/// plan stands on, and the watch derivation would then be reading a copy rather
/// than the account.
#[must_use]
pub fn expansion_context(draft: &RefusalDerivationDraft) -> ProjectionContext {
    ProjectionContext {
        graph: GraphAnchoring::CapturedDeclarationOnly(draft.surface().identity()),
        profile: rust_declaration_profile(),
        profile_version: rust_declaration_profile_version(),
        generator: generator_version(),
        target: TargetBinding::TargetFree,
    }
}

/// The complete logical membership one draft declares.
///
/// # Both surfaces are declared, never the production half alone
///
/// One implementation meaning is delivered as TWO surfaces, so every contract
/// this draft declares contributes two members: the production implementation
/// under its own role, and the mutation-evaluation copy under that role's twin
/// ([`RenderedImplementation::twin`]).
/// The output firewall is exactly that the declared set is the whole set, and the
/// closure rebuilds the membership role by role — so a copy standing outside the
/// membership would be a surface crossing the wall that the proof never looks at,
/// and "nothing is emitted that did not close" would be true of the production
/// half alone.
/// The twin is READ from the roster rather than named here, so a roster that
/// paired its seats differently pairs these members differently too.
///
/// # Totality
///
/// [`DerivedMembership`] has exactly two answers and each names a statically
/// known set of contracts, so there is no count to read, nothing to admit, and no
/// failure to repair.
/// The match below is the whole function: one answer builds a two-member
/// membership — one contract, two surfaces — the other builds a four-member one,
/// and [`PlannedMembership::complete`] settles the magnitude at compile time.
/// Every role in each array is written literally and no two of them are the same
/// seat, which is what the total constructor asks of a caller that names its
/// roles rather than reading a count.
#[must_use]
pub fn membership(draft: &RefusalDerivationDraft) -> PlannedMembership<RenderedImplementation> {
    let member = |role: RenderedImplementation| {
        let key = semantic_key(draft, role);
        PlannedMember {
            role,
            output: PlannedOutput {
                semantic_key: key,
                // The roster's own constant answer, not a literal repeated here:
                // where a member under a role lands is the role's fact, and a
                // second copy of it would be a second answer to one question.
                destination: role.destination(),
                origin: member_origin(draft, role),
                expected_profile: rust_declaration_profile(),
                expected_profile_version: rust_declaration_profile_version(),
                digest_contract: DigestContract::over(key),
            },
        }
    };
    let family = RenderedImplementation::RenderedFamilyImpl;
    let cause_order = RenderedImplementation::RenderedCauseOrderImpl;
    match draft.declared_membership() {
        DerivedMembership::FamilyOnly => {
            PlannedMembership::complete(member(family), [member(family.twin())])
        }
        DerivedMembership::FamilyAndCauseOrder => PlannedMembership::complete(
            member(family),
            [
                member(family.twin()),
                member(cause_order),
                member(cause_order.twin()),
            ],
        ),
    }
}

/// Plan one refusal-family derivation.
///
/// # Errors
///
/// Returns the planning family when a declared magnitude is exceeded while
/// assembling the trace or the watch set.
pub fn planned(
    draft: &RefusalDerivationDraft,
    owner_facts: RefusalOwnerFacts,
    nonclaims: Bounded<Nonclaim, NonclaimLimit>,
) -> Result<DerivedPlan, ProjectionPlanning> {
    let account = content_account(draft);
    let context = expansion_context(draft);
    let standing = draft.cause_order_standing();
    let traced: ProjectionIdentity<TracedSubject> =
        ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::Plan,
            &draft.surface().identity(),
            b"refusal-family-derivation",
            0,
        ));

    let trace = DecisionTrace::recorded(
        TraceEntry {
            subject: traced,
            decision: TraceDecision::SelectedBecause(owner_facts.body_shapes),
        },
        vec![
            TraceEntry {
                subject: traced,
                decision: match standing {
                    CauseOrderStanding::Declared => {
                        TraceDecision::SelectedBecause(owner_facts.canonical_order_is_shape_ruled)
                    }
                    CauseOrderStanding::NotApplicableToShape => {
                        TraceDecision::OmittedBecause(owner_facts.canonical_order_is_shape_ruled)
                    }
                },
            },
            TraceEntry {
                subject: traced,
                decision: TraceDecision::SelectedBecause(owner_facts.cause_key_grammar),
            },
        ],
    )?;

    // Derived from the context and the ACCOUNT rather than listed here: an
    // expansion-time context is decided against the same capture that caused it,
    // so a roster written at this call site would be knowledge held here about a
    // value declared elsewhere. The account is what the derivation reads for
    // "what does this content stand on", and it is the same value the plan is
    // planned over below.
    let invalidation = context.watch_set(&account)?;

    let derived_type: ProjectionIdentity<DerivedTypeSubject> =
        ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::GeneratedUnit,
            &draft.surface().identity(),
            draft.surface().family_name().as_bytes(),
            0,
        ));
    let contract: ProjectionIdentity<ImplementedContractSubject> =
        ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::GeneratedUnit,
            &draft.surface().identity(),
            contract_path_bytes(draft.surface()).as_slice(),
            1,
        ));

    let assumptions: Bounded<OwnerFactRef, AssumptionLimit> = Bounded::from_array([
        owner_facts.body_shapes,
        owner_facts.canonical_order_is_shape_ruled,
        owner_facts.cause_key_grammar,
    ]);

    // The five decided seats travel as ONE value, in the order a plan's
    // transcript writes them. Every field of the bundle is required, so a seat
    // added to it fails to compile here rather than arriving unwritten.
    let plan = ProjectionPlan::<DeriveImplProjection>::planned(
        account,
        context,
        DeriveImplContent {
            derived_type,
            contract,
            assumptions,
        },
        PlanDecisions {
            membership: membership(draft),
            invalidation,
            trace,
            origin: OriginTrail::from_edge(OriginEdge {
                from: authored_node(draft),
                relation: OriginRelation::AuthoredDeclaration,
                to: member_node(draft, RenderedImplementation::RenderedFamilyImpl),
            }),
            nonclaims,
        },
    )?;

    // The disposition names ONE output, because that is the shape a disposition
    // has, and the one it names is the cause-order contract's PRODUCTION member.
    // The complete set the contract contributes — that member and its
    // mutation-evaluation twin — is the plan's own membership above, which is
    // where a reader asking what was materialized reads; this seat answers the
    // narrower question the explanation protocol asks, which is what happened to
    // the cause-order projection at all.
    let cause_order_role = RenderedImplementation::RenderedCauseOrderImpl;
    let cause_order = match standing {
        CauseOrderStanding::Declared => {
            // Derived inside the arm that names it: a key is a BLAKE3
            // derivation, and the other arm has no output to state one for.
            let key = semantic_key(draft, cause_order_role);
            ProjectionDisposition::Generated {
                output: Box::new(PlannedOutput {
                    semantic_key: key,
                    destination: cause_order_role.destination(),
                    origin: member_origin(draft, cause_order_role),
                    expected_profile: rust_declaration_profile(),
                    expected_profile_version: rust_declaration_profile_version(),
                    digest_contract: DigestContract::over(key),
                }),
            }
        }
        CauseOrderStanding::NotApplicableToShape => ProjectionDisposition::NotApplicable {
            because: owner_facts.canonical_order_is_shape_ruled,
        },
    };

    Ok(DerivedPlan { plan, cause_order })
}

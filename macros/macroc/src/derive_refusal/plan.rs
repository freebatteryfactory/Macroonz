//! Planning one refusal-family derivation: the identities, the membership, the
//! origin graph, the decision trace, and the watch set — all of it before a
//! token of Rust exists.
//!
//! # Every identity here is DERIVED, and the derivation is readable
//!
//! Nothing in this module is handed an identity by a caller. Each one is derived
//! from a complete transcript that names the profile and its version, the role,
//! the anchor it hangs off at full width, the material it stands over at full
//! length, and the generator that produced it — so the whole plan is a
//! deterministic function of the captured declaration, and two captures of the
//! same declaration produce the same plan on every machine.
//!
//! That is what makes the plan comparable to the rendering afterwards. A plan
//! whose identities were supplied could be made to agree with any rendering by
//! supplying different ones.

use super::types::{CauseOrderStanding, RefusalDerivationDraft, RefusalOwnerFacts};
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
    CauseAnchoring, DeriveImplContent, DeriveImplProjection, DigestContract, GraphAnchoring,
    InvalidationTrigger, MemberDestination, PlannedMember, PlannedMembership, PlannedOutput,
    ProjectionContext, ProjectionDisposition, ProjectionPlan, RenderedImplementation,
    TargetBinding,
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
/// its schema version — and nothing else. The package version is deliberately
/// absent: it sat here before, spelled `threadpak-macroc@0.0.0`, and at `0.0.0`
/// before the first release it never moved, so a plan watching
/// [`InvalidationTrigger::GeneratorVersionChanged`] was watching a value that
/// could not change. The schema version is the fact
/// that moves when the rendered shape moves, so it is the fact this identity
/// turns on.
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedPlan {
    plan: ProjectionPlan<DeriveImplProjection>,
    cause_order: ProjectionDisposition,
}

impl DerivedPlan {
    /// The identity-bearing plan.
    #[must_use]
    pub const fn plan(&self) -> &ProjectionPlan<DeriveImplProjection> {
        &self.plan
    }

    /// What happened to the typed cause-order projection.
    #[must_use]
    pub const fn cause_order(&self) -> &ProjectionDisposition {
        &self.cause_order
    }

    /// Take the plan out, for a closed expansion to bind.
    #[must_use]
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

/// The origin node the authored declaration sits at.
#[must_use]
pub fn authored_node(draft: &RefusalDerivationDraft) -> ProjectionIdentity<OriginNodeSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::OriginNode,
        &draft.surface().identity(),
        b"authored-declaration",
        u32::MAX,
    ))
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
#[must_use]
pub fn expansion_context(draft: &RefusalDerivationDraft) -> ProjectionContext {
    let captured = draft.surface().identity();
    ProjectionContext {
        graph: GraphAnchoring::CapturedDeclarationOnly(captured),
        profile: rust_declaration_profile(),
        profile_version: rust_declaration_profile_version(),
        sources: CauseAnchoring::CapturedDeclaration(captured),
        generator: generator_version(),
        target: TargetBinding::TargetFree,
    }
}

/// The complete logical membership one draft declares.
#[must_use]
pub fn membership(draft: &RefusalDerivationDraft) -> PlannedMembership<RenderedImplementation> {
    let member = |role: RenderedImplementation| {
        let key = semantic_key(draft, role);
        PlannedMember {
            role,
            output: PlannedOutput {
                semantic_key: key,
                destination: MemberDestination::AtDeclarationSite,
                origin: member_origin(draft, role),
                expected_profile: rust_declaration_profile(),
                expected_profile_version: rust_declaration_profile_version(),
                digest_contract: DigestContract::over(key),
            },
        }
    };
    match draft.declared_membership().roles() {
        [first, rest @ ..] => {
            let rows: Vec<PlannedMember<RenderedImplementation>> =
                rest.iter().copied().map(member).collect();
            PlannedMembership::declared(member(*first), rows)
                .unwrap_or_else(|_| PlannedMembership::from_member(member(*first)))
        }
        // Unreachable: `DerivedMembership::roles` is non-empty for both answers,
        // and `DerivedMembership::is_empty` is a constant `false` proving it.
        [] => PlannedMembership::from_member(member(RenderedImplementation::RenderedFamilyImpl)),
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

    let invalidation = InvalidationTrigger::watched(
        context.cause_trigger(),
        vec![
            InvalidationTrigger::ProjectionProfileChanged {
                watched: context.profile,
            },
            InvalidationTrigger::GeneratorVersionChanged {
                watched: context.generator,
            },
        ],
    )?;

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
            binding_contract_bytes(draft).as_slice(),
            1,
        ));

    let assumptions: Bounded<OwnerFactRef, AssumptionLimit> = Bounded::from_array([
        owner_facts.body_shapes,
        owner_facts.canonical_order_is_shape_ruled,
        owner_facts.cause_key_grammar,
    ]);

    let plan = ProjectionPlan::<DeriveImplProjection>::planned(
        context,
        DeriveImplContent {
            derived_type,
            contract,
            assumptions,
        },
        membership(draft),
        invalidation,
        trace,
        OriginTrail::from_edge(OriginEdge {
            from: authored_node(draft),
            relation: OriginRelation::AuthoredDeclaration,
            to: member_node(draft, RenderedImplementation::RenderedFamilyImpl),
        }),
        nonclaims,
    )?;

    let cause_order = match standing {
        CauseOrderStanding::Declared => ProjectionDisposition::Generated {
            output: Box::new(PlannedOutput {
                semantic_key: semantic_key(draft, RenderedImplementation::RenderedCauseOrderImpl),
                destination: MemberDestination::AtDeclarationSite,
                origin: member_origin(draft, RenderedImplementation::RenderedCauseOrderImpl),
                expected_profile: rust_declaration_profile(),
                expected_profile_version: rust_declaration_profile_version(),
                digest_contract: DigestContract::over(semantic_key(
                    draft,
                    RenderedImplementation::RenderedCauseOrderImpl,
                )),
            }),
        },
        CauseOrderStanding::NotApplicableToShape => ProjectionDisposition::NotApplicable {
            because: owner_facts.canonical_order_is_shape_ruled,
        },
    };

    Ok(DerivedPlan { plan, cause_order })
}

/// The contract identity's transcript material: the crate binding travels into it,
/// because a rendering against a renamed dependency realizes the contract under
/// a different path and is a different generated unit.
fn binding_contract_bytes(draft: &RefusalDerivationDraft) -> Vec<u8> {
    let mut material = Vec::new();
    material.extend_from_slice(draft.surface().binding().spelling().as_bytes());
    material.push(b'.');
    material.extend_from_slice(b"refusal.RefusalFamily");
    material
}

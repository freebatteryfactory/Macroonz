//! Carrying one derivation across the wall: the test-descriptor projection the
//! exported shell IS, planned, rendered, closed, and bound over the same
//! captured surface the implementation projection was.
//!
//! # Why the door owns this at all
//!
//! The implementation projection plans its mutation-evaluation copies into the
//! TEST CARRIER, and a carrier is a vehicle: a plan that declares cargo into one
//! has said where the tokens are compiled and nothing about how they get there.
//! The vehicle is a second projection — the generated support shell — with its
//! own plan, its own rendering, its own proof, and its own terminal. Until this
//! road existed the copies were proved, partitioned, and had no way to reach a
//! consumption target at all.
//!
//! # One captured surface, two plans
//!
//! Both plans stand over the SAME entry account: the captured declaration's own
//! semantic commitment. That is what makes the assembly's one-root check
//! answerable — the carrier and the cargo name one declaration — and it is why
//! neither plan is derived from the other's identity: two projections over one
//! piece of content are two plans, not one plan read twice.
//!
//! # What this road never invents
//!
//! It states no obligation, no row, and no owner meaning. What the descriptor
//! challenges is carried under the posture an expansion can honestly state
//! ([`ObligationAnchoring::CapturedDeclarationOnly`]), and the trials axis is
//! ABSENT under the disposition that says why: the row material a descriptor
//! states about itself is the caller's declaration, and a derive door holds
//! none. That absence is exactly the evaluation-only delivery the carrier
//! renders an empty trials seat for.

use super::plan::{
    authored_node, expansion_context, rust_declaration_profile, rust_declaration_profile_version,
};
use super::render::EVALUATION_SUBJECT;
use super::types::{DerivedMembership, RefusalDerivationDraft, RefusalDeriveFact};
use super::{diagnose, evaluation_spellings, explain};
use crate::closure::{ClosedExpansion, ProjectionClosure, RenderedProjection, RenderedUnit};
use crate::diagnostics::MacrocDiagnostic;
use crate::explanation_protocol::{
    ExplanationAnswer, ProjectionExplanation, ProjectionExplanationView,
};
use crate::generated_support::{
    AxisCargo, CargoAxis, ProvedCargo, ShellComposition, SupportAssembly, assembled_shell,
};
use crate::origin_graph::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
use crate::plane::{
    GeneratedUnitSubject, OriginNodeSubject, OutputBytesSubject, ProjectionIdentity,
    ProjectionKindSubject, ProjectionRole, ProjectionTranscript, RenderedRole, SoleRenderedUnit,
};
use crate::planning::{
    DigestContract, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID, EmissionPartition, ObligationAnchoring,
    OwnerContentAccount, PlanDecisions, PlannedMember, PlannedMembership, PlannedOutput,
    ProjectionDisposition, ProjectionKind, ProjectionPlan, RenderedImplementation,
    RowMaterialPosture, TestDescriptorContent, TestDescriptorProjection,
};
use crate::test_descriptor::{
    ActivePointSelector, DeferredCargo, GeneratedSupportShell, ShellDeclarationRefusal,
    descriptor_plan,
};
use threadpak::types::Bounded;

/// The carrier projection's kind identity.
///
/// A declared name on the terms every other declared name in this home stands
/// under: one stable spelling this crate wrote down, derived under
/// [`ProjectionRole::DeclaredName`], separated from its neighbours by its own
/// content.
#[must_use]
pub fn carrier_kind() -> ProjectionIdentity<ProjectionKindSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::DeclaredName,
        b"macroc.kind.test-descriptor-projection",
        0,
    ))
}

/// The material the carrier member's own identities stand over.
///
/// One spelling, read by the semantic key, the origin node, and the trace's
/// subject, so the three cannot part company about which member they are about.
const CARRIER_MEMBER: &[u8] = b"generated-support-shell";

/// The row of every active-point roster that renders each point's original
/// operation.
///
/// The control, and the only row a copy this door renders stands at: the door
/// admits no mutation point, so the copies carry the production surface's own
/// operations under another subject's head.
const NO_MUTATION: &str = "NoMutation";

/// The evaluation contracts one declared membership delivers, read from the
/// roster's own twins rather than spelled at the road below.
///
/// # Authority
///
/// **The MEMBERSHIP is the quantifier, not the roster.** A shape that declares
/// no canonical cause order renders no cause-order copy at all, so the
/// cause-order active-point roster is never declared in that cargo — and a
/// constant standing at a row of a type nobody declared is a consumer's test
/// target failing inside an expansion it did not write. The twin is READ from
/// each production role, exactly as the plan and the rendering read it, so a
/// roster that paired its seats differently pairs these differently too.
fn deferred_contracts(membership: DerivedMembership) -> Vec<RenderedImplementation> {
    let family = RenderedImplementation::RenderedFamilyImpl;
    let cause_order = RenderedImplementation::RenderedCauseOrderImpl;
    match membership {
        DerivedMembership::FamilyOnly => vec![family.twin()],
        DerivedMembership::FamilyAndCauseOrder => vec![family.twin(), cause_order.twin()],
    }
}

/// The carrier member's semantic key.
///
/// Derived from the captured declaration and this member's own material, so the
/// carrier and every implementation member of one declaration are distinct
/// identities by construction.
#[must_use]
pub fn carrier_semantic_key(
    draft: &RefusalDerivationDraft,
) -> ProjectionIdentity<GeneratedUnitSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::GeneratedUnit,
        &draft.surface().identity(),
        CARRIER_MEMBER,
        SoleRenderedUnit::Sole.slot(),
    ))
}

/// The origin node the carrier member sits at.
#[must_use]
pub fn carrier_node(draft: &RefusalDerivationDraft) -> ProjectionIdentity<OriginNodeSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::OriginNode,
        &draft.surface().identity(),
        CARRIER_MEMBER,
        SoleRenderedUnit::Sole.slot(),
    ))
}

/// The origin trail the carrier member walks back along, to the authored
/// declaration the entry account already stands at.
#[must_use]
pub fn carrier_origin(draft: &RefusalDerivationDraft) -> OriginTrail {
    OriginTrail::from_edge(OriginEdge {
        from: authored_node(draft),
        relation: OriginRelation::SemanticDerivation,
        to: carrier_node(draft),
    })
}

/// One selector per evaluation contract THIS membership delivers: the constant
/// every activation site reads, the roster that constant stands on, and the row
/// it stands at.
///
/// One per contract rather than one for the home, because each contract's copy
/// declares its own active-point roster and one constant spelling across them
/// would name a single item twice wherever the shell lands them.
///
/// # Errors
///
/// Returns the carrier's declaration refusal where a spelling this home declared
/// is not one Rust identifier, or where two selectors are read through one
/// constant. Both are facts about the literal spellings the home declares beside
/// its evaluation roads, so neither is reachable while they are what they are —
/// and the road refuses rather than electing a selector, because a cargo that
/// dropped one would ship a copy that reads a name nobody brought into scope.
pub fn deferred_selectors(
    membership: DerivedMembership,
) -> Result<Vec<ActivePointSelector>, ShellDeclarationRefusal> {
    deferred_contracts(membership)
        .into_iter()
        .map(|role| {
            let (active_enum, selector) = evaluation_spellings(role);
            ActivePointSelector::declared(selector, active_enum, NO_MUTATION)
        })
        .collect()
}

/// What happened to the descriptor rows this carrier would have declared.
///
/// NOT APPLICABLE, citing the fact that says why: the claim, the suite, the
/// roles, the tags, the subject, the check, the population, and the callable a
/// row states are the caller's declarations, and a derive door is handed a
/// declaration and an expansion context — neither of which carries one. A door
/// that answered otherwise would be producing its own row material and then
/// proving it.
#[must_use]
pub fn rows_disposition() -> ProjectionDisposition {
    ProjectionDisposition::NotApplicable {
        because: RefusalDeriveFact::ARowIsTheCallersDeclarationAndNeverTheProducers.citation(),
    }
}

/// What happened to the bench material this carrier would have delivered.
///
/// NOT APPLICABLE, citing the seat rule: the carrier's published grammar writes
/// a trials seat and a deferred seat, and neither is the bench seat. The bench
/// crossing renders real material and rides its own shell today; this axis opens
/// when the reserved seat is written.
#[must_use]
pub fn bench_disposition() -> ProjectionDisposition {
    ProjectionDisposition::NotApplicable {
        because: RefusalDeriveFact::ACarrierSeatIsWrittenBeforeItIsFilled.citation(),
    }
}

/// Plan the carrier projection over one captured surface.
///
/// # Errors
///
/// Returns the planning diagnostic where the watch set cannot represent what the
/// entry account declares, or where a declared magnitude is passed while the
/// plan's seats are assembled.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the settled service road returns; this helper hands \n              it straight through"
)]
pub fn carrier_plan(
    draft: &RefusalDerivationDraft,
) -> Result<ProjectionPlan<TestDescriptorProjection>, MacrocDiagnostic> {
    let account =
        OwnerContentAccount::<TestDescriptorProjection>::captured(draft.surface().identity());
    let context = expansion_context(draft);
    let invalidation = context
        .watch_set(&account)
        .map_err(|refusal| diagnose::planning_refused(&refusal))?;
    let key = carrier_semantic_key(draft);
    let origin = carrier_origin(draft);
    let trace = DecisionTrace::from_entry(TraceEntry {
        subject: ProjectionIdentity::derived(ProjectionTranscript::under_projection(
            ProjectionRole::Plan,
            &draft.surface().identity(),
            CARRIER_MEMBER,
            0,
        )),
        decision: TraceDecision::SelectedBecause(
            RefusalDeriveFact::AnEvaluationCopyStandsOverALocalSubject.citation(),
        ),
    });
    ProjectionPlan::<TestDescriptorProjection>::planned(
        account,
        context,
        TestDescriptorContent {
            // The posture an expansion can honestly state: nothing has been
            // linked, so no obligation identity exists for anybody to name, and
            // the declaration this descriptor was derived from is named instead.
            obligation: ObligationAnchoring::CapturedDeclarationOnly(draft.surface().identity()),
            rows: RowMaterialPosture::CallerSupplied,
        },
        PlanDecisions {
            membership: PlannedMembership::from_member(PlannedMember {
                role: SoleRenderedUnit::Sole,
                output: PlannedOutput {
                    semantic_key: key,
                    // The shell is DEFINED where the declaration is — that is
                    // what makes it reachable — and the cargo it carries rides
                    // inside it under the cargo's own delivery. The answer is
                    // the carrier's own constant rather than a literal repeated
                    // here.
                    destination: GeneratedSupportShell::DESTINATION,
                    origin: origin.clone(),
                    expected_profile: rust_declaration_profile(),
                    expected_profile_version: rust_declaration_profile_version(),
                    digest_contract: DigestContract::over(key),
                },
            }),
            invalidation,
            trace,
            origin,
            nonclaims: Bounded::empty(),
        },
    )
    .map_err(|refusal| diagnose::planning_refused(&refusal))
}

/// Read one implementation terminal's test-carrier cargo into the evaluation
/// axis, or state that nothing was planned into it.
///
/// # What is read, and what is declared
///
/// The TOKENS are the terminal's, read off its own proved partition by the
/// assembly home's crate-internal promotion road, which refuses anything that is
/// not that partition's own. The SUBJECT and the SELECTIONS are this home's
/// declarations — the local type the copies stand over and the constants they
/// read their active points through — and they travel as the data they are.
///
/// # This road is the one lawful promotion point
///
/// The promotion road is crate-internal, and THIS is the road it exists for: the
/// envelope it authenticates nothing about — the subject and the selectors — is
/// declared here, three lines above the call, so the value that hands it in is
/// the value that owns it. A public promotion road would let any caller wrap
/// proved tokens in an envelope of its own and hand back cargo whose whole claim
/// is that its contents are one terminal's own.
///
/// # Errors
///
/// Returns the carrier's declaration diagnostic where a spelling this home
/// declared is not one the cargo admits, and the assembly diagnostic where the
/// tokens are not the terminal's own.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the settled service road returns; this helper hands \n              it straight through"
)]
pub fn evaluation_axis<K: ProjectionKind>(
    draft: &RefusalDerivationDraft,
    implementation: &ClosedExpansion<K>,
) -> Result<AxisCargo<ProvedCargo>, MacrocDiagnostic> {
    let Some(tree) = implementation.test_carrier().tokens() else {
        // Nothing was planned into the carrier at all — a stated fact about the
        // plan rather than an empty cargo, and the disposition says which.
        return Ok(AxisCargo::Absent {
            because: ProjectionDisposition::NotRequested,
        });
    };
    let selectors = deferred_selectors(draft.declared_membership())
        .map_err(diagnose::carrier_declaration_refused)?;
    let cargo = DeferredCargo::deferred(EVALUATION_SUBJECT, selectors, tree.clone())
        .map_err(diagnose::carrier_declaration_refused)?;
    ProvedCargo::carried(
        implementation,
        CargoAxis::Evaluation,
        EmissionPartition::TestCarrier,
        cargo,
    )
    .map(AxisCargo::Carried)
    .map_err(|refusal| diagnose::assembly_refused(&refusal))
}

/// Assemble the carrier over one implementation terminal.
///
/// # Errors
///
/// Returns the assembly diagnostic naming every way the outputs do not compose
/// into one exported shell.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the settled service road returns; this helper hands \n              it straight through"
)]
pub fn assembly<K: ProjectionKind>(
    draft: &RefusalDerivationDraft,
    implementation: &ClosedExpansion<K>,
) -> Result<SupportAssembly, MacrocDiagnostic> {
    let evaluation = evaluation_axis(draft, implementation)?;
    SupportAssembly::assembled(
        // The root is the IMPLEMENTATION terminal's own entry account, read off
        // the terminal: what the cargo was planned over is the cargo's fact, and
        // a root composed here would be a second account of it.
        implementation.plan().account().commitment(),
        EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
        AxisCargo::Absent {
            because: rows_disposition(),
        },
        evaluation,
        AxisCargo::Absent {
            because: bench_disposition(),
        },
    )
    .map_err(|refusal| diagnose::assembly_refused(&refusal))
}

/// Close the carrier: render the shell from the verified assembly, materialize
/// it as the plan's one member, prove the closure, answer the protocol, and bind
/// the terminal.
///
/// # Errors
///
/// Returns the plan-reading diagnostic where the plan does not state its one
/// member at the declaration site, the ASSEMBLY diagnostic where this plan's
/// declared root is not the assembly's — the shell road establishes that join,
/// and this seat projects the body it establishes through the assembly family's
/// own projection rather than restating it — the rendering diagnostic where the
/// shell passes the declared token magnitude, the materialization diagnostic
/// where its bytes pass theirs, the closure diagnostic where the rendering and
/// the plan disagree, the explanation diagnostic where a seat cannot be bound or
/// the coverage is short, and the binding diagnostic where the three values do
/// not name one another.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the settled service road returns; this helper hands \n              it straight through"
)]
pub fn carrier_expansion(
    plan: ProjectionPlan<TestDescriptorProjection>,
    assembly: &SupportAssembly,
) -> Result<ClosedExpansion<TestDescriptorProjection>, MacrocDiagnostic> {
    let stated = descriptor_plan(&plan).map_err(diagnose::descriptor_plan_refused)?;
    // Two homes refuse at this seam and each is projected by its own home's
    // projection: a pair that is not one declaration's is a COMPOSITION fact and
    // reads in the assembly family, and a tree past its bound is the CARRIER's
    // fact and reads in the shell family. A single projection over both would
    // compose one line about two unrelated observations.
    let shell = assembled_shell(&stated, assembly).map_err(|refusal| match refusal {
        ShellComposition::NotOneDeclarations(composed) => diagnose::assembly_refused(&composed),
        ShellComposition::Rendering(rendering) => diagnose::shell_refused(&rendering),
    })?;
    let unit = RenderedUnit::materialized(
        stated.role,
        stated.semantic_key,
        GeneratedSupportShell::DESTINATION,
        stated.profile,
        stated.profile_version,
        stated.origin.clone(),
        shell.tree().clone(),
    )
    .map_err(|refusal| diagnose::rendering_refused(refusal, stated.role))?;
    let digest = unit.digest();
    let closure = ProjectionClosure::proved(
        plan.identity(),
        plan.membership(),
        RenderedProjection::of_one(unit),
    )
    .map_err(|refusal| diagnose::closure_refused(&refusal))?;
    let explanation = carrier_explanation(&plan, &closure, digest)?;
    ClosedExpansion::bound(plan, closure, explanation)
        .map_err(|refusal| diagnose::expansion_refused(&refusal))
}

/// Answer the explanation protocol over the carrier's plan and proof.
///
/// Nine seats: the eight every kind owes, plus the one this kind declares —
/// which tests challenge the subject.
///
/// The challenging-tests seat answers with an EMPTY roster and the answer is
/// true: this carrier declares no descriptor rows, because the rows are the
/// caller's declaration and this door holds none. The why-not-generated seat
/// carries that same absence as the disposition that states it, so the two seats
/// agree by construction rather than by care.
///
/// # Errors
///
/// Returns the explanation diagnostic where the plan's one member cannot be
/// bound, or where the written view does not cover this kind's questions.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the settled service road returns; this helper hands \n              it straight through"
)]
fn carrier_explanation(
    plan: &ProjectionPlan<TestDescriptorProjection>,
    closure: &ProjectionClosure<SoleRenderedUnit>,
    digest: ProjectionIdentity<OutputBytesSubject>,
) -> Result<ProjectionExplanationView<TestDescriptorProjection>, MacrocDiagnostic> {
    let member = plan
        .membership()
        .under(SoleRenderedUnit::Sole)
        .ok_or_else(|| {
            diagnose::explanation_refused(
                &explain::ExplanationBindingRefusal::RequiredOutputAbsent {
                    seat: explain::ExplanationSeat::PlannedCarrierMember,
                },
            )
        })?;
    let owner = RefusalDeriveFact::AnEvaluationCopyStandsOverALocalSubject.citation();
    let answers = vec![
        ProjectionExplanation::answered(ExplanationAnswer::Kind {
            kind: carrier_kind(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Owner { owner }),
        ProjectionExplanation::answered(ExplanationAnswer::CausingDeclarations {
            sources: plan.account().commitment(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::GraphAndProfile {
            graph: plan.context().graph,
            profile: plan.context().profile,
            version: plan.context().profile_version,
        }),
        ProjectionExplanation::answered(ExplanationAnswer::OutputAndDigest {
            output: Box::new(member.output.clone()),
            digest,
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Invalidators {
            triggers: plan.invalidation().clone(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::RelatedProjectionDisposition {
            related: carrier_kind(),
            disposition: rows_disposition(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Repairs {
            repairs: Bounded::empty(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::ChallengingTests {
            descriptors: Bounded::empty(),
        }),
    ];
    ProjectionExplanationView::<TestDescriptorProjection>::complete(plan, closure, answers).map_err(
        |coverage| {
            diagnose::explanation_refused(&explain::ExplanationBindingRefusal::Coverage(coverage))
        },
    )
}

//! Carrying one derivation across the wall: the test-descriptor projection the
//! exported shell IS, planned, rendered, closed, and bound over the same
//! captured surface the implementation projection was.
//!
//! # Why the door owns this at all
//!
//! The implementation projection plans one generated mutation module into the TEST CARRIER, and a carrier is a vehicle: a plan that declares cargo into one has said where the tokens are compiled and nothing about how they get there.
//! The vehicle is a second projection — the generated support shell — with its own plan, rendering, proof, and terminal.
//! Until this road existed, the module was proved and partitioned but had no way to reach a consumption target.
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
//! ([`ObligationAnchoring::CapturedDeclarationOnly`]), and the trials axis
//! carries exactly what the DECLARATION states and nothing else: the claim, the
//! suite, the roles, the tags, the subject, the check, and the population a row
//! names are the caller's own words, read by the home that owns the vocabulary
//! they are spelled in and handed over unchanged. A declaration that states none
//! leaves the axis ABSENT under the disposition that says why, which is exactly the mutation-only delivery the carrier renders an empty trials seat for.
//!
//! # What moves when a trial row moves
//!
//! The carrier's own member identities stand under the CARRIER ANCHOR, which is
//! the declaration's trial commitment where it states rows and its semantic
//! commitment where it does not. So a declaration whose rows were edited plans a
//! different carrier and mints a different exported name, while every
//! implementation member beside it keeps the name it had — which is the whole of
//! what the third reading of a captured surface is for.
//!
//! The carrier plan's own ACCOUNT stays over the semantic commitment on both
//! postures, because the assembly's one-root check compares it against the root
//! the implementation terminal stands under: the two terminals are one
//! declaration's, or the carrier is delivering somebody else's cargo.

use super::plan::{
    authored_node, expansion_context, rust_declaration_profile, rust_declaration_profile_version,
};
use super::types::{
    CarrierRoadRefusal, ExplanationBindingRefusal, ExplanationSeat, MemberRenderCause,
    MemberRenderRefusal, MutationDeclarationPosture, RefusalDerivationDraft, RefusalDeriveFact,
    TrialDeclarationPosture,
};
use crate::closure::{ClosedExpansion, ProjectionClosure, RenderedProjection, RenderedUnit};
use crate::explanation_protocol::{
    ExplanationAnswer, ProjectionExplanation, ProjectionExplanationView,
};
use crate::generated_support::{
    AxisCargo, CargoAxis, DeclaredTrialCargo, EvaluationCargo, ProvedCargo, SupportAssembly,
    assembled_shell,
};
use crate::origin_graph::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
use crate::plane::{
    CapturedDeclarationSubject, GeneratedUnitSubject, MembershipLimit, OriginNodeSubject,
    OutputBytesSubject, ProjectionIdentity, ProjectionKindSubject, ProjectionRole,
    ProjectionTranscript, RenderedRole, SoleRenderedUnit, encode_bytes,
};
use crate::planning::{
    DigestContract, EXPECTED_GENERATED_SUPPORT_SCHEMA_ID, EmissionPartition, ObligationAnchoring,
    OwnerContentAccount, PlanDecisions, PlannedMember, PlannedMembership, PlannedOutput,
    ProjectionDisposition, ProjectionKind, ProjectionPlan, RowMaterialPosture,
    TestDescriptorContent, TestDescriptorProjection,
};
use crate::refusal::ProjectionPlanning;
use crate::test_descriptor::{DeferredCargo, GeneratedSupportShell, descriptor_plan};
use macroonz::Bounded;

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

/// The carrier member's semantic key.
///
/// Derived from the CARRIER ANCHOR and this member's own material, so the carrier
/// and every implementation member of one declaration are distinct identities by
/// construction — and so a declaration whose trial rows moved plans a different
/// carrier while every implementation member keeps the name it had.
/// The carrier derivation below selects the declaration commitment that owns each posture rather than substituting the account's commitment.
#[must_use]
pub fn carrier_semantic_key(
    draft: &RefusalDerivationDraft,
) -> ProjectionIdentity<GeneratedUnitSubject> {
    let (anchor, material) = carrier_derivation(draft);
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::GeneratedUnit,
        &anchor,
        &material,
        SoleRenderedUnit::Sole.slot(),
    ))
}

/// The origin node the carrier member sits at.
#[must_use]
pub fn carrier_node(draft: &RefusalDerivationDraft) -> ProjectionIdentity<OriginNodeSubject> {
    let (anchor, material) = carrier_derivation(draft);
    ProjectionIdentity::derived(ProjectionTranscript::under_projection(
        ProjectionRole::OriginNode,
        &anchor,
        &material,
        SoleRenderedUnit::Sole.slot(),
    ))
}

/// The carrier member's payload-sensitive material.
fn carrier_derivation(
    draft: &RefusalDerivationDraft,
) -> (ProjectionIdentity<CapturedDeclarationSubject>, Vec<u8>) {
    match (draft.surface().trials(), draft.surface().mutations()) {
        (TrialDeclarationPosture::NotDeclared, MutationDeclarationPosture::NotDeclared) => {
            (draft.surface().identity(), CARRIER_MEMBER.to_vec())
        }
        (TrialDeclarationPosture::Declared(trials), MutationDeclarationPosture::NotDeclared) => {
            (trials.commitment(), CARRIER_MEMBER.to_vec())
        }
        (TrialDeclarationPosture::NotDeclared, MutationDeclarationPosture::Declared(mutations)) => {
            (mutations.commitment(), CARRIER_MEMBER.to_vec())
        }
        (
            TrialDeclarationPosture::Declared(trials),
            MutationDeclarationPosture::Declared(mutations),
        ) => {
            let mut material = Vec::new();
            encode_bytes(CARRIER_MEMBER, &mut material);
            encode_bytes(trials.commitment().as_bytes(), &mut material);
            encode_bytes(mutations.commitment().as_bytes(), &mut material);
            (draft.surface().identity(), material)
        }
    }
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

/// What happened to the descriptor rows this carrier declares.
///
/// # Two answers, and the declaration decides which
///
/// GENERATED where the declaration states trial rows: the author wrote them, the
/// grammar read them, and the carrier's own member carries them across the wall.
/// The output the disposition names is the carrier's, read off the plan, because
/// the rows are rendered INSIDE that one unit rather than as units of their own.
///
/// NOT APPLICABLE where it states none, citing the fact that says why: the claim,
/// the suite, the roles, the tags, the subject, the check, and the population a
/// row states are the caller's declarations, and a door handed none declares no
/// rows rather than inventing the material it would then prove.
///
/// The two never read alike, and neither is a refusal: a refusal-family
/// declaration that states no trials is exactly the declaration this derive has
/// always compiled.
pub fn rows_disposition(
    draft: &RefusalDerivationDraft,
    plan: &ProjectionPlan<TestDescriptorProjection>,
) -> ProjectionDisposition {
    match draft.surface().trials() {
        TrialDeclarationPosture::NotDeclared => ProjectionDisposition::NotApplicable {
            because: RefusalDeriveFact::ARowIsTheCallersDeclarationAndNeverTheProducers.citation(),
        },
        TrialDeclarationPosture::Declared(_) => ProjectionDisposition::Generated {
            output: Box::new(plan.membership().first().output.clone()),
        },
    }
}

/// What the caller declared into this carrier's TRIALS axis.
///
/// # What is read, and what is declared
///
/// Everything. The payload is the declaration's own, read by the home that owns
/// the vocabulary it states and carried here unchanged — this road elects no
/// row, no name, and no suite. Where the declaration states none the axis is
/// ABSENT under the disposition that says why, which is exactly the mutation-only delivery the carrier renders an empty trials seat for.
///
/// # Bounds
///
/// The trial axis carries a DECLARED payload rather than one terminal's proved
/// cargo, and the two are different materials for a reason: a row's meaning is
/// the author's statement and was never rendered by anybody, so there is no
/// partition for a promotion road to read it off. What keeps it honest is the
/// grammar that read it and the carrier vocabulary that admitted it, both of
/// which refuse before this seat is reached.
pub fn trials_axis(draft: &RefusalDerivationDraft) -> AxisCargo<DeclaredTrialCargo> {
    match draft.surface().trials() {
        TrialDeclarationPosture::NotDeclared => AxisCargo::Absent {
            because: ProjectionDisposition::NotApplicable {
                because: RefusalDeriveFact::ARowIsTheCallersDeclarationAndNeverTheProducers
                    .citation(),
            },
        },
        TrialDeclarationPosture::Declared(declared) => AxisCargo::Carried(
            DeclaredTrialCargo::carried(declared.commitment(), declared.payload().clone()),
        ),
    }
}

/// The carrier account over every independently committed helper it delivers.
fn carrier_account(
    draft: &RefusalDerivationDraft,
) -> Result<OwnerContentAccount<TestDescriptorProjection>, ProjectionPlanning> {
    let mut dependencies = Vec::new();
    if let TrialDeclarationPosture::Declared(trials) = draft.surface().trials() {
        dependencies.push(trials.commitment());
    }
    if let MutationDeclarationPosture::Declared(mutations) = draft.surface().mutations() {
        dependencies.push(mutations.commitment());
    }
    if dependencies.is_empty() {
        Ok(OwnerContentAccount::captured(draft.surface().identity()))
    } else {
        OwnerContentAccount::captured_over(draft.surface().identity(), dependencies)
    }
}

/// What happened to the bench material this carrier would have delivered.
///
/// NOT APPLICABLE, citing the seat rule: the carrier's published grammar writes
/// a trials seat and a deferred seat, and neither is the bench seat. The bench
/// crossing renders real material and rides its own shell today; this axis opens
/// when the reserved seat is written.
pub fn bench_disposition() -> ProjectionDisposition {
    ProjectionDisposition::NotApplicable {
        because: RefusalDeriveFact::ACarrierSeatIsWrittenBeforeItIsFilled.citation(),
    }
}

/// Plan the carrier projection over one captured surface.
///
/// # Errors
///
/// Returns the PLANNING home's own body where the watch set cannot represent
/// what the entry account declares, or where a declared magnitude is passed while
/// the plan's seats are assembled — narrow, because planning is the only home
/// this road reaches.
pub fn carrier_plan(
    draft: &RefusalDerivationDraft,
) -> Result<ProjectionPlan<TestDescriptorProjection>, ProjectionPlanning> {
    let account = carrier_account(draft)?;
    let context = expansion_context();
    let invalidation = context.watch_set(&account)?;
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
            RefusalDeriveFact::OneCarrierDeliversOneDeclarationsProvedCargo.citation(),
        ),
    });
    Ok(ProjectionPlan::<TestDescriptorProjection>::planned(
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
    ))
}

/// Read one implementation terminal's test-carrier cargo into the evaluation
/// axis, or state that nothing was planned into it.
///
/// # What is read, and what is declared
///
/// The TOKENS are the terminal's, read off its own proved partition by the assembly home's crate-internal promotion road, which refuses anything that is not that partition's own.
/// The optional public support address is read from the mutation declaration that owns it and is structurally bound to those proved tokens.
///
/// # This road is the one lawful promotion point
///
/// The promotion road is crate-internal, and THIS is the road it exists for.
/// The road that reads the terminal also reads the optional support address from its owning declaration, so no caller can loosely pair an address beside another terminal's proved cargo.
///
/// # Errors
///
/// Returns the CARRIER-DECLARATION home's body where a spelling this home
/// declared is not one the cargo admits, and the ASSEMBLY home's body where the
/// tokens are not the terminal's own — each carried whole, under the arm of the
/// carrier road that names which home refused.
pub fn evaluation_axis<K: ProjectionKind>(
    draft: &RefusalDerivationDraft,
    implementation: &ClosedExpansion<K>,
) -> Result<AxisCargo<EvaluationCargo>, CarrierRoadRefusal> {
    let Some(tree) = implementation.test_carrier().tokens() else {
        // Nothing was planned into the carrier at all — a stated fact about the
        // plan rather than an empty cargo, and the disposition says which.
        return Ok(AxisCargo::Absent {
            because: ProjectionDisposition::NotRequested,
        });
    };
    let cargo = DeferredCargo::deferred(tree.clone());
    let proved = ProvedCargo::carried(
        implementation,
        CargoAxis::Evaluation,
        EmissionPartition::TestCarrier,
        cargo,
    )?;
    let mutation_support = match draft.surface().trials() {
        TrialDeclarationPosture::Declared(_) => None,
        TrialDeclarationPosture::NotDeclared => match draft.surface().mutations() {
            MutationDeclarationPosture::Declared(mutations) => {
                mutations.declaration().support().cloned()
            }
            MutationDeclarationPosture::NotDeclared => None,
        },
    };
    Ok(AxisCargo::Carried(EvaluationCargo::carried(
        proved,
        mutation_support,
    )))
}

/// Assemble the carrier over one implementation terminal.
///
/// # Errors
///
/// Returns the ASSEMBLY home's own body naming every way the outputs do not
/// compose into one exported shell, and whatever the evaluation axis refused
/// with.
pub fn assembly<K: ProjectionKind>(
    draft: &RefusalDerivationDraft,
    implementation: &ClosedExpansion<K>,
) -> Result<SupportAssembly, CarrierRoadRefusal> {
    let evaluation = evaluation_axis(draft, implementation)?;
    let carrier_addressing = carrier_account(draft)?.addressing().clone();
    SupportAssembly::assembled(
        implementation.plan().account().addressing(),
        carrier_addressing,
        EXPECTED_GENERATED_SUPPORT_SCHEMA_ID,
        trials_axis(draft),
        evaluation,
        AxisCargo::Absent {
            because: bench_disposition(),
        },
    )
    .map_err(CarrierRoadRefusal::from)
}

/// Close the carrier: render the shell from the verified assembly, materialize
/// it as the plan's one member, prove the closure, answer the protocol, and bind
/// the terminal.
///
/// # Errors
///
/// Returns the arm of the carrier road that names which step refused, carrying that step's own body whole: the plan reading where the plan does not state its one member at the declaration site, the composition where this plan's declared root is not the assembly's or the shell's tokens pass their magnitude, the materialization where the shell's bytes pass theirs, the closure where the rendering and the plan disagree, the explanation where a seat cannot be bound or the coverage is short, and the binding where the three values do not name one another.
///
/// Nothing here composes a line.
/// Which step failed is a value the caller holds, and `diagnose::carrier_road_refused` is the one seam that turns it into the one line a compiler shows.
pub fn carrier_expansion(
    draft: &RefusalDerivationDraft,
    plan: ProjectionPlan<TestDescriptorProjection>,
    assembly: &SupportAssembly,
) -> Result<ClosedExpansion<TestDescriptorProjection>, CarrierRoadRefusal> {
    let stated = descriptor_plan(&plan)?;
    let shell = assembled_shell(&stated, assembly)?;
    let unit = RenderedUnit::materialized(
        stated.role,
        stated.semantic_key,
        GeneratedSupportShell::DESTINATION,
        stated.profile,
        stated.profile_version,
        stated.origin.clone(),
        shell.tree().clone(),
    )
    .map_err(|cause| MemberRenderRefusal {
        role: stated.role,
        cause: MemberRenderCause::Materialized(cause),
    })?;
    let digest = unit.digest();
    let closure = ProjectionClosure::proved(&plan, RenderedProjection::of_one(unit))?;
    let explanation = carrier_explanation(draft, &plan, &closure, digest)?;
    Ok(ClosedExpansion::bound(plan, closure, explanation)?)
}

/// Answer the explanation protocol over the carrier's plan and proof.
///
/// Nine seats: the eight every kind owes, plus the one this kind declares —
/// which tests challenge the subject.
///
/// # The challenging-tests seat
///
/// The roster names GENERATED UNITS, and this projection materializes exactly
/// one: the carrier. So the seat answers with that one member's semantic key
/// where the declaration states trial rows, and with an EMPTY roster where it
/// states none — a carrier that delivers no rows challenges nothing, and one that
/// delivers them challenges through the single unit they are rendered inside.
///
/// The rows themselves are not named here and cannot be: a row is declaration
/// material rendered inside the carrier's one unit rather than a unit of its own,
/// and a roster that listed them would be naming values the plane never minted.
///
/// The why-not-generated seat carries the same posture as the disposition that
/// states it ([`rows_disposition`]), read off the same declaration, so the two
/// seats agree by construction rather than by care.
///
/// # Errors
///
/// Returns the EXPLANATION home's own body where the plan's one member cannot be
/// bound, or where the written view does not cover this kind's questions —
/// narrow, because that is the only home this road reaches.
fn carrier_explanation(
    draft: &RefusalDerivationDraft,
    plan: &ProjectionPlan<TestDescriptorProjection>,
    closure: &ProjectionClosure<SoleRenderedUnit>,
    digest: ProjectionIdentity<OutputBytesSubject>,
) -> Result<ProjectionExplanationView<TestDescriptorProjection>, ExplanationBindingRefusal> {
    let member = plan.membership().under(SoleRenderedUnit::Sole).ok_or(
        ExplanationBindingRefusal::RequiredOutputAbsent {
            seat: ExplanationSeat::PlannedCarrierMember,
        },
    )?;
    let owner = RefusalDeriveFact::OneCarrierDeliversOneDeclarationsProvedCargo.citation();
    let challenging: Bounded<ProjectionIdentity<GeneratedUnitSubject>, MembershipLimit> =
        match draft.surface().trials() {
            TrialDeclarationPosture::NotDeclared => Bounded::empty(),
            TrialDeclarationPosture::Declared(_) => {
                Bounded::from_array([member.output.semantic_key])
            }
        };
    let answers = vec![
        ProjectionExplanation::answered(ExplanationAnswer::Kind {
            kind: carrier_kind(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Owner { owner }),
        ProjectionExplanation::answered(ExplanationAnswer::CausingDeclarations {
            sources: plan.account().commitment(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Profile {
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
            disposition: rows_disposition(draft, plan),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Repairs {
            repairs: Bounded::empty(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::ChallengingTests {
            descriptors: challenging,
        }),
    ];
    ProjectionExplanationView::<TestDescriptorProjection>::complete(plan, closure, answers)
        .map_err(ExplanationBindingRefusal::Coverage)
}

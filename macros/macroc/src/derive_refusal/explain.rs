//! Answering the explanation protocol over one closed derivation.
//!
//! # Ordering
//!
//! The explanation is written AFTER the closure.
//! One of the nine seats is "which output identity and digest are you", and a
//! digest is a fact about bytes.
//! Written before the closure, that seat could only carry a value the plan
//! invented; written after it, the seat carries the digest the closure proved
//! over the bytes that were actually rendered — so the explanation is a reading
//! of what happened rather than a restatement of what was intended.
//!
//! No human rendering is written here at all.
//! The rendering is projected from the answer — see
//! `explanation_protocol::project` — so no sentence composed here can disagree
//! with the typed answer beside it.
//!
//! # The parentage travels with the answers
//!
//! The view is completed over the PLAN and the PROOF themselves, not over two
//! identities named beside the seats. That is what makes the view's own name a
//! fact about this expansion: a caller reaching this road holds a plan it
//! planned and a closure it proved, and the terminal that binds the three
//! compares all of them.
//!
//! # Binding refusals
//!
//! Three seats here are bound to something the plan or the closure must already
//! hold: the planned member standing under the family role, the digest the
//! closure proved over that member's bytes, and the first owner fact the plan
//! declares.
//! An absent subject is [`ExplanationBindingRefusal::RequiredOutputAbsent`] and
//! the refusal propagates, because an explanation that answered off a
//! NEIGHBOURING value instead would be a confident, well-formed,
//! complete-looking answer about a different value.

use super::plan::DerivedPlan;
use crate::closure::{ProjectionClosure, RenderedUnit};
use crate::explanation_protocol::{
    ExplanationAnswer, ProjectionExplanation, ProjectionExplanationView,
};
use crate::plane::OwnerFactRef;
use crate::planning::{DeriveImplProjection, RenderedImplementation};
use threadpak::types::Bounded;

use super::plan::derive_impl_kind;
use super::types::{ExplanationBindingRefusal, ExplanationSeat};

/// Answer the explanation protocol over one planned and closed derivation.
///
/// Nine seats: the eight every kind owes, plus the assumptions this kind
/// declares.
/// The why-NOT-generated seat is answered by the cause-order disposition —
/// where the shape declares no canonical order, the answer names the band 00
/// fact rather than saying nothing.
///
/// # Errors
///
/// Returns [`ExplanationBindingRefusal::RequiredOutputAbsent`] naming the seat
/// whose subject is absent, and [`ExplanationBindingRefusal::Coverage`] naming
/// every unanswered, doubled, or inadmissible seat.
pub fn explained(
    planned: &DerivedPlan,
    closure: &ProjectionClosure<RenderedImplementation>,
) -> Result<ProjectionExplanationView<DeriveImplProjection>, ExplanationBindingRefusal> {
    let plan = planned.plan();
    let family = plan
        .membership()
        .under(RenderedImplementation::RenderedFamilyImpl)
        .ok_or(ExplanationBindingRefusal::RequiredOutputAbsent {
            seat: ExplanationSeat::PlannedFamilyMember,
        })?;
    let digest = closure
        .rendered()
        .under(family.role)
        .map(RenderedUnit::digest)
        .ok_or(ExplanationBindingRefusal::RequiredOutputAbsent {
            seat: ExplanationSeat::ProvedFamilyDigest,
        })?;
    let owner = *plan.content().assumptions.iter().next().ok_or(
        ExplanationBindingRefusal::RequiredOutputAbsent {
            seat: ExplanationSeat::DeclaredAssumption,
        },
    )?;

    // The plan and the proof travel into the view as themselves, so the view
    // records the parentage it was actually answered over rather than a pair of
    // identities this seat could have named. The terminal compares all three
    // afterwards, and a view built here can only agree.
    ProjectionExplanationView::<DeriveImplProjection>::complete(
        plan,
        closure,
        seats(planned, family, digest, owner),
    )
    .map_err(ExplanationBindingRefusal::Coverage)
}

/// The nine seats this kind owes, in the order the protocol states them.
fn seats(
    planned: &DerivedPlan,
    family: &crate::planning::PlannedMember<RenderedImplementation>,
    digest: crate::plane::ProjectionIdentity<crate::plane::OutputBytesSubject>,
    owner: OwnerFactRef,
) -> Vec<ProjectionExplanation> {
    let plan = planned.plan();
    vec![
        ProjectionExplanation::answered(ExplanationAnswer::Kind {
            kind: derive_impl_kind(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Owner { owner }),
        // Read off the plan's own entry account, which is the one holder of
        // "which declaration caused you". A second seat carrying the same answer
        // could be answered from the copy after the account moved.
        ProjectionExplanation::answered(ExplanationAnswer::CausingDeclarations {
            sources: plan.account().commitment(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::GraphAndProfile {
            graph: plan.context().graph,
            profile: plan.context().profile,
            version: plan.context().profile_version,
        }),
        ProjectionExplanation::answered(ExplanationAnswer::OutputAndDigest {
            output: Box::new(family.output.clone()),
            digest,
        }),
        ProjectionExplanation::answered(ExplanationAnswer::AssumptionsAndSpecializations {
            assumptions: plan.content().assumptions.clone(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Invalidators {
            triggers: plan.invalidation().clone(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::RelatedProjectionDisposition {
            related: derive_impl_kind(),
            disposition: planned.cause_order().clone(),
        }),
        ProjectionExplanation::answered(ExplanationAnswer::Repairs {
            repairs: Bounded::empty(),
        }),
    ]
}

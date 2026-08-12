//! Answering the explanation protocol over one closed derivation.
//!
//! # The explanation is written AFTER the closure, on purpose
//!
//! One of the nine seats is "which output identity and digest are you", and a
//! digest is a fact about bytes. Written before the closure, that seat could
//! only carry a value the plan invented. Written after it, the seat carries the
//! digest the closure proved over the bytes that were actually rendered — so the
//! explanation is a reading of what happened rather than a restatement of what
//! was intended.
//!
//! No human rendering is written here at all. A seat used to stand beside every
//! answer for a sentence this module composed, which meant the sentence and the
//! typed answer were two values that could disagree. The rendering is projected
//! from the answer now — see `explanation_protocol::project` — so there is
//! nothing to write and nothing to keep in agreement.
//!
//! # An explanation that cannot bind its subject refuses
//!
//! Three seats here are bound to something the plan or the closure must already
//! hold: the planned member standing under the family role, the digest the
//! closure proved over that member's bytes, and the first owner fact the plan
//! declares. Each of the three used to fall back to a NEIGHBOUR — the first
//! planned member whatever its role, the first rendered unit's digest whatever
//! it was a digest of, a hardcoded owner fact nobody's plan cited. An
//! explanation built that way is worse than no explanation: it is a confident,
//! well-formed, complete-looking answer about a different value. All three are
//! [`ExplanationBindingRefusal::RequiredOutputAbsent`] now, and the refusal
//! propagates.

use super::plan::DerivedPlan;
use crate::closure::{ProjectionClosure, RenderedUnit};
use crate::explanation_protocol::{
    ExplanationAnswer, ExplanationCoverage, ProjectionExplanation, ProjectionExplanationView,
};
use crate::plane::OwnerFactRef;
use crate::planning::{DeriveImplProjection, RenderedImplementation};
use threadpak::types::Bounded;

use super::plan::derive_impl_kind;

threadpak::closed_register! {
    /// The seat one explanation could not bind its subject to.
    ///
    /// Named seats rather than one "something was missing": a caller repairing a
    /// derivation needs to know whether the PLAN failed to declare the member, the
    /// CLOSURE failed to prove its bytes, or the plan cited no owner fact at all,
    /// and those are three different repairs.
    pub enum ExplanationSeat {
        /// The planned member standing under the family implementation's role.
        PlannedFamilyMember = "planned-family-member",
            "the planned member under the family role";
        /// The digest the closure proved over that member's rendered bytes.
        ProvedFamilyDigest = "proved-family-digest",
            "the digest the closure proved over the family bytes";
        /// The first owner fact the plan declares as an assumption.
        DeclaredAssumption = "declared-assumption", "the first owner fact the plan declares";
    }
}

/// How writing one explanation refuses.
///
/// Two postures, and they are different observations. A view that could not be
/// BOUND never reached the coverage check — there was no subject to write nine
/// seats about. A view that was written and does not cover its kind's questions
/// reached it and failed it.
#[must_use = "a refusal carries the unbound seat or the coverage the view failed"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExplanationBindingRefusal {
    /// A required seat's subject is absent. The explanation refuses rather than
    /// answering about a neighbouring value.
    RequiredOutputAbsent {
        /// Which seat had no subject.
        seat: ExplanationSeat,
    },
    /// The written view does not cover the kind's applicable questions.
    Coverage(ExplanationCoverage),
}

/// Answer the explanation protocol over one planned and closed derivation.
///
/// Nine seats: the eight every kind owes, plus the assumptions this kind
/// declares. The why-NOT-generated seat is answered by the cause-order
/// disposition — where the shape declares no canonical order, the answer names
/// the band 00 fact rather than saying nothing.
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

    ProjectionExplanationView::<DeriveImplProjection>::complete(seats(
        planned, family, digest, owner,
    ))
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
        ProjectionExplanation::answered(ExplanationAnswer::CausingDeclarations {
            sources: plan.context().sources.clone(),
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

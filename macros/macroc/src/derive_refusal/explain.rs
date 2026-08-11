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
//! Every human rendering below is a static literal proven to fit its limit
//! family at compile time. There is no road here that swallows an over-long
//! projection and hands back an empty one.

use super::plan::DerivedPlan;
use crate::closure::ProjectionClosure;
use crate::explanation_protocol::{
    ExplanationAnswer, ExplanationCoverage, ProjectionExplanation, ProjectionExplanationView,
};
use crate::plane::{HumanTextLimit, human_projection};
use crate::planning::{DeriveImplProjection, RenderedImplementation};
use threadpak::types::Bounded;

use super::plan::derive_impl_kind;

/// Answer the explanation protocol over one planned and closed derivation.
///
/// Nine seats: the eight every kind owes, plus the assumptions this kind
/// declares. The why-NOT-generated seat is answered by the cause-order
/// disposition — where the shape declares no canonical order, the answer names
/// the band 00 fact rather than saying nothing.
///
/// # Errors
///
/// Returns [`ExplanationCoverage`] naming every unanswered, doubled, or
/// inadmissible seat.
pub fn explained(
    planned: &DerivedPlan,
    closure: &ProjectionClosure<RenderedImplementation>,
) -> Result<ProjectionExplanationView<DeriveImplProjection>, ExplanationCoverage> {
    let plan = planned.plan();
    let family = plan
        .membership()
        .under(RenderedImplementation::RenderedFamilyImpl)
        .unwrap_or_else(|| plan.membership().first());
    let digest = closure.rendered().under(family.role).map_or_else(
        || {
            closure
                .rendered()
                .units()
                .next()
                .map(crate::closure::RenderedUnit::digest)
        },
        |unit| Some(unit.digest()),
    );
    let Some(digest) = digest else {
        // Unreachable: a rendering is structurally non-empty, so there is always
        // a unit to read a digest off. Stated as a coverage refusal rather than
        // as an assumption, because the seat must never be answered with a
        // value nobody computed.
        return ProjectionExplanationView::<DeriveImplProjection>::complete(Vec::new());
    };

    ProjectionExplanationView::<DeriveImplProjection>::complete(seats(planned, family, digest))
}

/// The nine seats this kind owes, in the order the protocol states them.
fn seats(
    planned: &DerivedPlan,
    family: &crate::planning::PlannedMember<RenderedImplementation>,
    digest: crate::plane::ProjectionIdentity<crate::plane::OutputBytesSubject>,
) -> Vec<ProjectionExplanation> {
    let plan = planned.plan();
    vec![
        ProjectionExplanation::answered(
            ExplanationAnswer::Kind {
                kind: derive_impl_kind(),
            },
            human_projection!(
                HumanTextLimit,
                "an implementation projection over a declared refusal family"
            ),
        ),
        ProjectionExplanation::answered(
            ExplanationAnswer::Owner {
                owner: *plan.content().assumptions.iter().next().unwrap_or(
                    &crate::plane::OwnerFactRef::named(
                        "refusal",
                        "family-shapes-are-three-and-closed",
                    ),
                ),
            },
            human_projection!(
                HumanTextLimit,
                "the refusal home requires a declared body shape"
            ),
        ),
        ProjectionExplanation::answered(
            ExplanationAnswer::CausingDeclarations {
                sources: plan.context().sources.clone(),
            },
            human_projection!(HumanTextLimit, "the enum declaration the caller wrote"),
        ),
        ProjectionExplanation::answered(
            ExplanationAnswer::GraphAndProfile {
                graph: plan.context().graph,
                profile: plan.context().profile,
                version: plan.context().profile_version,
            },
            human_projection!(
                HumanTextLimit,
                "what the plan was decided against, and the selected projection profile"
            ),
        ),
        ProjectionExplanation::answered(
            ExplanationAnswer::OutputAndDigest {
                output: Box::new(family.output.clone()),
                digest,
            },
            human_projection!(
                HumanTextLimit,
                "the family implementation, and the digest the closure proved over its bytes"
            ),
        ),
        ProjectionExplanation::answered(
            ExplanationAnswer::AssumptionsAndSpecializations {
                assumptions: plan.content().assumptions.clone(),
            },
            human_projection!(
                HumanTextLimit,
                "the refusal home's shape, order, and cause-key facts"
            ),
        ),
        ProjectionExplanation::answered(
            ExplanationAnswer::Invalidators {
                triggers: plan.invalidation().clone(),
            },
            human_projection!(
                HumanTextLimit,
                "the captured declaration, the projection profile, and the generator version"
            ),
        ),
        ProjectionExplanation::answered(
            ExplanationAnswer::RelatedProjectionDisposition {
                related: derive_impl_kind(),
                disposition: planned.cause_order().clone(),
            },
            human_projection!(
                HumanTextLimit,
                "what happened to the typed cause-order projection"
            ),
        ),
        ProjectionExplanation::answered(
            ExplanationAnswer::Repairs {
                repairs: Bounded::empty(),
            },
            human_projection!(HumanTextLimit, "nothing was refused, so no repair applies"),
        ),
    ]
}

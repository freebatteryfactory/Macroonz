//! The planning half of the road: what the plan already decided about the
//! documentation, read off the plan's own public surface — and the explanation
//! station's answers composed from that same reading.
//!
//! Nothing here decides meaning and nothing here mints an identity. The planned
//! member's semantic key, its expected profile at its version, and its origin
//! trail are the PLAN's answers, read exactly; the address the material stands on
//! is the entry account's one commitment; the rendering engine is the generator
//! the plan's context names; and the subject, the audience, and the covered
//! facets are the kind content's, read and not interpreted.
//!
//! # The prose is not here
//!
//! The plan's kind content names a SUBJECT, an AUDIENCE, and the FACETS covered.
//! It carries no sentence, no heading, and no line — so
//! [`DocumentedItem`](super::DocumentedItem) arrives from the CALLER and this
//! file reads only what the plan actually decided. A generator that composed the
//! owner's summary would be writing a claim about the owner's item that the owner
//! did not make.
//!
//! # The facets ARE read, and they decide what is earned
//!
//! The facet roster is the one kind-content seat the composition turns on: it is
//! the quantifier the earned sections are checked against, in both directions. It
//! is read by IDENTITY and never by spelling, which is why the coverage law stands
//! whole while the one fact that would name a facet refuses.
//!
//! # The station's four unheld facts
//!
//! This kind lists no questions of its own, so its plans answer exactly the
//! universal roster. Four of those eight are the plan's own and are read here;
//! the other four — the kind's identity, the requiring owner, the digest proved
//! over bytes that exist, and the related kind's disposition beside its repairs —
//! are not a plan's to hold, and they arrive as
//! [`DocumentationExplanationAnchors`]. A road that minted any of them would be
//! answering the station with a value nobody computed.

use super::{DocumentationExplanationAnchors, DocumentationIssue, DocumentationPlan};
use crate::explanation_protocol::{
    ExplanationAnswer, ExplanationCoverage, ProjectionExplanation, ProjectionExplanationView,
};
use crate::plane::{RenderedRole, SoleRenderedUnit};
use crate::planning::{DocumentationProjection, MemberDestination, ProjectionPlan};

/// Read one documentation plan into the statement of what its material will be.
///
/// # Errors
///
/// Returns [`DocumentationIssue::RoleNotPlanned`] where the plan declares no
/// member under its kind's one rendered role — the membership is the quantifier,
/// so an unplanned role is an absence the plan itself states rather than a failure
/// to look hard enough.
///
/// Returns [`DocumentationIssue::DestinationNotDeclarationSite`] where the planned
/// member lands anywhere but the declaration site: doc material is an attribute
/// run spliced ahead of the owner's own item, so a standalone artifact, deferred
/// test cargo, and deferred bench cargo are three other deliveries and each
/// reaches this answer.
///
/// The two checks are DEPENDENT — there is no destination to read until a member
/// was found — so exactly one of them is ever established.
pub fn documentation_plan(
    plan: &ProjectionPlan<DocumentationProjection>,
) -> Result<DocumentationPlan, DocumentationIssue> {
    let role = SoleRenderedUnit::Sole;
    let Some(member) = plan.membership().under(role) else {
        return Err(DocumentationIssue::RoleNotPlanned {
            role_slot: role.slot(),
        });
    };
    match member.output.destination {
        MemberDestination::AtDeclarationSite => {}
        // Every delivery this kind does not make reaches one answer, and the
        // arms are written out one by one rather than under a wildcard: a
        // delivery admitted later stops the compiler here until somebody says
        // whether doc material is ever written into it.
        MemberDestination::AsArtifact { .. }
        | MemberDestination::IntoTestCarrier
        | MemberDestination::IntoBenchCarrier => {
            return Err(DocumentationIssue::DestinationNotDeclarationSite {
                role_slot: role.slot(),
            });
        }
    }
    let content = plan.content();
    Ok(DocumentationPlan {
        role,
        semantic_key: member.output.semantic_key,
        profile: member.output.expected_profile,
        profile_version: member.output.expected_profile_version,
        origin: member.output.origin.clone(),
        declaration: plan.account().commitment(),
        graph: plan.context().graph,
        engine: plan.context().generator,
        subject: content.subject,
        audience: content.audience,
        facets: content.facets.clone(),
    })
}

/// The explanation station's answers for one documentation plan.
///
/// Eight answers, one per universal question, in the roster's own order. Four are
/// read off the plan and the statement taken from it; four arrive as anchors,
/// because a plan does not hold them and nothing here mints them.
///
/// # Bounds
///
/// It answers the UNIVERSAL roster and nothing beyond it, because this kind
/// declares no questions of its own. A kind that later declares one adds an
/// answer here and the station's own coverage check says so until it does — the
/// view is checked against the kind's roster, not against this function's length.
#[must_use]
pub fn explanation_answers(
    plan: &ProjectionPlan<DocumentationProjection>,
    stated: &DocumentationPlan,
    anchors: &DocumentationExplanationAnchors,
) -> Vec<ExplanationAnswer> {
    vec![
        ExplanationAnswer::Kind {
            kind: anchors.kind,
        },
        ExplanationAnswer::Owner {
            owner: anchors.owner,
        },
        ExplanationAnswer::CausingDeclarations {
            sources: stated.declaration,
        },
        ExplanationAnswer::GraphAndProfile {
            graph: stated.graph,
            profile: stated.profile,
            version: stated.profile_version,
        },
        ExplanationAnswer::OutputAndDigest {
            output: Box::new(plan.membership().first().output.clone()),
            digest: anchors.digest,
        },
        ExplanationAnswer::Invalidators {
            triggers: plan.invalidation().clone(),
        },
        ExplanationAnswer::RelatedProjectionDisposition {
            related: anchors.related,
            disposition: anchors.disposition.clone(),
        },
        ExplanationAnswer::Repairs {
            repairs: anchors.repairs.clone(),
        },
    ]
}

/// The complete explanation view one documentation plan answers the station with.
///
/// The question each answer belongs to is taken from the ANSWER, so there is no
/// seam here that files a true answer under a question somebody supplied.
///
/// # Errors
///
/// Returns the station's own coverage refusal where the answers do not cover this
/// kind's applicable roster exactly once each — which is the station's question
/// and not this home's, and is why the refusal is the station's family rather than
/// this home's.
pub fn explanation_view(
    plan: &ProjectionPlan<DocumentationProjection>,
    stated: &DocumentationPlan,
    anchors: &DocumentationExplanationAnchors,
) -> Result<ProjectionExplanationView<DocumentationProjection>, ExplanationCoverage> {
    let answered: Vec<ProjectionExplanation> = explanation_answers(plan, stated, anchors)
        .into_iter()
        .map(ProjectionExplanation::answered)
        .collect();
    ProjectionExplanationView::<DocumentationProjection>::complete(answered)
}

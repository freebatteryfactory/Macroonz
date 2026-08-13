//! Projecting each refusal family into the services' structured diagnostic.
//!
//! # One projection per family, and each one keeps its family's distinctions
//!
//! Five steps of the compile road refuse, and each refuses in its own
//! vocabulary: a planning body names an axis, a magnitude, and a count; a
//! closure body names a role and how the two disagreed at it; a coverage body
//! names every question that was unanswered, doubled, or inadmissible; a
//! rendering refusal names the exact bound and the unit that overran it.
//!
//! All five used to collapse into one sentence through a single helper, under
//! one classification, with [`CauseDisposition::UnresolvedCause`] and an empty
//! related set. That helper was a smoke alarm: it told a caller that something
//! in the building was on fire. Every function here projects the typed body it
//! is handed and keeps what the body knew:
//!
//! - the **summary** is composed from the typed values — the axis and its
//!   magnitude, the role and its disagreement, the question and its coverage,
//!   the bound and its unit — and is a summary, because one line is all rustc
//!   shows;
//! - the **observed classification** is the first established issue's own, not a
//!   single word shared by five families;
//! - the **related set** carries one identity per established issue, derived
//!   over that issue's complete canonical encoding, behind one identity over the
//!   complete body. Two bodies that differ in any typed member derive different
//!   identities, so no distinction is lost on the way through. This module hands
//!   over the issue MATERIAL and the set derives both levels itself, so no seam
//!   here ever holds a body identity it could seat over somebody else's issues.
//!   Where a body arrives at the set's own declared magnitude the body identity
//!   is carried alone, and the diagnostic states that posture and the count it
//!   dropped rather than handing back a coarser set shaped like a complete one;
//! - the **posture** rides in the summary, so a body that stopped at its own
//!   declared bound says so rather than reading as complete.
//!
//! # What this module still does not do
//!
//! It elects no machine cause posture. Narrowing is the machine's progress to
//! report and the plane observes rather than concludes, so every diagnostic here
//! carries [`CauseDisposition::UnresolvedCause`] exactly as the capture road
//! does.
//!
//! And it invents no repair. Every [`RepairAction`] below cites a fact the
//! services' own charter declares — that a plan states its complete output set,
//! that nothing is emitted that did not close, that every kind answers the
//! explanation protocol, that every rendered seat stands under a declared
//! magnitude — by the declared names those facts are written down under.

use super::explain::ExplanationBindingRefusal;
use super::render::RenderRefusal;
use super::types::{callable_entry, expected_contract};
use crate::closure::{ClosureIssue, ProjectionClosureRefusal, RenderingRefusal};
use crate::diagnostics::{
    DiagnosticSite, MacrocDiagnostic, MacrocPhase, ObservedClassification, RelatedSet,
    RelatedSetCompletion, ReleasePosture, RepairAction, ReproductionRoute, SiteCoordinate,
};
use crate::explanation_protocol::{ExplanationCoverage, ExplanationCoverageIssue};
use crate::plane::{
    GeneratedTokenLimit, HumanProjection, HumanTextLimit, MembershipLimit, OwnerFactRef,
    RenderedByteLimit, RenderedRole, encode_bytes, human_projection,
};
use crate::refusal::{ProjectionPlanning, ProjectionPlanningIssue};
use threadpak::evidence::CauseDisposition;
use threadpak::refusal::CompletionPosture;
use threadpak::types::{Bounded, BoundedConstruction, ConstLimit};

/// The family tag written ahead of every related issue's material, so two
/// families' issues never encode alike.
const PLANNING_FAMILY: u8 = 0;

/// The closure family's tag.
const CLOSURE_FAMILY: u8 = 1;

/// The explanation-coverage family's tag.
const COVERAGE_FAMILY: u8 = 2;

/// The rendering family's tag.
const RENDERING_FAMILY: u8 = 3;

// ---------------------------------------------------------------------------
// Planning.
// ---------------------------------------------------------------------------

/// Project one planning refusal.
///
/// Every issue's axis, declared magnitude, observed count, seat, and doubled
/// role survive: the first one in the summary, all of them in the related set.
pub fn planning_refused(refusal: &ProjectionPlanning) -> MacrocDiagnostic {
    let first = refusal.body().carried().first();
    let material: Vec<Vec<u8>> = refusal
        .body()
        .carried()
        .iter()
        .map(planning_bytes)
        .collect();
    diagnosed(
        MacrocPhase::Planning,
        planning_observed(first),
        &summary(
            "planning refused",
            &planning_line(first),
            refusal.body().carried().len().saturating_sub(1),
            refusal.body().completion(),
        ),
        PLANNING_FAMILY,
        &material,
        OwnerFactRef::named("macroc", "a-plan-states-its-complete-output-set-or-refuses"),
        human_projection!(
            HumanTextLimit,
            "a plan states its complete output set inside its declared magnitudes, once per role, \
             or it refuses"
        ),
    )
}

/// How one planning issue reads for a person.
fn planning_line(issue: &ProjectionPlanningIssue) -> String {
    match issue {
        ProjectionPlanningIssue::MissingOwnerFact { seat } => {
            format!("a required seat is unfurnished: {}", seat.described())
        }
        ProjectionPlanningIssue::ContradictoryOwnerFacts { .. } => {
            String::from("two owner facts that decided this plan disagree")
        }
        ProjectionPlanningIssue::UnknownProjectionKind { .. } => {
            String::from("the named projection kind is one the plane does not implement")
        }
        ProjectionPlanningIssue::ProfileUnsupported { version, .. } => format!(
            "the named profile at version {} admits no such projection",
            version.position()
        ),
        ProjectionPlanningIssue::BoundExceeded {
            axis,
            bound,
            observed,
        } => format!(
            "{} exceeded: declared {bound}, observed {observed}",
            axis.described()
        ),
        ProjectionPlanningIssue::MembershipIncomplete { .. } => {
            String::from("a declared sibling output is absent from the membership")
        }
        ProjectionPlanningIssue::OrphanGeneratedNode { .. } => {
            String::from("a generated node arrived with no origin edge")
        }
        ProjectionPlanningIssue::MembershipDoubled {
            role_slot,
            observed,
        } => format!("rendered role {role_slot} carries {observed} planned members"),
        ProjectionPlanningIssue::TrailDiscontinuous { at } => format!(
            "the origin trail does not join: edge {at} starts at a node the edge before it did \
             not produce"
        ),
    }
}

/// One planning issue's complete canonical encoding.
fn planning_bytes(issue: &ProjectionPlanningIssue) -> Vec<u8> {
    let mut bytes = vec![issue.slot()];
    match issue {
        ProjectionPlanningIssue::MissingOwnerFact { seat } => bytes.push(seat.slot()),
        ProjectionPlanningIssue::ContradictoryOwnerFacts { between } => {
            encode_bytes(&between.left.citation_bytes(), &mut bytes);
            encode_bytes(&between.right.citation_bytes(), &mut bytes);
        }
        ProjectionPlanningIssue::UnknownProjectionKind { named } => {
            encode_bytes(named.as_bytes(), &mut bytes);
        }
        ProjectionPlanningIssue::ProfileUnsupported { profile, version } => {
            encode_bytes(profile.as_bytes(), &mut bytes);
            bytes.extend_from_slice(&version.position().to_be_bytes());
        }
        ProjectionPlanningIssue::BoundExceeded {
            axis,
            bound,
            observed,
        } => {
            bytes.push(axis.slot());
            bytes.extend_from_slice(&bound.to_be_bytes());
            bytes.extend_from_slice(&observed.to_be_bytes());
        }
        ProjectionPlanningIssue::MembershipIncomplete { absent } => {
            encode_bytes(absent.as_bytes(), &mut bytes);
        }
        ProjectionPlanningIssue::OrphanGeneratedNode { node } => {
            encode_bytes(node.as_bytes(), &mut bytes);
        }
        ProjectionPlanningIssue::MembershipDoubled {
            role_slot,
            observed,
        } => {
            bytes.extend_from_slice(&role_slot.to_be_bytes());
            bytes.extend_from_slice(&observed.to_be_bytes());
        }
        ProjectionPlanningIssue::TrailDiscontinuous { at } => {
            bytes.extend_from_slice(&at.to_be_bytes());
        }
    }
    bytes
}

/// How the first planning issue classifies what was observed.
const fn planning_observed(issue: &ProjectionPlanningIssue) -> ObservedClassification {
    match issue {
        ProjectionPlanningIssue::MissingOwnerFact { .. }
        | ProjectionPlanningIssue::MembershipIncomplete { .. } => {
            ObservedClassification::SeatAbsent
        }
        ProjectionPlanningIssue::ContradictoryOwnerFacts { .. }
        | ProjectionPlanningIssue::UnknownProjectionKind { .. } => {
            ObservedClassification::ContractDisagreement
        }
        ProjectionPlanningIssue::ProfileUnsupported { .. } => {
            ObservedClassification::ProfileDisagreement
        }
        ProjectionPlanningIssue::BoundExceeded { .. } => ObservedClassification::BoundExceeded,
        // A break in a walk is an origin the trail does not establish, which is
        // the same observation the orphan issue carries at the other extreme —
        // the classification says the provenance is absent, and the ISSUE says
        // whether it is missing entirely or missing from a position. The two
        // arms are joined because they are one observation, not two that happen
        // to agree today.
        ProjectionPlanningIssue::OrphanGeneratedNode { .. }
        | ProjectionPlanningIssue::TrailDiscontinuous { .. } => {
            ObservedClassification::OriginAbsent
        }
        ProjectionPlanningIssue::MembershipDoubled { .. } => {
            ObservedClassification::IdentityDisagreement
        }
    }
}

// ---------------------------------------------------------------------------
// Closure.
// ---------------------------------------------------------------------------

/// Project one closure refusal.
///
/// Every issue's role and its kind of disagreement survive, role by role.
pub fn closure_refused<R: RenderedRole>(refusal: &ProjectionClosureRefusal<R>) -> MacrocDiagnostic {
    let first = refusal.body().carried().first();
    let material: Vec<Vec<u8>> = refusal.body().carried().iter().map(closure_bytes).collect();
    diagnosed(
        MacrocPhase::Rendering,
        closure_observed(first),
        &summary(
            "the rendering does not close over the plan it claims to materialize",
            &closure_line(first),
            refusal.body().carried().len().saturating_sub(1),
            refusal.body().completion(),
        ),
        CLOSURE_FAMILY,
        &material,
        OwnerFactRef::named("macroc", "nothing-is-emitted-that-did-not-close"),
        human_projection!(
            HumanTextLimit,
            "the membership rebuilt out of the rendered units must equal the plan's declared \
             membership, role for role and set for set, before a token exists"
        ),
    )
}

/// How one closure issue reads for a person, naming the role where it has one.
fn closure_line<R: RenderedRole>(issue: &ClosureIssue<R>) -> String {
    let described = issue.described();
    match issue.role() {
        Some(role) => format!("{described} — at {}", role.described()),
        None => String::from(described),
    }
}

/// One closure issue's complete canonical encoding.
fn closure_bytes<R: RenderedRole>(issue: &ClosureIssue<R>) -> Vec<u8> {
    let mut bytes = vec![issue.slot()];
    match issue.role() {
        Some(role) => {
            bytes.push(1);
            bytes.extend_from_slice(&role.slot().to_be_bytes());
        }
        None => bytes.push(0),
    }
    match issue {
        ClosureIssue::MemberDuplicated { observed, .. }
        | ClosureIssue::MemberPlannedTwice { observed, .. }
        | ClosureIssue::ReconstructionUndeclarable { observed } => {
            bytes.extend_from_slice(&observed.to_be_bytes());
        }
        ClosureIssue::MemberMissing { .. }
        | ClosureIssue::MemberUnplanned { .. }
        | ClosureIssue::OriginOrphan { .. }
        | ClosureIssue::DigestMismatch { .. }
        | ClosureIssue::SemanticKeyMismatch { .. }
        | ClosureIssue::MaterializationMismatch { .. }
        | ClosureIssue::MembershipDisagreement { .. }
        | ClosureIssue::ReconstructionEmpty
        | ClosureIssue::JoinedTreeUnbounded => {}
    }
    bytes
}

/// How the first closure issue classifies what was observed.
const fn closure_observed<R: RenderedRole>(issue: &ClosureIssue<R>) -> ObservedClassification {
    match issue {
        ClosureIssue::MemberMissing { .. } | ClosureIssue::ReconstructionEmpty => {
            ObservedClassification::SeatAbsent
        }
        ClosureIssue::MemberUnplanned { .. } => ObservedClassification::ContractDisagreement,
        ClosureIssue::MemberDuplicated { .. }
        | ClosureIssue::MemberPlannedTwice { .. }
        | ClosureIssue::DigestMismatch { .. }
        | ClosureIssue::SemanticKeyMismatch { .. }
        | ClosureIssue::MembershipDisagreement { .. } => {
            ObservedClassification::IdentityDisagreement
        }
        ClosureIssue::OriginOrphan { .. } => ObservedClassification::OriginAbsent,
        ClosureIssue::MaterializationMismatch { .. } => ObservedClassification::ProfileDisagreement,
        ClosureIssue::ReconstructionUndeclarable { .. } | ClosureIssue::JoinedTreeUnbounded => {
            ObservedClassification::BoundExceeded
        }
    }
}

// ---------------------------------------------------------------------------
// Explanation.
// ---------------------------------------------------------------------------

/// Project one explanation-binding refusal.
///
/// An absent subject names its seat; a coverage refusal names every question it
/// established an issue about.
pub fn explanation_refused(refusal: &ExplanationBindingRefusal) -> MacrocDiagnostic {
    let owner = OwnerFactRef::named("macroc", "every-kind-answers-the-explanation-protocol");
    let repair = human_projection!(
        HumanTextLimit,
        "a projection answers every question its kind admits, exactly once, about its own subject \
         — an unbindable seat refuses rather than answering about a neighbouring value"
    );
    match refusal {
        ExplanationBindingRefusal::RequiredOutputAbsent { seat } => diagnosed(
            MacrocPhase::Inspection,
            ObservedClassification::SeatAbsent,
            &format!(
                "threadpak refusal-family derive: the explanation cannot bind its subject — {} is \
                 absent",
                seat.described()
            ),
            COVERAGE_FAMILY,
            &[vec![u8::MAX, seat.slot()]],
            owner,
            repair,
        ),
        ExplanationBindingRefusal::Coverage(coverage) => coverage_refused(coverage, owner, repair),
    }
}

/// Project one coverage body: every unanswered, doubled, and inadmissible seat.
fn coverage_refused(
    coverage: &ExplanationCoverage,
    owner: OwnerFactRef,
    repair: HumanProjection<HumanTextLimit>,
) -> MacrocDiagnostic {
    let first = coverage.body().carried().first();
    let material: Vec<Vec<u8>> = coverage
        .body()
        .carried()
        .iter()
        .map(coverage_bytes)
        .collect();
    diagnosed(
        MacrocPhase::Inspection,
        coverage_observed(first),
        &summary(
            "the explanation does not cover its kind's questions",
            &coverage_line(first),
            coverage.body().carried().len().saturating_sub(1),
            coverage.body().completion(),
        ),
        COVERAGE_FAMILY,
        &material,
        owner,
        repair,
    )
}

/// How one coverage issue reads for a person.
fn coverage_line(issue: &ExplanationCoverageIssue) -> String {
    match issue {
        ExplanationCoverageIssue::QuestionUnanswered(question) => {
            format!("unanswered: {}", question.described())
        }
        ExplanationCoverageIssue::QuestionAnsweredTwice(question) => {
            format!("answered twice: {}", question.described())
        }
        ExplanationCoverageIssue::QuestionNotApplicableToKind(question) => {
            format!("not admitted by this kind: {}", question.described())
        }
        ExplanationCoverageIssue::SeatBoundExceeded { bound, observed } => {
            format!("the explanation seats exceeded: declared {bound}, observed {observed}")
        }
    }
}

/// One coverage issue's complete canonical encoding.
fn coverage_bytes(issue: &ExplanationCoverageIssue) -> Vec<u8> {
    match issue {
        ExplanationCoverageIssue::QuestionUnanswered(question) => vec![0, question.slot()],
        ExplanationCoverageIssue::QuestionAnsweredTwice(question) => vec![1, question.slot()],
        ExplanationCoverageIssue::QuestionNotApplicableToKind(question) => vec![2, question.slot()],
        ExplanationCoverageIssue::SeatBoundExceeded { bound, observed } => {
            let mut bytes = vec![3];
            bytes.extend_from_slice(&bound.to_be_bytes());
            bytes.extend_from_slice(&observed.to_be_bytes());
            bytes
        }
    }
}

/// How the first coverage issue classifies what was observed.
const fn coverage_observed(issue: &ExplanationCoverageIssue) -> ObservedClassification {
    match issue {
        ExplanationCoverageIssue::QuestionUnanswered(_) => ObservedClassification::SeatAbsent,
        ExplanationCoverageIssue::QuestionAnsweredTwice(_)
        | ExplanationCoverageIssue::QuestionNotApplicableToKind(_) => {
            ObservedClassification::ContractDisagreement
        }
        ExplanationCoverageIssue::SeatBoundExceeded { .. } => ObservedClassification::BoundExceeded,
    }
}

// ---------------------------------------------------------------------------
// Rendering.
// ---------------------------------------------------------------------------

/// Project one materialization refusal: the exact declared magnitude, and the
/// unit it governs.
pub fn rendering_refused<R: RenderedRole>(refusal: RenderingRefusal, role: R) -> MacrocDiagnostic {
    let (bound, unit, slot) = match refusal {
        RenderingRefusal::BytesUnbounded => (
            RenderedByteLimit::MAX,
            "the bytes one rendered unit may carry",
            0u8,
        ),
        RenderingRefusal::UnitsUnbounded => (
            MembershipLimit::MAX,
            "the units one rendering may carry",
            1u8,
        ),
    };
    bounded_rendering(bound, unit, slot, role)
}

/// Project one assembly refusal: the tree magnitude, and the role that overran
/// it.
pub fn render_refused<R: RenderedRole>(refusal: RenderRefusal, role: R) -> MacrocDiagnostic {
    match refusal {
        RenderRefusal::Unbounded => bounded_rendering(
            GeneratedTokenLimit::MAX,
            "the tokens one generated tree may carry at one nesting level",
            2u8,
            role,
        ),
    }
}

/// The shared body of both rendering projections: which magnitude, which unit,
/// which role.
fn bounded_rendering<R: RenderedRole>(
    bound: usize,
    unit: &str,
    slot: u8,
    role: R,
) -> MacrocDiagnostic {
    let mut material = vec![slot];
    material.extend_from_slice(&role.slot().to_be_bytes());
    material.extend_from_slice(&u64::try_from(bound).unwrap_or(u64::MAX).to_be_bytes());
    diagnosed(
        MacrocPhase::Rendering,
        ObservedClassification::BoundExceeded,
        &format!(
            "threadpak refusal-family derive: rendering {} exceeded {unit}, declared {bound}",
            role.described()
        ),
        RENDERING_FAMILY,
        &[material],
        OwnerFactRef::named(
            "macroc",
            "every-rendered-seat-stands-under-a-declared-magnitude",
        ),
        human_projection!(
            HumanTextLimit,
            "a renderer that would emit past its declared magnitude refuses rather than \
             materializing part of a unit"
        ),
    )
}

// ---------------------------------------------------------------------------
// The shared shape.
// ---------------------------------------------------------------------------

/// One line for rustc, composed out of the typed issues.
///
/// It is a SUMMARY and says so: the first established issue in full, then how
/// many others there were, then whether the body examined everything it could.
/// The remainder is not lost — every issue has its own identity in the related
/// set, and the body itself is the value a caller of the underlying seam holds.
fn summary(family: &str, first: &str, further: usize, posture: CompletionPosture) -> String {
    let more = if further > 0 {
        format!(" (and {further} further established issues)")
    } else {
        String::new()
    };
    // The three postures are three different facts and the line says which one
    // it is carrying. A halted examination knows nothing about the sites past
    // its bound; a truncated report examined every site and knows exactly how
    // many findings it has no room for. Projecting both as "stopped" would tell
    // a reader to re-run a pass that already covered everything.
    let coverage = match posture {
        CompletionPosture::Complete => String::new(),
        CompletionPosture::EarlyStopped { .. } => {
            String::from(" (examination stopped at the declared issue bound)")
        }
        CompletionPosture::ReportTruncated(truncation) => {
            let omitted = truncation.omitted();
            format!(
                " (every site was examined; {omitted} further established issues do not fit the \
                 declared issue bound)"
            )
        }
    };
    format!("threadpak refusal-family derive: {family}: {first}{more}{coverage}")
}

/// One diagnostic over one refusal body's projected material.
fn diagnosed(
    phase: MacrocPhase,
    observed: ObservedClassification,
    composed: &str,
    family: u8,
    material: &[Vec<u8>],
    declared_by: OwnerFactRef,
    repair: HumanProjection<HumanTextLimit>,
) -> MacrocDiagnostic {
    // The material goes over, and the set derives both identity levels itself.
    // This seam holds one refusal family's issue material and nothing else, so
    // there is no body identity here to pair with somebody else's issues.
    let related = RelatedSet::derived_over(family, material);
    MacrocDiagnostic {
        // The one line says which of the two sets stands behind it, because the
        // typed posture beside it is not what rustc shows.
        summary: shown(&witnessed(composed, related.completion())),
        machine: crate::diagnostics::MachineAnchoring::UnmintedAtThisSeam,
        phase,
        // The declaration's first token. The disagreement is about the
        // declaration as a whole rather than about one token inside it, and
        // pretending otherwise would send a reader to an arbitrary spot.
        site: DiagnosticSite {
            token: crate::token::SpanHandle::at(0),
            // Composed here rather than resolved: this seat names the
            // declaration itself, so the semantic-origin role at position zero
            // IS the claim, not a stand-in for a table that did not reach.
            coordinate: SiteCoordinate::Resolved(threadpak::declaration::SourceCoordinate {
                role: threadpak::declaration::CoordinateRole::SemanticOrigin,
                position: 0,
            }),
        },
        expected: expected_contract(),
        observed,
        // The plane classifies what it observed and never elects the machine's
        // cause posture: narrowing is the machine's progress to report.
        cause: CauseDisposition::UnresolvedCause,
        related,
        repairs: Bounded::from_array([RepairAction {
            declared_by,
            description: repair,
        }]),
        reproduction: ReproductionRoute::CallableServices {
            entry: callable_entry(),
        },
        release: ReleasePosture::NoReleasePromise,
    }
}

/// One composed line, with the related set's own posture written into it.
///
/// A complete set adds nothing: the line already reads as a summary of a
/// complete body. A truncated set says so and says by how much, because the
/// typed posture beside it is not something rustc shows, and a reader given only
/// the body's own identity would otherwise take the coarser commitment for the
/// full one.
pub(crate) fn witnessed(composed: &str, completion: RelatedSetCompletion) -> String {
    match completion {
        RelatedSetCompletion::Complete => composed.to_owned(),
        RelatedSetCompletion::ReportTruncated(truncation) => {
            let omitted = truncation.omitted();
            format!(
                "{composed} (the related set was truncated at the declared issue bound: one \
                 identity over the complete body is carried and {omitted} per-issue identities \
                 are not)"
            )
        }
    }
}

/// One composed summary as a bounded projection.
///
/// The composition is a summary of a body whose issue count is bounded but whose
/// rendering is not, so it may outgrow the declared text magnitude. It is not
/// repaired with an empty line and not cut in half: the alternative is a static
/// line that is TRUE of the same refusal and says where the detail went. The
/// typed distinctions never depended on this seat — they ride on the observed
/// classification, on one identity per established issue, and on the related
/// set's own posture, which the static line points at rather than pre-empting.
fn shown(composed: &str) -> HumanProjection<HumanTextLimit> {
    match HumanProjection::<HumanTextLimit>::projected(composed) {
        Ok(projection) => projection,
        Err(BoundedConstruction::OverLimit) => human_projection!(
            HumanTextLimit,
            "threadpak refusal-family derive: the established issues do not fit one line; the \
             diagnostic's related set names them, and its related-set posture says whether every \
             one of them is named"
        ),
    }
}

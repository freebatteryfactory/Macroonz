//! Projecting each refusal family into the services' structured diagnostic.
//!
//! # One projection per family
//!
//! Six steps of the compile road refuse, and each refuses in its own
//! vocabulary: a planning body names an axis, a magnitude, and a count; a
//! closure body names a role and how the two disagreed at it; a coverage body
//! names every question that was unanswered, doubled, or inadmissible; a
//! rendering refusal names the exact bound and the unit that overran it, or the
//! role whose body observes the target its delivery must move it off; and a
//! binding refusal names which of the three parentage pairs disagreed and the
//! two identities it was asked to hold as one.
//!
//! Every function here projects the typed body it is handed and keeps what the
//! body knew:
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
//! # One grammar
//!
//! Every line this home hands a compiler is composed by [`composed`] and by
//! nothing else — including the capture family's, which is composed there and
//! read back by both of its projections.
//! Two grammars for one home is two shapes a reader has to learn, and the second
//! one is always the one that drifts: it is the one no law was written against.
//!
//! # Nonclaims
//!
//! It elects no machine cause posture.
//! Narrowing is the machine's progress to report and the plane observes rather
//! than concludes, so every diagnostic here carries
//! [`CauseDisposition::UnresolvedCause`] exactly as the capture road does.
//!
//! And it invents no repair.
//! Every [`RepairAction`] below cites a fact declared once on
//! [`RefusalDeriveFact`] and shows that fact's own repair, so a citation and the
//! sentence beside it are one row rather than two arguments that happened to be
//! passed together.

use super::types::{
    CarrierRoadRefusal, DIAGNOSTIC_PREFIX, ExplanationBindingRefusal, LineBody, LineSite,
    MemberRenderCause, MemberRenderRefusal, RefusalClass, RefusalDeriveFact, RefusalLine,
    RenderRefusal, RenderedMagnitude, callable_entry, expected_contract,
};
use crate::closure::{
    ClosureIssue, ExpansionBindingRefusal, ProjectionClosureRefusal, RenderingRefusal,
};
use crate::diagnostics::{
    DiagnosticSite, MacrocDiagnostic, MacrocPhase, ObservedClassification, RelatedSet,
    RelatedSetCompletion, ReleasePosture, RepairAction, ReproductionRoute, SiteCoordinate,
};
use crate::explanation_protocol::{ExplanationCoverage, ExplanationCoverageIssue};
use crate::generated_support::{AssemblyIssue, CarrierAssembly, ShellComposition};
use crate::plane::{HumanProjection, HumanTextLimit, RenderedRole, encode_bytes, human_projection};
use crate::refusal::{ProjectionPlanning, ProjectionPlanningIssue};
use crate::test_descriptor::{
    DescriptorPlanIssue, ShellDeclarationRefusal, ShellRenderIssue, ShellRendering,
    TrialDeclarationCause, TrialDeclarationRefusal,
};
use crate::token::{SpanHandle, SpanTable};
use threadpak::declaration::CoordinateRole;
use threadpak::evidence::CauseDisposition;
use threadpak::refusal::CompletionPosture;
use threadpak::types::{Bounded, BoundedConstruction};

/// The family tag written ahead of every related issue's material, so two
/// families' issues never encode alike.
const PLANNING_FAMILY: u8 = 0;

/// The closure family's tag.
const CLOSURE_FAMILY: u8 = 1;

/// The explanation-coverage family's tag.
const COVERAGE_FAMILY: u8 = 2;

/// The rendering family's tag.
const RENDERING_FAMILY: u8 = 3;

/// The expansion-binding family's tag.
///
/// The NUMBER is not renamed with the constant: a family tag is preimage
/// material, and every related identity already derived under this family stands
/// over the byte four. Renaming the spelling renames nothing derived.
const EXPANSION_FAMILY: u8 = 4;

/// The carrier-assembly family's tag.
const ASSEMBLY_FAMILY: u8 = 5;

/// The shell-rendering family's tag.
///
/// Its own tag rather than the rendering family's, because they are two
/// families: a rendering refusal names a role that passed a plane magnitude,
/// and a shell-rendering refusal names a token magnitude the CARRIER passed.
/// One tag for both would derive one related identity for two bodies that
/// happened to carry the same numbers.
const SHELL_FAMILY: u8 = 6;

/// The carrier-declaration family's tag.
const DECLARATION_FAMILY: u8 = 7;

/// The carrier plan-reading family's tag.
const DESCRIPTOR_PLAN_FAMILY: u8 = 8;

/// The trial-declaration grammar's tag.
///
/// Its own tag rather than the carrier-declaration family's, because they are two
/// families: a carrier-declaration refusal names a seat of the carrier's own
/// vocabulary, and a trial-declaration refusal names a clause of the authored
/// grammar that vocabulary is read out of. One tag for both would derive one
/// related identity for two bodies that happened to carry the same slot.
const TRIAL_DECLARATION_FAMILY: u8 = 9;

// ---------------------------------------------------------------------------
// The one compiler-facing grammar.
// ---------------------------------------------------------------------------

/// The word one coordinate role counts its positions in.
///
/// The ONE reading of [`CoordinateRole`] this home performs.
///
/// # Nonclaims
///
/// The phrase belongs beside the role, in the declaration home that owns the
/// roster; this is the reading seat until a projection lands there, and it is
/// exhaustive on purpose so a seventh role stops compiling here rather than
/// being shown under a sixth role's word.
const fn coordinate_role_word(role: CoordinateRole) -> &'static str {
    match role {
        CoordinateRole::Byte => "byte",
        CoordinateRole::UnicodeScalar => "unicode-scalar position",
        CoordinateRole::Utf16 => "utf-16 position",
        CoordinateRole::LineColumn => "line-column position",
        CoordinateRole::NormalizedSource => "normalized-source position",
        CoordinateRole::SemanticOrigin => "semantic-origin position",
    }
}

/// How much of a body one line is not carrying, composed from the typed body.
fn body_clause(body: LineBody) -> String {
    let (further, posture) = match body {
        LineBody::SingleCause => return String::new(),
        LineBody::Body { further, posture } => (further, posture),
    };
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
    format!("{more}{coverage}")
}

/// Where one line says the refusal sits, composed from the typed coordinate.
///
/// The role travels with the position, so a byte offset never reads as a token
/// ordinal and an ordinal never reads as a byte.
/// Where the producer's table does not reach the handle, the clause is that
/// refusal's own rendering — the locating half is missing and the reader is told
/// so, rather than handed a number that means nothing.
fn site_clause(site: LineSite) -> String {
    match site {
        LineSite::WholeDeclaration => String::new(),
        LineSite::At(SiteCoordinate::Resolved(coordinate)) => {
            let word = coordinate_role_word(coordinate.role);
            let position = coordinate.position;
            format!(" (at {word} {position})")
        }
        LineSite::At(SiteCoordinate::NotReached(refusal)) => format!(" ({})", refusal.described()),
    }
}

/// Compose the one line this home hands a compiler.
///
/// `<prefix>: <class>: <first>[<body>][<site>]`, and there is no second
/// composition anywhere in this home.
///
/// It is a SUMMARY and says so: the first established issue in full, then how
/// many others there were, then whether the body examined everything it could,
/// then where it sits if it sits anywhere narrower than the declaration.
///
/// The remainder is not lost — every issue has its own identity in the related
/// set, and the body itself is the value a caller of the underlying seam holds.
#[must_use]
pub fn composed(line: &RefusalLine<'_>, site: LineSite) -> String {
    let class_word = line.class.described();
    let first = line.first;
    let stated_body = body_clause(line.body);
    let stated_site = site_clause(site);
    format!("{DIAGNOSTIC_PREFIX}: {class_word}: {first}{stated_body}{stated_site}")
}

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
        &RefusalLine {
            class: RefusalClass::PlanNotStated,
            first: &planning_line(first),
            body: LineBody::Body {
                further: refusal.body().carried().len().saturating_sub(1),
                posture: refusal.body().completion(),
            },
        },
        PLANNING_FAMILY,
        &material,
        RefusalDeriveFact::APlanStatesItsCompleteOutputSetOrRefuses,
        &Placement::WholeDeclaration,
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
        ProjectionPlanningIssue::CauseSetUnwatchable { named, watchable } => format!(
            "the cause set names {named} source declarations and the trigger roster watches \
             {watchable}, so no watch set represents this context"
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
        ProjectionPlanningIssue::CauseSetUnwatchable { named, watchable } => {
            bytes.extend_from_slice(&named.to_be_bytes());
            bytes.extend_from_slice(&watchable.to_be_bytes());
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
        // A cause set the roster cannot watch is a context this PROFILE does not
        // admit, which is the same observation the unsupported-profile issue
        // carries: the plan is well formed, and the profile it was asked for
        // cannot represent it. The two arms are joined because they are one
        // observation, not two that happen to agree today.
        ProjectionPlanningIssue::ProfileUnsupported { .. }
        | ProjectionPlanningIssue::CauseSetUnwatchable { .. } => {
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
        &RefusalLine {
            class: RefusalClass::RenderingNotClosed,
            first: &closure_line(first),
            body: LineBody::Body {
                further: refusal.body().carried().len().saturating_sub(1),
                posture: refusal.body().completion(),
            },
        },
        CLOSURE_FAMILY,
        &material,
        RefusalDeriveFact::NothingIsEmittedThatDidNotClose,
        &Placement::WholeDeclaration,
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
        // The emission is the issue's own distinction: three joins are three
        // byte streams for three builds, and a caller told only that "the tree"
        // overran does not know which delivery to cut.
        ClosureIssue::JoinedTreeUnbounded { partition } => bytes.push(partition.slot()),
        // The ADDRESS is what collided, so the address is what separates two of
        // these issues: the roles are already written above, and two units at
        // two addresses under one pair of roles is a different observation from
        // two units at one.
        ClosureIssue::ArtifactAddressDoubled { byte_role, .. } => {
            encode_bytes(byte_role.as_bytes(), &mut bytes);
        }
        ClosureIssue::MemberMissing { .. }
        | ClosureIssue::MemberUnplanned { .. }
        | ClosureIssue::OriginOrphan { .. }
        | ClosureIssue::DigestMismatch { .. }
        | ClosureIssue::SemanticKeyMismatch { .. }
        | ClosureIssue::MaterializationMismatch { .. }
        | ClosureIssue::MembershipDisagreement { .. }
        | ClosureIssue::ReconstructionEmpty => {}
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
        // An address answering for two units is an identity that had to be
        // distinct and was not, which is the observation the doubled and
        // mismatched seats already carry.
        ClosureIssue::MemberDuplicated { .. }
        | ClosureIssue::MemberPlannedTwice { .. }
        | ClosureIssue::DigestMismatch { .. }
        | ClosureIssue::SemanticKeyMismatch { .. }
        | ClosureIssue::MembershipDisagreement { .. }
        | ClosureIssue::ArtifactAddressDoubled { .. } => {
            ObservedClassification::IdentityDisagreement
        }
        ClosureIssue::OriginOrphan { .. } => ObservedClassification::OriginAbsent,
        ClosureIssue::MaterializationMismatch { .. } => ObservedClassification::ProfileDisagreement,
        ClosureIssue::ReconstructionUndeclarable { .. }
        | ClosureIssue::JoinedTreeUnbounded { .. } => ObservedClassification::BoundExceeded,
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
    match refusal {
        ExplanationBindingRefusal::RequiredOutputAbsent { seat } => diagnosed(
            MacrocPhase::Inspection,
            ObservedClassification::SeatAbsent,
            &RefusalLine {
                class: RefusalClass::ExplanationNotBound,
                first: &format!("{} is absent", seat.described()),
                // One unbindable seat: the binding road refuses at the first
                // subject it cannot reach, so there is no body of seats behind
                // this one.
                body: LineBody::SingleCause,
            },
            COVERAGE_FAMILY,
            &[vec![u8::MAX, seat.slot()]],
            RefusalDeriveFact::EveryKindAnswersTheExplanationProtocol,
            &Placement::WholeDeclaration,
        ),
        ExplanationBindingRefusal::Coverage(coverage) => coverage_refused(coverage),
    }
}

/// Project one coverage body: every unanswered, doubled, and inadmissible seat.
fn coverage_refused(coverage: &ExplanationCoverage) -> MacrocDiagnostic {
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
        &RefusalLine {
            class: RefusalClass::ExplanationNotCovered,
            first: &coverage_line(first),
            body: LineBody::Body {
                further: coverage.body().carried().len().saturating_sub(1),
                posture: coverage.body().completion(),
            },
        },
        COVERAGE_FAMILY,
        &material,
        RefusalDeriveFact::EveryKindAnswersTheExplanationProtocol,
        &Placement::WholeDeclaration,
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
    bounded_rendering(materialization_magnitude(refusal), role)
}

/// Project one MEMBER-RENDER refusal, through the projection its own home owns.
///
/// The value names the role and which home refused, so this road dispatches and
/// composes nothing: a renderer's refusal reaches [`render_refused`] and a
/// materialization reaches [`rendering_refused`], each with the role the member
/// stands under. A projection that flattened the two would give a body observing
/// its own target and a byte count one sentence and one related identity.
pub fn member_render_refused<R: RenderedRole>(refusal: MemberRenderRefusal<R>) -> MacrocDiagnostic {
    match refusal.cause {
        MemberRenderCause::Rendered(cause) => render_refused(cause, refusal.role),
        MemberRenderCause::Materialized(cause) => rendering_refused(cause, refusal.role),
    }
}

/// Project one CARRIER-ROAD refusal, through the projection its own home owns.
///
/// # One boundary, nine homes
///
/// Every step of the carrier road refuses in the vocabulary of the home that owns
/// it, and this is the seam where those bodies become the one line a compiler
/// shows. It composes nothing of its own: each arm hands its body to the road
/// that already projects that home, so a reader of a diagnostic reads what the
/// step said rather than a summary this seat wrote.
///
/// Exhaustive on purpose. A step added to the carrier road stops compiling here
/// until somebody says which projection its refusal reaches, which is the whole
/// reason the road answers in a typed sum rather than in the diagnostic itself.
pub fn carrier_road_refused(refusal: CarrierRoadRefusal) -> MacrocDiagnostic {
    match refusal {
        CarrierRoadRefusal::Planned(body) => planning_refused(&body),
        CarrierRoadRefusal::Declared(body) => carrier_declaration_refused(body),
        CarrierRoadRefusal::Assembled(body) => assembly_refused(&body),
        CarrierRoadRefusal::PlanNotRead(issue) => descriptor_plan_refused(issue),
        CarrierRoadRefusal::Composed(body) => match *body {
            // Two homes answer at the composition seam and each is projected by
            // its own: a pair that is not one declaration's is a COMPOSITION fact
            // and reads in the assembly family, and a tree past its bound is the
            // CARRIER's fact and reads in the shell family.
            ShellComposition::NotOneDeclarations(composed) => assembly_refused(&composed),
            ShellComposition::Rendering(rendering) => shell_refused(&rendering),
        },
        CarrierRoadRefusal::Rendered(body) => member_render_refused(body),
        CarrierRoadRefusal::Closed(body) => closure_refused(&body),
        CarrierRoadRefusal::Explained(body) => explanation_refused(&body),
        CarrierRoadRefusal::Bound(body) => expansion_refused(&body),
    }
}

/// The declared magnitude one materialization refusal names.
///
/// A projection of the foreign typed value onto this home's magnitude roster,
/// and the only place the two are related: the closure home states WHICH
/// magnitude was passed, and the roster states what that magnitude governs and
/// how large it is.
const fn materialization_magnitude(refusal: RenderingRefusal) -> RenderedMagnitude {
    match refusal {
        RenderingRefusal::BytesUnbounded => RenderedMagnitude::RenderedBytes,
        RenderingRefusal::UnitsUnbounded => RenderedMagnitude::RenderedUnits,
    }
}

/// Project one assembly refusal: the tree magnitude and the role that overran
/// it, or the body that observes the target its delivery must move it off.
///
/// Two arms and two projections, because they are two observations rather than
/// one with a different number in it. A tree past a magnitude is a fact about
/// SIZE and its repair is a smaller declaration; a body that observes its own
/// target is a fact about MEANING and no size makes it lawful, so a line naming
/// a bound would send a reader to shorten a declaration that is already short
/// enough.
pub fn render_refused<R: RenderedRole>(refusal: RenderRefusal, role: R) -> MacrocDiagnostic {
    match refusal {
        RenderRefusal::Unbounded => bounded_rendering(RenderedMagnitude::GeneratedTokens, role),
        RenderRefusal::TargetObserved => target_observed(role),
    }
}

/// Project one relocation refusal: the role whose body observes the target the
/// declaration named.
///
/// The role's own description is the whole of the line's subject, because the
/// role IS which delivery could not be rendered — and the two evaluation roles
/// are the only ones this refusal is establishable at, since the production
/// roles are rendered for the declared target and move nowhere.
fn target_observed<R: RenderedRole>(role: R) -> MacrocDiagnostic {
    let mut material = vec![u8::MAX];
    material.extend_from_slice(&role.slot().to_be_bytes());
    let described = role.described();
    diagnosed(
        MacrocPhase::Rendering,
        ObservedClassification::ContractDisagreement,
        &RefusalLine {
            class: RefusalClass::SubjectNotSubstitutable,
            first: &format!(
                "{described} observes the type the declaration named, so the copy does not stand \
                 over the support shell's own subject"
            ),
            // One role, one body, one observation: the walk answers about the
            // tree it was handed and enumerates nothing behind it.
            body: LineBody::SingleCause,
        },
        RENDERING_FAMILY,
        &[material],
        RefusalDeriveFact::AnEvaluationCopyStandsOverALocalSubject,
        &Placement::WholeDeclaration,
    )
}

/// Project one expansion-binding refusal: the two identities the binding was
/// asked to hold as one.
///
/// Both identities travel, and neither is elected: an expansion bound over any
/// of the three disagreeing pairs would name one plan, one proof, or one
/// explanation while carrying another's, and would answer every question
/// correctly about the wrong expansion — so the diagnostic carries what would
/// have been bound rather than a summary of it.
///
/// The three arms compose three different lines and three different materials,
/// because they are three different repairs: a proof taken against another plan,
/// an explanation answered over another plan, and an explanation answered over
/// another proof. A shared sentence under a shared material would tell a caller
/// that "something did not agree" and derive one related identity for all three.
///
/// The line names the disagreement and the related set names the two identities.
/// Spelling thirty-two bytes into a sentence rustc shows one line of would hand a
/// reader a digest where a repair belongs; the identities are typed values and
/// travel as the material this projection derives its related identities over.
pub fn expansion_refused(refusal: &ExpansionBindingRefusal) -> MacrocDiagnostic {
    // The arm's own discriminant leads the material, so two arms carrying two
    // identities that happened to coincide are still two related identities.
    let (line, material) = match refusal {
        ExpansionBindingRefusal::ClosureProvedAgainstAnotherPlan { planned, proved } => (
            "the closure proves a rendering against a plan other than the one bound beside it",
            binding_material(0, planned.as_bytes(), proved.as_bytes()),
        ),
        ExpansionBindingRefusal::ExplanationAnsweredOverAnotherPlan { planned, answered } => (
            "the explanation was answered over a plan other than the one bound beside it",
            binding_material(1, planned.as_bytes(), answered.as_bytes()),
        ),
        ExpansionBindingRefusal::ExplanationAnsweredOverAnotherClosure { proved, answered } => (
            "the explanation was answered over a proof other than the one bound beside it",
            binding_material(2, proved.as_bytes(), answered.as_bytes()),
        ),
    };
    diagnosed(
        MacrocPhase::Inspection,
        ObservedClassification::IdentityDisagreement,
        &RefusalLine {
            class: RefusalClass::ExpansionNotBound,
            first: line,
            // One disagreement: the binding checks each pair in turn and refuses
            // at the first that disagrees, so there is no body behind it.
            body: LineBody::SingleCause,
        },
        EXPANSION_FAMILY,
        &[material],
        RefusalDeriveFact::NothingIsHandedOutThatDidNotBind,
        &Placement::WholeDeclaration,
    )
}

/// One binding disagreement's complete canonical material: which pair
/// disagreed, then the identity that was BOUND and the identity that was
/// actually carried, each at full width.
///
/// The order is the refusal's own — what the binding was handed first, what the
/// value it was handed turned out to name second — so a reader of the two
/// identities knows which is which without the diagnostic saying so twice.
fn binding_material(pair: u8, bound: &[u8; 32], carried: &[u8; 32]) -> Vec<u8> {
    let mut material = vec![pair];
    encode_bytes(bound, &mut material);
    encode_bytes(carried, &mut material);
    material
}

/// The shared body of both rendering projections: which magnitude, and which
/// role passed it.
///
/// Every part of the line is read off the two typed values — the role's own
/// description, the magnitude's own description, and the magnitude's own
/// declared number — so no phrase here restates a bound the plane declares.
fn bounded_rendering<R: RenderedRole>(magnitude: RenderedMagnitude, role: R) -> MacrocDiagnostic {
    let bound = magnitude.declared();
    let mut material = vec![magnitude.slot()];
    material.extend_from_slice(&role.slot().to_be_bytes());
    material.extend_from_slice(&u64::try_from(bound).unwrap_or(u64::MAX).to_be_bytes());
    let described = role.described();
    let governs = magnitude.described();
    diagnosed(
        MacrocPhase::Rendering,
        ObservedClassification::BoundExceeded,
        &RefusalLine {
            class: RefusalClass::MagnitudeNotHeld,
            first: &format!("{described} passed {governs}, declared {bound}"),
            // One magnitude, one role: a renderer refuses at the bound it
            // reached and enumerates nothing behind it.
            body: LineBody::SingleCause,
        },
        RENDERING_FAMILY,
        &[material],
        RefusalDeriveFact::EveryRenderedSeatStandsUnderADeclaredMagnitude,
        &Placement::WholeDeclaration,
    )
}

/// Project one carrier-assembly refusal: every way a set of closed outputs does
/// not compose into one exported shell, and the way a carrier plan does not
/// belong to the assembly it would close around.
///
/// The line names the first established disagreement in full and says how many
/// stand behind it, and the axis rides in the line where the issue is about one:
/// a caller told only that "the assembly failed" has three axes to inspect and
/// no reason to prefer any of them.
///
/// One projection for the whole family whichever seam established the body. The
/// composing pass and the shell road both refuse in the assembly family because
/// both are answering the same question — whether one carrier delivers one
/// declaration's proved cargo — and a second projection for the second seam
/// would derive a second related identity for issues under one roster.
pub fn assembly_refused(refusal: &CarrierAssembly) -> MacrocDiagnostic {
    let first = refusal.body().carried().first();
    let material: Vec<Vec<u8>> = refusal
        .body()
        .carried()
        .iter()
        .map(assembly_bytes)
        .collect();
    diagnosed(
        MacrocPhase::Rendering,
        assembly_observed(first),
        &RefusalLine {
            class: RefusalClass::CarrierNotAssembled,
            first: &assembly_line(first),
            body: LineBody::Body {
                further: refusal.body().carried().len().saturating_sub(1),
                posture: refusal.body().completion(),
            },
        },
        ASSEMBLY_FAMILY,
        &material,
        RefusalDeriveFact::OneCarrierDeliversOneDeclarationsProvedCargo,
        &Placement::WholeDeclaration,
    )
}

/// How one assembly issue reads for a person, naming the axis where it is about
/// one.
fn assembly_line(issue: &AssemblyIssue) -> String {
    let described = issue.described();
    match issue.axis() {
        Some(axis) => format!("{described} — at {}", axis.described()),
        None => String::from(described),
    }
}

/// What one assembly issue observed, read off the issue rather than shared
/// across the family.
///
/// The three classifications are three different observations: a root or a
/// parentage that disagrees is an IDENTITY disagreement, a partition read where
/// another was declared is a CONTRACT disagreement, and a seat the grammar does
/// not write is one the profile does not offer.
///
/// A carrier plan standing under another declaration is an identity
/// disagreement on exactly the terms the axis-level root issue is: two roots
/// were asked to be one, and neither is elected. That the two are established at
/// different seams — one while the assembly is built, one at the road that
/// renders the shell over it — is a fact about WHERE, and this reading answers
/// WHAT.
const fn assembly_observed(issue: &AssemblyIssue) -> ObservedClassification {
    match issue {
        AssemblyIssue::RootsDisagree { .. }
        | AssemblyIssue::SchemaExpectationNotPublished { .. }
        | AssemblyIssue::CarrierRootIsNotTheAssemblys { .. }
        | AssemblyIssue::CargoNotTheSourcesOwn { .. } => {
            ObservedClassification::IdentityDisagreement
        }
        AssemblyIssue::CargoConsumedTwice { .. }
        | AssemblyIssue::CargoReachesASecondDestination { .. } => {
            ObservedClassification::ContractDisagreement
        }
        AssemblyIssue::BenchVehicleNotOpen => ObservedClassification::ProfileDisagreement,
    }
}

/// One assembly issue's complete canonical encoding, through the road its own
/// home declares beside the roster.
fn assembly_bytes(issue: &AssemblyIssue) -> Vec<u8> {
    let mut material = Vec::new();
    issue.encode_into(&mut material);
    material
}

/// Project one carrier-declaration refusal: the seat of the carrier's own
/// vocabulary a declaration did not fill.
///
/// The carrier's declaration family is SINGLE-CAUSE — its checks are dependent
/// and in a declared order, so exactly one cause is true of any refused
/// declaration — which is why the line enumerates nothing behind it.
pub fn carrier_declaration_refused(refusal: ShellDeclarationRefusal) -> MacrocDiagnostic {
    diagnosed(
        MacrocPhase::Rendering,
        ObservedClassification::ContractDisagreement,
        &RefusalLine {
            class: RefusalClass::CarrierNotDeclared,
            first: refusal.described(),
            body: LineBody::SingleCause,
        },
        DECLARATION_FAMILY,
        &[vec![refusal.slot()]],
        RefusalDeriveFact::ACarrierSpellingIsOneRustIdentifier,
        &Placement::WholeDeclaration,
    )
}

/// Project one plan-reading refusal: the carrier's own role the plan did not
/// declare, or the delivery it declared instead.
///
/// The two are the plan's facts rather than the rendering's, so the class is the
/// planning one and the cited fact is the output firewall's: a plan that states
/// no member under its kind's one role has not stated its complete output set.
pub fn descriptor_plan_refused(issue: DescriptorPlanIssue) -> MacrocDiagnostic {
    let (described, slot, role_slot) = match issue {
        DescriptorPlanIssue::RoleNotPlanned { role_slot } => (
            "the plan declares no member under the carrier's one rendered role",
            0_u8,
            role_slot,
        ),
        DescriptorPlanIssue::DestinationNotDeclarationSite { role_slot } => (
            "the planned carrier member lands somewhere other than the declaration site",
            1_u8,
            role_slot,
        ),
    };
    let mut material = vec![slot];
    material.extend_from_slice(&role_slot.to_be_bytes());
    diagnosed(
        MacrocPhase::Planning,
        ObservedClassification::ContractDisagreement,
        &RefusalLine {
            class: RefusalClass::PlanNotStated,
            first: described,
            // The two checks are dependent — there is no destination to read
            // until a member was found — so exactly one is ever established.
            body: LineBody::SingleCause,
        },
        DESCRIPTOR_PLAN_FAMILY,
        &[material],
        RefusalDeriveFact::APlanStatesItsCompleteOutputSetOrRefuses,
        &Placement::WholeDeclaration,
    )
}

/// Project one trial-declaration refusal: the clause of the authored grammar
/// that was not read, or the seat of the carrier's own vocabulary the value it
/// read did not fill — at the token the clause sits at.
///
/// # Two homes, two bodies, one projection
///
/// The refusal names which grammar refused and carries that grammar's body whole,
/// so this road unwraps rather than summarizes: the trial grammar's cause is
/// shown under the CAPTURE class, because a trial declaration is read out of the
/// declaration's own tokens, and the carrier's cause is shown under the
/// carrier-declaration class it already answers in. Two family tags, so a
/// malformed clause and a doubled role never derive one related identity.
///
/// # The site
///
/// A TOKEN, in both arms. Every refusal on this road is a fact about one clause
/// an author wrote, and the reader is sent to that clause rather than to the
/// declaration's opening. Where the producer's table does not reach the handle
/// the line says THAT rather than a position, exactly as the capture family's own
/// projection does.
pub fn trial_declaration_refused(
    refusal: TrialDeclarationRefusal,
    spans: &SpanTable,
) -> MacrocDiagnostic {
    match refusal {
        TrialDeclarationRefusal::Grammar { cause, at } => diagnosed(
            MacrocPhase::Capture,
            trial_observed(cause),
            &RefusalLine {
                class: RefusalClass::DeclarationNotRead,
                first: cause.described(),
                // The trial grammar is single-cause: the reader refuses at the
                // first clause it cannot read and enumerates nothing behind it.
                body: LineBody::SingleCause,
            },
            TRIAL_DECLARATION_FAMILY,
            &[vec![cause.slot()]],
            RefusalDeriveFact::ATrialDeclarationStatesDescriptorMeaningAlone,
            &Placement::AtToken { token: at, spans },
        ),
        TrialDeclarationRefusal::Carrier {
            refusal: carried,
            at,
        } => diagnosed(
            MacrocPhase::Capture,
            ObservedClassification::ContractDisagreement,
            &RefusalLine {
                class: RefusalClass::CarrierNotDeclared,
                first: carried.described(),
                body: LineBody::SingleCause,
            },
            DECLARATION_FAMILY,
            &[vec![carried.slot()]],
            RefusalDeriveFact::ACarrierSpellingIsOneRustIdentifier,
            &Placement::AtToken { token: at, spans },
        ),
    }
}

/// What one trial-grammar cause observed.
///
/// A clause the grammar does not declare and a value whose shape it cannot read
/// are CONTRACT disagreements: the declaration is well formed Rust and says
/// something this grammar does not admit. A clause that is absent is a SEAT that
/// was not filled, and a clause stated twice is an IDENTITY disagreement — one
/// key answering for two values.
const fn trial_observed(cause: TrialDeclarationCause) -> ObservedClassification {
    match cause {
        TrialDeclarationCause::NotCovered | TrialDeclarationCause::NotBodied => {
            ObservedClassification::SeatAbsent
        }
        TrialDeclarationCause::NotDeclaredOnce | TrialDeclarationCause::NotDistinct => {
            ObservedClassification::IdentityDisagreement
        }
        TrialDeclarationCause::NotAClause
        | TrialDeclarationCause::NotADeclarableClause
        | TrialDeclarationCause::NotANamedReference
        | TrialDeclarationCause::NotARoster
        | TrialDeclarationCause::NotASuiteGroup
        | TrialDeclarationCause::NotARow => ObservedClassification::ContractDisagreement,
    }
}

/// Project one shell-rendering refusal: the token magnitude the carrier passed.
///
/// The carrier's own family, which both crossings of the wall refuse in, so this
/// projection stands for the trial crossing and the bench crossing alike.
pub fn shell_refused(refusal: &ShellRendering) -> MacrocDiagnostic {
    let material: Vec<Vec<u8>> = refusal.body().carried().iter().map(shell_bytes).collect();
    let bound = RenderedMagnitude::GeneratedTokens.declared();
    let governs = RenderedMagnitude::GeneratedTokens.described();
    diagnosed(
        MacrocPhase::Rendering,
        ObservedClassification::BoundExceeded,
        &RefusalLine {
            class: RefusalClass::MagnitudeNotHeld,
            first: &format!("the generated support shell passed {governs}, declared {bound}"),
            body: LineBody::Body {
                further: refusal.body().carried().len().saturating_sub(1),
                posture: refusal.body().completion(),
            },
        },
        SHELL_FAMILY,
        &material,
        RefusalDeriveFact::EveryRenderedSeatStandsUnderADeclaredMagnitude,
        &Placement::WholeDeclaration,
    )
}

/// One shell-rendering issue's complete canonical encoding: the declared bound
/// it names, at the width the issue carries it.
///
/// Exhaustive over the roster on purpose: an issue added to
/// [`ShellRenderIssue`] stops compiling here until somebody says what of it a
/// related identity stands over.
fn shell_bytes(issue: &ShellRenderIssue) -> Vec<u8> {
    match issue {
        ShellRenderIssue::ShellTreeUnbounded { bound } => bound.to_be_bytes().to_vec(),
    }
}

// ---------------------------------------------------------------------------
// The shared shape.
// ---------------------------------------------------------------------------

/// Where one projected refusal sits.
///
/// Two placements, and they are different observations rather than one with a
/// number left out. A refusal about the DECLARATION — a plan's output set, a
/// rendering's closure, an explanation's coverage, a magnitude a role passed —
/// has nowhere narrower to point, and a line that named a position inside it
/// would send a reader to an arbitrary spot. A refusal about one CLAUSE of an
/// authored attribute has exactly one place, and the reader is sent there.
enum Placement<'table> {
    /// The refusal is about the declaration as a whole.
    WholeDeclaration,
    /// The refusal sits at one token, resolved through the producer's own table.
    AtToken {
        /// The token it sits at.
        token: SpanHandle,
        /// The table the producer resolves handles through.
        spans: &'table SpanTable,
    },
}

impl Placement<'_> {
    /// The diagnostics home's own site for this placement.
    ///
    /// The whole-declaration placement answers with the AT-TOKEN arm at the
    /// declaration's first token, deliberately: every refusal that reaches it is
    /// established at or after planning, which is downstream of a capture that
    /// succeeded, so a table was built and a handle means something. The
    /// pre-capture arm belongs to a text read that refused before any of that,
    /// and no road into this function stands under it.
    fn site(&self) -> DiagnosticSite {
        match *self {
            Self::WholeDeclaration => DiagnosticSite::at_token(
                SpanHandle::at(0),
                // Composed here rather than resolved: this seat names the
                // declaration itself, so the semantic-origin role at position
                // zero IS the claim, not a stand-in for a table that did not
                // reach.
                SiteCoordinate::Resolved(threadpak::declaration::SourceCoordinate {
                    role: CoordinateRole::SemanticOrigin,
                    position: 0,
                }),
            ),
            Self::AtToken { token, spans } => DiagnosticSite::at_token(
                token,
                SiteCoordinate::answered(spans.coordinate_of(token)),
            ),
        }
    }

    /// What the composed line says about where the refusal sits.
    fn line_site(&self, site: &DiagnosticSite) -> LineSite {
        match *self {
            Self::WholeDeclaration => LineSite::WholeDeclaration,
            Self::AtToken { .. } => LineSite::At(site.coordinate()),
        }
    }
}

/// One diagnostic over one refusal body's projected material.
///
/// Every seat that could be written two ways is written once here: the line
/// through [`composed`], the citation and its repair through one
/// [`RefusalDeriveFact`] row, and the site through the placement the caller
/// states.
fn diagnosed(
    phase: MacrocPhase,
    observed: ObservedClassification,
    line: &RefusalLine<'_>,
    family: u8,
    material: &[Vec<u8>],
    fact: RefusalDeriveFact,
    placement: &Placement<'_>,
) -> MacrocDiagnostic {
    // The material goes over, and the set derives both identity levels itself.
    // This seam holds one refusal family's issue material and nothing else, so
    // there is no body identity here to pair with somebody else's issues.
    let related = RelatedSet::derived_over(family, material);
    // Built once and read twice: the prose and the diagnostic's own site are
    // projections of the SAME value, so a line saying one position beside a seat
    // holding another is unrepresentable here.
    let site = placement.site();
    let composed_line = composed(line, placement.line_site(&site));
    MacrocDiagnostic {
        // The one line says which of the two sets stands behind it, because the
        // typed posture beside it is not what rustc shows.
        summary: shown(&witnessed(&composed_line, related.completion())),
        machine: crate::diagnostics::MachineAnchoring::UnmintedAtThisSeam,
        phase,
        site,
        expected: expected_contract(),
        observed,
        // The plane classifies what it observed and never elects the machine's
        // cause posture: narrowing is the machine's progress to report.
        cause: CauseDisposition::UnresolvedCause,
        related,
        repairs: Bounded::from_array([RepairAction {
            declared_by: fact.citation(),
            description: fact.repair(),
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
/// complete body.
/// A truncated set says so and says by how much, because the typed posture
/// beside it is not something rustc shows, and a reader given only the body's
/// own identity would otherwise take the coarser commitment for the full one.
pub(crate) fn witnessed(line: &str, completion: RelatedSetCompletion) -> String {
    match completion {
        RelatedSetCompletion::Complete => line.to_owned(),
        RelatedSetCompletion::ReportTruncated(truncation) => {
            let omitted = truncation.omitted();
            format!(
                "{line} (the related set was truncated at the declared issue bound: one identity \
                 over the complete body is carried and {omitted} per-issue identities are not)"
            )
        }
    }
}

/// One composed summary as a bounded projection.
///
/// The composition is a summary of a body whose issue count is bounded but whose
/// rendering is not, so it may outgrow the declared text magnitude.
/// It is not repaired with an empty line and not cut in half: the alternative is
/// a static line that is TRUE of the same refusal and says where the detail
/// went.
/// The typed distinctions never depended on this seat — they ride on the
/// observed classification, on one identity per established issue, and on the
/// related set's own posture, which the static line points at rather than
/// pre-empting.
///
/// # The one second spelling of the prefix
///
/// The static line below spells [`DIAGNOSTIC_PREFIX`] a second time, and it is
/// the only place in this home that does.
/// [`human_projection!`] proves a rendering fits its limit family during const
/// evaluation and therefore takes a LITERAL, so a line composed out of the
/// prefix constant cannot be proven at compile time — and a runtime projection
/// here would return a refusal this seat has no honest value to fill, which is
/// exactly the empty fallback the projection road exists to avoid.
/// Closing it is a `const` road to a proven projection in the plane, which is
/// not this home's to write.
pub(crate) fn shown(line: &str) -> HumanProjection<HumanTextLimit> {
    match HumanProjection::<HumanTextLimit>::projected(line) {
        Ok(projection) => projection,
        Err(BoundedConstruction::OverLimit) => human_projection!(
            HumanTextLimit,
            "threadpak refusal-family derive: the established issues do not fit one line; the \
             diagnostic's related set names them, and its related-set posture says whether every \
             one of them is named"
        ),
    }
}

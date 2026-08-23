//! The diagnostics home's invariant nucleus: the one road that reaches the one
//! seat a caller may not write, the one place a related set's identities are
//! derived, and the roads a diagnostic's site is built and read through.
//!
//! Declared inside `types.rs` as its own child, so the truncation's bound and
//! count are reachable here and nowhere else.
//! Most of this home is a plain readable seat; what lands here is what is about
//! an ACT rather than about a value — how many per-issue identities a
//! set-building road left behind, and which of two postures a site was
//! established under.
//!
//! The related-set road below takes the issue material, not a number and not
//! identities somebody else already derived, and it builds the set rather than
//! being handed one.
//! The count is read off what the road itself dropped and lands in the same
//! value as what it kept, so the posture and the set are two readings of one
//! act.
//! Both identity levels are derived here out of that one material, so there is
//! no loose half for a caller to hold and no way to seat one refusal's coarse
//! commitment over another refusal's issues.
//!
//! The site roads are here because one of them decides something.
//! A site under either posture answers "where does this sit?", but only one of
//! the two postures ever had a table to resolve an answer with: a pre-capture
//! observation carries the byte it was born at, which no table was ever
//! consulted for, and lifting that byte into the answered posture is a statement
//! about what did and did not happen.
//! It is stated once, below, rather than at each seam that asks.

use super::{
    DiagnosticSite, RelatedIdentity, RelatedIssueLimit, RelatedSet, RelatedSetCompletion,
    RelatedSetTruncation, SiteCoordinate,
};
use crate::plane::{
    AuthoringLimitProfile, ProjectionIdentity, ProjectionRole, ProjectionTranscript,
    RelatedBodySubject, RelatedIssueSubject, encode_bytes,
};
use crate::token::{SourceCoordinate, SpanHandle};
use core::num::NonZeroUsize;
use macroonz::{AdmittedLimit, Bounded, BoundedConstruction, StopBound};

/// The content one related identity is derived over, at either level.
///
/// The family tag separates two families' spaces so the same bytes raised under
/// two families never encode alike, and the material is length-framed behind it
/// so no two materials share a preimage.
/// One composition serves both levels deliberately: what separates them is the
/// subject, which is a segment of the derive-key context rather than a
/// discriminant somebody could forget to write into a preimage.
fn related_content(family: u8, material: &[u8]) -> Vec<u8> {
    let mut content = vec![family];
    encode_bytes(material, &mut content);
    content
}

/// One related-issue identity over one established issue's material.
///
/// The single derivation for this subject, private on purpose: an identity of
/// this subject exists only as part of a set this file built.
///
/// # Its own role, and therefore its own ladder
///
/// The role is [`ProjectionRole::DiagnosticRelation`], which is what these two
/// levels actually are. They stood under the CLOSED-EXPANSION role, which put
/// every related identity in every diagnostic on the terminal's version ladder —
/// so a widening of what a terminal commits to renamed them, and neither level
/// holds a member of that grammar.
fn related_issue_identity(family: u8, material: &[u8]) -> ProjectionIdentity<RelatedIssueSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::DiagnosticRelation,
        &related_content(family, material),
        u32::from(family),
    ))
}

/// One related-body identity over the framing of a whole body's issues.
///
/// The same private discipline and the same role, under the other subject.
/// The separation is deliberately not a byte inside the preimage: the subject
/// rides in the derive-key context and in the transcript, so two levels over
/// identical content are separated before a byte of that content is read and
/// disagree inside it as well.
fn related_body_identity(family: u8, material: &[u8]) -> ProjectionIdentity<RelatedBodySubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::DiagnosticRelation,
        &related_content(family, material),
        u32::from(family),
    ))
}

impl RelatedSetTruncation {
    /// The declared bound the set was truncated at.
    #[must_use]
    pub const fn stopped_at(self) -> StopBound {
        self.stopped_at
    }

    /// How many per-issue identities the set does not carry; at least one, by
    /// shape.
    /// A truncation that dropped nothing is
    /// [`RelatedSetCompletion::Complete`] and is unrepresentable here.
    #[must_use]
    pub const fn omitted(self) -> NonZeroUsize {
        self.omitted
    }
}

impl RelatedSet {
    /// The related set one refusal body amounts to, derived over that body's
    /// own issue material: the whole body's identity first, then one per
    /// established issue, and the posture that says whether that is all of them.
    ///
    /// The road is handed the issues' canonical material and derives both levels
    /// itself.
    /// A road taking a body identity and a set of per-issue identities as two
    /// arguments takes two halves that do not check each other: each half is
    /// honestly derived on its own, so the pair can name one refusal's body over
    /// another refusal's issues and still read exactly like a set that belongs
    /// together.
    /// Deriving here removes the pairing instead of policing it.
    ///
    /// # Construction
    ///
    /// The per-issue identities are derived first, each over one issue's framed
    /// material, and the body's identity is derived over the framing of exactly
    /// that material in exactly that order.
    /// The body's preimage is therefore the issues: two different issue sets
    /// cannot reach one body identity, and one issue set cannot reach two.
    /// That is what makes the coarser commitment carried alone under truncation
    /// a commitment to these issues rather than a word about issues in general.
    ///
    /// The two levels are two subjects — `related-body` and `related-issue` — so
    /// the same content at the two levels derives under two derive-key contexts
    /// and is two unrelated values.
    ///
    /// This is a mint site, so its content grammar is stated here in full, the
    /// way [`crate::plane::ProjectionTranscript`] requires of every mint site.
    /// Both levels derive at role `diagnostic-relation`, ROOTED, at roster
    /// position `family`, over
    ///
    /// ```text
    /// content = family_byte || u64be(material.len()) || material
    /// ```
    ///
    /// where the material of an issue is that issue's own canonical bytes, and
    /// the material of the body is `u64be(issue.len()) || issue` for every issue
    /// in order, concatenated.
    ///
    /// An independent reader re-derives both levels from the issues, this
    /// paragraph, and the transcript specification on
    /// [`ProjectionTranscript`](crate::plane::ProjectionTranscript) — which is
    /// where the ten members are listed, and where the rooted posture's own
    /// discriminant byte is published, on
    /// [`TranscriptAnchoring`](crate::plane::TranscriptAnchoring). Nothing else
    /// is needed and nothing else is held back: the two subjects are
    /// `related-issue` and `related-body`, the family segment and version come
    /// off [`DIAGNOSTIC_RELATION_IDENTITY_PROFILE`], and the derive-key context
    /// is spelled by the domain grammar
    /// [`IdentityProfile`](crate::plane::IdentityProfile) states.
    ///
    /// [`DIAGNOSTIC_RELATION_IDENTITY_PROFILE`]: crate::plane::DIAGNOSTIC_RELATION_IDENTITY_PROFILE
    ///
    /// # Bounds
    ///
    /// [`RelatedIssueLimit`] is this home's own magnitude and is read off no
    /// refusal family's: the services declare issue bodies wider than it — the
    /// surface-composition family at one hundred and twenty-eight, the
    /// template-construction family at ninety-six — so a body this set cannot
    /// enumerate is a case the road MEETS rather than one the magnitudes rule
    /// out. It overruns at the boundary too, by exactly one: the body's own
    /// identity sits ahead of the per-issue ones, so a body AT this magnitude
    /// needs one seat more than the set has.
    ///
    /// Where that happens the body's own identity is carried alone — a coarser
    /// commitment to the same refusal, never a shorter commitment to a different
    /// one — and the completion beside it states `ReportTruncated` with the
    /// count this road dropped.
    /// The bound is named here rather than taken as a parameter: a caller naming
    /// the bound would be a caller labelling somebody else's act.
    ///
    /// Handed no issues, this road answers with
    /// [`RelatedSet::nothing_enumerated`] — the same value the single-cause road
    /// answers with, not a second value that means the same thing.
    /// Deriving instead would carry a body identity over empty material and call
    /// the result `Complete`, so "nothing was enumerated" would have two
    /// distinguishable representations and a reader comparing two diagnostics
    /// that enumerated nothing would find them unequal.
    /// It routes rather than refuses because the one seam that reaches this road holds a [`macroonz::NonEmptyBounded`] carry and cannot produce the case at all, so an error branch here would have no honest value to fill it.
    #[must_use]
    pub fn derived_over(family: u8, issues: &[Vec<u8>]) -> Self {
        if issues.is_empty() {
            return Self::nothing_enumerated();
        }
        let mut body_material = Vec::new();
        let mut per_issue = Vec::with_capacity(issues.len());
        for issue in issues {
            per_issue.push(RelatedIdentity::Issue(related_issue_identity(
                family, issue,
            )));
            encode_bytes(issue, &mut body_material);
        }
        let body = RelatedIdentity::Body(related_body_identity(family, &body_material));

        let mut all = Vec::with_capacity(per_issue.len().saturating_add(1));
        all.push(body);
        all.append(&mut per_issue);
        match Bounded::admitted_const(
            all,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        ) {
            Ok(carried) => Self {
                carried,
                completion: RelatedSetCompletion::Complete,
            },
            Err(BoundedConstruction::OverLimit) => Self {
                carried: Bounded::from_array([body]),
                completion: match NonZeroUsize::new(issues.len()) {
                    None => RelatedSetCompletion::Complete,
                    Some(omitted) => RelatedSetCompletion::ReportTruncated(RelatedSetTruncation {
                        stopped_at: StopBound::DeclaredIssueBound,
                        omitted,
                    }),
                },
            },
        }
    }

    /// The canonical empty relation: the set a road that enumerated nothing
    /// amounts to, and the only value in the plane that means it.
    ///
    /// A single-cause road establishes one cause and enumerates nothing, so
    /// there is no per-issue set to stop short of: zero identities are carried
    /// and zero are omitted.
    /// Total, and `Complete` by shape — there is no material it could have
    /// dropped.
    ///
    /// # Nonclaims
    ///
    /// Emptiness here is a stated posture about an act that ran: the road
    /// looked, and there was nothing to enumerate.
    /// It is not an absent set, not a set that failed to build, and not a
    /// truncation that dropped everything — a truncation names a bound and a
    /// non-zero count, and this names neither.
    #[must_use]
    pub fn nothing_enumerated() -> Self {
        Self {
            carried: Bounded::empty(),
            completion: RelatedSetCompletion::Complete,
        }
    }

    /// The identities the set carries.
    ///
    /// Borrowed and never owned: an owned set is half of the pair this type
    /// exists to keep together, and a caller holding it could seat it under
    /// another diagnostic's completion.
    #[must_use]
    pub const fn carried(&self) -> &Bounded<RelatedIdentity, RelatedIssueLimit> {
        &self.carried
    }

    /// Whether that set names every established issue.
    #[must_use]
    pub const fn completion(&self) -> RelatedSetCompletion {
        self.completion
    }
}

impl DiagnosticSite {
    /// The site of an observation about one token of a captured declaration.
    ///
    /// The handle is the load-bearing half: the producer resolves it to the
    /// exact compiler span, and the services never do.
    /// The coordinate beside it is whatever that producer's table answered,
    /// including the typed statement that the table does not reach the handle —
    /// which is a fact about the TABLE and leaves the observation itself
    /// standing.
    pub const fn at_token(token: SpanHandle, coordinate: SiteCoordinate) -> Self {
        Self::AtToken { token, coordinate }
    }

    /// The site of an observation established before any capture existed to
    /// issue a handle: the byte the read was born at, and no handle at all.
    ///
    /// It takes no [`SpanHandle`], which is the whole point of the road: there
    /// is no seat here for a caller to fill with handle zero, and no branch in
    /// which one is invented.
    pub const fn before_capture(coordinate: SourceCoordinate) -> Self {
        Self::BeforeCapture { coordinate }
    }

    /// The token this diagnostic points at, where a capture issued one.
    ///
    /// # Nonclaims
    ///
    /// It answers with nothing for a site established BEFORE a capture, because
    /// no table was built and no handle was issued.
    /// That is a stated posture rather than a missing value: a handle answered
    /// here would index a table that never existed and would read exactly like
    /// an honest handle naming the declaration's first token.
    /// [`DiagnosticSite::coordinate`] is the seat that carries the whole of what
    /// is known about such an observation.
    #[must_use]
    pub const fn token(self) -> Option<SpanHandle> {
        match self {
            Self::AtToken { token, .. } => Some(token),
            Self::BeforeCapture { .. } => None,
        }
    }

    /// Where this diagnostic sits, whichever posture it stands under.
    ///
    /// The ONE place a pre-capture byte is lifted into the answered posture, and
    /// it lifts to [`SiteCoordinate::Resolved`] honestly: nothing was resolved
    /// because nothing needed resolving, and the coordinate's own role says
    /// which text the position counts into.
    /// A seam that matched the arms itself would be making that same statement
    /// again, somewhere it could be made differently.
    #[must_use]
    pub const fn coordinate(self) -> SiteCoordinate {
        match self {
            Self::AtToken { coordinate, .. } => coordinate,
            Self::BeforeCapture { coordinate } => SiteCoordinate::Resolved(coordinate),
        }
    }
}

//! The diagnostics home's invariant nucleus: the one road that reaches the one
//! seat a caller may not write, and the one place a related set's identities are
//! derived.
//!
//! Declared inside `types.rs` as its own child, so the truncation's bound and
//! count are reachable here and nowhere else. Everything else in this home is a
//! plain readable seat, and this file exists for the single fact that is about an
//! ACT rather than about a value: how many per-issue identities a set-building
//! road left behind.
//!
//! The road below takes the issue MATERIAL — not a number, and not identities
//! somebody else already derived — and it BUILDS the set rather than being handed
//! one. That is the whole discipline, and it closes at two levels. The count is
//! read off what the road itself dropped and lands in the same value as what it
//! kept, so the posture and the set are two readings of one act, and a set that
//! dropped nothing has nothing to read a count off. Both identity levels are
//! derived here out of that one material, so the body's identity and the
//! per-issue identities are two readings of one body: there is no loose half for
//! a caller to hold, and therefore no way to seat one refusal's coarse
//! commitment over another refusal's issues.

use super::{RelatedIdentity, RelatedSet, RelatedSetCompletion, RelatedSetTruncation};
use crate::plane::{
    AuthoringLimitProfile, ProjectionIdentity, ProjectionRole, ProjectionTranscript,
    RelatedBodySubject, RelatedIssueLimit, RelatedIssueSubject, encode_bytes,
};
use core::num::NonZeroUsize;
use threadpak::refusal::StopBound;
use threadpak::types::{AdmittedLimit, Bounded, BoundedConstruction};

/// The content one related identity is derived over, at either level.
///
/// The family tag separates two families' spaces so the same bytes raised under
/// two families never encode alike, and the material is length-framed behind it
/// so no two materials share a preimage. It is composed here and used at both
/// levels deliberately: what separates the two levels is the SUBJECT, which is a
/// segment of the derive-key context, not a discriminant somebody could forget
/// to write into a preimage.
fn related_content(family: u8, material: &[u8]) -> Vec<u8> {
    let mut content = vec![family];
    encode_bytes(material, &mut content);
    content
}

/// One related-ISSUE identity over one established issue's material.
///
/// The single derivation for this subject, and it is private on purpose: an
/// identity of this subject exists only as part of a set this file built. The
/// role is the closed expansion the services were producing when the
/// disagreement was observed.
fn related_issue_identity(family: u8, material: &[u8]) -> ProjectionIdentity<RelatedIssueSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::ClosedExpansion,
        &related_content(family, material),
        u32::from(family),
    ))
}

/// One related-BODY identity over the framing of a whole body's issues.
///
/// The same private discipline and the same role, under the OTHER subject. That
/// is the whole of the level separation and it is deliberately not a byte inside
/// the preimage: the subject rides in the derive-key context AND in the
/// transcript, so two levels over identical content are separated before a byte
/// of that content is read and disagree inside it as well.
fn related_body_identity(family: u8, material: &[u8]) -> ProjectionIdentity<RelatedBodySubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::ClosedExpansion,
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
    /// shape — a truncation that dropped nothing is
    /// [`RelatedSetCompletion::Complete`] and is unrepresentable here.
    #[must_use]
    pub const fn omitted(self) -> NonZeroUsize {
        self.omitted
    }
}

impl RelatedSet {
    /// The related set one refusal body amounts to, derived over that body's own
    /// issue material: the whole body's identity first, then one per established
    /// issue, and the posture that says whether that is all of them.
    ///
    /// # One material in, both identity levels out
    ///
    /// The road is handed the issues' canonical material and derives both levels
    /// itself. A road taking a body identity and a set of per-issue identities as
    /// two arguments takes two halves that do not check each other: each half is
    /// honestly derived on its own, so the pair can name one refusal's body over
    /// another refusal's issues and still read exactly like a set that belongs
    /// together. Deriving here removes the pairing instead of policing it, because
    /// there is no caller-held half left to mispair.
    ///
    /// The per-issue identities are derived first, each over one issue's framed
    /// material, and the body's identity is derived over the framing of exactly
    /// that material in exactly that order. The body's preimage therefore IS the
    /// issues: two different issue sets cannot reach one body identity, and one
    /// issue set cannot reach two. That is what makes the coarser commitment
    /// carried alone under truncation a commitment to THESE issues rather than a
    /// word about issues in general.
    ///
    /// # The two levels are two subjects, and the content grammar is published
    ///
    /// Because the body's preimage is the framing of its issues, one namespace
    /// over both levels collided by construction: an issue whose own material
    /// happened to be that framing derived the body's exact identity. The levels
    /// are therefore two subjects — `related-body` and `related-issue` — so the
    /// same content at the two levels derives under two derive-key contexts and
    /// is two unrelated values.
    ///
    /// This is a mint site, so its CONTENT grammar is stated here in full, the
    /// way [`crate::plane::ProjectionTranscript`] requires of every mint site. Both
    /// levels derive at role `closed-expansion`, rooted, at roster position
    /// `family`, over
    ///
    /// ```text
    /// content = family_byte || u64be(material.len()) || material
    /// ```
    ///
    /// where the material of an ISSUE is that issue's own canonical bytes, and
    /// the material of the BODY is `u64be(issue.len()) || issue` for every issue
    /// in order, concatenated. An independent reader holding the issues and this
    /// paragraph re-derives both levels and needs nothing else.
    ///
    /// # The magnitude, the posture, and the count
    ///
    /// [`RelatedIssueLimit`] is declared at the widest refusal-body magnitude in
    /// the plane, so a body built through the typed seams always fits — but the
    /// widest body and the set are the same width, and the body's own identity
    /// sits ahead of the per-issue ones, so a body AT the magnitude overruns by
    /// exactly one.
    ///
    /// Where that happens the body's own identity is carried alone — a coarser
    /// commitment to the same refusal, never a shorter commitment to a different
    /// one — and the completion beside it states `ReportTruncated` with the count
    /// this road dropped. Carrying the coarser set silently is the defect: it has
    /// the shape of a complete answer, and the reader has nothing to compare it
    /// against.
    ///
    /// The bound is named here rather than taken as a parameter, because the
    /// magnitude this road stops at is the declared related-issue magnitude and
    /// nothing else. A caller naming the bound would be a caller labelling
    /// somebody else's act.
    ///
    /// The posture is spelled for truncation rather than for an early stop, on
    /// band 00's distinction: the refusal body is complete before the set is
    /// built, so nothing here ever halts an examination.
    #[must_use]
    pub fn derived_over(family: u8, issues: &[Vec<u8>]) -> Self {
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

    /// The set a road that enumerated nothing amounts to.
    ///
    /// A single-cause road establishes one cause and enumerates nothing, so
    /// there is no per-issue set to stop short of: zero identities are carried
    /// and zero are omitted. Total, and `Complete` by shape — there is no
    /// material it could have dropped.
    #[must_use]
    pub fn nothing_enumerated() -> Self {
        Self {
            carried: Bounded::empty(),
            completion: RelatedSetCompletion::Complete,
        }
    }

    /// The identities the set carries.
    ///
    /// Borrowed and never owned. An owned set is half of the pair this type
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

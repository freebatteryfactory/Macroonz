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

use super::{RelatedSet, RelatedSetCompletion, RelatedSetTruncation};
use crate::plane::{
    AuthoringLimitProfile, ProjectionIdentity, ProjectionRole, ProjectionTranscript,
    RelatedIssueLimit, RelatedIssueSubject, encode_bytes,
};
use core::num::NonZeroUsize;
use threadpak::refusal::StopBound;
use threadpak::types::{AdmittedLimit, Bounded, BoundedConstruction};

/// One related-issue identity over one refusal family's material.
///
/// The single derivation for this subject, and it is private on purpose: an
/// identity of this subject exists only as part of a set this file built. The
/// family tag separates two families' issue spaces so the same bytes raised under
/// two families never encode alike, the material is length-framed behind it so no
/// two materials share a preimage, and the role is the closed expansion the
/// services were producing when the disagreement was observed.
fn related_identity(family: u8, material: &[u8]) -> ProjectionIdentity<RelatedIssueSubject> {
    let mut content = vec![family];
    encode_bytes(material, &mut content);
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::ClosedExpansion,
        &content,
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
            per_issue.push(related_identity(family, issue));
            encode_bytes(issue, &mut body_material);
        }
        let body = related_identity(family, &body_material);

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
    pub const fn carried(
        &self,
    ) -> &Bounded<ProjectionIdentity<RelatedIssueSubject>, RelatedIssueLimit> {
        &self.carried
    }

    /// Whether that set names every established issue.
    #[must_use]
    pub const fn completion(&self) -> RelatedSetCompletion {
        self.completion
    }
}

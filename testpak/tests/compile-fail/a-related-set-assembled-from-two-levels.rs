//! The reversal for the two identity LEVELS: a related set is built from issue
//! material, never from identities somebody already derived.
//!
//! A road taking a whole-body commitment and a set of per-issue identities as
//! two arguments takes two halves that do not check each other. Each half
//! derives honestly on its own, so the pair can name one refusal's body over
//! another refusal's issues and still read exactly like a set that belongs
//! together — the same shape as the carry-and-posture marriage one file over,
//! one level further down.
//!
//! The plane closes it by removing the pairing rather than policing it. The one
//! road in is handed the issue MATERIAL and derives BOTH levels itself, so
//! neither half is ever a caller's to hold. A caller may still mint plane
//! identities of its own — the identity seam is public and the subjects are
//! public — and that is exactly what makes this fixture the honest test: holding
//! both levels is allowed, and seating them is not expressible, because the only
//! entry point takes bytes that were established as issues rather than names
//! somebody chose.
//!
//! The fixture stays on that one shape on purpose. A second attempt failing
//! earlier would swallow this error and leave the record attesting something
//! else.

use threadpak_macroc::RelatedSet;
use threadpak_macroc::plane::{
    ProjectionIdentity, ProjectionRole, ProjectionTranscript, RelatedIssueSubject,
};

fn main() {
    // The per-issue level, minted by the caller through the public seam. This
    // much is lawful: a plane identity of a public subject is a value anybody
    // may derive.
    let issues = vec![ProjectionIdentity::<RelatedIssueSubject>::derived(
        ProjectionTranscript::rooted(ProjectionRole::ClosedExpansion, b"an issue", 1),
    )];

    // Supplying that level as an input to the set. The one road in takes the
    // issues' MATERIAL and derives both levels from it, so there is no seat
    // here for an identity a caller chose.
    let _set = RelatedSet::derived_over(1_u8, &issues);
}

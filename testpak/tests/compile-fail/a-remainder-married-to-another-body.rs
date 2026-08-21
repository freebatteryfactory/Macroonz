//! The reversal for the marriage: a completion belongs to the body it was
//! minted with, and to no other.
//!
//! Provenance alone was never enough. A count minted by the one road that
//! truncates is a count some truncation really performed — but a road handing
//! the carry and the count back as two values hands a caller two things it may
//! pair freely, so the body one pass truncated could be reported under the count
//! another pass dropped. Both halves stay individually honest and the pair is a
//! lie, which is the kind no runtime check catches: there is nothing wrong to
//! detect at either end.
//!
//! So the two leave the road married inside one `AdmittedPrefix` and the seats
//! are private. What is left to a caller wanting the cross-wire is the struct
//! literal below, and writing the seats is not the caller's to do.
//!
//! The fixture stays on that one shape on purpose. Privacy is checked after type
//! checking, so a second attempt failing earlier would swallow this error and
//! leave the record attesting something else.
//!
//! # What this file establishes, exactly
//!
//! REPRESENTATION PRIVACY: the two seats are not a caller's to write, so the
//! cross-wired pair is not a value that can be assembled. It does NOT establish
//! that no road back out to a loose pair exists. An `into_parts` handing both
//! halves over, or a second mint taking a carry and a posture, would leave this
//! error exactly where it is — the seats stay private either way — and the
//! sentence this header used to carry would have gone on reading as discharged.
//!
//! That absence is not derived anywhere, and the reason is stated rather than
//! left as an omission. Nothing in the tree separates a package sealed by
//! intent from a declaration record beside it that is transparent by intent:
//! `CauseId` also has two private seats, and it hands both back on purpose,
//! through `family()` and `local()`, beside a public mint that takes them. A
//! reader that condemned one would condemn the other. The stamped scope guards
//! escape that because the stamp is a DECLARATION of the seal, machine-readable,
//! which is exactly what this package does not have.

use threadpak::refusal::{AdmittedPrefix, StopBound};
use threadpak::types::{
    ConstLimit, DeclaredMagnitude, Limit, LimitAdmissionProfile, PositiveLimit,
};

/// This file's own plane, declared here because this file is the plane
/// declaring it.
struct FixtureProfile;

impl LimitAdmissionProfile for FixtureProfile {
    const MAX_DECLARED_LIMIT: usize = 64;
}

/// A family that holds three items. It derives what every declared limit family
/// derives, so nothing below fails for a reason other than the one this fixture
/// is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct IssueFamily;

impl Limit for IssueFamily {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for IssueFamily {
    const MAX: usize = 3;
}

fn main() {
    let admitted: PositiveLimit<IssueFamily, FixtureProfile> =
        PositiveLimit::inhabited_under_profile();

    // Two passes, two truncations. Each report is honest about itself: the
    // first dropped four issues, the second dropped none.
    let dropped_four = AdmittedPrefix::examined_completely(
        1_u8,
        vec![2, 3, 4, 5, 6, 7],
        &admitted,
        StopBound::DeclaredIssueBound,
    );
    let dropped_none =
        AdmittedPrefix::examined_completely(8_u8, vec![9], &admitted, StopBound::DeclaredIssueBound);

    // The marriage: the second body's carry wearing the first body's
    // completion. The seats are not the caller's to write.
    let _crossed = AdmittedPrefix {
        carried: dropped_none.carried().clone(),
        completion: dropped_four.completion(),
    };
}

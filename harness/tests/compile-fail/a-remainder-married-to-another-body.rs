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
//! The two private seats prevent an outside caller from assembling a cross-wired pair. This fixture does not claim that no public road can return the parts or mint an informed value.
//!
//! `CauseId` demonstrates that private representation does not imply opacity because it deliberately exposes accessors. Without an explicit closed declaration, representation privacy is the strongest claim this fixture establishes.

use macroonz::{AdmittedPrefix, StopBound};
use macroonz::{
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

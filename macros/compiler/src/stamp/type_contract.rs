//! The stamp home's declarative surface: the tables this home states rather than computes, and the contracts a refusal stands under.
//!
//! The visibility transport is stated once as a total answer over the closed reach roster.
//! A stamped item sits one module deeper than the coordinate the site wrote its reach at, so a reach copied straight through would publish it to the site's own parent, and a reach widened by guesswork would publish it further than that.

use super::{StampError, TransportedReach, Visibility};
use crate::bounded::Overflow;

impl Visibility {
    /// The reach a stamped item carries inside the module a pattern seats it in.
    ///
    /// This is the one place the transport is decided, so the front arm a definition renders and the reach the stamped item wears are one answer rather than two that agree until one is edited.
    /// A constant answer over a closed roster, so a sixth reach admitted later stops the compiler here until somebody says what it becomes one level in.
    ///
    /// # Bounds
    ///
    /// The two private reaches name the same scope and transport to the same one; the parent-facing reach gains a segment; the crate-facing and public reaches are absolute and the extra module does not move them.
    #[must_use]
    pub const fn transported(self) -> TransportedReach {
        match self {
            Self::Private | Self::Module => TransportedReach::Enclosing,
            Self::Parent => TransportedReach::Ancestor,
            Self::Crate => TransportedReach::Crate,
            Self::Public => TransportedReach::Public,
        }
    }
}

/// Every overflow a rendering road meets is a tree that outgrew the declared token magnitude.
///
/// The nonclaim is stated here: this is the only overflow the roads in this home can raise, because the only bounded collection they build is a token group.
impl From<Overflow> for StampError {
    fn from(overflow: Overflow) -> Self {
        Self::TokensUnbounded { overflow }
    }
}

impl core::fmt::Display for StampError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAnIdentifier => write!(
                into,
                "a spelling this stamp writes as a token is not one Rust identifier"
            ),
            Self::PathEmpty => write!(into, "a spelled path names no segment, so it names nothing"),
            Self::PathUnbounded { overflow } => write!(
                into,
                "a spelled path carries {} segments where at most {} fit",
                overflow.offered, overflow.capacity
            ),
            Self::PatternEmpty => {
                write!(into, "a pattern declares no part, so it declares no shape")
            }
            Self::PatternUnbounded { overflow } => write!(
                into,
                "a pattern declares {} parts where at most {} fit",
                overflow.offered, overflow.capacity
            ),
            Self::SeatNameDoubled { at } => write!(
                into,
                "the seat at part {at} carries a name an earlier seat already binds"
            ),
            Self::SitesAbsent => write!(
                into,
                "a stamp covers no site, and a definition nobody invokes has no reader"
            ),
            Self::SitesUnbounded { overflow } => write!(
                into,
                "a stamp covers {} sites where at most {} fit",
                overflow.offered, overflow.capacity
            ),
            Self::SiteNameDoubled { at } => write!(
                into,
                "the site at position {at} carries a name an earlier site already has"
            ),
            Self::ArgumentsUnbounded { overflow } => write!(
                into,
                "a site carries {} arguments where at most {} fit",
                overflow.offered, overflow.capacity
            ),
            Self::ArgumentsUnmatched {
                at,
                seats,
                supplied,
            } => write!(
                into,
                "the site at position {at} supplies {supplied} arguments for {seats} declared seats"
            ),
            Self::ReachUnseated { at } => write!(
                into,
                "the site at position {at} declares a reach its pattern gives no coordinate to"
            ),
            Self::SeatNotPlanned { role_slot } => write!(
                into,
                "the plan declares no member under the seat at roster position {role_slot}"
            ),
            Self::DestinationNotArtifact { role_slot } => write!(
                into,
                "the member at roster position {role_slot} lands somewhere other than an artifact"
            ),
            Self::TokensUnbounded { overflow } => write!(
                into,
                "a rendered tree carries {} tokens where at most {} fit",
                overflow.offered, overflow.capacity
            ),
        }
    }
}

impl core::error::Error for StampError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::PathUnbounded { overflow }
            | Self::PatternUnbounded { overflow }
            | Self::SitesUnbounded { overflow }
            | Self::ArgumentsUnbounded { overflow }
            | Self::TokensUnbounded { overflow } => Some(overflow),
            Self::NotAnIdentifier
            | Self::PathEmpty
            | Self::PatternEmpty
            | Self::SeatNameDoubled { .. }
            | Self::SitesAbsent
            | Self::SiteNameDoubled { .. }
            | Self::ArgumentsUnmatched { .. }
            | Self::ReachUnseated { .. }
            | Self::SeatNotPlanned { .. }
            | Self::DestinationNotArtifact { .. } => None,
        }
    }
}

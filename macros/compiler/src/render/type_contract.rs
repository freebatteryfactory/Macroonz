//! The constant answers this home's refusal roster settles, and the contracts a rendering refusal stands under.
//!
//! Each table is total, so a row admitted later stops the compiler in every one of them until somebody says what that row's position, sentence, and classification are.

use super::RenderError;
use crate::bounded::{Bounded, Overflow};
use crate::diagnostic::{
    Family, LineBody, Observed, Phase, RENDERING_FAMILY, REPAIR_LIMIT, RefusalClass, Refused,
    RenderedMagnitude, Repair,
};
use core::fmt;

impl RenderError {
    /// This row's position in the declared roster, written ahead of the refusal's own material.
    ///
    /// Appended and never renumbered: the byte stands inside every related identity derived over a refusal that carries it.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::NothingRendered => 1,
            Self::SeatUnplanned { .. } => 2,
            Self::BytesUnbounded { .. } => 3,
            Self::UnitsUnbounded { .. } => 4,
            Self::TokensUnbounded { .. } => 5,
        }
    }
}

impl fmt::Display for RenderError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NothingRendered => into.write_str("the renderer materialized no unit at all"),
            Self::SeatUnplanned { role } => write!(
                into,
                "a unit was rendered under the seat {role}, which this plan declares no member for"
            ),
            Self::BytesUnbounded {
                role,
                bound,
                observed,
            } => write!(
                into,
                "the unit rendered under the seat {role} passed {}: {observed} offered where {bound} are declared",
                RenderedMagnitude::RenderedBytes.described()
            ),
            Self::UnitsUnbounded { bound, observed } => write!(
                into,
                "the rendering passed {}: {observed} offered where {bound} are declared",
                RenderedMagnitude::RenderedUnits.described()
            ),
            Self::TokensUnbounded { bound, observed } => write!(
                into,
                "a generated tree passed {}: {observed} offered where {bound} are declared",
                RenderedMagnitude::GeneratedTokens.described()
            ),
        }
    }
}

impl core::error::Error for RenderError {}

impl From<Overflow> for RenderError {
    /// The refusal a generated tree that outgrew its per-level magnitude makes.
    ///
    /// The one overflow a renderer meets, so `?` carries a composition helper's answer straight out of a renderer body.
    fn from(overflow: Overflow) -> Self {
        Self::TokensUnbounded {
            bound: overflow.capacity,
            observed: overflow.offered,
        }
    }
}

impl Refused for RenderError {
    const PHASE: Phase = Phase::Rendering;
    const FAMILY: Family = RENDERING_FAMILY;

    fn class(&self) -> RefusalClass {
        match self {
            Self::NothingRendered | Self::SeatUnplanned { .. } => {
                RefusalClass::RenderingNotProduced
            }
            Self::BytesUnbounded { .. }
            | Self::UnitsUnbounded { .. }
            | Self::TokensUnbounded { .. } => RefusalClass::MagnitudeNotHeld,
        }
    }

    fn first(&self) -> String {
        self.to_string()
    }

    fn observed(&self) -> Observed {
        match self {
            Self::NothingRendered => Observed::SeatAbsent,
            Self::SeatUnplanned { .. } => Observed::ContractDisagreement,
            Self::BytesUnbounded { .. }
            | Self::UnitsUnbounded { .. }
            | Self::TokensUnbounded { .. } => Observed::BoundExceeded,
        }
    }

    /// Rendering establishes one cause and enumerates nothing.
    ///
    /// A unit that cannot be materialized is not a unit, and the units after it were never written, so there is no remainder for a line to count.
    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    /// A single cause enumerates nothing: the primary cause is the summary's own subject, never a member of its related set.
    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    /// This home declares no repair of its own.
    ///
    /// Every row above is about the renderer the caller wrote or the plan the caller declared, so the repair is one of those two; a sentence composed here would be this compiler citing a fact nobody declared.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}

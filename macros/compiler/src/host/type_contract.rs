//! The constant answer this home's one roster settles, the contracts a capture refusal stands under, and what an expansion delivers to be emitted.

use super::types::{CaptureError, Emittable};
use crate::bounded::Bounded;
use crate::closure::PartitionCargo;
use crate::diagnostic::{
    CAPTURE_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
};
use crate::expansion::Expansion;
use crate::kind::Kind;
use crate::token::CaptureBound;
use core::fmt;

impl CaptureError {
    /// This row's position in the declared roster, written ahead of the material it carries.
    ///
    /// Appended and never renumbered: the byte stands inside every related identity derived over a refused capture.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Unbounded { .. } => 0,
            Self::Unread { .. } => 1,
        }
    }
}

impl From<CaptureBound> for CaptureError {
    fn from(bound: CaptureBound) -> Self {
        Self::Unbounded { bound }
    }
}

impl fmt::Display for CaptureError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unbounded { bound } => write!(into, "{bound}"),
            Self::Unread { cause, .. } => write!(into, "{cause}"),
        }
    }
}

impl core::error::Error for CaptureError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Unbounded { bound } => Some(bound),
            Self::Unread { cause, .. } => Some(cause),
        }
    }
}

impl Refused for CaptureError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = CAPTURE_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::DeclarationNotRead
    }

    fn first(&self) -> String {
        self.to_string()
    }

    fn observed(&self) -> Observed {
        match self {
            Self::Unbounded { .. } => Observed::BoundExceeded,
            Self::Unread { .. } => Observed::ContractDisagreement,
        }
    }

    /// One cause, always: a capture stops at the first thing it cannot read, so nothing co-establishes behind it.
    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    /// A single cause enumerates nothing: the primary cause is the summary's own subject, never a member of its related set.
    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    /// No repair is cited: both rows are about what the declared input carries, so the repair is that declaration, and a sentence composed here would be this compiler citing a fact nobody declared.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}

impl<K: Kind> Emittable for Expansion<K> {
    /// One cargo: an expansion carries exactly one declaration-site delivery, the one its own closure proved.
    fn cargos(&self) -> impl Iterator<Item = &PartitionCargo> {
        core::iter::once(self.emit())
    }
}

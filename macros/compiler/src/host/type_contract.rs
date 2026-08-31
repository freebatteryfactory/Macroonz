//! The constant answer this home's one roster settles, the contracts a capture refusal stands under, and what an expansion delivers to be emitted.

use super::types::{CaptureError, EmissionError, Emittable};
use crate::closure::PartitionCargo;
use crate::expansion::Expansion;
use crate::kind::Kind;
use crate::token::CaptureBound;
use core::fmt;

impl CaptureError {
    /// This row's position in the declared roster, written ahead of the material it carries.
    ///
    /// Appended and never renumbered: the byte stands inside every related identity derived over a refused capture.
    #[must_use]
    pub const fn slot(&self) -> u8 {
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

impl fmt::Display for EmissionError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NumberRejected { spelling } => write!(
                into,
                "the compiler host rejected the admitted numeric literal `{spelling}`"
            ),
            Self::NulTerminatedTextRejected => into
                .write_str("the compiler host rejected admitted NUL-terminated literal material"),
        }
    }
}

impl core::error::Error for EmissionError {}

impl<K: Kind> Emittable for Expansion<K> {
    /// One cargo: an expansion carries exactly one declaration-site delivery, the one its own closure proved.
    fn cargos(&self) -> impl Iterator<Item = &PartitionCargo> {
        core::iter::once(self.emit())
    }
}

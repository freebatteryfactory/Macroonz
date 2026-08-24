//! The canonical bytes one refused capture is named by.

use super::types::CaptureError;
use crate::identity::encode_bytes;

impl CaptureError {
    /// This refusal's complete canonical material: which row it is, the stable name of what stopped the read, and — for a refusal about one token — that token's position in reading order.
    ///
    /// The row's position leads, so two rows whose names happened to coincide still derive two related identities.
    /// The position is a fact about the declaration rather than about a producer: every producer issues handles in reading order, so two captures of one declaration name the same token by the same number.
    #[must_use]
    pub fn canonical_bytes(self) -> Vec<u8> {
        let mut material = vec![self.slot()];
        match self {
            Self::Unbounded { bound } => encode_bytes(bound.name().as_bytes(), &mut material),
            Self::Unread { cause, at } => {
                encode_bytes(cause.name().as_bytes(), &mut material);
                material.extend_from_slice(&at.index().to_be_bytes());
            }
        }
        material
    }
}

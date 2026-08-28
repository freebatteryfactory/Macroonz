//! The canonical bytes one refused capture is named by.

use super::types::CaptureError;
use crate::identity::encode_bytes;
use crate::token::encode_token_path;

impl CaptureError {
    /// This refusal's complete canonical material: which row it is, the stable name of what stopped the read, and — for a refusal about one token — that token's declaration-local path.
    ///
    /// The row's position leads, so two rows whose names happened to coincide still derive two related identities.
    /// The producer-local span handle is excluded: captures of one declaration may issue different handles when they share a span table, while their declaration-local paths remain one fact.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut material = vec![self.slot()];
        match self {
            Self::Unbounded { bound } => encode_bytes(bound.name().as_bytes(), &mut material),
            Self::Unread { cause, path, at: _ } => {
                encode_bytes(cause.name().as_bytes(), &mut material);
                encode_token_path(path, &mut material);
            }
        }
        material
    }
}

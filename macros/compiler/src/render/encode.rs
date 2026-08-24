//! The canonical bytes one rendering refusal is.
//!
//! The row's position rides ahead of the material it governs, and the material is framed through the identity home's one framing, so two rows carrying the same counts never encode alike.
//!
//! A rendered unit has no whole-value encoding here, and the absence is the no-double-entry law: what a unit IS reaches a preimage through its own identity and its digest, both derived over the tree's bytes at full width, and what it ANSWERS TO reaches one through the planned member it reconstructs, which the plan home already spells.

use super::RenderError;
use crate::identity::encode_bytes;

impl RenderError {
    /// This refusal's canonical bytes on their own, for the related identity a diagnostic derives over it.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }

    /// Appends this refusal's canonical bytes: the row's position in the declared roster, then the typed material that row carries, framed.
    ///
    /// Exhaustive over the roster on purpose: a row added to [`RenderError`] stops compiling HERE until somebody says what of it a preimage commits to.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let mut material = Vec::new();
        self.material_into(&mut material);
        encode_bytes(&material, into);
    }

    /// The typed material one refusal carries.
    ///
    /// The two magnitude rows that carry only counts share this spelling and are separated by the row position written ahead of them.
    fn material_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::NothingRendered => {}
            Self::SeatUnplanned { role } => encode_bytes(role.as_bytes(), into),
            Self::BytesUnbounded {
                role,
                bound,
                observed,
            } => {
                encode_bytes(role.as_bytes(), into);
                counted_into(*bound, into);
                counted_into(*observed, into);
            }
            Self::UnitsUnbounded { bound, observed }
            | Self::TokensUnbounded { bound, observed } => {
                counted_into(*bound, into);
                counted_into(*observed, into);
            }
        }
    }
}

/// Appends one count as eight big-endian bytes, saturating where a count outruns that width.
fn counted_into(value: usize, into: &mut Vec<u8>) {
    into.extend_from_slice(&u64::try_from(value).unwrap_or(u64::MAX).to_be_bytes());
}

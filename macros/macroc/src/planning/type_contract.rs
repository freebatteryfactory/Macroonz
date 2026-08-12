//! The plan family's declarative trait implementations.
//!
//! The kind roster's own [`ProjectionKind`] implementations are written by the
//! `kinds!` declaration in `types.rs`, because a kind's contract is the
//! declaration rather than something added to it afterwards. What stands here is
//! the one roster a kind names rather than declares: the two rendered roles an
//! implementation projection materializes, stated as a constant roster, a
//! constant slot, and a constant sentence.
//!
//! [`ProjectionKind`]: super::ProjectionKind

use super::RenderedImplementation;
use crate::plane::{RenderedRole, RenderedRoleSeal};

impl RenderedRole for RenderedImplementation {
    const SEAL: RenderedRoleSeal = RenderedRoleSeal::admitted();
    const ROLES: &'static [Self] = &[Self::RenderedFamilyImpl, Self::RenderedCauseOrderImpl];

    fn slot(self) -> u32 {
        match self {
            Self::RenderedFamilyImpl => 0,
            Self::RenderedCauseOrderImpl => 1,
        }
    }

    fn described(self) -> &'static str {
        match self {
            Self::RenderedFamilyImpl => "the family contract's implementation",
            Self::RenderedCauseOrderImpl => "the typed cause order's implementation",
        }
    }
}

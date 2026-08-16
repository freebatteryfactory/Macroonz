//! The plane's declarative trait implementations: the sole-unit rendered role.
//!
//! A constant roster, a constant slot, and a constant sentence — nothing here is
//! computed.

use super::{RenderedRole, RenderedRoleSeal, SoleRenderedUnit};

impl RenderedRole for SoleRenderedUnit {
    const SEAL: RenderedRoleSeal = RenderedRoleSeal::admitted();
    const ROLES: &'static [Self] = &[Self::Sole];

    fn slot(self) -> u32 {
        0
    }

    fn described(self) -> &'static str {
        "the kind's one rendered unit"
    }
}

//! The plane's declarative trait implementations.
//!
//! One roster stands here: the sole-unit rendered role. It is declarative in the
//! strict sense — a constant roster, a constant slot, and a constant sentence —
//! so it is stated as a contract rather than computed by anything.

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

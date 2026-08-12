//! The derive home's declarative surface: the closed table the declared output
//! set is read through.
//!
//! The home's two principal trait implementations — the capture family's shape
//! and selection order, and its typed cause order — are written by the
//! `capture_causes!` declaration in `types.rs`, because a cause is stated once
//! there and both contracts follow from that one statement. Moving them here
//! would make one cause two declarations in two files.
//!
//! What stands here is the other declaration in method form: which rendered
//! roles each membership answer names, stated as a constant table rather than
//! counted or discovered.

use super::DerivedMembership;
use crate::planning::RenderedImplementation;

impl DerivedMembership {
    /// The rendered roles this membership declares, in roster order.
    #[must_use]
    pub const fn roles(self) -> &'static [RenderedImplementation] {
        match self {
            Self::FamilyOnly => &[RenderedImplementation::RenderedFamilyImpl],
            Self::FamilyAndCauseOrder => &[
                RenderedImplementation::RenderedFamilyImpl,
                RenderedImplementation::RenderedCauseOrderImpl,
            ],
        }
    }

    /// The number of declared roles; structurally at least one.
    #[must_use]
    pub const fn len(self) -> usize {
        self.roles().len()
    }

    /// Always `false`: an empty declared output set is unrepresentable here.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        false
    }
}

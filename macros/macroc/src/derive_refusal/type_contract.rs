//! The derive home's declarative surface: the closed table the declared output
//! set is read through.
//!
//! The home's two principal trait implementations — the capture family's shape
//! and selection order, and its typed cause order — are written by the
//! `capture_causes!` declaration in `types.rs`, because a cause is stated once
//! there and both contracts follow from that one statement.
//!
//! What stands here is the other declaration in method form: which rendered
//! roles each membership answer names, stated as a constant table rather than
//! counted or discovered.
//!
//! The table names every production contract and, where declared, the one
//! generated mutation module delivered to TestPak.

use super::DerivedMembership;
use crate::planning::RenderedImplementation;

impl DerivedMembership {
    /// The rendered roles this membership declares, in roster order.
    ///
    /// # Ordering
    ///
    /// Roster order — the order
    /// [`RenderedRole::ROLES`](crate::plane::RenderedRole::ROLES) declares — so
    /// the production seats stand ahead of the generated mutation module. It
    /// ranks nothing: the closure matches role for role.
    #[must_use]
    pub const fn roles(self) -> &'static [RenderedImplementation] {
        match self {
            Self::FamilyOnly => &[RenderedImplementation::RenderedFamilyImpl],
            Self::FamilyAndCauseOrder => &[
                RenderedImplementation::RenderedFamilyImpl,
                RenderedImplementation::RenderedCauseOrderImpl,
            ],
            Self::FamilyAndMutationEvaluation => &[
                RenderedImplementation::RenderedFamilyImpl,
                RenderedImplementation::RenderedMutationEvaluation,
            ],
            Self::FamilyCauseOrderAndMutationEvaluation => &[
                RenderedImplementation::RenderedFamilyImpl,
                RenderedImplementation::RenderedCauseOrderImpl,
                RenderedImplementation::RenderedMutationEvaluation,
            ],
        }
    }

    /// The number of declared roles; structurally at least one.
    #[must_use]
    pub const fn count(self) -> usize {
        self.roles().len()
    }
}

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
//! The table names TWO roles per contract, because one implementation meaning
//! is delivered as two surfaces. A membership answer states which CONTRACTS a
//! shape declares; the roles it names are the complete delivery those contracts
//! amount to, and that is a wider set than the contracts it is read off.

use super::DerivedMembership;
use crate::planning::RenderedImplementation;

impl DerivedMembership {
    /// The rendered roles this membership declares, in roster order.
    ///
    /// # Both surfaces, never the production half alone
    ///
    /// Every contract a membership answer names contributes TWO roles: the
    /// production implementation under its own role, and the mutation-evaluation
    /// copy under that role's twin ([`RenderedImplementation::twin`]). This table
    /// is the same set [`plan::membership`](super::plan::membership) declares and
    /// the same set the rendering materializes — so a table naming the production
    /// halves alone would say the declared output set is half the delivery, and
    /// the output firewall is exactly that the declared set IS the whole set.
    ///
    /// # Bounds
    ///
    /// The twins are written literally rather than read through
    /// [`RenderedImplementation::twin`], because a `'static` table admits no call;
    /// what stands here is that answer, spelled. A roster that paired its seats
    /// differently would disagree with this table, and the disagreement is a
    /// change to the roster rather than a state this road can be in.
    ///
    /// # Ordering
    ///
    /// Roster order — the order
    /// [`RenderedRole::ROLES`](crate::plane::RenderedRole::ROLES) declares — so
    /// the two production seats stand ahead of the two evaluation ones. It ranks
    /// nothing: the closure matches role for role, and a reader asking which
    /// surface is delivered first is asking a question no seat here answers.
    #[must_use]
    pub const fn roles(self) -> &'static [RenderedImplementation] {
        match self {
            Self::FamilyOnly => &[
                RenderedImplementation::RenderedFamilyImpl,
                RenderedImplementation::RenderedFamilyEvaluation,
            ],
            Self::FamilyAndCauseOrder => &[
                RenderedImplementation::RenderedFamilyImpl,
                RenderedImplementation::RenderedCauseOrderImpl,
                RenderedImplementation::RenderedFamilyEvaluation,
                RenderedImplementation::RenderedCauseOrderEvaluation,
            ],
        }
    }

    /// The number of declared roles; structurally at least two, because the
    /// smallest delivery this home admits is one contract's two surfaces.
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

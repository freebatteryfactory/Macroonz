//! The plan family's declarative trait implementations.
//!
//! The kind roster's own [`ProjectionKind`] implementations are written by the
//! `kinds!` declaration in `types.rs`, because a kind's contract is the
//! declaration rather than something added to it afterwards.
//! What stands here is the one roster a kind names rather than declares: the
//! rendered roles an implementation projection materializes, stated as a constant
//! roster, a constant slot, and a constant sentence — together with the three
//! facts that roster settles for every seat on it, each written as a constant
//! answer over the closed roster: which role is the other half of a role's PAIR,
//! which HALF of that pair a role is, and where a member under a role LANDS.
//!
//! Constant answers and never derivations: a fifth role admitted later stops the
//! compiler in each `match` below until somebody says which role it pairs with,
//! which half of that pair it is, and where it lands.
//!
//! [`ProjectionKind`]: super::ProjectionKind

use super::{MemberDestination, RenderedImplementation};
use crate::plane::{RenderedRole, RenderedRoleSeal};

impl RenderedRole for RenderedImplementation {
    const SEAL: RenderedRoleSeal = RenderedRoleSeal::admitted();
    const ROLES: &'static [Self] = &[
        Self::RenderedFamilyImpl,
        Self::RenderedCauseOrderImpl,
        Self::RenderedFamilyEvaluation,
        Self::RenderedCauseOrderEvaluation,
    ];

    fn slot(self) -> u32 {
        match self {
            Self::RenderedFamilyImpl => 0,
            Self::RenderedCauseOrderImpl => 1,
            Self::RenderedFamilyEvaluation => 2,
            Self::RenderedCauseOrderEvaluation => 3,
        }
    }

    fn described(self) -> &'static str {
        match self {
            Self::RenderedFamilyImpl => "the family contract's production implementation",
            Self::RenderedCauseOrderImpl => "the typed cause order's production implementation",
            Self::RenderedFamilyEvaluation => {
                "the family implementation's mutation-evaluation copy"
            }
            Self::RenderedCauseOrderEvaluation => {
                "the cause-order implementation's mutation-evaluation copy"
            }
        }
    }
}

impl RenderedImplementation {
    /// The other half of this role's pair: a production role's evaluation copy,
    /// and an evaluation role's production original.
    ///
    /// One implementation meaning is delivered as two surfaces, so the roster's
    /// seats come in pairs and the pairing is stated once, here, rather than
    /// re-derived by every reader that needs it. A composition that rendered an
    /// evaluation copy of one contract and matched it against the other
    /// contract's production member would be comparing two implementations of
    /// two contracts, and the parity it then stated would be a statement about
    /// nothing.
    ///
    /// # Bounds
    ///
    /// Total, and an involution: every seat has exactly one twin and the twin's
    /// twin is the seat. It never answers with an absence, because a roster
    /// entry without a pair would be a surface delivered on its own — a
    /// production implementation nothing can be evaluated against, or a copy of
    /// an implementation nobody planned.
    #[must_use]
    pub const fn twin(self) -> Self {
        match self {
            Self::RenderedFamilyImpl => Self::RenderedFamilyEvaluation,
            Self::RenderedCauseOrderImpl => Self::RenderedCauseOrderEvaluation,
            Self::RenderedFamilyEvaluation => Self::RenderedFamilyImpl,
            Self::RenderedCauseOrderEvaluation => Self::RenderedCauseOrderImpl,
        }
    }

    /// Whether a member under this role is the mutation-evaluation copy rather
    /// than the implementation the consumer's normal build compiles.
    ///
    /// The one fact that separates the two halves of every pair, and the reason
    /// they are two: the evaluation copy carries the active-point selector and
    /// crosses the wall inside the shell, while the production implementation
    /// carries no selector under any condition there is — no profile, no
    /// feature, no configuration, and no caller can move it.
    #[must_use]
    pub const fn is_evaluation_copy(self) -> bool {
        match self {
            Self::RenderedFamilyImpl | Self::RenderedCauseOrderImpl => false,
            Self::RenderedFamilyEvaluation | Self::RenderedCauseOrderEvaluation => true,
        }
    }

    /// Where a member under this role lands once it is rendered.
    ///
    /// Every seat on this roster lands at the declaration site, and the two
    /// halves land there for two different reasons: the production
    /// implementation IS the item the caller's declaration expands into, and the
    /// evaluation copy rides the generated support shell, which is emitted at the
    /// declaration site as deferred tokens and invoked by the consumer's test
    /// target. Where a copy is CONSUMED is not where it lands, and only the
    /// landing is a destination.
    ///
    /// Stated as a constant answer over the closed roster rather than read off a
    /// planned member, so a plan that declared a member of this kind as a
    /// standalone artifact is a plan the reading roads refuse against this answer
    /// instead of a delivery nobody recognized.
    #[must_use]
    pub const fn destination(self) -> MemberDestination {
        match self {
            Self::RenderedFamilyImpl
            | Self::RenderedCauseOrderImpl
            | Self::RenderedFamilyEvaluation
            | Self::RenderedCauseOrderEvaluation => MemberDestination::AtDeclarationSite,
        }
    }
}

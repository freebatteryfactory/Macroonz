//! The derive-implementation home's declarative surface: the tables and trait
//! implementations this home states rather than computes.
//!
//! Three declarations stand here.
//!
//! The LIMIT FAMILIES: each family's capacity authority and its magnitude are
//! written on adjacent rows, so a family cannot be declared on the compile-time
//! ladder while wearing another road's authority — [`Limit::Authority`] resolves
//! to one type, and naming [`DeclaredMagnitude`] there is what makes
//! [`ConstLimit`] implementable at all. The families themselves are declared
//! beside the capacities they govern in `types.rs`; what a family is FOR is said
//! there, and the number is said here.
//!
//! The REFUSAL FAMILY's declared shape: an issue collection, because several
//! points may be doubled while another claims the control's name, and a caller
//! repairing a surface one point per attempt is a caller this home failed.
//!
//! The SURFACE ROSTER's one separating fact: whether a surface carries a
//! selector. Stated as a constant table over a closed roster rather than as a
//! sentence, so "production carries no selector, ever" is a value a reader can
//! read back and a match the compiler keeps exhaustive.

use super::{
    ImplementationSurface, ImplementationSurfaceComposition, MutationAlternativeLimit,
    MutationPointLimit, SurfaceIssueLimit,
};
use threadpak::refusal::{FamilyShape, RefusalFamily};
use threadpak::types::{ConstLimit, DeclaredMagnitude, Limit};

impl Limit for MutationPointLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for MutationPointLimit {
    const MAX: usize = 64;
}

impl Limit for MutationAlternativeLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for MutationAlternativeLimit {
    const MAX: usize = 8;
}

impl Limit for SurfaceIssueLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for SurfaceIssueLimit {
    /// Twice the mutation-point magnitude, because the naming pass asks two
    /// independent questions of every admitted point and both can hold at once.
    /// Written as the number rather than as a product of the family beside it:
    /// a magnitude derived from another magnitude reads as a fact when it is a
    /// choice, and this home would still owe the same number if the point
    /// magnitude moved for its own reasons.
    const MAX: usize = 128;
}

impl RefusalFamily for ImplementationSurfaceComposition {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl ImplementationSurface {
    /// Whether this surface carries the active-point selector.
    ///
    /// The one fact that separates the two, and the reason they are two: the
    /// production surface answers `false` under every condition there is — no
    /// profile, no feature, no configuration, and no caller can move it — while
    /// the evaluation surface exists precisely to carry one.
    ///
    /// A constant answer over a closed roster, so a third surface admitted later
    /// stops the compiler here until somebody says which side of this line it
    /// stands on.
    #[must_use]
    pub const fn carries_selector(self) -> bool {
        match self {
            Self::Production => false,
            Self::MutationEvaluation => true,
        }
    }
}

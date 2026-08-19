//! The derive-implementation home's declarative surface: the tables and trait
//! implementations this home states rather than computes.
//!
//! Two declarations stand here.
//!
//! There is no LIMIT FAMILY table here, and its absence is the point. The three
//! magnitudes this home's capacities are governed by — the mutation points one
//! evaluation surface may admit, the alternatives one point may admit, and the
//! issues one surface-composition refusal body may carry — stand on the PLANE's
//! own limit roster, beside every other magnitude the services declare, and the
//! seats they govern name them by importing them. One roster is what keeps "the
//! widest magnitude the plane declares" a question with one answer, and a family
//! declared beside its own seats would be a magnitude that roster cannot see.
//!
//! The REFUSAL FAMILY's declared shape: an issue collection, because several
//! points may be doubled while another claims the control's name, and a caller
//! repairing a surface one point per attempt is a caller this home failed.
//!
//! The SURFACE ROSTER's one separating fact: whether a surface carries a
//! selector. Stated as a constant table over a closed roster rather than as a
//! sentence, so "production carries no selector, ever" is a value a reader can
//! read back and a match the compiler keeps exhaustive.

use super::{ImplementationSurface, ImplementationSurfaceComposition};
use threadpak::refusal::{FamilyShape, RefusalFamily};

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

//! The pattern-stamp home's declarative surface: the tables this home states
//! rather than computes.
//!
//! The LIMIT FAMILIES: each family's capacity authority and its magnitude are
//! written on adjacent rows, so a family cannot be declared on the compile-time
//! ladder while wearing another road's authority — [`Limit::Authority`] resolves
//! to one type, and naming [`DeclaredMagnitude`] there is what makes
//! [`ConstLimit`] implementable at all. The families themselves are declared
//! beside the capacities they govern in `types.rs`; what a family is FOR is said
//! there, and the number is said here.
//!
//! The VISIBILITY TRANSPORT: which reach a stamped item carries inside the
//! module the stamp seats it in, stated once as a total answer over the closed
//! reach roster. The stamped item sits one module deeper than the coordinate the
//! caller wrote its reach at, so a reach copied straight through would publish a
//! private seat to the caller's own parent — and a reach widened by guesswork
//! would publish it further than that.

use super::{
    SeatDeclarationLimit, SeatMint, SeatMintForm, SeatPathSegmentLimit, SeatVisibility,
    TransportedReach,
};
use threadpak::types::{ConstLimit, DeclaredMagnitude, Limit};

impl Limit for SeatPathSegmentLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for SeatPathSegmentLimit {
    const MAX: usize = 8;
}

impl Limit for SeatDeclarationLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for SeatDeclarationLimit {
    /// Sixty-four. One publication unit is one artifact landed under one
    /// receipt, and what it covers is the set of homes that will invoke it.
    /// Written as the number rather than as a count of the machine's bands: a
    /// magnitude derived from another structure's size reads as a fact when it
    /// is a choice, and it would move for a reason nobody's publication changed
    /// by.
    const MAX: usize = 64;
}

impl SeatVisibility {
    /// The reach the stamped item carries INSIDE the module the stamp seats it
    /// in.
    ///
    /// The stamped item sits one module deeper than the coordinate the caller
    /// wrote, so a reach that reads the same at both coordinates names a
    /// different scope at each. This is the one place that transport is decided,
    /// so the front arm the stamp renders and the reach the stamped item wears
    /// are one answer rather than two that agree until one is edited.
    ///
    /// A constant answer over a closed roster, so a sixth reach admitted later
    /// stops the compiler here until somebody says what it becomes one level in.
    ///
    /// # Bounds
    ///
    /// The two private reaches — no token at all, and `pub(self)` — name the
    /// same scope and therefore transport to the same one. The parent-facing
    /// reach gains one segment. The crate-facing and public reaches are absolute
    /// and the extra module does not move them, so they transport to themselves.
    ///
    /// Nothing transports to a private reach, which is why the answer's roster
    /// is [`TransportedReach`] and not this one: a stamped item that landed
    /// private inside the seat module could not be re-exported out of it at all,
    /// and the caller's own coordinate would name nothing.
    #[must_use]
    pub const fn transported(self) -> TransportedReach {
        match self {
            Self::Private | Self::SelfReach => TransportedReach::SuperReach,
            Self::SuperReach => TransportedReach::AncestorReach,
            Self::CrateReach => TransportedReach::CrateReach,
            Self::PublicReach => TransportedReach::PublicReach,
        }
    }
}

impl SeatMint {
    /// Which mint form this declared mint asks for.
    ///
    /// The one road from a seat's own fact to the form the stamp definition is
    /// written from: the definition covers every seat and cannot carry one
    /// seat's admission profile, and this is where the profile is dropped and
    /// the form kept.
    #[must_use]
    pub const fn form(&self) -> SeatMintForm {
        match self {
            Self::ReadersOnly => SeatMintForm::ReadersOnly,
            Self::EstablishedUnder(_) => SeatMintForm::Minting,
        }
    }
}

//! Read-only historical target and activation projections.

use super::{
    ArchivedActivationReading, ArchivedMutationIdentity, ArchivedMutationSite,
    ArchivedMutationTarget,
};
use crate::descriptor::archive::ArchivedName;
use crate::report::archive::AddressClaim;

#[path = "read.rs"]
mod read;
pub(crate) use read::{read_activation, read_target};

impl ArchivedMutationIdentity {
    /// The historical point, absent for external coordinate identities.
    #[must_use]
    pub const fn point(&self) -> Option<&ArchivedName> {
        match self {
            Self::External(_) => None,
            Self::Interpreted {
                point,
                alternative: _,
            }
            | Self::CompiledProjection {
                point,
                alternative: _,
            } => Some(point),
        }
    }
}

impl ArchivedMutationTarget {
    /// The exact historical identity arm.
    #[must_use]
    pub const fn identity(&self) -> &ArchivedMutationIdentity {
        &self.identity
    }

    /// The historical family slug, absent for an outside-bank attribution.
    #[must_use]
    pub fn family(&self) -> Option<&str> {
        self.family.as_deref()
    }

    /// The historical source coordinate or declared activation site.
    #[must_use]
    pub const fn site(&self) -> &ArchivedMutationSite {
        &self.site
    }

    /// The historical owning claim, absent when unmapped.
    #[must_use]
    pub const fn owner(&self) -> Option<&ArchivedName> {
        self.owner.as_ref()
    }
}

impl ArchivedActivationReading {
    /// The historical surface address claim.
    #[must_use]
    pub const fn surface(&self) -> AddressClaim {
        self.surface
    }

    /// The historical point selected on that surface.
    #[must_use]
    pub const fn point(&self) -> &ArchivedName {
        &self.point
    }

    /// The selected alternative's address claim.
    #[must_use]
    pub const fn alternative(&self) -> AddressClaim {
        self.alternative
    }

    /// The callback's historical witness claim.
    #[must_use]
    pub const fn witness(&self) -> AddressClaim {
        self.witness
    }

    /// The positive count reported by that callback.
    #[must_use]
    pub const fn firings(&self) -> u32 {
        self.firings
    }
}

//! Historical mutation coordinates without execution or activation authority.

use crate::descriptor::archive::ArchivedName;
use crate::muterprater::SourceCoordinate;
use crate::report::archive::AddressClaim;

#[path = "type_guard.rs"]
mod guard;

pub(crate) use guard::{read_activation, read_target};

/// The historical identity of a damaged subject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedMutationIdentity {
    /// A backend coordinate-and-damage address whose preimage is absent.
    External(AddressClaim),
    /// An interpreted point and its historical alternative.
    Interpreted {
        /// The historical point name.
        point: ArchivedName,
        /// The claimed alternative address.
        alternative: AddressClaim,
    },
    /// A compiled projection's point and historical alternative.
    CompiledProjection {
        /// The historical point name.
        point: ArchivedName,
        /// The claimed alternative address.
        alternative: AddressClaim,
    },
}

/// The source or declared site retained by a historical target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedMutationSite {
    /// An exact reported source coordinate.
    Reported(SourceCoordinate),
    /// A historical declared activation name.
    Declared(ArchivedName),
}

/// A target's retained identity, attribution, site and optional owning claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedMutationTarget {
    identity: ArchivedMutationIdentity,
    family: Option<String>,
    site: ArchivedMutationSite,
    owner: Option<ArchivedName>,
}

/// A positive historical callback count and its exact selection and witness claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedActivationReading {
    surface: AddressClaim,
    point: ArchivedName,
    alternative: AddressClaim,
    witness: AddressClaim,
    firings: u32,
}

/// The historical activation disposition without a live activation mint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedActivation {
    /// A positive callback report under these historical coordinates.
    Observed(ArchivedActivationReading),
    /// No positive activation was reported.
    NotObserved,
    /// The source backend had no activation channel.
    UnobservableUnderBackend,
}

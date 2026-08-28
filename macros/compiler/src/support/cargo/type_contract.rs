//! The total cargo-axis table.
use super::CargoAxis;
use crate::kind::Destination;
impl CargoAxis {
    /// The proved delivery this axis reads.
    #[must_use]
    pub const fn reads_from(self) -> Destination {
        match self {
            Self::Declared => Destination::DeclarationSite,
            Self::Deferred => Destination::TestCarrier,
            Self::Bench => Destination::BenchCarrier,
        }
    }
}

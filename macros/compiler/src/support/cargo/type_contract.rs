//! The total cargo-axis table.
use super::CargoAxis;
use crate::kind::Destination;
impl CargoAxis {
    /// The proved delivery this axis reads, if any.
    #[must_use]
    pub const fn reads_from(self) -> Option<Destination> {
        match self {
            Self::Declared => None,
            Self::Deferred => Some(Destination::TestCarrier),
            Self::Bench => Some(Destination::BenchCarrier),
        }
    }
}

//! Shared record refusals keep their existing diagnostic owner.

use super::SurfaceArchiveRefusal;
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for SurfaceArchiveRefusal {
    fn from(cause: ArchiveRefusal) -> Self {
        Self::Record(cause)
    }
}

//! Conversion of nested bounded-record diagnostics.

use super::{BackendArchiveRefusal, SuitePressureArchiveRefusal};
use crate::muterprater::verdict_archive::MutationArchiveRefusal;
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for BackendArchiveRefusal {
    fn from(refusal: ArchiveRefusal) -> Self {
        Self::Record(refusal)
    }
}

impl From<MutationArchiveRefusal> for BackendArchiveRefusal {
    fn from(refusal: MutationArchiveRefusal) -> Self {
        Self::Mutation(refusal)
    }
}

impl From<ArchiveRefusal> for SuitePressureArchiveRefusal {
    fn from(cause: ArchiveRefusal) -> Self {
        Self::Record(cause)
    }
}

impl From<BackendArchiveRefusal> for SuitePressureArchiveRefusal {
    fn from(cause: BackendArchiveRefusal) -> Self {
        Self::Backend(cause)
    }
}

//! Conversion of nested bounded-record diagnostics.

use super::BackendArchiveRefusal;
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

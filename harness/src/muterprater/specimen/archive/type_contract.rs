//! Historical projection refusal composition.

use super::ProjectionArchiveRefusal;
use crate::muterprater::interpretation_archive::ParityArchiveRefusal;
use crate::muterprater::verdict_archive::MutationArchiveRefusal;
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for ProjectionArchiveRefusal {
    fn from(cause: ArchiveRefusal) -> Self {
        Self::Record(cause)
    }
}

impl From<ParityArchiveRefusal> for ProjectionArchiveRefusal {
    fn from(cause: ParityArchiveRefusal) -> Self {
        Self::Parity(cause)
    }
}

impl From<MutationArchiveRefusal> for ProjectionArchiveRefusal {
    fn from(cause: MutationArchiveRefusal) -> Self {
        Self::Mutation(cause)
    }
}

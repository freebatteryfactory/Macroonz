//! Historical mutation refusal and outcome projections.

use super::{ArchivedMutationOutcome, MutationArchiveRefusal};
use crate::muterprater::MutationVerdict;
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for MutationArchiveRefusal {
    fn from(refusal: ArchiveRefusal) -> Self {
        Self::Record(refusal)
    }
}

impl From<&ArchivedMutationOutcome> for MutationVerdict {
    /// The historical verdict word, without a live mutation record.
    fn from(outcome: &ArchivedMutationOutcome) -> Self {
        match outcome {
            ArchivedMutationOutcome::Killed(_) => Self::Killed,
            ArchivedMutationOutcome::Survived => Self::Survived,
            ArchivedMutationOutcome::Inconclusive(_) => Self::Inconclusive,
        }
    }
}

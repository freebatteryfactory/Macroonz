//! Shared archive refusals retain their original reason.

use super::ReductionArchiveRefusal;
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for ReductionArchiveRefusal {
    fn from(refusal: ArchiveRefusal) -> Self {
        Self::Archive(refusal)
    }
}

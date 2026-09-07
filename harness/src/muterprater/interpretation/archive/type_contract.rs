//! Error conversion preserves the semantic owner of each refusal.

use super::ParityArchiveRefusal;
use crate::descriptor::archive::BindingArchiveRefusal;
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for ParityArchiveRefusal {
    fn from(cause: ArchiveRefusal) -> Self {
        Self::Record(cause)
    }
}

impl From<BindingArchiveRefusal> for ParityArchiveRefusal {
    fn from(cause: BindingArchiveRefusal) -> Self {
        Self::Binding(cause)
    }
}

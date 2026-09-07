#![doc = include_str!("README.md")]

mod types;
mod encode_binding;

pub use encode_binding::retain_binding;
pub(crate) use encode_binding::{binding_size, write_revision};
pub(crate) use types::read_revision;
pub use types::{
    ArchivedBinding, ArchivedProvenance, ArchivedRevisionBinding, BindingArchiveLimits,
    BindingArchiveRefusal, read_binding,
};

pub use types::{
    ArchivedCandidate, ArchivedName, ArchivedSynthesis, CandidateArchiveLimits,
    CandidateArchiveRefusal, read_candidate, retain_candidate,
};

pub use types::{
    ArchivedOrigin, ArchivedRow, RowArchiveLimits, RowArchiveRefusal, read_row, retain_row,
};

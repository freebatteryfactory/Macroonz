#![doc = include_str!("README.md")]

mod types;

pub use types::{
    ArchivedCandidate, ArchivedName, ArchivedSynthesis, CandidateArchiveLimits,
    CandidateArchiveRefusal, read_candidate, retain_candidate,
};

pub use types::{
    ArchivedOrigin, ArchivedRow, RowArchiveLimits, RowArchiveRefusal, read_row, retain_row,
};

#![doc = include_str!("README.md")]

mod types;

pub use types::{
    ArchivedCandidate, ArchivedName, ArchivedSynthesis, CandidateArchiveLimits,
    CandidateArchiveRefusal, read_candidate, retain_candidate,
};

use super::{MutationCustodyError, MutationError, MutationObservationError};
use std::fmt;

impl fmt::Display for MutationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(cause) => write!(formatter, "mutation configuration: {cause}"),
            Self::Filesystem(cause) => write!(formatter, "mutation filesystem: {cause}"),
            Self::SourceBound { bound } => {
                write!(formatter, "mutation source exceeds {bound} bytes")
            }
            Self::Process(cause) => write!(formatter, "mutation process: {cause}"),
            Self::Query {
                phase,
                request: _,
                run: _,
                cause,
            } => write!(formatter, "mutation {phase:?}: {cause}"),
        }
    }
}

impl std::error::Error for MutationError {}

impl fmt::Display for MutationObservationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "mutation observation: {self:?}")
    }
}

impl std::error::Error for MutationObservationError {}

impl fmt::Display for MutationCustodyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "mutation custody: {self:?}")
    }
}

impl std::error::Error for MutationCustodyError {}

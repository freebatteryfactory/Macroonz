use super::{FormatError, FormatObservationError};
use std::fmt;

impl fmt::Display for FormatError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(reason) => write!(into, "formatter configuration: {reason}"),
            Self::Filesystem(error) => write!(into, "formatter filesystem: {error}"),
            Self::Process(error) => write!(into, "formatter process: {error}"),
            Self::Query { .. } => {
                into.write_str("formatter query did not establish rustfmt 1.9.0-stable")
            }
        }
    }
}

impl std::error::Error for FormatError {}

impl fmt::Display for FormatObservationError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Process => into.write_str("formatter did not complete successfully"),
            Self::Text => into.write_str("formatter stdout is not complete UTF-8 LF source"),
            Self::Configuration(reason) => write!(into, "formatter configuration: {reason}"),
        }
    }
}

impl std::error::Error for FormatObservationError {}

use super::DependencyError;
use std::fmt;

impl fmt::Display for DependencyError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotCompiled => {
                into.write_str("dependency capture requires a successful selected artifact")
            }
            Self::Filesystem(reason) => write!(into, "dependency file: {reason}"),
            Self::NotRegular => into.write_str("dependency input is not a regular file"),
            Self::ByteBound => into.write_str("dependency input exceeds its byte bound"),
            Self::FileBound => into.write_str("dependency source roster exceeds its file bound"),
            Self::Artifact => {
                into.write_str("dependency rule does not uniquely name a reported artifact")
            }
            Self::Representation(reason) => write!(into, "dependency representation: {reason}"),
        }
    }
}

impl std::error::Error for DependencyError {}

use super::DestinationError;
use std::fmt;

impl fmt::Display for DestinationError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inventory(error) => write!(into, "destination inventory: {error}"),
            Self::Metadata(reason) => write!(into, "destination ownership: {reason}"),
            Self::Entry(path) => write!(into, "destination entry refused: {path}"),
            Self::Storage(error) => write!(into, "destination storage: {error:?}"),
            Self::Filesystem(error) => write!(into, "destination filesystem: {error}"),
            Self::MissingLock => into.write_str("installed ownership has no cooperating lock"),
            Self::Pending => into.write_str("publication installation requires recovery"),
            Self::Conflict(path) => write!(
                into,
                "publication intent conflicts with current destination: {path}"
            ),
            Self::Incomplete => {
                into.write_str("publication still has unapplied destination operations")
            }
            Self::Unavailable => into.write_str("native destination operations unavailable"),
        }
    }
}

impl std::error::Error for DestinationError {}

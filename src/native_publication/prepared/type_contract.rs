use super::PreparationError;
use std::fmt;

impl fmt::Display for PreparationError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inventory => {
                into.write_str("physical outputs differ from the publication roster")
            }
            Self::Source => {
                into.write_str("formatter output belongs to different canonical source")
            }
            Self::Formatter(error) => write!(into, "formatter refused publication text: {error}"),
            Self::ByteBound => {
                into.write_str("physical publication exceeds its aggregate byte bound")
            }
        }
    }
}

impl std::error::Error for PreparationError {}

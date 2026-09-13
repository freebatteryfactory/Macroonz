use super::StagingError;
use std::fmt;

impl fmt::Display for StagingError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inventory(error) => write!(into, "staging inventory: {error}"),
            Self::Configuration(reason) => write!(into, "staging configuration: {reason}"),
            Self::Storage(error) => write!(into, "staging storage: {error:?}"),
            Self::Filesystem(error) => write!(into, "staging filesystem: {error}"),
            Self::Source(path) => write!(into, "staged source differs: {path}"),
            Self::Compiler(error) => write!(into, "staging compiler: {error}"),
            Self::Unavailable => into.write_str("native staging is unavailable on this target"),
        }
    }
}

impl std::error::Error for StagingError {}

impl crate::compiler::identity::Subject for super::types::StagingSource {
    const STEM: &'static str = "macroonz/native-publication";
    const NAME: &'static str = "staging-source";
}

use super::BakeError;
use crate::compiler::Kind;
use std::fmt;

impl<K: Kind + fmt::Debug, E: fmt::Debug> fmt::Display for BakeError<K, E> {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(output, "publication command refused: {:?}", self.cause)
    }
}

impl<K: Kind + fmt::Debug, E: fmt::Debug> std::error::Error for BakeError<K, E> {}

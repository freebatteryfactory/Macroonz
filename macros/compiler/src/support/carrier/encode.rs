//! Canonical shell-refusal encoding.
use super::ShellError;
use crate::identity::{encode_bytes, encode_length};
impl ShellError {
    /// Returns canonical refusal bytes.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }
    /// Appends canonical refusal bytes.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        match self {
            Self::NotOneDeclaration { stated, planned } => {
                encode_bytes(stated.as_bytes(), into);
                encode_bytes(planned.as_bytes(), into);
            }
            Self::TreeUnbounded { bound, observed } => {
                encode_length(*bound, into);
                encode_length(*observed, into);
            }
        }
    }
}

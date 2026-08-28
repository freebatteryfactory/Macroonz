//! Canonical support-declaration refusal encoding.
use super::DeclarationError;
impl DeclarationError {
    /// Returns canonical bytes.
    #[must_use]
    pub fn canonical_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }
    /// Appends canonical bytes.
    pub fn encode_into(self, into: &mut Vec<u8>) {
        into.push(self.slot());
    }
}

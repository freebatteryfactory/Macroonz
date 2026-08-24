//! The transcript byte string, written from the specification stated on [`Transcript`].
//!
//! It reads the transcript through the transcript's own accessors and touches no private field.
//! The role's declared name and slot are not restated here either, and neither is the grammar's: the roster answers them at their declarations, and this file reads those answers back.
//!
//! The generator is not written.
//! It is carried on the transcript for the derivation record and named by no grammar, so a producer's rendered shape moving renames nothing here.

use super::encode::encode_bytes;
use super::{Subject, Transcript};

impl Transcript<'_> {
    /// The transcript's bytes for one identity subject, exactly as the specification on [`Transcript`] states them.
    ///
    /// # Authority
    ///
    /// **The subject is the TYPE's and never an argument.** Member four of the preimage is the subject's declared name, and it is part of what separates one subject's identities from another's — so an encoder that took the name as text would let a caller write member four to any key space it could spell, and the typed identity above it would be a promise this road never had to keep.
    #[must_use]
    pub fn encoded<S: Subject>(&self) -> Vec<u8> {
        let profile = self.profile();
        let role = self.role();
        let mut bytes = Vec::new();
        encode_bytes(profile.stem().as_bytes(), &mut bytes);
        encode_bytes(profile.name().as_bytes(), &mut bytes);
        bytes.extend_from_slice(&profile.version().position().to_be_bytes());
        encode_bytes(S::NAME.as_bytes(), &mut bytes);
        encode_bytes(role.name().as_bytes(), &mut bytes);
        bytes.push(role.slot());
        bytes.push(self.anchoring().slot());
        match self.anchoring().commitment() {
            Some(anchor) => encode_bytes(anchor, &mut bytes),
            None => encode_bytes(&[], &mut bytes),
        }
        encode_bytes(self.material(), &mut bytes);
        bytes.extend_from_slice(&self.position().to_be_bytes());
        bytes
    }
}

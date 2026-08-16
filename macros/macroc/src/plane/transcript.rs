//! The transcript byte string, written from the specification stated on
//! [`ProjectionTranscript`].
//!
//! It reads the transcript through the transcript's own public accessors and
//! touches no private field.
//! The role's stable name and slot are not restated here either: the
//! closed-roster stamp writes them onto [`ProjectionRole`] at its declaration,
//! and this file reads them back.

use super::encode::encode_bytes;
use super::{ProjectionTranscript, TranscriptAnchoring};

impl TranscriptAnchoring {
    /// The discriminant byte the transcript carries for this posture.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::Rooted => 0,
            Self::UnderOwnerIdentity(_) => 1,
            Self::UnderProjectionIdentity(_) => 2,
        }
    }

    /// The anchor commitment at full width, where there is one.
    #[must_use]
    pub const fn commitment(&self) -> Option<&[u8; 32]> {
        match self {
            Self::Rooted => None,
            Self::UnderOwnerIdentity(anchor) | Self::UnderProjectionIdentity(anchor) => {
                Some(anchor)
            }
        }
    }
}

impl ProjectionTranscript<'_> {
    /// The transcript's bytes for one identity subject, exactly as the
    /// specification on [`ProjectionTranscript`] states them.
    #[must_use]
    pub fn encoded(&self, subject: &str) -> Vec<u8> {
        let mut bytes = Vec::new();
        encode_bytes(self.profile().stem().as_bytes(), &mut bytes);
        bytes.extend_from_slice(&self.profile().version().position().to_be_bytes());
        encode_bytes(subject.as_bytes(), &mut bytes);
        encode_bytes(self.role().stable_name().as_bytes(), &mut bytes);
        bytes.push(self.role().slot());
        bytes.push(self.anchoring().slot());
        match self.anchoring().commitment() {
            Some(anchor) => encode_bytes(anchor, &mut bytes),
            None => encode_bytes(&[], &mut bytes),
        }
        encode_bytes(self.content(), &mut bytes);
        bytes.extend_from_slice(&self.position().to_be_bytes());
        encode_bytes(self.generator().profile().spelling().as_bytes(), &mut bytes);
        bytes.extend_from_slice(&self.generator().schema().position().to_be_bytes());
        bytes
    }
}

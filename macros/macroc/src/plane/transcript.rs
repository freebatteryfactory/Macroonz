//! The transcript byte string, written from the specification stated on
//! [`ProjectionTranscript`].
//!
//! It reads the transcript through the transcript's own public accessors and
//! touches no private field.
//! The role's stable name and slot are not restated here either, and neither is
//! the preimage family's: the closed-roster stamp writes them onto
//! [`ProjectionRole`] and [`PreimageFamily`] at their declarations, and this
//! file reads them back.
//!
//! The generator is not written. It is carried on the transcript for the
//! derivation record and named by no family's grammar, so a producer's rendered
//! shape moving renames nothing here.
//!
//! [`ProjectionRole`]: super::ProjectionRole
//! [`PreimageFamily`]: super::PreimageFamily

use super::encode::encode_bytes;
use super::{IDENTITY_PROFILE_STEM, IdentitySubject, ProjectionTranscript, TranscriptAnchoring};

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
    ///
    /// # Authority
    ///
    /// **The subject is the TYPE's and never an argument.** Member four of the
    /// preimage is the subject's declared name, and it is what separates one
    /// subject's identities from another's — so an encoder that took the name as
    /// text would let a caller write member four to any name space it could
    /// spell, and the typed identity above it would be a promise this road never
    /// had to keep. The subject arrives as a parameter of the sealed
    /// [`IdentitySubject`](super::IdentitySubject) roster instead, so encoding
    /// under the wrong subject is unwritable rather than discouraged.
    #[must_use]
    pub fn encoded<Subject: IdentitySubject>(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        encode_bytes(IDENTITY_PROFILE_STEM.as_bytes(), &mut bytes);
        encode_bytes(self.profile().family().stable_name().as_bytes(), &mut bytes);
        bytes.extend_from_slice(&self.profile().version().position().to_be_bytes());
        encode_bytes(Subject::SUBJECT_NAME.as_bytes(), &mut bytes);
        encode_bytes(self.role().stable_name().as_bytes(), &mut bytes);
        bytes.push(self.role().slot());
        bytes.push(self.anchoring().slot());
        match self.anchoring().commitment() {
            Some(anchor) => encode_bytes(anchor, &mut bytes),
            None => encode_bytes(&[], &mut bytes),
        }
        encode_bytes(self.content(), &mut bytes);
        bytes.extend_from_slice(&self.position().to_be_bytes());
        bytes
    }
}

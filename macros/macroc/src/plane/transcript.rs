//! The transcript's executable specification: the declared segments and slots a
//! transcript is written from, and the byte string itself.
//!
//! The specification a reader implements against is the table on
//! [`ProjectionTranscript`]; this file is that table as code. It reads the
//! transcript through the transcript's own public accessors and touches no
//! private field, because the encoding is an operation over a value that is
//! already informed rather than part of the invariant that made it.
//!
//! The role's slot and its declared context segment sit here beside the encoder
//! that writes them. They are declarations in method form: renaming a variant
//! must not rename every identity derived under it, so the spelling is stated
//! rather than taken from Rust.

use super::encode::encode_bytes;
use super::{ProjectionRole, ProjectionTranscript, TranscriptAnchoring};

impl ProjectionRole {
    /// The role's position in the declared roster — the byte the transcript
    /// carries for it.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::CapturedDeclaration => 0,
            Self::Plan => 1,
            Self::OriginNode => 2,
            Self::GeneratedUnit => 3,
            Self::RenderedUnit => 4,
            Self::OutputBytes => 5,
            Self::Bundle => 6,
            Self::Closure => 7,
            Self::ClosedExpansion => 8,
        }
    }

    /// The role's declared segment of the derive-key context.
    ///
    /// Declared rather than taken from the Rust spelling, for the same reason
    /// [`IdentitySubject::SUBJECT_NAME`] is: renaming a variant must not rename
    /// every identity derived under it.
    ///
    /// [`IdentitySubject::SUBJECT_NAME`]: super::IdentitySubject::SUBJECT_NAME
    #[must_use]
    pub const fn context_name(self) -> &'static str {
        match self {
            Self::CapturedDeclaration => "captured-declaration",
            Self::Plan => "plan",
            Self::OriginNode => "origin-node",
            Self::GeneratedUnit => "generated-unit",
            Self::RenderedUnit => "rendered-unit",
            Self::OutputBytes => "output-bytes",
            Self::Bundle => "bundle",
            Self::Closure => "closure",
            Self::ClosedExpansion => "closed-expansion",
        }
    }
}

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
        encode_bytes(self.role().context_name().as_bytes(), &mut bytes);
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

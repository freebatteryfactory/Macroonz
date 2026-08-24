//! The canonical bytes one refused issue is, for the related identity a diagnostic derives over it.
//!
//! The row's position rides ahead of the material it governs, and every variable-length member is framed through the identity home's one framing, so two issues differing in any typed member never encode alike.

use super::CodecIssue;
use crate::identity::encode_bytes;

impl CodecIssue {
    /// This issue's canonical bytes on their own.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }

    /// Appends this issue's canonical bytes: the row's position in the declared roster, then the typed material that row carries, framed.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let mut material = Vec::new();
        self.material_into(&mut material);
        encode_bytes(&material, into);
    }

    /// The typed material one issue carries, through each value's own spelling.
    ///
    /// Exhaustive over the roster on purpose: an issue added to [`CodecIssue`] stops compiling HERE until somebody says what of it a preimage commits to, so no issue can be admitted and left out of every identity derived over a refusal that carries it.
    fn material_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::PathSegmentsAbsent
            | Self::MemberSpellingAbsent
            | Self::AssemblyRoadAbsent
            | Self::MembersAbsent => {}
            Self::SegmentNotAnIdentifier { segment } => encode_bytes(segment.as_bytes(), into),
            Self::MemberSpellingNotAnIdentifier { spelling }
            | Self::MemberSpellingDoubled { spelling }
            | Self::AssemblyRoadNotAnIdentifier { spelling }
            | Self::RefusalSpellingNotAnIdentifier { spelling }
            | Self::ModuleSpellingNotAnIdentifier { spelling } => {
                encode_bytes(spelling.as_bytes(), into);
            }
            Self::MemberShadowsBinding { spelling, binding } => {
                encode_bytes(spelling.as_bytes(), into);
                encode_bytes(binding.as_bytes(), into);
            }
            Self::PathSegmentsUnbounded { bound, observed }
            | Self::MembersUnbounded { bound, observed } => {
                into.extend_from_slice(&bound.to_be_bytes());
                into.extend_from_slice(&observed.to_be_bytes());
            }
        }
    }
}

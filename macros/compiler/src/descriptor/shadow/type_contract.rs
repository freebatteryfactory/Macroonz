//! The shadow home's stated contracts: what the kind is, and how its capture refusal composes into a diagnostic.

use super::{ShadowCaptureError, ShadowFace, Shadows};
use crate::diagnostic::SHADOW_HELPER_FAMILY;
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{CanonicalContent, Kind, NoQuestions, SoleRole};

impl CanonicalContent for Shadows {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_length(self.loom().segments().count(), into);
        for segment in self.loom().segments() {
            encode_bytes(segment.as_bytes(), into);
        }
        encode_length(self.chosen().len(), into);
        for row in self.chosen() {
            let mut encoded = Vec::new();
            encode_bytes(row.name().as_bytes(), &mut encoded);
            encode_length(row.std_path().len(), &mut encoded);
            for segment in row.std_path() {
                encode_bytes(segment.as_bytes(), &mut encoded);
            }
            encode_length(row.shadow_path().len(), &mut encoded);
            for segment in row.shadow_path() {
                encode_bytes(segment.as_bytes(), &mut encoded);
            }
            encode_bytes(&encoded, into);
        }
    }
}

impl Kind for ShadowFace {
    const NAME: &'static str = "shadow-face";
    type Content = Shadows;
    type Role = SoleRole;
    type Question = NoQuestions;
}

crate::descriptor::impl_helper_capture_contract!(
    ShadowCaptureError,
    SHADOW_HELPER_FAMILY,
    canonical
);

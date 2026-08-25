//! The shadow home's stated contracts: what the kind is, and how its capture refusal composes into a diagnostic.

use super::{ShadowCaptureError, ShadowFace, Shadows};
use crate::bounded::Bounded;
use crate::diagnostic::{
    Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
    SHADOW_HELPER_FAMILY,
};
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

impl Refused for ShadowCaptureError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = SHADOW_HELPER_FAMILY;

    fn class(&self) -> RefusalClass {
        self.refusal().class()
    }

    fn first(&self) -> String {
        self.refusal().first()
    }

    fn observed(&self) -> Observed {
        self.refusal().classified()
    }

    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    fn related(&self) -> Vec<Vec<u8>> {
        vec![self.refusal().canonical_bytes()]
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        self.refusal().repairs()
    }
}

impl core::fmt::Display for ShadowCaptureError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let refusal = self.refusal();
        write!(into, "{refusal}")
    }
}

impl core::error::Error for ShadowCaptureError {}

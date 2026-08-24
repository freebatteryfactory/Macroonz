//! The shadow home's stated contracts: what the kind is, and how its capture refusal composes into a diagnostic.

use super::{ShadowCaptureError, ShadowFace, Shadows};
use crate::bounded::Bounded;
use crate::diagnostic::{
    Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
    SHADOW_HELPER_FAMILY,
};
use crate::kind::{Kind, NoQuestions, SoleRole};

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

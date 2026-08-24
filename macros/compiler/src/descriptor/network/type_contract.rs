//! The network declaration home's stated contracts: what the kind is, and how its capture refusal composes into a diagnostic.

use super::{NetworkCaptureError, NetworkDeclaration, NetworkModule};
use crate::bounded::Bounded;
use crate::diagnostic::{
    Family, LineBody, NETWORK_HELPER_FAMILY, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused,
    Repair,
};
use crate::kind::{Kind, NoQuestions, SoleRole};

impl Kind for NetworkModule {
    const NAME: &'static str = "network-module";
    type Content = NetworkDeclaration;
    type Role = SoleRole;
    type Question = NoQuestions;
}

impl Refused for NetworkCaptureError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = NETWORK_HELPER_FAMILY;

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

impl core::fmt::Display for NetworkCaptureError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let refusal = self.refusal();
        write!(into, "{refusal}")
    }
}

impl core::error::Error for NetworkCaptureError {}

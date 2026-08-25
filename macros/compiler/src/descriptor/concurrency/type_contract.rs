//! The concurrency declaration home's stated contracts: what the kind is, and how its capture refusal composes into a diagnostic.

use super::{ConcurrencyCaptureError, ConcurrencyDeclaration, ConcurrencyModule};
use crate::bounded::Bounded;
use crate::diagnostic::{
    CONCURRENCY_HELPER_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass,
    Refused, Repair,
};
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{CanonicalContent, Kind, NoQuestions, SoleRole};

impl CanonicalContent for ConcurrencyDeclaration {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.module().as_bytes(), into);
        encode_bytes(self.namespace().as_bytes(), into);
        encode_length(self.rows().len(), into);
        for row in self.rows() {
            let mut encoded = Vec::new();
            encode_bytes(row.name().as_bytes(), &mut encoded);
            encode_bytes(row.population().as_bytes(), &mut encoded);
            encoded.extend_from_slice(&row.interleavings().to_be_bytes());
            encoded.extend_from_slice(&row.samples().to_be_bytes());
            encoded.extend_from_slice(&row.seed().to_be_bytes());
            encode_bytes(&encoded, into);
        }
    }
}

impl Kind for ConcurrencyModule {
    const NAME: &'static str = "concurrency-module";
    type Content = ConcurrencyDeclaration;
    type Role = SoleRole;
    type Question = NoQuestions;
}

impl Refused for ConcurrencyCaptureError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = CONCURRENCY_HELPER_FAMILY;

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

impl core::fmt::Display for ConcurrencyCaptureError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let refusal = self.refusal();
        write!(into, "{refusal}")
    }
}

impl core::error::Error for ConcurrencyCaptureError {}

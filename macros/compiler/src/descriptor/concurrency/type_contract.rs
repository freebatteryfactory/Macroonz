//! The concurrency declaration home's stated contracts: what the kind is, and how its capture refusal composes into a diagnostic.

use super::{ConcurrencyCaptureError, ConcurrencyDeclaration, ConcurrencyModule};
use crate::diagnostic::CONCURRENCY_HELPER_FAMILY;
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{CanonicalContent, Kind, NoQuestions, SoleRole};

impl CanonicalContent for ConcurrencyDeclaration {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_length(self.harness().segments().count(), into);
        for segment in self.harness().segments() {
            encode_bytes(segment.as_bytes(), into);
        }
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

crate::descriptor::impl_helper_capture_contract!(
    ConcurrencyCaptureError,
    CONCURRENCY_HELPER_FAMILY,
    canonical
);

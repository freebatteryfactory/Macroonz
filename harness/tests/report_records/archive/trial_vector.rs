//! Independently framed trial envelopes with mutable attempt and measurement members.

use super::vector::{self, InputKind, Vector};

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = vector::hash("historical-trial-report/v1", body).to_vec();
    encoded.extend_from_slice(body);
    encoded
}

pub(super) fn body(attempt: &[u8], measurement: &[u8]) -> Vec<u8> {
    let source = Vector::declared(InputKind::Bound);
    let mut body = Vec::new();
    for value in [1u32, 2, 0] {
        body.extend_from_slice(&value.to_be_bytes());
    }
    vector::frame(&source.key, &mut body);
    body.extend_from_slice(&source.metadata);
    body.push(1);
    vector::frame(b"outside::subject", &mut body);
    vector::frame(b"subject.rs", &mut body);
    body.extend_from_slice(&11u32.to_be_bytes());
    vector::frame(b"original trial", &mut body);
    body.extend_from_slice(attempt);
    body.extend_from_slice(measurement);
    body
}

pub(super) fn refusal(foreign: &[u8]) -> Vec<u8> {
    let mut attempt = vec![1];
    vector::frame(
        &Vector::declared(InputKind::Bound).fingerprint,
        &mut attempt,
    );
    vector::frame(b"check.rs", &mut attempt);
    attempt.extend_from_slice(&29u32.to_be_bytes());
    attempt.extend_from_slice(foreign);
    attempt
}

pub(super) fn foreign(bytes: &[u8], tail: &[u8]) -> Vec<u8> {
    let mut foreign = vec![1];
    vector::frame(bytes, &mut foreign);
    foreign.extend_from_slice(tail);
    foreign
}

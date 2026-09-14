//! Independently authored format-two envelopes for the declared two-node topology.

use macroonz_harness::identity::{ContentAddress, encode_bytes};
use macroonz_harness::network::TRANSCRIPT_TAG;

fn name(stem: &[u8], into: &mut Vec<u8>) {
    encode_bytes(b"lane", into);
    encode_bytes(stem, into);
}

fn link(from: &[u8], to: &[u8], into: &mut Vec<u8>) {
    name(from, into);
    name(to, into);
}

fn prefix(source: u32) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&2u32.to_be_bytes());
    body.extend_from_slice(&source.to_be_bytes());
    body.extend_from_slice(&2u64.to_be_bytes());
    name(b"client", &mut body);
    name(b"server", &mut body);
    body.extend_from_slice(&2u64.to_be_bytes());
    link(b"client", b"server", &mut body);
    link(b"server", b"client", &mut body);
    body
}

fn entry(payload: &[u8], copy: u32, into: &mut Vec<u8>) {
    link(b"client", b"server", into);
    into.extend_from_slice(&0u32.to_be_bytes());
    encode_bytes(payload, into);
    into.extend_from_slice(&0u64.to_be_bytes());
    into.extend_from_slice(&1u64.to_be_bytes());
    into.extend_from_slice(&copy.to_be_bytes());
}

pub(super) fn envelope(body: &[u8]) -> Vec<u8> {
    let mut encoded = ContentAddress::derived(TRANSCRIPT_TAG, body)
        .as_bytes()
        .to_vec();
    encoded.extend_from_slice(body);
    encoded
}

pub(super) fn live(payload: &[u8], entries: u64) -> Vec<u8> {
    let mut body = prefix(1);
    body.extend_from_slice(&entries.to_be_bytes());
    entry(payload, 0, &mut body);
    envelope(&body)
}

pub(super) fn simulated(payload: &[u8], actions: u64, entries: u64) -> Vec<u8> {
    let mut body = prefix(0);
    name(b"duplicate-the-request", &mut body);
    body.extend_from_slice(&1u64.to_be_bytes());
    link(b"client", b"server", &mut body);
    body.extend_from_slice(&1u64.to_be_bytes());
    body.extend_from_slice(&2u32.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    body.extend_from_slice(&actions.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    link(b"client", b"server", &mut body);
    encode_bytes(payload, &mut body);
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&entries.to_be_bytes());
    entry(payload, 0, &mut body);
    entry(payload, 1, &mut body);
    envelope(&body)
}

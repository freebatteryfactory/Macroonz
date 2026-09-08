//! Expected bytes are authored from the published grammar without a product encoder.

pub(super) fn frame(material: &[u8], body: &mut Vec<u8>) {
    body.extend_from_slice(
        &u64::try_from(material.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    body.extend_from_slice(material);
}

pub(super) fn body(method: u32, disposition: u8, payload: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    for word in [1u32, method, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    body.push(disposition);
    body.extend_from_slice(payload);
    body
}

pub(super) fn address(body: &[u8]) -> Vec<u8> {
    let mut encoded = blake3::derive_key(
        "macroonz/harness-identity/historical-oracle-verdict/v1",
        body,
    )
    .to_vec();
    encoded.extend_from_slice(body);
    encoded
}

pub(super) fn envelope(method: u32, disposition: u8, payload: &[u8]) -> Vec<u8> {
    address(&body(method, disposition, payload))
}

pub(super) fn position(slot: u8, at: u64) -> Vec<u8> {
    let mut bytes = vec![slot];
    bytes.extend_from_slice(&at.to_be_bytes());
    bytes
}

pub(super) fn named(slot: u8, text: &str) -> Vec<u8> {
    let mut bytes = vec![slot];
    frame(text.as_bytes(), &mut bytes);
    bytes
}

pub(super) fn positioned_text(slot: u8, at: u64, text: &str) -> Vec<u8> {
    let mut bytes = position(slot, at);
    frame(text.as_bytes(), &mut bytes);
    bytes
}

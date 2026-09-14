//! Canonical input-envelope writing through the shared identity framing.

use super::{
    INPUT_CASE_TAG, INPUT_FORMAT_VERSION, InputCaseId, InputEnvelope, InputLimits, InputProfile,
    InputRefusal,
};
use crate::identity::{ContentAddress, encode_bytes};

/// One content-addressed specimen under its declared input convention.
///
/// The envelope is a raw thirty-two-byte address followed by its body.
/// The body contains, in order: format u32be, framed namespace, framed stem, profile u32be, framed schema address, and framed specimen.
/// Each frame is the identity substrate's u64be byte length followed by those bytes.
///
/// # Errors
///
/// Refuses oversized payloads, unrepresentable envelope sizes and oversized envelopes before allocating their storage.
pub fn pack(
    profile: InputProfile,
    payload: &[u8],
    limits: InputLimits,
) -> Result<InputEnvelope, InputRefusal> {
    if payload.len() > limits.payload() {
        return Err(InputRefusal::PayloadTooLarge);
    }
    let name = profile.name();
    let namespace = name.namespace().written().as_bytes();
    let stem = name.stem().written().as_bytes();
    let width = [namespace.len(), stem.len(), payload.len()]
        .into_iter()
        .try_fold(104usize, usize::checked_add)
        .ok_or(InputRefusal::SizeOutsidePlatform)?;
    if width > limits.envelope() {
        return Err(InputRefusal::EnvelopeTooLarge);
    }
    let mut body = Vec::new();
    body.extend_from_slice(&INPUT_FORMAT_VERSION.to_be_bytes());
    name.encode_into(&mut body);
    body.extend_from_slice(&profile.version().to_be_bytes());
    encode_bytes(profile.schema().as_bytes(), &mut body);
    encode_bytes(payload, &mut body);
    let address = ContentAddress::derived(INPUT_CASE_TAG, &body);
    let mut encoded = Vec::with_capacity(width);
    encoded.extend_from_slice(address.as_bytes());
    encoded.extend_from_slice(&body);
    Ok(InputEnvelope::admitted(
        profile,
        InputCaseId::derived(address),
        payload.to_vec(),
        encoded,
    ))
}

//! Bounded input-envelope reading against an independently supplied profile.

use super::{
    INPUT_CASE_TAG, INPUT_FORMAT_VERSION, InputCaseId, InputEnvelope, InputLimits, InputProfile,
    InputRefusal,
};
use crate::identity::{BodyReader, addressed_body};

/// One complete specimen envelope admitted for an independently expected profile.
///
/// The canonical format is owned by [`pack`](super::pack).
///
/// # Errors
///
/// Refuses the envelope bound before hashing, then malformed addressing, format, profile, schema, lengths, payload bounds or trailing bytes before copying any input.
pub fn read(
    expected: InputProfile,
    encoded: &[u8],
    limits: InputLimits,
) -> Result<InputEnvelope, InputRefusal> {
    if encoded.len() > limits.envelope() {
        return Err(InputRefusal::EnvelopeTooLarge);
    }
    let (case, body) = addressed_body(
        encoded,
        INPUT_CASE_TAG,
        InputCaseId::derived,
        InputRefusal::Truncated,
        |_derived| InputRefusal::AddressMismatch,
    )?;
    let payload = read_body(expected, body, limits)?;
    Ok(InputEnvelope::admitted(
        expected,
        case,
        payload.to_vec(),
        encoded.to_vec(),
    ))
}

fn read_body(
    expected: InputProfile,
    body: &[u8],
    limits: InputLimits,
) -> Result<&[u8], InputRefusal> {
    let mut reader = BodyReader::over(body, InputRefusal::Truncated, |declared| {
        InputRefusal::LengthOutsidePlatform { declared }
    });
    let found = reader.u32()?;
    if found != INPUT_FORMAT_VERSION {
        return Err(InputRefusal::UnsupportedFormat { found });
    }
    let namespace = reader.bytes()?;
    let stem = reader.bytes()?;
    let version = reader.u32()?;
    if namespace != expected.name().namespace().written().as_bytes()
        || stem != expected.name().stem().written().as_bytes()
        || version != expected.version()
    {
        return Err(InputRefusal::ProfileMismatch);
    }
    if reader.bytes()? != expected.schema().as_bytes() {
        return Err(InputRefusal::SchemaMismatch);
    }
    let payload = reader.bytes()?;
    if payload.len() > limits.payload() {
        return Err(InputRefusal::PayloadTooLarge);
    }
    if reader.remaining() != 0 {
        return Err(InputRefusal::TrailingEnvelopeBytes {
            count: reader.remaining(),
        });
    }
    Ok(payload)
}

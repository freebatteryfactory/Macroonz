//! Mechanical reading operations for canonical content-addressed bodies.

use super::{ContentAddress, DomainTag};

/// Split one content-addressed envelope and retain its body only when the body derives the leading claim.
pub(crate) fn addressed_body<Address, Refusal>(
    encoded: &[u8],
    tag: DomainTag,
    wrap: impl FnOnce(ContentAddress) -> Address,
    truncated: Refusal,
    mismatch: impl FnOnce(Address) -> Refusal,
) -> Result<(Address, &[u8]), Refusal> {
    let Some((claimed, body)) = encoded.split_at_checked(ContentAddress::WIDTH) else {
        return Err(truncated);
    };
    let derived = ContentAddress::derived(tag, body);
    if claimed != derived.as_bytes() {
        return Err(mismatch(wrap(derived)));
    }
    Ok((wrap(derived), body))
}

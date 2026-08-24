//! The complete preimages the two generation identities are derived over.
//!
//! Two primitives, both borrowed from the record instrument rather than invented a second time here:
//!
//! - `u32be(n)` / `u64be(n)` — the integer in four or eight big-endian bytes.
//! - `bytes(x)` — `u64be(x.len())` followed by the bytes of `x`, which is [`crate::report::encode_bytes`].
//!
//! Members follow one another with no separators and no padding.

use super::types::{
    ByteSourceAddress, GENERATION_CHUNK_TAG, GenerationPlan, InputOrigin, SOURCE_CHUNK_BYTES,
};
use crate::identity::ContentAddress;
use crate::report::encode_bytes;

/// The preimage one [`ByteSourceAddress`] is derived from.
///
/// Six members, in this order: the population namespace as `bytes(utf8)`, the population stem as `bytes(utf8)`, the generation profile name as `bytes(utf8)`, that profile's version as `u32be`, the origin arm as the one byte [`InputOrigin::slot`] gives it, and the origin payload as either `u64be` of the seed or `bytes(…)` of the supplied material.
///
/// The budgets and the size progression are deliberately absent.
/// A stream is what the population, the profile, and the origin name; how much of it one run draws and how it is cut into cases are the plan's windowing.
pub(super) fn source_preimage(plan: &GenerationPlan) -> Vec<u8> {
    let mut preimage: Vec<u8> = Vec::new();
    plan.population().name().encode_into(&mut preimage);
    encode_bytes(plan.profile().name().as_bytes(), &mut preimage);
    preimage.extend_from_slice(&plan.profile().version().to_be_bytes());
    preimage.push(plan.origin().slot());
    match plan.origin() {
        InputOrigin::Seeded(seed) => preimage.extend_from_slice(&seed.value().to_be_bytes()),
        InputOrigin::Supplied(material) => encode_bytes(material, &mut preimage),
    }
    preimage
}

/// The bytes of one derived chunk, addressed by its source and its counter.
///
/// Two members, in this order: the source address as `bytes(…)` of the full thirty-two, then the counter as `u64be`.
/// Nothing carries between chunks, so chunk N is a function of the address and N alone.
pub(super) fn chunk_material(address: ByteSourceAddress, counter: u64) -> [u8; SOURCE_CHUNK_BYTES] {
    let mut preimage: Vec<u8> = Vec::new();
    encode_bytes(address.address().as_bytes(), &mut preimage);
    preimage.extend_from_slice(&counter.to_be_bytes());
    *ContentAddress::derived(GENERATION_CHUNK_TAG, &preimage).as_bytes()
}

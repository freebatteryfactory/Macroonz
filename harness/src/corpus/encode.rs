use super::{
    SEED_PACK_FORMAT_VERSION, SEED_PACK_TAG, SeedInput, SeedPack, SeedPackAddress, SeedPackRefusal,
};
use crate::descriptor::PopulationRef;
use crate::identity::{ContentAddress, encode_bytes, encode_length};

/// Write one content-addressed seed pack in authored seed order.
///
/// The address is derived here rather than accepted from anyone.
///
/// # Canonical envelope
///
/// `u32be(n)` and `u64be(n)` are fixed-width big-endian integers, and `bytes(x)` is `u64be(x.len())` followed by `x`.
///
/// The address preimage is exactly this body, with no separators or padding:
///
/// ```text
/// u32be(SEED_PACK_FORMAT_VERSION)
/// bytes(population namespace)
/// bytes(population stem)
/// u64be(seed count)
/// bytes(seed)                       repeated in authored order
/// ```
///
/// The leading address is [`ContentAddress::derived`] under [`SEED_PACK_TAG`] over that body, and the complete envelope is the address bytes followed by the body.
/// The address never covers itself.
///
/// # Errors
///
/// Refuses a pack with no seed, then the first exact repeat in pack order.
pub fn pack(population: PopulationRef, seeds: Vec<SeedInput>) -> Result<SeedPack, SeedPackRefusal> {
    let body = encode_body(population, &seeds);
    let address = SeedPackAddress::derived(ContentAddress::derived(SEED_PACK_TAG, &body));
    let claim = address.address();
    let mut encoded = Vec::with_capacity(claim.as_bytes().len().saturating_add(body.len()));
    encoded.extend_from_slice(claim.as_bytes());
    encoded.extend_from_slice(&body);
    SeedPack::assembled(population, address, seeds, encoded)
}

fn encode_body(population: PopulationRef, seeds: &[SeedInput]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&SEED_PACK_FORMAT_VERSION.to_be_bytes());
    population.name().encode_into(&mut body);
    encode_length(seeds.len(), &mut body);
    for seed in seeds {
        encode_bytes(seed.bytes(), &mut body);
    }
    body
}

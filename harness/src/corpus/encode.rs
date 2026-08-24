//! Writing a pack: the canonical body, the address over it, and the envelope that carries both.

use super::{
    SEED_PACK_FORMAT_VERSION, SEED_PACK_TAG, SeedInput, SeedPack, SeedPackAddress, SeedPackRefusal,
};
use crate::descriptor::PopulationRef;
use crate::identity::ContentAddress;
use crate::report::{encode_bytes, encode_length};

/// Write one content-addressed seed pack in authored seed order.
///
/// The address is derived here rather than accepted from anyone.
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

/// The complete address preimage of one pack.
fn encode_body(population: PopulationRef, seeds: &[SeedInput]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&SEED_PACK_FORMAT_VERSION.to_be_bytes());
    let name = population.name();
    encode_bytes(name.namespace().written().as_bytes(), &mut body);
    encode_bytes(name.stem().written().as_bytes(), &mut body);
    encode_length(seeds.len(), &mut body);
    for seed in seeds {
        encode_bytes(seed.bytes(), &mut body);
    }
    body
}

use super::{
    SEED_PACK_FORMAT_VERSION, SEED_PACK_TAG, SeedInput, SeedPack, SeedPackAddress, SeedPackRefusal,
};
use crate::descriptor::PopulationRef;
use crate::identity::{BodyReader, addressed_body};

/// Read one content-addressed seed-pack envelope for the population the caller expects.
///
/// This reads exactly the canonical envelope [`pack`](super::pack) writes.
/// The leading claim is settled before a single member of the body is interpreted, and the caller hands in a population already parsed, so foreign bytes never mint a name.
///
/// # Errors
///
/// Refuses, in reading order: truncated address material, a claim the body does not derive, an unsupported format, a population that is not the expected one, a length this platform cannot index, a truncated member, an empty seed, trailing bytes, then a pack holding no seed or a repeated one.
pub fn read(
    expected_population: PopulationRef,
    encoded: &[u8],
) -> Result<SeedPack, SeedPackRefusal> {
    let (address, body) = addressed_body(
        encoded,
        SEED_PACK_TAG,
        SeedPackAddress::derived,
        SeedPackRefusal::Truncated,
        |derived| SeedPackRefusal::AddressMismatch { derived },
    )?;
    let seeds = read_body(expected_population, body)?;
    SeedPack::assembled(expected_population, address, seeds, encoded.to_vec())
}

fn read_body(
    expected_population: PopulationRef,
    body: &[u8],
) -> Result<Vec<SeedInput>, SeedPackRefusal> {
    let mut reader = BodyReader::over(body, SeedPackRefusal::Truncated, |declared| {
        SeedPackRefusal::LengthOutsidePlatform { declared }
    });
    let found = reader.u32()?;
    if found != SEED_PACK_FORMAT_VERSION {
        return Err(SeedPackRefusal::UnsupportedFormat { found });
    }
    let namespace = reader.bytes()?;
    let stem = reader.bytes()?;
    let expected = expected_population.name();
    if namespace != expected.namespace().written().as_bytes()
        || stem != expected.stem().written().as_bytes()
    {
        return Err(SeedPackRefusal::PopulationMismatch);
    }
    let seed_count = reader.count()?;
    let mut seeds = Vec::new();
    for at in 0..seed_count {
        let bytes = reader.bytes()?;
        if bytes.is_empty() {
            return Err(SeedPackRefusal::EmptySeed { at });
        }
        seeds.push(SeedInput::from_envelope(bytes.to_vec()));
    }
    let trailing = reader.remaining();
    if trailing != 0 {
        return Err(SeedPackRefusal::TrailingBytes { count: trailing });
    }
    Ok(seeds)
}

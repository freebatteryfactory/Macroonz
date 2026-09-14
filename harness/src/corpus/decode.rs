use super::{
    SEED_PACK_FORMAT_VERSION, SEED_PACK_TAG, SeedInput, SeedPack, SeedPackAddress, SeedPackLimits,
    SeedPackRefusal,
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
/// Refuses the envelope ceiling before hashing, then malformed addressing, format, population, framing, member or seed-count ceilings, empty seeds, trailing bytes and duplicate or empty rosters.
pub fn read(
    expected_population: PopulationRef,
    encoded: &[u8],
    limits: SeedPackLimits,
) -> Result<SeedPack, SeedPackRefusal> {
    if encoded.len() > limits.bytes().envelope() {
        return Err(SeedPackRefusal::EnvelopeTooLarge);
    }
    let (address, body) = addressed_body(
        encoded,
        SEED_PACK_TAG,
        SeedPackAddress::derived,
        SeedPackRefusal::Truncated,
        |derived| SeedPackRefusal::AddressMismatch { derived },
    )?;
    let seeds = read_body(expected_population, body, limits)?;
    SeedPack::assembled(expected_population, address, seeds, encoded.to_vec())
}

fn read_body(
    expected_population: PopulationRef,
    body: &[u8],
    limits: SeedPackLimits,
) -> Result<Vec<SeedInput>, SeedPackRefusal> {
    let mut reader = BodyReader::over(body, SeedPackRefusal::Truncated, |declared| {
        SeedPackRefusal::LengthOutsidePlatform { declared }
    });
    let found = reader.u32()?;
    if found != SEED_PACK_FORMAT_VERSION {
        return Err(SeedPackRefusal::UnsupportedFormat { found });
    }
    let namespace = reader.bounded_bytes(limits.bytes().field(), SeedPackRefusal::FieldTooLarge)?;
    let stem = reader.bounded_bytes(limits.bytes().field(), SeedPackRefusal::FieldTooLarge)?;
    let expected = expected_population.name();
    if namespace != expected.namespace().written().as_bytes()
        || stem != expected.stem().written().as_bytes()
    {
        return Err(SeedPackRefusal::PopulationMismatch);
    }
    let seed_count = reader.bounded_count(limits.seeds(), SeedPackRefusal::TooManySeeds)?;
    let mut seeds = Vec::new();
    for at in 0..seed_count {
        let bytes = reader.bounded_bytes(limits.bytes().field(), SeedPackRefusal::FieldTooLarge)?;
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

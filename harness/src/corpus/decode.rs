use super::{
    SEED_PACK_FORMAT_VERSION, SEED_PACK_TAG, SeedInput, SeedPack, SeedPackAddress, SeedPackRefusal,
};
use crate::descriptor::PopulationRef;
use crate::identity::ContentAddress;

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
    let (address, body) = addressed_body(encoded)?;
    let seeds = read_body(expected_population, body)?;
    SeedPack::assembled(expected_population, address, seeds, encoded.to_vec())
}

/// Split the envelope at its address claim and keep the body only if the body derives that claim.
fn addressed_body(encoded: &[u8]) -> Result<(SeedPackAddress, &[u8]), SeedPackRefusal> {
    let width = ContentAddress::derived(SEED_PACK_TAG, &[]).as_bytes().len();
    let Some((claimed, body)) = encoded.split_at_checked(width) else {
        return Err(SeedPackRefusal::Truncated);
    };
    let address = SeedPackAddress::derived(ContentAddress::derived(SEED_PACK_TAG, body));
    if claimed != address.address().as_bytes() {
        return Err(SeedPackRefusal::AddressMismatch { derived: address });
    }
    Ok((address, body))
}

fn read_body(
    expected_population: PopulationRef,
    body: &[u8],
) -> Result<Vec<SeedInput>, SeedPackRefusal> {
    let mut reader = BodyReader::over(body);
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
    let declared = reader.u64()?;
    let Ok(seed_count) = usize::try_from(declared) else {
        return Err(SeedPackRefusal::LengthOutsidePlatform { declared });
    };
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

struct BodyReader<'body> {
    body: &'body [u8],
    at: usize,
}

impl<'body> BodyReader<'body> {
    const fn over(body: &'body [u8]) -> Self {
        Self { body, at: 0 }
    }

    fn u32(&mut self) -> Result<u32, SeedPackRefusal> {
        self.fixed::<4>().map(u32::from_be_bytes)
    }

    fn u64(&mut self) -> Result<u64, SeedPackRefusal> {
        self.fixed::<8>().map(u64::from_be_bytes)
    }

    fn bytes(&mut self) -> Result<&'body [u8], SeedPackRefusal> {
        let declared = self.u64()?;
        let Ok(length) = usize::try_from(declared) else {
            return Err(SeedPackRefusal::LengthOutsidePlatform { declared });
        };
        self.take(length)
    }

    fn fixed<const WIDTH: usize>(&mut self) -> Result<[u8; WIDTH], SeedPackRefusal> {
        let bytes = self.take(WIDTH)?;
        <[u8; WIDTH]>::try_from(bytes).map_err(|_unexpected_width| SeedPackRefusal::Truncated)
    }

    fn take(&mut self, width: usize) -> Result<&'body [u8], SeedPackRefusal> {
        let Some(end) = self.at.checked_add(width) else {
            return Err(SeedPackRefusal::Truncated);
        };
        let Some(bytes) = self.body.get(self.at..end) else {
            return Err(SeedPackRefusal::Truncated);
        };
        self.at = end;
        Ok(bytes)
    }

    const fn remaining(&self) -> usize {
        self.body.len().saturating_sub(self.at)
    }
}

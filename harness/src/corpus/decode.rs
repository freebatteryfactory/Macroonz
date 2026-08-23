//! Reading an untrusted seed-pack envelope under one caller-declared expected population.

use super::{
    SEED_PACK_FORMAT_VERSION, SEED_PACK_TAG, SeedInput, SeedPack, SeedPackAddress, SeedPackRefusal,
};
use crate::descriptor::PopulationRef;
use crate::identity::ContentAddress;

/// A cursor over one seed-pack body.
struct BodyReader<'body> {
    body: &'body [u8],
    at: usize,
}

impl<'body> BodyReader<'body> {
    /// Open at the first body byte.
    const fn over(body: &'body [u8]) -> Self {
        Self { body, at: 0 }
    }

    /// Read one fixed-width 32-bit integer.
    fn u32(&mut self) -> Result<u32, SeedPackRefusal> {
        let bytes = self.fixed::<4>()?;
        Ok(u32::from_be_bytes(bytes))
    }

    /// Read one fixed-width 64-bit integer.
    fn u64(&mut self) -> Result<u64, SeedPackRefusal> {
        let bytes = self.fixed::<8>()?;
        Ok(u64::from_be_bytes(bytes))
    }

    /// Read one length-prefixed byte string.
    fn bytes(&mut self) -> Result<&'body [u8], SeedPackRefusal> {
        let declared = self.u64()?;
        let Ok(length) = usize::try_from(declared) else {
            return Err(SeedPackRefusal::LengthOutsidePlatform { declared });
        };
        let Some(end) = self.at.checked_add(length) else {
            return Err(SeedPackRefusal::Truncated);
        };
        let Some(bytes) = self.body.get(self.at..end) else {
            return Err(SeedPackRefusal::Truncated);
        };
        self.at = end;
        Ok(bytes)
    }

    /// Read one fixed-width byte array.
    fn fixed<const WIDTH: usize>(&mut self) -> Result<[u8; WIDTH], SeedPackRefusal> {
        let Some(end) = self.at.checked_add(WIDTH) else {
            return Err(SeedPackRefusal::Truncated);
        };
        let Some(bytes) = self.body.get(self.at..end) else {
            return Err(SeedPackRefusal::Truncated);
        };
        let Ok(fixed) = <[u8; WIDTH]>::try_from(bytes) else {
            return Err(SeedPackRefusal::Truncated);
        };
        self.at = end;
        Ok(fixed)
    }

    /// How many unread bytes remain.
    const fn remaining(&self) -> usize {
        self.body.len().saturating_sub(self.at)
    }
}

/// Read one content-addressed seed-pack envelope for the population the caller expects.
///
/// The leading address is checked before the body's semantic members are interpreted. The caller supplies an already-parsed population, so foreign bytes never mint an authored static name.
///
/// # Errors
///
/// Refuses truncated address material, address mismatch, unsupported format, population mismatch, unrepresentable or truncated lengths, empty or duplicate seeds, then trailing bytes.
pub fn read(
    expected_population: PopulationRef,
    encoded: &[u8],
) -> Result<SeedPack, SeedPackRefusal> {
    let empty_address = ContentAddress::derived(SEED_PACK_TAG, &[]);
    let address_width = empty_address.as_bytes().len();
    let Some((claimed, body)) = encoded.split_at_checked(address_width) else {
        return Err(SeedPackRefusal::Truncated);
    };
    let address = SeedPackAddress::derived(ContentAddress::derived(SEED_PACK_TAG, body));
    if claimed != address.address().as_bytes() {
        return Err(SeedPackRefusal::AddressMismatch { derived: address });
    }

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
        seeds.push(SeedInput::read(bytes.to_vec()));
    }
    let trailing = reader.remaining();
    if trailing != 0 {
        return Err(SeedPackRefusal::TrailingBytes { count: trailing });
    }
    SeedPack::assembled(expected_population, address, seeds, encoded.to_vec())
}

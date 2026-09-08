use super::{
    SeedInput, SeedInputRefusal, SeedPack, SeedPackAddress, SeedPackLimits, SeedPackRefusal,
};
use crate::descriptor::PopulationRef;
use crate::identity::ContentAddress;
use crate::report::archive::ArchiveLimits;
use std::collections::BTreeMap;

impl SeedPackLimits {
    /// The independently selected envelope, member and seed-count ceilings.
    #[must_use]
    pub const fn declared(bytes: ArchiveLimits, seeds: usize) -> Self {
        Self { bytes, seeds }
    }

    /// The envelope and per-member byte ceilings.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }

    /// The maximum number of retained seeds.
    #[must_use]
    pub const fn seeds(self) -> usize {
        self.seeds
    }
}

impl SeedInput {
    /// One exact caller-supplied input.
    ///
    /// # Errors
    ///
    /// Refuses empty material.
    pub fn declared(bytes: Vec<u8>) -> Result<Self, SeedInputRefusal> {
        if bytes.is_empty() {
            return Err(SeedInputRefusal::Empty);
        }
        Ok(Self(bytes))
    }

    /// One seed lifted out of a foreign envelope, once the reader has ruled on its position.
    #[must_use]
    pub(in crate::corpus) const fn from_envelope(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// The exact seed bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl SeedPackAddress {
    /// The address a complete canonical body derives.
    #[must_use]
    pub(in crate::corpus) const fn derived(address: ContentAddress) -> Self {
        Self(address)
    }
}

crate::identity::content_address_reference! {
    /// The content address this pack address carries.
    value SeedPackAddress;
}

impl SeedPack {
    /// Assemble one pack, once the writer or the reader has established that its envelope coheres.
    ///
    /// # Errors
    ///
    /// Refuses a pack with no seed, then the first exact repeat in pack order.
    pub(in crate::corpus) fn assembled(
        population: PopulationRef,
        address: SeedPackAddress,
        seeds: Vec<SeedInput>,
        encoded: Vec<u8>,
    ) -> Result<Self, SeedPackRefusal> {
        if seeds.is_empty() {
            return Err(SeedPackRefusal::NoSeed);
        }
        if let Some(repeat) = duplicate_in(&seeds) {
            return Err(repeat);
        }
        Ok(Self {
            population,
            address,
            seeds,
            encoded,
        })
    }

    /// The population this pack warm-starts.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.population
    }

    /// The address derived over the pack's complete body.
    #[must_use]
    pub const fn address(&self) -> SeedPackAddress {
        self.address
    }

    /// The admitted seeds, in pack order.
    #[must_use]
    pub fn seeds(&self) -> &[SeedInput] {
        &self.seeds
    }

    /// The complete envelope: the leading address claim, then the body it addresses.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
}

fn duplicate_in(seeds: &[SeedInput]) -> Option<SeedPackRefusal> {
    let mut seen: BTreeMap<&[u8], usize> = BTreeMap::new();
    for (duplicate, seed) in seeds.iter().enumerate() {
        if let Some(first) = seen.insert(seed.bytes(), duplicate) {
            return Some(SeedPackRefusal::DuplicateSeed { first, duplicate });
        }
    }
    None
}

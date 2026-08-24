//! Construction and the readers over private fields: nothing else reaches inside a seed or a pack.

use super::{SeedInput, SeedInputRefusal, SeedPack, SeedPackAddress, SeedPackRefusal};
use crate::descriptor::PopulationRef;
use crate::identity::ContentAddress;
use std::collections::BTreeMap;

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

    /// The content address this pack address carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
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

/// The first exact repeat in pack order, as the refusal that names both positions.
fn duplicate_in(seeds: &[SeedInput]) -> Option<SeedPackRefusal> {
    let mut seen: BTreeMap<&[u8], usize> = BTreeMap::new();
    for (duplicate, seed) in seeds.iter().enumerate() {
        if let Some(first) = seen.insert(seed.bytes(), duplicate) {
            return Some(SeedPackRefusal::DuplicateSeed { first, duplicate });
        }
    }
    None
}

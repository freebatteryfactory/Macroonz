//! The corpus instrument's invariant nucleus: seed construction, admitted pack assembly, and readers over private fields.

use super::{SeedInput, SeedInputRefusal, SeedPack, SeedPackAddress, SeedPackRefusal};
use crate::descriptor::PopulationRef;
use crate::identity::ContentAddress;
use std::collections::BTreeMap;

impl SeedInput {
    /// One exact caller-supplied input.
    ///
    /// # Errors
    ///
    /// Refuses empty material because it cannot enter the current supplied-material generation road.
    pub fn declared(bytes: Vec<u8>) -> Result<Self, SeedInputRefusal> {
        if bytes.is_empty() {
            return Err(SeedInputRefusal::Empty);
        }
        Ok(Self(bytes))
    }

    /// One nonempty seed read from a foreign pack after its position-specific refusal was assigned.
    #[must_use]
    pub(crate) const fn read(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// The exact seed bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl SeedPackAddress {
    /// The pack address derived over a complete canonical body.
    #[must_use]
    pub(crate) const fn derived(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The content address this typed pack address carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl SeedPack {
    /// Assemble one pack after the writer or reader established envelope coherence.
    ///
    /// # Errors
    ///
    /// Refuses no seed, then the first exact duplicate in pack order.
    pub(crate) fn assembled(
        population: PopulationRef,
        address: SeedPackAddress,
        seeds: Vec<SeedInput>,
        encoded: Vec<u8>,
    ) -> Result<Self, SeedPackRefusal> {
        if seeds.is_empty() {
            return Err(SeedPackRefusal::NoSeed);
        }
        let mut positions: BTreeMap<&[u8], usize> = BTreeMap::new();
        for (duplicate, seed) in seeds.iter().enumerate() {
            if let Some(first) = positions.insert(seed.bytes(), duplicate) {
                return Err(SeedPackRefusal::DuplicateSeed { first, duplicate });
            }
        }
        drop(positions);
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

    /// The admitted seeds in pack order.
    #[must_use]
    pub fn seeds(&self) -> &[SeedInput] {
        &self.seeds
    }

    /// The complete canonical envelope: leading address claim followed by its addressed body.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
}

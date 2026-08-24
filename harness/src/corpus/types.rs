//! One informed seed, one content-addressed pack, and every reason either is refused.
//!
//! Declarations only; every road that reaches a private field lives in this file's child, `type_guard.rs`.

#[path = "type_guard.rs"]
mod guard;

use crate::descriptor::PopulationRef;
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};

/// The body format this reader understands.
pub const SEED_PACK_FORMAT_VERSION: u32 = 1;

/// The content-address family every pack body is derived under.
///
/// The format version governs how bytes decode; this tag's position governs whether an address held elsewhere still means what it meant.
pub const SEED_PACK_TAG: DomainTag =
    DomainTag::declared("seed-pack", IdentityProfileVersion::declared(1));

/// One nonempty input admitted to a pack.
///
/// Empty material is refused because the warm start hands each seed over as [`InputOrigin::Supplied`](crate::generate::InputOrigin::Supplied), and a generation plan refuses empty supplied material.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeedInput(Vec<u8>);

/// Why one seed input was refused.
#[must_use = "a refusal is the reason a seed input was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeedInputRefusal {
    /// The material carries no byte.
    Empty,
}

/// The content address of one complete pack body.
///
/// Only the writer and the reader mint one, and both derive it under [`SEED_PACK_TAG`].
/// No road wraps the leading bytes of an untrusted envelope as an address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeedPackAddress(ContentAddress);

/// One admitted pack: a population, its seeds in authored order, and the envelope carrying them.
///
/// The envelope is retained exactly as it was derived, so persisting a pack never needs a second writer that could disagree with the first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedPack {
    population: PopulationRef,
    address: SeedPackAddress,
    seeds: Vec<SeedInput>,
    encoded: Vec<u8>,
}

/// Why one pack was not written, or not read.
#[must_use = "a refusal is the reason a seed pack was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedPackRefusal {
    /// The pack declares no seed, and an empty pack warm-starts nothing.
    NoSeed,
    /// Two seeds carry exactly the same bytes, which would narrow the roster in silence.
    DuplicateSeed {
        /// Where the bytes first appear.
        first: usize,
        /// Where they repeat.
        duplicate: usize,
    },
    /// The envelope ends inside a member it had already declared.
    Truncated,
    /// The leading claim is not the address the body derives.
    AddressMismatch {
        /// What the body actually derives.
        derived: SeedPackAddress,
    },
    /// The body declares a format this reader does not understand.
    UnsupportedFormat {
        /// The format position found in the body.
        found: u32,
    },
    /// The encoded population is not the one the caller opened the pack for.
    PopulationMismatch,
    /// A declared length is wider than this platform can index.
    LengthOutsidePlatform {
        /// The unrepresentable length.
        declared: u64,
    },
    /// A foreign seed carries no byte.
    EmptySeed {
        /// Its position in pack order.
        at: usize,
    },
    /// Bytes remain after the last seed the declared count admitted.
    TrailingBytes {
        /// How many are left over.
        count: usize,
    },
}

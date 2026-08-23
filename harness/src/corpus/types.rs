//! The corpus instrument's declarations: one informed seed, one content-addressed pack, and every reason either is refused.
//!
//! Declarations only. Every road that reaches a private field lives in this file's own child, `type_guard.rs`; canonical writing, reading, and warm-start projection live in their role-named modules.

#[path = "type_guard.rs"]
mod guard;

use crate::descriptor::PopulationRef;
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};

/// The seed-pack body format this reader understands.
pub const SEED_PACK_FORMAT_VERSION: u32 = 1;

/// The content-address family of a seed-pack body.
///
/// The format version governs byte decoding; this tag's position governs compatibility among externally held addresses of this preimage family.
pub const SEED_PACK_TAG: DomainTag =
    DomainTag::declared("seed-pack", IdentityProfileVersion::declared(1));

/// One nonempty exact input admitted to a seed pack.
///
/// Empty material is refused because the current warm-start handoff is [`InputOrigin::Supplied`](crate::generate::InputOrigin::Supplied), whose generation plan refuses empty supplied material.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeedInput(Vec<u8>);

/// Why one seed input was refused.
#[must_use = "a refusal is the reason a seed input was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeedInputRefusal {
    /// The input contains no byte and cannot enter the current supplied-material generation road.
    Empty,
}

/// The content address of one complete seed-pack body.
///
/// # Construction
///
/// The pack writer and reader derive this value under [`SEED_PACK_TAG`]. No road wraps the leading bytes of an untrusted envelope as an address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeedPackAddress(ContentAddress);

/// One admitted content-addressed seed pack for one declared population.
///
/// Seed order is retained because it is warm-start exploration order and therefore part of the addressed body. The encoded envelope is retained exactly so a caller may persist it without a second writer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedPack {
    population: PopulationRef,
    address: SeedPackAddress,
    seeds: Vec<SeedInput>,
    encoded: Vec<u8>,
}

/// Why one seed pack was not written or read.
#[must_use = "a refusal is the reason a seed pack was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeedPackRefusal {
    /// The pack declares no seed and therefore cannot warm-start a search.
    NoSeed,
    /// One seed repeats earlier material exactly.
    DuplicateSeed {
        /// The first seed carrying the bytes.
        first: usize,
        /// The later seed repeating them.
        duplicate: usize,
    },
    /// The envelope ends before a declared fixed-width or framed member does.
    Truncated,
    /// The leading address claim differs from the address derived over the body.
    AddressMismatch {
        /// The address the body actually derives.
        derived: SeedPackAddress,
    },
    /// The body declares a format this reader does not understand.
    UnsupportedFormat {
        /// The format position found in the body.
        found: u32,
    },
    /// The encoded population differs from the population the caller expected to open.
    PopulationMismatch,
    /// A foreign length cannot be represented on this platform.
    LengthOutsidePlatform {
        /// The unrepresentable length from the envelope.
        declared: u64,
    },
    /// A foreign seed is empty and cannot enter the current supplied-material generation road.
    EmptySeed {
        /// The seed's position in pack order.
        at: usize,
    },
    /// Bytes remain after every member admitted by the seed count.
    TrailingBytes {
        /// The number of bytes after the admitted body.
        count: usize,
    },
}

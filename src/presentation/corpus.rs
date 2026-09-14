//! Seed-pack inspection through the existing corpus record and refusal readers.

use super::{
    Presentation, context,
    value::{array, hex, object, tagged},
};
use crate::harness::corpus::{SeedPack, SeedPackRefusal};
use serde_json::Value;

/// Project one admitted seed pack without assigning a verdict or replay standing.
pub fn seed_pack(record: &SeedPack) -> Presentation {
    Presentation::projected(
        "seed-pack",
        "macroonz-harness/corpus",
        "recorded",
        object([
            (
                "population",
                context::declared_name(record.population().name()),
            ),
            ("address", hex(record.address().address().as_bytes())),
            ("encoded", hex(record.encoded())),
            (
                "seeds",
                array(record.seeds().iter().map(|seed| hex(seed.bytes()))),
            ),
        ]),
    )
}

/// Project a seed-pack refusal without inventing the caller's read or write phase.
pub fn seed_pack_refusal(record: &SeedPackRefusal) -> Presentation {
    let cause = match record {
        SeedPackRefusal::EnvelopeTooLarge => tagged("envelope-too-large", Value::Null),
        SeedPackRefusal::FieldTooLarge => tagged("field-too-large", Value::Null),
        SeedPackRefusal::TooManySeeds => tagged("too-many-seeds", Value::Null),
        SeedPackRefusal::NoSeed => tagged("no-seed", Value::Null),
        SeedPackRefusal::DuplicateSeed { first, duplicate } => tagged(
            "duplicate-seed",
            object([
                ("first", (*first).into()),
                ("duplicate", (*duplicate).into()),
            ]),
        ),
        SeedPackRefusal::Truncated => tagged("truncated", Value::Null),
        SeedPackRefusal::AddressMismatch { derived } => tagged(
            "address-mismatch",
            object([("derived", hex(derived.address().as_bytes()))]),
        ),
        SeedPackRefusal::UnsupportedFormat { found } => {
            tagged("unsupported-format", (*found).into())
        }
        SeedPackRefusal::PopulationMismatch => tagged("population-mismatch", Value::Null),
        SeedPackRefusal::LengthOutsidePlatform { declared } => {
            tagged("length-outside-platform", (*declared).into())
        }
        SeedPackRefusal::EmptySeed { at } => tagged("empty-seed", (*at).into()),
        SeedPackRefusal::TrailingBytes { count } => tagged("trailing-bytes", (*count).into()),
    };
    Presentation::projected(
        "seed-pack-refusal",
        "macroonz-harness/corpus",
        "recorded",
        cause,
    )
}

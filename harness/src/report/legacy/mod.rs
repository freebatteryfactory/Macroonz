#![doc = include_str!("README.md")]

mod types;

pub use types::{
    LEGACY_SOURCE_TAG, LegacyField, LegacyInputProfile, LegacyLimits, LegacyPresence,
    LegacyProfile, LegacyRecord, LegacyRefusal, read_record,
};

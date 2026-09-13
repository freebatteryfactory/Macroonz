//! Native storage causes without an inferred transaction state or subject verdict.

use super::refusal;
use crate::native_storage::StorageError;
use crate::presentation::{Presentation, value::tagged};
use serde_json::Value;

/// Project a storage refusal without reading files or changing transaction custody.
pub fn storage_error(record: &StorageError) -> Presentation {
    Presentation::projected(
        "storage-error",
        "macroonz/native_storage",
        "recorded",
        cause(record),
    )
}

pub(super) fn cause(record: &StorageError) -> Value {
    match record {
        StorageError::InvalidName => tagged("invalid-name", Value::Null),
        StorageError::EmptyBatch => tagged("empty-batch", Value::Null),
        StorageError::ArtifactBound => tagged("artifact-bound", Value::Null),
        StorageError::ByteBound => tagged("byte-bound", Value::Null),
        StorageError::DuplicateName => tagged("duplicate-name", Value::Null),
        StorageError::Busy => tagged("busy", Value::Null),
        StorageError::Collision => tagged("collision", Value::Null),
        StorageError::Incomplete => tagged("incomplete", Value::Null),
        StorageError::UnexpectedEntry => tagged("unexpected-entry", Value::Null),
        StorageError::InventoryMismatch => tagged("inventory-mismatch", Value::Null),
        StorageError::NotRegular => tagged("not-regular", Value::Null),
        StorageError::InvalidMarker => tagged("invalid-marker", Value::Null),
        StorageError::Unavailable => tagged("unavailable", Value::Null),
        StorageError::Io(error) => tagged("io", refusal::io_error(error)),
    }
}

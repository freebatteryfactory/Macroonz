//! Destination observations retain the full discrepancy sequence and owner-defined currency.

use super::super::{refusal, storage};
use super::inventory;
use crate::native_publication::{
    DestinationCheck, DestinationError, DestinationProblem, DestinationState,
};
use crate::presentation::{
    Presentation,
    value::{array, object, tagged},
};
use serde_json::Value;

/// Project every reached destination discrepancy and the owner's current-state judgment.
pub fn publication_destination_check(record: &DestinationCheck) -> Presentation {
    Presentation::projected(
        "publication-destination-check",
        "macroonz/native_publication/destination",
        "recorded",
        check(record),
    )
}

/// Project a destination refusal without inferring whether any installation steps were applied.
pub fn publication_destination_error(record: &DestinationError) -> Presentation {
    Presentation::projected(
        "publication-destination-error",
        "macroonz/native_publication/destination",
        "recorded",
        error(record),
    )
}

pub(super) fn check(record: &DestinationCheck) -> Value {
    let state = match record.state() {
        DestinationState::Uninitialized => "uninitialized",
        DestinationState::Installed => "installed",
        DestinationState::Preparing => "preparing",
        DestinationState::Updating => "updating",
    };
    object([
        ("state", state.into()),
        ("is_current", record.is_current().into()),
        (
            "issues",
            array(record.issues().iter().map(|issue| {
                object([
                    ("path", issue.path.spelling().into()),
                    ("problem", problem(issue.problem).into()),
                ])
            })),
        ),
    ])
}

pub(super) fn error(record: &DestinationError) -> Value {
    match record {
        DestinationError::Inventory(value) => tagged("inventory", inventory::error(value)),
        DestinationError::Metadata(detail) => tagged("metadata", detail.as_str().into()),
        DestinationError::Entry(detail) => tagged("entry", detail.as_str().into()),
        DestinationError::Storage(value) => tagged("storage", storage::cause(value)),
        DestinationError::Filesystem(value) => tagged("filesystem", refusal::io_error(value)),
        DestinationError::MissingLock => tagged("missing-lock", Value::Null),
        DestinationError::Pending => tagged("pending", Value::Null),
        DestinationError::Conflict(detail) => tagged("conflict", detail.as_str().into()),
        DestinationError::Incomplete => tagged("incomplete", Value::Null),
        DestinationError::Unavailable => tagged("unavailable", Value::Null),
    }
}

fn problem(record: DestinationProblem) -> &'static str {
    match record {
        DestinationProblem::Missing => "missing",
        DestinationProblem::Unowned => "unowned",
        DestinationProblem::Stale => "stale",
        DestinationProblem::Tampered => "tampered",
        DestinationProblem::ExtraOwned => "extra-owned",
    }
}

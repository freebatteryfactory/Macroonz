//! Command output and phase-specific failures without repeating command execution.

use super::super::{refusal, storage};
use super::{destination, formatting, inventory, staging};
use crate::compiler::Kind;
use crate::native_publication::{BakeCause, BakeError, BakeOutput, PreparationError};
use crate::presentation::{
    Presentation,
    value::{array, object, optional, tagged},
};
use serde_json::Value;

/// Project a supplied command output without authenticating command execution.
pub fn bake_output<K: Kind>(record: &BakeOutput<K>) -> Presentation {
    let value = match record {
        BakeOutput::Declared(value) => tagged("declared", inventory::declared(value)),
        BakeOutput::Prepared(value) => tagged("prepared", inventory::prepared(value)),
        BakeOutput::Checked {
            prepared,
            comparison,
        } => tagged(
            "checked",
            object([
                ("prepared", inventory::prepared(prepared)),
                ("comparison", destination::check(comparison)),
            ]),
        ),
        BakeOutput::Generated(value) => tagged("generated", staging::compiled(value)),
        BakeOutput::Recovered => tagged("recovered", Value::Null),
    };
    let standing = if matches!(record, BakeOutput::Recovered) {
        "historical-unauthenticated"
    } else {
        "recorded"
    };
    Presentation::projected(
        "bake-output",
        "macroonz/native_publication/command",
        standing,
        value,
    )
}

/// Project command failure while preserving cleanup custody and caller-owned debug text.
pub fn bake_error<K: Kind, E: std::fmt::Debug>(record: &BakeError<K, E>) -> Presentation {
    Presentation::projected(
        "bake-error",
        "macroonz/native_publication/command",
        "recorded",
        object([
            ("pending_cleanup", record.is_pending().into()),
            ("cause", cause(record.cause())),
        ]),
    )
}

fn cause<K: Kind, E: std::fmt::Debug>(record: &BakeCause<K, E>) -> Value {
    match record {
        BakeCause::Declaration(value) => tagged(
            "declaration",
            object([
                ("owner", "caller".into()),
                ("representation", "debug-text".into()),
                ("detail", format!("{value:?}").into()),
            ]),
        ),
        BakeCause::Configuration(detail) => tagged("configuration", detail.as_str().into()),
        BakeCause::Inventory(value) => tagged("inventory", inventory::error(value)),
        BakeCause::Storage(value) => tagged("storage", storage::cause(value)),
        BakeCause::Filesystem(value) => tagged("filesystem", refusal::io_error(value)),
        BakeCause::Formatter(value) => tagged("formatter", formatting::error(value)),
        BakeCause::Formatting { completed, run } => tagged(
            "formatting",
            object([
                ("completed", array(completed.iter().map(formatting::output))),
                ("run", formatting::run(run)),
            ]),
        ),
        BakeCause::Preparation(value) => tagged("preparation", preparation(value)),
        BakeCause::Staging(value) => tagged("staging", staging::error(value)),
        BakeCause::Compilation(value) => tagged("compilation", staging::run(value)),
        BakeCause::Destination { error, compiled } => tagged(
            "destination",
            object([
                ("error", destination::error(error)),
                ("compiled", optional(compiled.as_deref(), staging::compiled)),
            ]),
        ),
    }
}

fn preparation(record: &PreparationError) -> Value {
    match record {
        PreparationError::Inventory => tagged("inventory", Value::Null),
        PreparationError::Source => tagged("source", Value::Null),
        PreparationError::Formatter(value) => tagged("formatter", formatting::observation(value)),
        PreparationError::ByteBound => tagged("byte-bound", Value::Null),
    }
}

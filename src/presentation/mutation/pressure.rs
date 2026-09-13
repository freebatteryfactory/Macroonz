//! Historical compiled pressure with complete source buffers and supporting records.

use super::{backend_history, historical, parity, surface};
use crate::harness::muterprater::{
    ArtifactContent,
    backend_archive::ArchivedSuitePressure,
    specimen_archive::{ArchivedProjectionPressure, ArchivedSpecimenStanding},
};
use crate::presentation::{
    Presentation, context, historical as report,
    value::{hex, object},
};
use serde_json::Value;

/// Historical selected-projection pressure with the retained parity and both source artifacts.
pub fn archived_projection(record: &ArchivedProjectionPressure) -> Presentation {
    Presentation::projected(
        "compiled-projection-pressure",
        "harness/muterprater/specimen/archive",
        "historical-unauthenticated",
        projection(record),
    )
}

pub(super) fn projection(record: &ArchivedProjectionPressure) -> Value {
    object([
        ("archive_address", hex(record.address().as_bytes())),
        ("parity", parity::reading(record.parity())),
        ("baseline_content", content(record.baseline_content())),
        ("selected_content", content(record.selected_content())),
        ("standing", standing(record.standing())),
        (
            "baseline_report",
            report::trial_value(record.baseline_report()),
        ),
        (
            "selected_report",
            report::trial_value(record.selected_report()),
        ),
        ("mutation", historical::report(record.mutation())),
    ])
}

fn content(record: &ArtifactContent) -> Value {
    object([
        ("identity", hex(record.identity().address().as_bytes())),
        ("bytes", hex(record.bytes())),
    ])
}

fn standing(record: &ArchivedSpecimenStanding) -> Value {
    object([
        ("artifact", hex(record.artifact().address().as_bytes())),
        ("pair", parity::pair(record.pair())),
        ("selection", surface::selection(record.selection())),
        (
            "execution",
            context::historical_execution(record.execution()),
        ),
        ("check", context::historical_name(record.check())),
    ])
}

pub(super) fn suite(record: &ArchivedSuitePressure) -> Value {
    object([
        ("archive_address", hex(record.address().as_bytes())),
        ("checked_version", record.checked_version().into()),
        ("manifest", backend_history::manifest(record.manifest())),
        ("kill_ordinal", record.kill_ordinal().into()),
        ("kill", historical::report(record.kill())),
    ])
}

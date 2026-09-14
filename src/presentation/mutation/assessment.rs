//! Historical assessments with retained compiled meanings and actual witness reports.

use super::{historical, parity, pressure, surface};
use crate::harness::muterprater::interpretation_archive::ArchivedAssessment;
use crate::harness::properties::Agreement;
use crate::presentation::{
    Presentation, descriptor, historical as report,
    value::{array, hex, object},
};

/// A historical mutation assessment with every retained road and its witness judgment.
pub fn archived_assessment(record: &ArchivedAssessment) -> Presentation {
    let roads = [
        "production",
        "baseline-evaluation",
        "compiled-baseline",
        "compiled-selected",
        "selected-evaluation",
    ]
    .into_iter()
    .zip(record.meanings())
    .zip(record.reports())
    .map(|((role, meaning), trial)| {
        object([
            ("role", role.into()),
            ("meaning", parity::encoded_value(meaning)),
            ("report", report::trial_value(trial)),
        ])
    });
    Presentation::projected(
        "mutation-assessment",
        "harness/muterprater/interpretation/archive",
        "historical-unauthenticated",
        object([
            ("archive_address", hex(record.address().as_bytes())),
            ("surface", surface::surface(record.surface())),
            ("pair", parity::pair(record.pair())),
            ("selection", surface::selection(record.selection())),
            (
                "baseline_content",
                pressure::content(record.baseline_content()),
            ),
            (
                "selected_content",
                pressure::content(record.selected_content()),
            ),
            ("witness", descriptor::binding(record.witness())),
            ("input", parity::encoded_value(record.input())),
            ("roads", array(roads)),
            ("substrate", parity::substrate(record.substrate())),
            (
                "difference",
                match record.difference() {
                    Agreement::Agrees => "agrees",
                    Agreement::Differs => "differs",
                }
                .into(),
            ),
            ("mutation", historical::report(record.mutation())),
        ]),
    )
}

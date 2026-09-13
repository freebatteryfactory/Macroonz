//! Historical interpreted results with every supporting pressure and exact active meaning.

use super::{historical, parity, pressure, surface};
use crate::harness::muterprater::interpretation_archive::ArchivedInterpretedEvidence;
use crate::presentation::{
    Presentation, historical as report,
    value::{hex, object},
};

/// An interpreted historical result with its complete recorded trust inputs.
pub fn archived_interpreted(record: &ArchivedInterpretedEvidence) -> Presentation {
    let trust = record.trust();
    Presentation::projected(
        "interpreted-mutation-evidence",
        "harness/muterprater/interpretation/archive",
        "historical-unauthenticated",
        object([
            ("archive_address", hex(record.address().as_bytes())),
            (
                "trust",
                object([
                    ("surface", surface::surface(trust.surface())),
                    ("suite", pressure::suite(trust.suite())),
                    ("projection", pressure::projection(trust.projection())),
                ]),
            ),
            ("meaning", parity::encoded_value(record.meaning())),
            ("report", report::trial_value(record.report())),
            ("mutation", historical::report(record.mutation())),
        ]),
    )
}

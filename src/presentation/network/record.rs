//! Exact addressed transcript material without reproduction or playback.

use super::declaration;
use crate::harness::network::{TranscriptEntry, TranscriptPack};
use crate::presentation::{
    Presentation,
    value::{array, hex, object, optional},
};
use serde_json::Value;

/// Project an admitted transcript's source claim, full manifest and ordered delivery roster.
pub fn network_transcript(record: &TranscriptPack) -> Presentation {
    Presentation::projected(
        "network-transcript",
        "macroonz-harness/network",
        "recorded",
        object([
            ("address", hex(record.address().address().as_bytes())),
            ("encoded", hex(record.encoded())),
            ("topology", declaration::topology(record.topology())),
            (
                "source_claim",
                declaration::source(record.source_claim()).into(),
            ),
            (
                "simulation_manifest",
                optional(record.simulation_manifest(), |manifest| {
                    object([
                        ("schedule", declaration::schedule(manifest.schedule())),
                        (
                            "actions",
                            array(manifest.actions().iter().map(declaration::action)),
                        ),
                    ])
                }),
            ),
            ("entries", array(record.entries().iter().map(entry))),
        ]),
    )
}

fn entry(value: &TranscriptEntry) -> Value {
    object([
        ("link", declaration::link(value.link())),
        ("ordinal", value.ordinal().ordinal().into()),
        ("payload", hex(value.payload())),
        ("sent_at", value.sent_at().ordinal().into()),
        ("delivered_at", value.delivered_at().ordinal().into()),
        ("copy", declaration::copy(value.copy()).into()),
    ])
}

//! Transcript admission and reproduction refusals retain their existing typed fields.

use super::declaration;
use crate::harness::network::{SendRefusal, SimNetRefusal, TranscriptRefusal};
use crate::presentation::{
    Presentation,
    value::{hex, object, tagged},
};
use serde_json::Value;

/// Project a transcript refusal without guessing the invoking read, write or reproduction road.
pub fn network_transcript_refusal(record: &TranscriptRefusal) -> Presentation {
    Presentation::projected(
        "network-transcript-refusal",
        "macroonz-harness/network",
        "recorded",
        cause(record),
    )
}

fn cause(record: &TranscriptRefusal) -> Value {
    match record {
        TranscriptRefusal::EnvelopeTooLarge => tagged("envelope-too-large", Value::Null),
        TranscriptRefusal::FieldTooLarge => tagged("field-too-large", Value::Null),
        TranscriptRefusal::TooManyActions => tagged("too-many-actions", Value::Null),
        TranscriptRefusal::TooManyEntries => tagged("too-many-entries", Value::Null),
        TranscriptRefusal::NoDelivery => tagged("no-delivery", Value::Null),
        TranscriptRefusal::ForeignLink { at } => tagged("foreign-link", (*at).into()),
        TranscriptRefusal::DeliveryOrderBroken { at } => {
            tagged("delivery-order-broken", (*at).into())
        }
        TranscriptRefusal::DeliveryBeforeSend { at } => {
            tagged("delivery-before-send", (*at).into())
        }
        TranscriptRefusal::Truncated => tagged("truncated", Value::Null),
        TranscriptRefusal::AddressMismatch { derived } => tagged(
            "address-mismatch",
            object([("derived", hex(derived.address().as_bytes()))]),
        ),
        TranscriptRefusal::UnsupportedFormat { found } => {
            tagged("unsupported-format", (*found).into())
        }
        TranscriptRefusal::UnknownSourceClaim { found } => {
            tagged("unknown-source-claim", (*found).into())
        }
        TranscriptRefusal::SourceClaimMismatch { expected, found } => tagged(
            "source-claim-mismatch",
            object([
                ("expected", declaration::source(*expected).into()),
                ("found", declaration::source(*found).into()),
            ]),
        ),
        TranscriptRefusal::ScheduleMismatch => tagged("schedule-mismatch", Value::Null),
        TranscriptRefusal::UnknownFault { found } => tagged("unknown-fault", (*found).into()),
        TranscriptRefusal::UnknownAction { found } => tagged("unknown-action", (*found).into()),
        TranscriptRefusal::UnknownCopy { found } => tagged("unknown-copy", (*found).into()),
        TranscriptRefusal::TopologyMismatch => tagged("topology-mismatch", Value::Null),
        TranscriptRefusal::SimulationActionForeignLink { at } => {
            tagged("simulation-action-foreign-link", (*at).into())
        }
        TranscriptRefusal::SimulationNotOpened(SimNetRefusal::DisciplineForeignLink { link }) => {
            tagged(
                "simulation-not-opened",
                tagged("discipline-foreign-link", declaration::link(*link)),
            )
        }
        TranscriptRefusal::SimulationSendRefused {
            at,
            refusal: SendRefusal::LinkUndeclared(link),
        } => tagged(
            "simulation-send-refused",
            object([
                ("at", (*at).into()),
                (
                    "refusal",
                    tagged("link-undeclared", declaration::link(*link)),
                ),
            ]),
        ),
        TranscriptRefusal::SimulationRowsDiverge { at } => {
            tagged("simulation-rows-diverge", (*at).into())
        }
        TranscriptRefusal::RecordedLiveCannotReproduce => {
            tagged("recorded-live-cannot-reproduce", Value::Null)
        }
        TranscriptRefusal::LengthOutsidePlatform { declared } => {
            tagged("length-outside-platform", (*declared).into())
        }
        TranscriptRefusal::TrailingBytes { count } => tagged("trailing-bytes", (*count).into()),
    }
}

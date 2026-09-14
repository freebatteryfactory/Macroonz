//! Historical descriptor failures nested in retained record admission.

use super::value::tagged;
use crate::harness::descriptor::archive::{
    BindingArchiveRefusal, CandidateArchiveRefusal, RowArchiveRefusal,
};
use serde_json::Value;

pub(super) fn binding(record: &BindingArchiveRefusal) -> Value {
    match record {
        BindingArchiveRefusal::Canonical(error) => tagged("canonical", candidate(error)),
        BindingArchiveRefusal::Row(error) => tagged("row", row(error)),
        BindingArchiveRefusal::SizeOutsidePlatform => tagged("size-outside-platform", Value::Null),
        BindingArchiveRefusal::InvalidSlot => tagged("invalid-slot", Value::Null),
        BindingArchiveRefusal::InvalidAddressWidth => tagged("invalid-address-width", Value::Null),
        BindingArchiveRefusal::SubjectMismatch => tagged("subject-mismatch", Value::Null),
        BindingArchiveRefusal::CheckMismatch => tagged("check-mismatch", Value::Null),
        BindingArchiveRefusal::GeneratedWithoutSchemaPin => {
            tagged("generated-without-schema-pin", Value::Null)
        }
    }
}

fn row(record: &RowArchiveRefusal) -> Value {
    match record {
        RowArchiveRefusal::Canonical(error) => tagged("canonical", candidate(error)),
        RowArchiveRefusal::InvalidOrigin { found } => tagged("invalid-origin", (*found).into()),
        RowArchiveRefusal::InvalidReplayGround { found } => {
            tagged("invalid-replay-ground", (*found).into())
        }
        RowArchiveRefusal::InvalidAddressWidth => tagged("invalid-address-width", Value::Null),
    }
}

fn candidate(record: &CandidateArchiveRefusal) -> Value {
    match record {
        CandidateArchiveRefusal::BytesTooLarge => tagged("bytes-too-large", Value::Null),
        CandidateArchiveRefusal::FieldTooLarge => tagged("field-too-large", Value::Null),
        CandidateArchiveRefusal::TooManyLabels => tagged("too-many-labels", Value::Null),
        CandidateArchiveRefusal::Truncated => tagged("truncated", Value::Null),
        CandidateArchiveRefusal::LengthOutsidePlatform { declared } => {
            tagged("length-outside-platform", (*declared).into())
        }
        CandidateArchiveRefusal::UnsupportedFormat { found } => {
            tagged("unsupported-format", (*found).into())
        }
        CandidateArchiveRefusal::NotCandidate { found } => tagged("not-candidate", (*found).into()),
        CandidateArchiveRefusal::InvalidSynthesis => tagged("invalid-synthesis", Value::Null),
        CandidateArchiveRefusal::InvalidName => tagged("invalid-name", Value::Null),
        CandidateArchiveRefusal::NonCanonicalLabels => tagged("non-canonical-labels", Value::Null),
        CandidateArchiveRefusal::TrailingBytes => tagged("trailing-bytes", Value::Null),
    }
}

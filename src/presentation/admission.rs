//! Existing input and report admission failures without reconstructed operation phases.

use super::{
    Presentation,
    value::{object, tagged},
};
use crate::harness::input::InputRefusal;
use crate::harness::report::archive::ArchiveRefusal;
use serde_json::Value;

/// Project an input refusal without claiming that the subject ran.
pub fn input_refusal(record: &InputRefusal) -> Presentation {
    Presentation::projected(
        "input-refusal",
        "macroonz-harness/input",
        "recorded",
        input(record),
    )
}

pub(super) fn input(record: &InputRefusal) -> Value {
    match record {
        InputRefusal::EnvelopeTooLarge => tagged("envelope-too-large", Value::Null),
        InputRefusal::PayloadTooLarge => tagged("payload-too-large", Value::Null),
        InputRefusal::SizeOutsidePlatform => tagged("size-outside-platform", Value::Null),
        InputRefusal::Truncated => tagged("truncated", Value::Null),
        InputRefusal::AddressMismatch => tagged("address-mismatch", Value::Null),
        InputRefusal::UnsupportedFormat { found } => tagged("unsupported-format", (*found).into()),
        InputRefusal::ProfileMismatch => tagged("profile-mismatch", Value::Null),
        InputRefusal::SchemaMismatch => tagged("schema-mismatch", Value::Null),
        InputRefusal::LengthOutsidePlatform { declared } => {
            tagged("length-outside-platform", (*declared).into())
        }
        InputRefusal::TrailingEnvelopeBytes { count } => {
            tagged("trailing-envelope-bytes", (*count).into())
        }
        InputRefusal::DecoderRefused(error) => tagged(
            "decoder-refused",
            object([
                ("owner", "arbitrary".into()),
                ("kind", format!("{error:?}").into()),
                ("detail", error.to_string().into()),
            ]),
        ),
        InputRefusal::TrailingInputBytes { count } => {
            tagged("trailing-input-bytes", (*count).into())
        }
    }
}

/// Project the report archive owner's refusal without promoting historical material.
pub fn archive_refusal(record: &ArchiveRefusal) -> Presentation {
    Presentation::projected(
        "archive-refusal",
        "macroonz-harness/report/archive",
        "recorded",
        archive(record),
    )
}

pub(super) fn archive(record: &ArchiveRefusal) -> Value {
    match record {
        ArchiveRefusal::EnvelopeTooLarge => tagged("envelope-too-large", Value::Null),
        ArchiveRefusal::FieldTooLarge => tagged("field-too-large", Value::Null),
        ArchiveRefusal::SizeOutsidePlatform => tagged("size-outside-platform", Value::Null),
        ArchiveRefusal::Truncated => tagged("truncated", Value::Null),
        ArchiveRefusal::LengthOutsidePlatform { declared } => {
            tagged("length-outside-platform", (*declared).into())
        }
        ArchiveRefusal::AddressMismatch => tagged("address-mismatch", Value::Null),
        ArchiveRefusal::UnsupportedFormat { found } => {
            tagged("unsupported-format", (*found).into())
        }
        ArchiveRefusal::WrongKind { found } => tagged("wrong-kind", (*found).into()),
        ArchiveRefusal::UnsupportedCustody { found } => {
            tagged("unsupported-custody", (*found).into())
        }
        ArchiveRefusal::InvalidSlot => tagged("invalid-slot", Value::Null),
        ArchiveRefusal::InvalidText => tagged("invalid-text", Value::Null),
        ArchiveRefusal::InvalidAddressWidth => tagged("invalid-address-width", Value::Null),
        ArchiveRefusal::TrailingBytes => tagged("trailing-bytes", Value::Null),
        ArchiveRefusal::IdentityJoinMismatch => tagged("identity-join-mismatch", Value::Null),
        ArchiveRefusal::PostureMismatch => tagged("posture-mismatch", Value::Null),
        ArchiveRefusal::InvalidForeignText => tagged("invalid-foreign-text", Value::Null),
        ArchiveRefusal::InvalidMeasurement => tagged("invalid-measurement", Value::Null),
        ArchiveRefusal::TooManyRows => tagged("too-many-rows", Value::Null),
        ArchiveRefusal::DuplicateTrial => tagged("duplicate-trial", Value::Null),
        ArchiveRefusal::RunContextMismatch => tagged("run-context-mismatch", Value::Null),
        ArchiveRefusal::SelectionMismatch => tagged("selection-mismatch", Value::Null),
    }
}

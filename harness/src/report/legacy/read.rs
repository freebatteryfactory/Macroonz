//! Whole-source admission around the private streaming decoder.

use super::{
    Admission, DigestSeed, LegacyField, LegacyLimits, LegacyPresence, LegacyProfile, LegacyRecord,
    LegacyRefusal, Nullable, NumberSeed, ProfileSeed, RecordSeed, TextSeed, WitnessSeed,
};
use serde::de::DeserializeSeed;
use serde::de::MapAccess;
use std::collections::BTreeSet;

/// Admit only the historical fields present in one bounded JSON source.
///
/// # Errors
///
/// Refuses unsupported fields, malformed values, mismatched present headers or exceeded resource ceilings.
pub fn read_record(
    source: &[u8],
    expected: LegacyProfile<'_>,
    limits: LegacyLimits,
) -> Result<LegacyRecord, LegacyRefusal> {
    if source.len() > limits.source {
        return Err(LegacyRefusal::SourceTooLarge);
    }
    if source.len() > limits.retained {
        return Err(LegacyRefusal::RetainedTooLarge);
    }
    let mut admission = Admission {
        limits,
        retained: source.len(),
        members: 0,
        seen: BTreeSet::new(),
        refusal: None,
    };
    let mut decoder = serde_json::Deserializer::from_slice(source);
    let parsed = RecordSeed(&mut admission)
        .deserialize(&mut decoder)
        .and_then(|record| decoder.end().map(|()| record));
    let mut record = parsed.map_err(|error| {
        admission.refusal.unwrap_or(LegacyRefusal::InvalidJson {
            line: error.line(),
            column: error.column(),
        })
    })?;
    if let LegacyPresence::Present(kind) = &record.kind
        && kind != expected.kind
    {
        return Err(LegacyRefusal::KindMismatch);
    }
    if let LegacyPresence::Present(schema) = record.schema
        && schema != expected.schema
    {
        return Err(LegacyRefusal::SchemaMismatch);
    }
    record.source = source.to_vec();
    Ok(record)
}

pub(super) fn read_member<'de, A: MapAccess<'de>>(
    record: &mut LegacyRecord,
    field: LegacyField,
    object: &mut A,
    admission: &mut Admission,
) -> Result<(), A::Error> {
    match field {
        LegacyField::Kind => {
            record.kind = LegacyPresence::Present(object.next_value_seed(TextSeed(admission))?);
        }
        LegacyField::Schema => {
            record.schema = LegacyPresence::Present(object.next_value_seed(NumberSeed(admission))?);
        }
        LegacyField::Witness => {
            record.witness = object.next_value_seed(Nullable(WitnessSeed(admission)))?;
        }
        LegacyField::InputProfile => {
            record.input_profile = object.next_value_seed(Nullable(ProfileSeed(admission)))?;
        }
        LegacyField::TrialName => {
            record.trial_name = object.next_value_seed(Nullable(TextSeed(admission)))?;
        }
        LegacyField::SubjectName => {
            record.subject_name = object.next_value_seed(Nullable(TextSeed(admission)))?;
        }
        LegacyField::CheckName => {
            record.check_name = object.next_value_seed(Nullable(TextSeed(admission)))?;
        }
        LegacyField::SubjectRevision => {
            record.subject_revision = object.next_value_seed(Nullable(NumberSeed(admission)))?;
        }
        LegacyField::CheckRevision => {
            record.check_revision = object.next_value_seed(Nullable(NumberSeed(admission)))?;
        }
        LegacyField::Target => {
            record.target = object.next_value_seed(Nullable(TextSeed(admission)))?;
        }
        LegacyField::Toolchain => {
            record.toolchain = object.next_value_seed(Nullable(TextSeed(admission)))?;
        }
        LegacyField::ExecutionDigest => {
            record.execution_digest = object.next_value_seed(Nullable(DigestSeed(admission)))?;
        }
        LegacyField::FingerprintDigest => {
            record.fingerprint_digest = object.next_value_seed(Nullable(DigestSeed(admission)))?;
        }
        LegacyField::ReportedOutcome => {
            record.reported_outcome = object.next_value_seed(Nullable(TextSeed(admission)))?;
        }
        LegacyField::ProfileName | LegacyField::ProfileRevision => {
            return Err(admission.refuse(LegacyRefusal::UnknownField));
        }
    }
    Ok(())
}

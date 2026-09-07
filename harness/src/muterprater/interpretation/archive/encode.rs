//! Historical parity retention over explicit caller encodings and existing record owners.

use super::size::{admit_total, known_size, value_size};
use super::{
    ArchivedParity, ArchivedParityDisposition, PARITY_ARCHIVE_TAG, ParityArchiveLimits,
    ParityArchiveRefusal, ValueEncoder, ValueRole, read_parity,
};
use crate::descriptor::archive::{retain_binding, write_revision};
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::muterprater::{
    NoMutationParityReading, NoMutationParityStanding, ParityQualificationRefusal,
};
use crate::properties::SharedSubstrate;
use crate::report::TrialConclusion;
use crate::report::archive::{retain_trial, sum, write_finding};

/// Retain a complete raw reading through explicit typed input and meaning encoders.
///
/// # Errors
///
/// Refuses known metadata bounds before callbacks, then a callback refusal, oversized value or oversized complete envelope.
/// Each reached callback runs once with ordinary Rust effects, allocation and unwinding; retention cannot prove an earlier interior-state snapshot.
pub fn retain_parity<Input, Meaning>(
    reading: &NoMutationParityReading<'_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    limits: ParityArchiveLimits,
) -> Result<ArchivedParity, ParityArchiveRefusal> {
    retain(
        reading,
        input,
        meaning,
        ArchivedParityDisposition::Raw,
        limits,
    )
}

/// Retain a qualified or rejected reading with its actual historical disposition.
///
/// # Errors
///
/// Applies the same encoder and resource refusals as [`retain_parity`], borrowing the source on failure.
pub fn retain_parity_standing<Input, Meaning>(
    standing: &NoMutationParityStanding<'_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    limits: ParityArchiveLimits,
) -> Result<ArchivedParity, ParityArchiveRefusal> {
    match standing {
        NoMutationParityStanding::Qualified(qualified) => retain(
            qualified.reading(),
            input,
            meaning,
            ArchivedParityDisposition::Qualified,
            limits,
        ),
        NoMutationParityStanding::Rejected(rejected) => retain(
            rejected.reading(),
            input,
            meaning,
            ArchivedParityDisposition::Rejected(rejected.cause()),
            limits,
        ),
    }
}

fn retain<Input, Meaning>(
    reading: &NoMutationParityReading<'_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    disposition: ArchivedParityDisposition,
    limits: ParityArchiveLimits,
) -> Result<ArchivedParity, ParityArchiveRefusal> {
    let known = known_size(reading, input, meaning, disposition, limits)?;
    let input_bytes = encode_value(input, reading.input(), ValueRole::Input, limits)?;
    let production_bytes =
        encode_value(meaning, reading.production(), ValueRole::Production, limits)?;
    let evaluation_bytes =
        encode_value(meaning, reading.evaluation(), ValueRole::Evaluation, limits)?;
    let total = admit_total(
        sum(&[
            known,
            input_bytes.len(),
            production_bytes.len(),
            evaluation_bytes.len(),
        ])?,
        limits,
    )?;
    let witness = retain_binding(reading.witness().binding(), limits.binding())?;
    let production_report = retain_trial(reading.production_report(), limits.trial())?;
    let evaluation_report = retain_trial(reading.evaluation_report(), limits.trial())?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    let pair = reading.pair().standing();
    pair.family().name().encode_into(&mut body);
    write_revision(pair.production_revision(), &mut body);
    write_revision(pair.evaluation_revision(), &mut body);
    encode_bytes(pair.surface().address().as_bytes(), &mut body);
    encode_bytes(witness.encoded(), &mut body);
    write_convention(input, &mut body);
    encode_bytes(&input_bytes, &mut body);
    write_convention(meaning, &mut body);
    encode_bytes(&production_bytes, &mut body);
    encode_bytes(&evaluation_bytes, &mut body);
    body.extend_from_slice(&reading.evaluation_firings().to_be_bytes());
    write_substrate(reading.substrate(), &mut body);
    match reading.conclusion() {
        TrialConclusion::Passed => body.push(0),
        TrialConclusion::Refused(finding) => {
            body.push(1);
            write_finding(reading.production_report().trial(), finding, &mut body);
        }
    }
    encode_bytes(production_report.encoded(), &mut body);
    encode_bytes(evaluation_report.encoded(), &mut body);
    write_disposition(disposition, &mut body);
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(PARITY_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_parity(&encoded, limits)
}

fn encode_value<Value>(
    encoder: &ValueEncoder<Value>,
    value: &Value,
    role: ValueRole,
    limits: ParityArchiveLimits,
) -> Result<Vec<u8>, ParityArchiveRefusal> {
    let bytes = encoder
        .encode(value)
        .map_err(|cause| ParityArchiveRefusal::Encoder { role, cause })?;
    value_size(bytes.len(), role, limits)?;
    Ok(bytes)
}

fn write_convention<Value>(encoder: &ValueEncoder<Value>, body: &mut Vec<u8>) {
    encoder.convention().encode_into(body);
    body.extend_from_slice(&encoder.version().to_be_bytes());
    encode_bytes(encoder.schema().as_bytes(), body);
    write_revision(encoder.revision(), body);
}

fn write_substrate(substrate: &SharedSubstrate, body: &mut Vec<u8>) {
    match substrate {
        SharedSubstrate::DeclaredIndependent => body.push(0),
        SharedSubstrate::Standing(roster) => {
            body.push(1);
            encode_length(roster.standing().len(), body);
            for foundation in roster.standing() {
                foundation.name().encode_into(body);
            }
        }
    }
}

fn write_disposition(disposition: ArchivedParityDisposition, body: &mut Vec<u8>) {
    let ArchivedParityDisposition::Rejected(cause) = disposition else {
        body.push(u8::from(disposition != ArchivedParityDisposition::Raw));
        return;
    };
    body.push(2);
    match cause {
        ParityQualificationRefusal::ProductionDidNotQualify => body.push(0),
        ParityQualificationRefusal::EvaluationDidNotQualify => body.push(1),
        ParityQualificationRefusal::NoMutationActivated { firings } => {
            body.push(2);
            body.extend_from_slice(&firings.to_be_bytes());
        }
        ParityQualificationRefusal::MeaningsDisagreed => body.push(3),
    }
}

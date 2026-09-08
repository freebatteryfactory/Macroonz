//! Complete known-metadata admission before invoking caller encoders.

use super::{
    ArchivedParityDisposition, ParityArchiveLimits, ParityArchiveRefusal, ValueEncoder, ValueRole,
};
use crate::descriptor::archive::binding_size;
use crate::muterprater::{
    NoMutationParityQualification, NoMutationParityReading, ParityQualificationRefusal,
};
use crate::properties::SharedSubstrate;
use crate::report::TrialConclusion;
use crate::report::archive::{ArchiveRefusal, bounded, finding_size, name_size, sum, trial_size};

pub(crate) fn qualified_size<Input, Meaning>(
    qualified: &NoMutationParityQualification<'_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    limits: ParityArchiveLimits,
) -> Result<usize, ParityArchiveRefusal> {
    known_size(
        qualified.reading(),
        input,
        meaning,
        ArchivedParityDisposition::Qualified,
        limits,
    )
}

pub(super) fn known_size<Input, Meaning>(
    reading: &NoMutationParityReading<'_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    disposition: ArchivedParityDisposition,
    limits: ParityArchiveLimits,
) -> Result<usize, ParityArchiveRefusal> {
    let bytes = limits.bytes();
    bounded(41, bytes)?;
    let pair = name_size(reading.pair().standing().family().name(), bytes)?;
    let witness = bounded(
        binding_size(reading.witness().binding(), limits.binding())?,
        bytes,
    )?;
    let production = bounded(
        trial_size(reading.production_report(), limits.trial())?,
        bytes,
    )?;
    let evaluation = bounded(
        trial_size(reading.evaluation_report(), limits.trial())?,
        bytes,
    )?;
    let conclusion = match reading.conclusion() {
        TrialConclusion::Passed => 1,
        TrialConclusion::Refused(finding) => sum(&[1, finding_size(finding, bytes)?])?,
    };
    let disposition = match disposition {
        ArchivedParityDisposition::Raw | ArchivedParityDisposition::Qualified => 1,
        ArchivedParityDisposition::Rejected(ParityQualificationRefusal::NoMutationActivated {
            firings: _,
        }) => 6,
        ArchivedParityDisposition::Rejected(
            ParityQualificationRefusal::ProductionDidNotQualify
            | ParityQualificationRefusal::EvaluationDidNotQualify
            | ParityQualificationRefusal::MeaningsDisagreed,
        ) => 2,
    };
    let total = sum(&[
        234,
        pair,
        witness,
        production,
        evaluation,
        convention_size(input, limits)?,
        convention_size(meaning, limits)?,
        substrate_size(reading.substrate(), limits)?,
        conclusion,
        disposition,
    ])?;
    admit_total(total, limits)
}

fn convention_size<Value>(
    encoder: &ValueEncoder<Value>,
    limits: ParityArchiveLimits,
) -> Result<usize, ParityArchiveRefusal> {
    Ok(sum(&[
        93,
        name_size(encoder.convention(), limits.bytes())?,
    ])?)
}

fn substrate_size(
    substrate: &SharedSubstrate,
    limits: ParityArchiveLimits,
) -> Result<usize, ParityArchiveRefusal> {
    let SharedSubstrate::Standing(roster) = substrate else {
        return Ok(1);
    };
    if roster.standing().len() > limits.substrates() {
        return Err(ParityArchiveRefusal::TooManySubstrates);
    }
    let mut total = 9usize;
    for name in roster.standing() {
        total = sum(&[total, name_size(name.name(), limits.bytes())?])?;
    }
    Ok(total)
}

pub(super) fn value_size(
    length: usize,
    role: ValueRole,
    limits: ParityArchiveLimits,
) -> Result<usize, ParityArchiveRefusal> {
    if length > limits.value() {
        return Err(ParityArchiveRefusal::ValueTooLarge { role });
    }
    Ok(bounded(length, limits.bytes())?)
}

pub(super) fn admit_total(
    total: usize,
    limits: ParityArchiveLimits,
) -> Result<usize, ParityArchiveRefusal> {
    if total > limits.bytes().envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}

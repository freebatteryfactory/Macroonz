//! Bounded work populations and historical judgments without calling the judge.

use super::super::{
    ArchivedBenchRow, ArchivedSecondaryObservation, ArchivedWorkCause, ArchivedWorkConclusion,
    ArchivedWorkCount, ArchivedWorkCurve, ArchivedWorkGap, ArchivedWorkJudgment, ArchivedWorkPoint,
    BenchArchiveLimits, BenchArchiveRefusal,
};
use crate::bench::work::{WorkStage, qualification_stage};
use crate::identity::BodyReader;
use crate::report::archive::{ArchiveRefusal, measurement, name, read_attribution, text};
use std::collections::BTreeSet;

pub(super) fn curve(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    row: &ArchivedBenchRow,
    limits: BenchArchiveLimits,
) -> Result<ArchivedWorkCurve, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let count = reader.count().map_err(Canonical)?;
    let sizes = row.measurement().input_sizes().sizes();
    if count != sizes.len() {
        return Err(BenchArchiveRefusal::WorkPopulationMismatch);
    }
    let mut points = Vec::new();
    for size in sizes {
        let input_size = reader.u64().map_err(Canonical)?;
        if input_size != *size {
            return Err(BenchArchiveRefusal::WorkPopulationMismatch);
        }
        let counts = counts(reader, limits)?;
        if let Some(first) = points.first()
            && !same_counts(first, &counts)
        {
            return Err(BenchArchiveRefusal::WorkPopulationMismatch);
        }
        points.push(ArchivedWorkPoint { input_size, counts });
    }
    Ok(ArchivedWorkCurve { points })
}

fn same_counts(point: &ArchivedWorkPoint, counts: &[ArchivedWorkCount]) -> bool {
    point.counts().len() == counts.len()
        && point
            .counts()
            .iter()
            .zip(counts)
            .all(|(left, right)| left.observation() == right.observation())
}

fn counts(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: BenchArchiveLimits,
) -> Result<Vec<ArchivedWorkCount>, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let count = reader.count().map_err(Canonical)?;
    if count > limits.observations() {
        return Err(BenchArchiveRefusal::TooManyObservations);
    }
    if count == 0 {
        return Err(BenchArchiveRefusal::WorkPopulationMismatch);
    }
    let mut seen = BTreeSet::new();
    let mut counts = Vec::new();
    for _ in 0..count {
        let observation = name(reader, limits.bytes()).map_err(Canonical)?;
        if !seen.insert((
            observation.namespace().to_owned(),
            observation.stem().to_owned(),
        )) {
            return Err(BenchArchiveRefusal::WorkPopulationMismatch);
        }
        counts.push(ArchivedWorkCount {
            observation,
            count: reader.u64().map_err(Canonical)?,
        });
    }
    Ok(counts)
}

pub(super) fn join_curves(
    left: &ArchivedWorkCurve,
    right: &ArchivedWorkCurve,
) -> Result<(), BenchArchiveRefusal> {
    if left.points().len() != right.points().len()
        || !left
            .points()
            .iter()
            .zip(right.points())
            .all(|(a, b)| a.input_size() == b.input_size() && same_counts(a, b.counts()))
    {
        return Err(BenchArchiveRefusal::WorkPopulationMismatch);
    }
    Ok(())
}

fn cause(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: BenchArchiveLimits,
) -> Result<ArchivedWorkCause, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    Ok(ArchivedWorkCause {
        family: text(reader, limits.bytes()).map_err(Canonical)?.to_owned(),
        local: text(reader, limits.bytes()).map_err(Canonical)?.to_owned(),
    })
}

fn conclusion(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: BenchArchiveLimits,
) -> Result<ArchivedWorkConclusion, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    match reader.byte().map_err(Canonical)? {
        0 => Ok(ArchivedWorkConclusion::Satisfied),
        1 => Ok(ArchivedWorkConclusion::Refused(cause(reader, limits)?)),
        _ => Err(Canonical(ArchiveRefusal::InvalidSlot)),
    }
}

pub(super) fn judgment(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: BenchArchiveLimits,
) -> Result<ArchivedWorkJudgment, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let measured = conclusion(reader, limits)?;
    let planted_worse = conclusion(reader, limits)?;
    let gap = match reader.byte().map_err(Canonical)? {
        0 => ArchivedWorkGap::Distinguished,
        1 => ArchivedWorkGap::NotDistinguished(cause(reader, limits)?),
        _ => return Err(Canonical(ArchiveRefusal::InvalidSlot)),
    };
    Ok(ArchivedWorkJudgment {
        measured,
        planted_worse,
        gap,
    })
}

fn conclusion_reading(conclusion: &ArchivedWorkConclusion) -> Result<(), &ArchivedWorkCause> {
    match conclusion {
        ArchivedWorkConclusion::Satisfied => Ok(()),
        ArchivedWorkConclusion::Refused(cause) => Err(cause),
    }
}

pub(super) fn stage(judgment: &ArchivedWorkJudgment) -> WorkStage {
    let gap = match judgment.gap() {
        ArchivedWorkGap::Distinguished => Ok(()),
        ArchivedWorkGap::NotDistinguished(cause) => Err(cause),
    };
    qualification_stage(
        conclusion_reading(judgment.measured()),
        conclusion_reading(judgment.planted_worse()),
        gap,
    )
}

pub(super) fn secondary(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    row: &ArchivedBenchRow,
    primary: &ArchivedWorkCurve,
    limits: BenchArchiveLimits,
) -> Result<ArchivedSecondaryObservation, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let work = curve(reader, row, limits)?;
    join_curves(primary, &work)?;
    let judgment = judgment(reader, limits)?;
    if stage(&judgment) != WorkStage::Qualified {
        return Err(BenchArchiveRefusal::StageMismatch);
    }
    let clock_attribution =
        read_attribution(reader.byte().map_err(Canonical)?).map_err(Canonical)?;
    let count = reader.count().map_err(Canonical)?;
    if count > limits.measurements() {
        return Err(BenchArchiveRefusal::TooManyMeasurements);
    }
    let samples = usize::try_from(row.measurement().budgets().samples())
        .map_err(|_| Canonical(ArchiveRefusal::SizeOutsidePlatform))?;
    let expected = row
        .measurement()
        .input_sizes()
        .sizes()
        .len()
        .checked_mul(samples)
        .ok_or(Canonical(ArchiveRefusal::SizeOutsidePlatform))?;
    if count != expected {
        return Err(BenchArchiveRefusal::WorkPopulationMismatch);
    }
    let mut measurements = Vec::new();
    for _ in 0..count {
        measurements.push(measurement(reader).map_err(Canonical)?);
    }
    Ok(ArchivedSecondaryObservation {
        work,
        judgment,
        measurements,
        clock_attribution,
    })
}

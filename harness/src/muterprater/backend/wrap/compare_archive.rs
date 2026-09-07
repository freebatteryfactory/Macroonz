//! Historical backend claims compared through the existing console grammar.

use super::parse::{LineReading, read_line};
use super::read::inconclusive_cause;
use crate::muterprater::backend::archive::{ArchivedBackendManifest, BackendArchiveRefusal};
use crate::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutation, ArchivedMutationIdentity, ArchivedMutationOutcome,
    ArchivedMutationSite, ArchivedRejection,
};
use crate::muterprater::{
    AnnouncedRoster, BaselineAxis, EquivalenceAxis, ExecutionAxis, MaterializationAxis, MutantId,
    SourceCoordinate, WrapOutcomeWord,
};
use crate::report::archive::{ArchiveRefusal, ArchivedForeignText, ArchivedTruncation};
use crate::report::{ForeignText, Truncation};

pub(in crate::muterprater::backend) fn archived_word(
    report: &ArchivedMutation,
) -> Result<WrapOutcomeWord, BackendArchiveRefusal> {
    let cause = match report.outcome() {
        ArchivedMutationOutcome::Killed(ArchivedRejection::ReportedByBackend(_)) => None,
        ArchivedMutationOutcome::Inconclusive(cause) => Some(*cause),
        ArchivedMutationOutcome::Killed(ArchivedRejection::Demonstrated(_))
        | ArchivedMutationOutcome::Survived => {
            return Err(BackendArchiveRefusal::BackendRecordMismatch);
        }
    };
    let word = match report.materialization() {
        MaterializationAxis::Unviable => WrapOutcomeWord::Unviable,
        MaterializationAxis::ToolFailed => WrapOutcomeWord::ToolFailed,
        MaterializationAxis::Built if cause.is_none() => WrapOutcomeWord::Caught,
        MaterializationAxis::Built if report.execution() == ExecutionAxis::TimedOut => {
            WrapOutcomeWord::TimedOut
        }
        MaterializationAxis::Built => WrapOutcomeWord::Missed,
    };
    if report.baseline() != BaselineAxis::Qualified
        || report.activation() != &ArchivedActivation::UnobservableUnderBackend
        || report.equivalence() != EquivalenceAxis::NotAssessed
        || report.materialization() != MaterializationAxis::from(word)
        || report.execution() != ExecutionAxis::from(word)
        || cause != inconclusive_cause(word)
        || !matches!(
            report.target().identity(),
            ArchivedMutationIdentity::External(_)
        )
        || !matches!(report.target().site(), ArchivedMutationSite::Reported(_))
    {
        return Err(BackendArchiveRefusal::BackendRecordMismatch);
    }
    Ok(word)
}

pub(in crate::muterprater::backend) fn check_archived_console(
    manifest: &ArchivedBackendManifest,
    console: &str,
) -> Result<(), BackendArchiveRefusal> {
    let mut baseline = None;
    let mut announced = AnnouncedRoster::Unstated;
    let mut reports = manifest.run().reports().iter();
    let mut unparsed = manifest.unparsed().iter();
    for (ordinal, line) in console.lines().enumerate() {
        match read_line(line) {
            LineReading::Baseline(axis) => {
                baseline.get_or_insert(axis);
            }
            LineReading::Roster(count) => announced = AnnouncedRoster::Stated(count),
            LineReading::Mutant {
                word,
                coordinate,
                damage,
                line: whole,
            } => {
                let report = reports
                    .next()
                    .ok_or(BackendArchiveRefusal::ConsoleReadingMismatch)?;
                compare_mutant(report, word, &coordinate, damage.as_bytes(), whole)?;
            }
            LineReading::Unread => {
                let saved = unparsed
                    .next()
                    .ok_or(BackendArchiveRefusal::ConsoleReadingMismatch)?;
                let ordinal =
                    u64::try_from(ordinal).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
                if saved.ordinal() != ordinal {
                    return Err(BackendArchiveRefusal::ConsoleReadingMismatch);
                }
                compare_foreign(saved.text(), line.as_bytes())?;
            }
        }
    }
    if baseline != Some(BaselineAxis::Qualified)
        || announced != manifest.announced()
        || reports.next().is_some()
        || unparsed.next().is_some()
    {
        return Err(BackendArchiveRefusal::ConsoleReadingMismatch);
    }
    Ok(())
}

fn compare_mutant(
    report: &ArchivedMutation,
    word: WrapOutcomeWord,
    coordinate: &SourceCoordinate,
    damage: &[u8],
    line: &str,
) -> Result<(), BackendArchiveRefusal> {
    let ArchivedMutationIdentity::External(identity) = report.target().identity() else {
        return Err(BackendArchiveRefusal::BackendRecordMismatch);
    };
    let ArchivedMutationSite::Reported(site) = report.target().site() else {
        return Err(BackendArchiveRefusal::BackendRecordMismatch);
    };
    if archived_word(report)? != word
        || site != coordinate
        || identity.as_bytes() != MutantId::over(coordinate, damage).address().as_bytes()
    {
        return Err(BackendArchiveRefusal::ConsoleReadingMismatch);
    }
    if let ArchivedMutationOutcome::Killed(ArchivedRejection::ReportedByBackend(stated)) =
        report.outcome()
    {
        compare_foreign(stated, line.as_bytes())?;
    }
    Ok(())
}

fn compare_foreign(saved: &ArchivedForeignText, bytes: &[u8]) -> Result<(), BackendArchiveRefusal> {
    let admitted = ForeignText::admitted(bytes);
    let truncation = match admitted.truncation() {
        Truncation::Complete => ArchivedTruncation::Complete,
        Truncation::TruncatedAt { admitted, offered } => ArchivedTruncation::TruncatedAt {
            admitted: u64::try_from(admitted).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?,
            offered: u64::try_from(offered).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?,
        },
    };
    if saved.bytes() != admitted.bytes()
        || saved.fidelity() != admitted.fidelity()
        || saved.truncation() != truncation
    {
        return Err(BackendArchiveRefusal::ConsoleReadingMismatch);
    }
    Ok(())
}

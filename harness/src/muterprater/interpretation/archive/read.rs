//! Historical parity admission joins complete owned records without live evidence mints.

use super::super::{
    ArchivedEvaluationPair, ArchivedParity, ArchivedParityDisposition, ArchivedValue,
    PARITY_ARCHIVE_TAG, ParityArchiveLimits, ParityArchiveRefusal, ValueRole,
};
use super::members;
use crate::descriptor::archive::{ArchivedBinding, read_binding, read_revision};
use crate::identity::ContentAddress;
use crate::muterprater::interpretation::qualify::first_refusal;
use crate::report::archive::{
    ArchiveRefusal, ArchivedAttempt, ArchivedConclusion, ArchivedTrial, claim, envelope, finish,
    frame, name, read_trial,
};
use crate::report::{
    TRIAL_IDENTITY_TAG, TrialProfile, attachment_replay_posture, trial_claim_preimage,
};

/// Read an owned historical parity record under independent resource ceilings.
///
/// # Errors
///
/// Refuses malformed or oversized members and contradictory witness, report or disposition claims.
/// Integrity and internal consistency grant no writer authenticity or current qualification.
pub fn read_parity(
    encoded: &[u8],
    limits: ParityArchiveLimits,
) -> Result<ArchivedParity, ParityArchiveRefusal> {
    let bytes = limits.bytes();
    let (address, mut reader) = envelope(encoded, PARITY_ARCHIVE_TAG, 1, bytes)?;
    let pair = ArchivedEvaluationPair {
        family: name(&mut reader, bytes)?,
        production: read_revision(frame(&mut reader, bytes)?, bytes.field())?,
        evaluation: read_revision(frame(&mut reader, bytes)?, bytes.field())?,
        surface: claim(&mut reader, bytes)?,
    };
    let witness = read_binding(frame(&mut reader, bytes)?, limits.binding())?;
    let input = ArchivedValue {
        convention: members::convention(&mut reader, limits)?,
        bytes: members::value(&mut reader, ValueRole::Input, limits)?,
    };
    let meaning = members::convention(&mut reader, limits)?;
    let production = ArchivedValue {
        convention: meaning.clone(),
        bytes: members::value(&mut reader, ValueRole::Production, limits)?,
    };
    let evaluation = ArchivedValue {
        convention: meaning,
        bytes: members::value(&mut reader, ValueRole::Evaluation, limits)?,
    };
    let firings = reader.u32()?;
    let substrate = members::substrate(&mut reader, limits)?;
    let conclusion = members::conclusion(&mut reader, limits)?;
    let production_report = read_trial(frame(&mut reader, bytes)?, limits.trial())?;
    let evaluation_report = read_trial(frame(&mut reader, bytes)?, limits.trial())?;
    let disposition = members::disposition(&mut reader)?;
    finish(&reader)?;
    let trial = joined_reports(&witness, &production_report, &evaluation_report)?;
    if let ArchivedConclusion::Refused(finding) = &conclusion
        && finding.fingerprint().trial().as_bytes() != trial.as_bytes()
    {
        return Err(ArchiveRefusal::IdentityJoinMismatch.into());
    }
    let cause = first_refusal(
        matches!(
            production_report.attempt(),
            ArchivedAttempt::Executed(ArchivedConclusion::Passed)
        )
        .then_some(())
        .ok_or(()),
        matches!(
            evaluation_report.attempt(),
            ArchivedAttempt::Executed(ArchivedConclusion::Passed)
        )
        .then_some(())
        .ok_or(()),
        firings,
        matches!(conclusion, ArchivedConclusion::Passed)
            .then_some(())
            .ok_or(()),
    );
    match disposition {
        ArchivedParityDisposition::Raw => {}
        ArchivedParityDisposition::Qualified if cause.is_none() => {}
        ArchivedParityDisposition::Rejected(expected) if cause == Some(expected) => {}
        ArchivedParityDisposition::Qualified | ArchivedParityDisposition::Rejected(_) => {
            return Err(ParityArchiveRefusal::DispositionMismatch);
        }
    }
    Ok(ArchivedParity {
        encoded: encoded.to_vec(),
        address,
        pair,
        witness,
        input,
        production,
        evaluation,
        firings,
        substrate,
        conclusion,
        production_report,
        evaluation_report,
        disposition,
    })
}

fn joined_reports(
    witness: &ArchivedBinding,
    production: &ArchivedTrial,
    evaluation: &ArchivedTrial,
) -> Result<ContentAddress, ParityArchiveRefusal> {
    let key = witness.row().trial_key_address();
    let trial = ContentAddress::derived(
        TRIAL_IDENTITY_TAG,
        &trial_claim_preimage(key.as_bytes(), TrialProfile::Unprofiled),
    );
    let production_key = production.key();
    let replay = attachment_replay_posture(
        witness.subject_revision().posture(),
        witness.check_revision().posture(),
    );
    if production_key.trial().as_bytes() != trial.as_bytes()
        || production_key.subject().as_bytes() != witness.subject_revision().revision()
        || production_key.check().as_bytes() != witness.check_revision().revision()
        || production_key.input().is_some()
        || production_key != evaluation.key()
        || production.site() != evaluation.site()
        || production.claimed_posture() != replay
        || evaluation.claimed_posture() != replay
    {
        return Err(ParityArchiveRefusal::ReportJoinMismatch);
    }
    Ok(trial)
}

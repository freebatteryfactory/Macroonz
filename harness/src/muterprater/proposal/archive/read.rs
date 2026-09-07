//! Bounded historical proposal admission through the existing descriptor and report readers.

use super::super::{
    ArchivedDischargeGround, ArchivedKillGround, ArchivedPinGround, ArchivedProposal,
    ArchivedProposalGround, PROPOSAL_ARCHIVE_TAG, ProposalArchiveLimits, ProposalArchiveRefusal,
};
use crate::descriptor::AdmissionGround;
use crate::descriptor::archive::{ArchivedSynthesis, CandidateArchiveLimits, read_candidate};
use crate::identity::BodyReader;
use crate::muterprater::ObligationLane;
use crate::muterprater::proposal::encode::historical_identity;
use crate::muterprater::verdict::archive::{ArchivedActivation, read_activation, read_target};
use crate::report::archive::{
    AddressClaim, ArchiveRefusal, ArchivedAttempt, ArchivedConclusion, ArchivedDisposition,
    ArchivedFinding, ArchivedRun, ArchivedTablePosture, ArchivedTrial, RunArchiveLimits, claim,
    envelope, execution, fingerprint, finish, frame, name, read_capsule, read_run, text,
};

/// Read a historical proposal without granting execution, review custody or admission authority.
///
/// # Errors
///
/// Refuses malformed framing, excess bounds, unsupported kinds, inconsistent identities and invalid concrete grounds.
pub fn read_proposal(
    encoded: &[u8],
    limits: ProposalArchiveLimits,
) -> Result<ArchivedProposal, ProposalArchiveRefusal> {
    let bytes = limits.bytes();
    let (address, mut reader) = envelope(encoded, PROPOSAL_ARCHIVE_TAG, 1, bytes)?;
    let candidate = read_candidate(
        frame(&mut reader, bytes)?,
        CandidateArchiveLimits::declared(bytes.field(), bytes.field(), limits.labels()),
    )?;
    let summary = match reader.byte()? {
        1 => AdmissionGround::MutantKilled,
        2 => AdmissionGround::ClaimPinned,
        3 => AdmissionGround::ObligationDischarged,
        _ => return Err(ProposalArchiveRefusal::InvalidGround),
    };
    let destination = name(&mut reader, bytes)?;
    let stated_identity = claim(&mut reader, bytes)?;
    let identity = historical_identity(&candidate, summary, &destination);
    if stated_identity.as_bytes() != identity.as_bytes() {
        return Err(ArchiveRefusal::IdentityJoinMismatch.into());
    }
    let ground = match summary {
        AdmissionGround::MutantKilled => {
            ArchivedProposalGround::MutantKilled(Box::new(kill(&mut reader, limits)?))
        }
        AdmissionGround::ClaimPinned => {
            ArchivedProposalGround::ClaimPinned(Box::new(pin(&mut reader, limits)?))
        }
        AdmissionGround::ObligationDischarged => {
            ArchivedProposalGround::ObligationDischarged(Box::new(discharge(&mut reader, limits)?))
        }
    };
    finish(&reader)?;
    if let (ArchivedSynthesis::Survivor(synthesis), ArchivedProposalGround::MutantKilled(kill)) =
        (candidate.synthesis(), &ground)
        && let Some(point) = kill.target.identity().point()
        && synthesis != point
    {
        return Err(ProposalArchiveRefusal::SurvivorPointMismatch);
    }
    Ok(ArchivedProposal {
        encoded: encoded.to_vec(),
        address,
        identity,
        candidate,
        destination,
        ground,
    })
}

fn kill(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ProposalArchiveLimits,
) -> Result<ArchivedKillGround, ProposalArchiveRefusal> {
    let bytes = limits.bytes();
    let target = read_target(frame(reader, bytes)?, bytes)?;
    let activation = read_activation(frame(reader, bytes)?, bytes)?;
    if activation == ArchivedActivation::NotObserved {
        return Err(ArchiveRefusal::InvalidSlot.into());
    }
    let capsule = read_capsule(frame(reader, bytes)?, bytes)?;
    let report = read_run(
        frame(reader, bytes)?,
        RunArchiveLimits::declared(bytes, limits.rows()),
    )?;
    let candidate = claim(reader, bytes)?;
    let (trial, rejection) = demonstrated(&report, candidate)?;
    if capsule.key().address() != trial.key().address()
        || capsule.fingerprint() != rejection.fingerprint()
    {
        return Err(ArchiveRefusal::IdentityJoinMismatch.into());
    }
    let count = reader.count()?;
    if count > limits.known() {
        return Err(ProposalArchiveRefusal::TooManyKnownFailures);
    }
    let mut known = Vec::new();
    for _ in 0..count {
        let entry = fingerprint(frame(reader, bytes)?, bytes)?;
        if &entry == rejection.fingerprint() {
            return Err(ProposalArchiveRefusal::FailureAlreadyKnown);
        }
        known.push(entry);
    }
    let trial = trial.clone();
    let rejection = rejection.clone();
    Ok(ArchivedKillGround {
        target,
        activation,
        capsule,
        report,
        trial,
        rejection,
        known,
    })
}

fn demonstrated(
    report: &ArchivedRun,
    candidate: AddressClaim,
) -> Result<(&ArchivedTrial, &ArchivedFinding), ProposalArchiveRefusal> {
    if !matches!(report.posture(), ArchivedTablePosture::Staged { parent: _ }) {
        return Err(ProposalArchiveRefusal::DemonstrationRequired);
    }
    let row = report
        .census()
        .iter()
        .find(|row| row.trial() == candidate)
        .ok_or(ProposalArchiveRefusal::DemonstrationRequired)?;
    let ArchivedDisposition::Selected(trial) = row.disposition() else {
        return Err(ProposalArchiveRefusal::DemonstrationRequired);
    };
    let ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)) = trial.attempt() else {
        return Err(ProposalArchiveRefusal::DemonstrationRequired);
    };
    Ok((trial, finding))
}

fn pin(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ProposalArchiveLimits,
) -> Result<ArchivedPinGround, ProposalArchiveRefusal> {
    let bytes = limits.bytes();
    let claim = name(reader, bytes)?;
    let capsule = read_capsule(frame(reader, bytes)?, bytes)?;
    let before = reader.u64()?;
    let after = reader.u64()?;
    if after <= before {
        return Err(ProposalArchiveRefusal::InvalidProofDelta);
    }
    Ok(ArchivedPinGround {
        claim,
        capsule,
        before,
        after,
    })
}

fn discharge(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ProposalArchiveLimits,
) -> Result<ArchivedDischargeGround, ProposalArchiveRefusal> {
    let bytes = limits.bytes();
    let owed = name(reader, bytes)?;
    let opening = text(reader, bytes)?.to_owned();
    if opening.is_empty() {
        return Err(ProposalArchiveRefusal::MissingOpeningCondition);
    }
    let lane = match reader.byte()? {
        0 => ObligationLane::TestRow,
        1 => ObligationLane::FuzzSeed,
        2 => ObligationLane::ChaosScenario,
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    };
    let trial = claim(reader, bytes)?;
    let key = execution(frame(reader, bytes)?, reader, bytes)?;
    Ok(ArchivedDischargeGround {
        owed,
        opening,
        lane,
        trial,
        key,
    })
}

//! Complete byte and population admission before proposal encoding allocates.

use super::{ProposalArchiveLimits, ProposalArchiveRefusal};
use crate::descriptor::archive::CandidateArchiveRefusal;
use crate::muterprater::verdict::archive::{activation_size, target_size};
use crate::muterprater::{
    ClaimPinnedProposal, MutantKilledProposal, ObligationDischargedProposal, ProposalDocument,
};
use crate::report::ReplayCapsule;
use crate::report::archive::{
    ArchiveLimits, ArchiveRefusal, RunArchiveLimits, bounded, capsule_size, execution_size,
    fingerprint_size, name_size, run_size, sum,
};

fn common(
    proposal: &impl ProposalDocument,
    limits: ProposalArchiveLimits,
) -> Result<usize, ProposalArchiveRefusal> {
    let bytes = limits.bytes();
    bounded(32, bytes)?;
    let candidate = proposal.candidate();
    let labels = candidate.classification();
    if labels.roles().len() > limits.labels() || labels.tags().len() > limits.labels() {
        return Err(CandidateArchiveRefusal::TooManyLabels.into());
    }
    let canonical = bounded(candidate.canonical_bytes().as_bytes().len(), bytes)?;
    Ok(sum(&[
        93,
        canonical,
        name_size(proposal.destination().suite().name(), bytes)?,
    ])?)
}

fn capsule(capsule: &ReplayCapsule, limits: ArchiveLimits) -> Result<usize, ArchiveRefusal> {
    bounded(
        capsule_size(
            capsule.key(),
            capsule.fingerprint(),
            capsule.input(),
            capsule.generation(),
            capsule.minimization(),
            limits,
        )?,
        limits,
    )
}

fn total(
    common: usize,
    ground: usize,
    limits: ProposalArchiveLimits,
) -> Result<usize, ProposalArchiveRefusal> {
    let total = sum(&[common, ground])?;
    if total > limits.bytes().envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}

pub(super) fn kill(
    proposal: &MutantKilledProposal,
    limits: ProposalArchiveLimits,
) -> Result<usize, ProposalArchiveRefusal> {
    let common = common(proposal, limits)?;
    let bytes = limits.bytes();
    let ground = proposal.ground();
    let count = proposal.duplicate().known().count();
    if count > limits.known() {
        return Err(ProposalArchiveRefusal::TooManyKnownFailures);
    }
    count_word(count)?;
    let mut length = sum(&[
        80,
        target_size(ground.target(), bytes)?,
        activation_size(ground.activation(), bytes)?,
        capsule(ground.capsule(), bytes)?,
        bounded(
            run_size(
                ground.demonstration().report(),
                RunArchiveLimits::declared(bytes, limits.rows()),
            )?,
            bytes,
        )?,
    ])?;
    for fingerprint in proposal.duplicate().known() {
        length = sum(&[length, 8, fingerprint_size(fingerprint.cause(), bytes)?])?;
    }
    total(common, length, limits)
}

pub(super) fn pin(
    proposal: &ClaimPinnedProposal,
    limits: ProposalArchiveLimits,
) -> Result<usize, ProposalArchiveRefusal> {
    let common = common(proposal, limits)?;
    let ground = proposal.ground();
    let bytes = limits.bytes();
    count_word(ground.delta().before())?;
    count_word(ground.delta().after())?;
    let length = sum(&[
        24,
        name_size(ground.claim().name(), bytes)?,
        capsule(ground.capsule(), bytes)?,
    ])?;
    total(common, length, limits)
}

pub(super) fn discharge(
    proposal: &ObligationDischargedProposal,
    limits: ProposalArchiveLimits,
) -> Result<usize, ProposalArchiveRefusal> {
    let common = common(proposal, limits)?;
    let ground = proposal.ground();
    let bytes = limits.bytes();
    let length = sum(&[
        49,
        name_size(ground.owed().claim().name(), bytes)?,
        bounded(ground.owed().opening_condition().len(), bytes)?,
        execution_size(ground.discharge().key(), bytes)?,
    ])?;
    total(common, length, limits)
}

pub(super) fn count_word(count: usize) -> Result<u64, ArchiveRefusal> {
    u64::try_from(count).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)
}

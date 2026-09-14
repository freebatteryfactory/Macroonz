//! Historical proposal retention through the existing semantic writers.

use super::{
    ArchivedProposal, PROPOSAL_ARCHIVE_TAG, ProposalArchiveLimits, ProposalArchiveRefusal,
    read_proposal, size,
};
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::muterprater::verdict::archive::{write_activation, write_target};
use crate::muterprater::{
    ClaimPinnedProposal, MutantKilledProposal, ObligationDischargedProposal, ObligationLane,
    ProposalDocument,
};
use crate::report::archive::{RunArchiveLimits, retain_capsule, retain_run, sum, write_execution};
use crate::report::fingerprint_preimage;

/// Retain the complete concrete mutant-kill offer as historical data.
///
/// # Errors
///
/// Refuses byte or population ceilings before nested encoding, then requires every historical join.
pub fn retain_mutant_kill(
    proposal: &MutantKilledProposal,
    limits: ProposalArchiveLimits,
) -> Result<ArchivedProposal, ProposalArchiveRefusal> {
    let mut body = header(proposal, size::kill(proposal, limits)?);
    let ground = proposal.ground();
    let mut target = Vec::new();
    write_target(ground.target(), &mut target);
    encode_bytes(&target, &mut body);
    let mut activation = Vec::new();
    write_activation(ground.activation(), &mut activation);
    encode_bytes(&activation, &mut body);
    encode_bytes(
        retain_capsule(ground.capsule(), limits.bytes())?.encoded(),
        &mut body,
    );
    encode_bytes(
        retain_run(
            ground.demonstration().report(),
            RunArchiveLimits::declared(limits.bytes(), limits.rows()),
        )?
        .encoded(),
        &mut body,
    );
    encode_bytes(
        ground
            .demonstration()
            .rejection()
            .trial()
            .address()
            .as_bytes(),
        &mut body,
    );
    encode_length(proposal.duplicate().known().count(), &mut body);
    for fingerprint in proposal.duplicate().known() {
        encode_bytes(
            &fingerprint_preimage(
                fingerprint.trial(),
                fingerprint.cause(),
                fingerprint.class(),
            ),
            &mut body,
        );
    }
    complete(&body, limits)
}

/// Retain a claim-pin offer with its exact capsule and proof-count movement.
///
/// # Errors
///
/// Refuses independent byte and label ceilings before encoding, then requires historical admission.
pub fn retain_claim_pin(
    proposal: &ClaimPinnedProposal,
    limits: ProposalArchiveLimits,
) -> Result<ArchivedProposal, ProposalArchiveRefusal> {
    let mut body = header(proposal, size::pin(proposal, limits)?);
    let ground = proposal.ground();
    ground.claim().name().encode_into(&mut body);
    encode_bytes(
        retain_capsule(ground.capsule(), limits.bytes())?.encoded(),
        &mut body,
    );
    body.extend_from_slice(&size::count_word(ground.delta().before())?.to_be_bytes());
    body.extend_from_slice(&size::count_word(ground.delta().after())?.to_be_bytes());
    complete(&body, limits)
}

/// Retain an obligation-discharge offer without inventing replay or custody evidence.
///
/// # Errors
///
/// Refuses independent byte and label ceilings before encoding, then requires historical admission.
pub fn retain_obligation_discharge(
    proposal: &ObligationDischargedProposal,
    limits: ProposalArchiveLimits,
) -> Result<ArchivedProposal, ProposalArchiveRefusal> {
    let mut body = header(proposal, size::discharge(proposal, limits)?);
    let ground = proposal.ground();
    ground.owed().claim().name().encode_into(&mut body);
    encode_bytes(ground.owed().opening_condition().as_bytes(), &mut body);
    body.push(match ground.discharge().lane() {
        ObligationLane::TestRow => 0,
        ObligationLane::FuzzSeed => 1,
        ObligationLane::ChaosScenario => 2,
    });
    encode_bytes(ground.discharge().trial().address().as_bytes(), &mut body);
    write_execution(ground.discharge().key(), &mut body);
    complete(&body, limits)
}

fn header(proposal: &impl ProposalDocument, total: usize) -> Vec<u8> {
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    for word in [1u32, 1, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    encode_bytes(proposal.candidate().canonical_bytes().as_bytes(), &mut body);
    body.push(proposal.ground_summary().slot());
    proposal.destination().suite().name().encode_into(&mut body);
    encode_bytes(proposal.identity().address().as_bytes(), &mut body);
    body
}

fn complete(
    body: &[u8],
    limits: ProposalArchiveLimits,
) -> Result<ArchivedProposal, ProposalArchiveRefusal> {
    let mut encoded = Vec::with_capacity(sum(&[32, body.len()])?);
    encoded.extend_from_slice(ContentAddress::derived(PROPOSAL_ARCHIVE_TAG, body).as_bytes());
    encoded.extend_from_slice(body);
    read_proposal(&encoded, limits)
}

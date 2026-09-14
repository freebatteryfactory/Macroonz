//! Actual unadmitted offers retain all grounds and cross a fresh historical reader.

use super::{capture_demonstration_under, offered_kill};
use crate::archive_process;
use crate::support::{interpreted_kill, trial_binding_for};
use macroonz_harness::descriptor::{ClaimRef, Row};
use macroonz_harness::muterprater::proposal_archive::{
    ArchivedProposal, ArchivedProposalGround, ProposalArchiveLimits, ProposalArchiveRefusal,
    read_proposal, retain_claim_pin, retain_mutant_kill, retain_obligation_discharge,
};
use macroonz_harness::muterprater::propose::{
    offer_claim_pin, offer_mutant_kill, offer_obligation_discharge,
};
use macroonz_harness::muterprater::{
    DischargeEvidence, ObligationLane, OwedClaim, ProofDelta, ProposalDocument,
};
use macroonz_harness::report::Fingerprint;
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};
use macroonz_harness::runner::trial_identity;
use std::error::Error;
use std::fmt::Debug;
use std::io::Read as _;

const LIMITS: ProposalArchiveLimits =
    ProposalArchiveLimits::declared(ArchiveLimits::declared(65536, 32768), 8, 16, 8);

fn refused(refusal: impl Debug) -> std::io::Error {
    std::io::Error::other(format!("{refusal:?}"))
}

fn bounded_round_trip(
    retain: impl Fn(ProposalArchiveLimits) -> Result<ArchivedProposal, ProposalArchiveRefusal>,
    proposal: &impl ProposalDocument,
) -> Result<(), Box<dyn Error>> {
    let record = retain(LIMITS).map_err(refused)?;
    assert_eq!(record.identity(), proposal.identity().address());
    assert_eq!(
        record.candidate().canonical_bytes(),
        proposal.candidate().canonical_bytes().as_bytes()
    );
    assert_eq!(
        record.destination().namespace(),
        proposal.destination().suite().name().namespace().written()
    );
    let exact = ProposalArchiveLimits::declared(
        ArchiveLimits::declared(record.encoded().len(), LIMITS.bytes().field()),
        8,
        16,
        8,
    );
    assert_eq!(retain(exact).map_err(refused)?, record);
    let short = ProposalArchiveLimits::declared(
        ArchiveLimits::declared(
            record.encoded().len().saturating_sub(1),
            LIMITS.bytes().field(),
        ),
        8,
        16,
        8,
    );
    assert_eq!(
        retain(short),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::EnvelopeTooLarge
        ))
    );
    assert_eq!(
        read_proposal(record.encoded(), short),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::EnvelopeTooLarge
        ))
    );
    let tiny = ProposalArchiveLimits::declared(ArchiveLimits::declared(65536, 31), 8, 16, 8);
    assert_eq!(
        retain(tiny),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::FieldTooLarge
        ))
    );
    let bytes = record.encoded().to_vec();
    drop(record);
    let returned =
        archive_process::round_trip(&bytes, "proposal_admission::archive::child_loads_proposal")?;
    assert_eq!(returned, bytes);
    assert_eq!(
        read_proposal(&returned, LIMITS)
            .map_err(refused)?
            .identity(),
        proposal.identity().address()
    );
    Ok(())
}

fn discharge(
    candidate: Row,
    proposal: &impl ProposalDocument,
    key: &macroonz_harness::report::ExecutionKey,
) -> Result<macroonz_harness::muterprater::ObligationDischargedProposal, Box<dyn Error>> {
    let owed = OwedClaim::declared(
        ClaimRef::named("separate-owner", "owed").map_err(refused)?,
        "later-opening",
    )
    .map_err(refused)?;
    let other = trial_binding_for("separate-discharge").map_err(refused)?;
    let evidence = DischargeEvidence::recorded(
        ObligationLane::ChaosScenario,
        trial_identity(other.row()),
        key.clone(),
    );
    Ok(
        offer_obligation_discharge(candidate, owed, evidence, &[], proposal.destination())
            .map_err(refused)?,
    )
}

#[test]
fn all_actual_offer_forms_preserve_evidence_and_cross_a_fresh_process() -> Result<(), Box<dyn Error>>
{
    let mutation = interpreted_kill().map_err(refused)?;
    let fixture = offered_kill(&mutation).map_err(refused)?;
    let prior = trial_binding_for("prior-failure").map_err(refused)?;
    let known = Fingerprint::of(
        trial_identity(prior.row()),
        fixture.demonstration.rejection().finding(),
    );
    let kill = offer_mutant_kill(
        fixture.proposal.candidate().clone(),
        &mutation,
        fixture.capsule.clone(),
        fixture.demonstration.clone(),
        vec![known, known],
        fixture.proposal.destination(),
    )
    .map_err(refused)?;
    let pin = offer_claim_pin(
        kill.candidate().clone(),
        ClaimRef::named("other-owner", "pin").map_err(refused)?,
        fixture.capsule.clone(),
        ProofDelta::between(3, 8).map_err(refused)?,
        kill.destination(),
    )
    .map_err(refused)?;
    let discharge = discharge(kill.candidate().clone(), &kill, fixture.capsule.key())?;
    bounded_round_trip(|limits| retain_mutant_kill(&kill, limits), &kill)?;
    bounded_round_trip(|limits| retain_claim_pin(&pin, limits), &pin)?;
    bounded_round_trip(
        |limits| retain_obligation_discharge(&discharge, limits),
        &discharge,
    )?;
    let retained = retain_mutant_kill(&kill, LIMITS).map_err(refused)?;
    let ArchivedProposalGround::MutantKilled(ground) = retained.ground() else {
        return Err(refused("wrong ground").into());
    };
    assert_eq!(ground.capsule().identity(), fixture.capsule.identity());
    assert_eq!(
        ground.trial_report().key().address(),
        fixture
            .demonstration
            .trial_report()
            .standing()
            .key()
            .address()
    );
    assert_eq!(
        ground.rejection().fingerprint().address(),
        fixture.demonstration.rejection().fingerprint().address()
    );
    assert_eq!(
        ground.report().census().len(),
        fixture.demonstration.report().census().len()
    );
    assert_eq!(ground.known().len(), 2);
    assert_eq!(
        ground
            .known()
            .first()
            .ok_or_else(|| refused("missing known"))?
            .address(),
        known.address()
    );
    let revised_capsule = capture_demonstration_under(&fixture.demonstration, "changed-generation")
        .map_err(refused)?;
    let revised = offer_claim_pin(
        pin.candidate().clone(),
        pin.ground().claim(),
        revised_capsule,
        ProofDelta::between(4, 9).map_err(refused)?,
        pin.destination(),
    )
    .map_err(refused)?;
    assert_eq!(revised.identity(), pin.identity());
    assert_ne!(
        retain_claim_pin(&revised, LIMITS)
            .map_err(refused)?
            .address(),
        retain_claim_pin(&pin, LIMITS).map_err(refused)?.address()
    );
    assert_eq!(
        retain_mutant_kill(
            &kill,
            ProposalArchiveLimits::declared(LIMITS.bytes(), 8, 0, 8)
        ),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::TooManyRows
        ))
    );
    assert_eq!(
        retain_mutant_kill(
            &kill,
            ProposalArchiveLimits::declared(LIMITS.bytes(), 8, 16, 1)
        ),
        Err(ProposalArchiveRefusal::TooManyKnownFailures)
    );
    Ok(())
}

#[test]
#[ignore = "driven by the actual proposal archive crossing"]
fn child_loads_proposal() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(65537)
        .read_to_end(&mut encoded)?;
    let record = read_proposal(&encoded, LIMITS).map_err(refused)?;
    drop(encoded);
    archive_process::publish(record.encoded())
}

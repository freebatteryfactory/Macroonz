//! Independent historical proposal fields, identity movement, hostile joins and process crossings.

use super::candidate_vector::{CandidateVector, Opening};
use super::proposal_vector::{self, KillVector, TargetVector};
use super::vector::{InputKind, hash};
use super::{process, run_vector, trial_vector};
use macroonz_harness::descriptor::archive::CandidateArchiveRefusal;
use macroonz_harness::muterprater::proposal_archive::{
    ArchivedPinGround, ArchivedProposalGround, ProposalArchiveLimits, ProposalArchiveRefusal,
    read_proposal,
};
use macroonz_harness::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutationIdentity, ArchivedMutationSite,
};
use macroonz_harness::muterprater::{NoComparisonReason, ObligationLane};
use macroonz_harness::report::archive::ArchiveLimits;
use macroonz_harness::report::archive::ArchiveRefusal;
use std::error::Error;
use std::io::Read as _;

const LIMITS: ProposalArchiveLimits =
    ProposalArchiveLimits::declared(ArchiveLimits::declared(65536, 32768), 8, 16, 8);

#[test]
fn kill_vectors_retain_every_available_ground_and_comparison_field() -> Result<(), ()> {
    let vector = KillVector::declared();
    let proposal = vector.proposal();
    let encoded = proposal.encoded();
    let record = read_proposal(&encoded, LIMITS).map_err(|_| ())?;
    assert_eq!(record.identity().as_bytes(), &proposal.identity());
    assert_eq!(record.candidate().canonical_bytes(), proposal.candidate);
    assert_eq!(record.destination().namespace(), "révision");
    assert_eq!(record.destination().stem(), "destination");
    drop(encoded);
    let ArchivedProposalGround::MutantKilled(ground) = record.ground() else {
        return Err(());
    };
    assert_eq!(
        ground.target().family(),
        Some("historical-family-outside-current-bank")
    );
    assert_eq!(
        ground.target().owner().ok_or(())?.namespace(),
        "mapped-owner"
    );
    assert_eq!(ground.target().owner().ok_or(())?.stem(), "claim");
    assert!(
        matches!(ground.target().identity(), ArchivedMutationIdentity::Interpreted { point, alternative } if point.namespace() == "mutation-owner" && point.stem() == "point" && alternative.as_bytes() == &[11; 32])
    );
    assert!(
        matches!(ground.target().site(), ArchivedMutationSite::Declared(site) if site.namespace() == "activation-owner" && site.stem() == "site")
    );
    let ArchivedActivation::Observed(reading) = ground.activation() else {
        return Err(());
    };
    assert_eq!(reading.surface().as_bytes(), &[21; 32]);
    assert_eq!(reading.point().stem(), "point");
    assert_eq!(reading.alternative().as_bytes(), &[11; 32]);
    assert_eq!(reading.witness().as_bytes(), &[12; 32]);
    assert_eq!(reading.firings(), 5);
    assert_eq!(ground.capsule().encoded(), vector.capsule.encoded());
    assert_eq!(ground.report().encoded(), vector.run.encoded());
    assert_eq!(ground.report().census().len(), 3);
    assert_eq!(ground.trial_report().key().trial().as_bytes(), &[1; 32]);
    assert_eq!(ground.rejection().file(), "check.rs");
    assert_eq!(ground.rejection().line(), 29);
    assert_eq!(
        ground.rejection().foreign().ok_or(())?.bytes(),
        &[b'a', 255, b'b']
    );
    assert_eq!(ground.known().len(), 2);
    assert_eq!(ground.known().first(), ground.known().last());
    assert_eq!(ground.known().first().ok_or(())?.family(), "previous");
    Ok(())
}

#[test]
fn target_roads_keep_historical_coordinates_and_backend_activation_ceilings() -> Result<(), ()> {
    let mut vector = KillVector::declared();
    vector.target = TargetVector::selected(2);
    vector.activation = vec![2];
    let compiled = read_proposal(&vector.proposal().encoded(), LIMITS).map_err(|_| ())?;
    let ArchivedProposalGround::MutantKilled(ground) = compiled.ground() else {
        return Err(());
    };
    assert!(
        matches!(ground.target().identity(), ArchivedMutationIdentity::CompiledProjection { point, alternative } if point.stem() == "point" && alternative.as_bytes() == &[11; 32])
    );
    assert_eq!(
        ground.activation(),
        &ArchivedActivation::UnobservableUnderBackend
    );
    vector.target = TargetVector::external();
    let external = read_proposal(&vector.proposal().encoded(), LIMITS).map_err(|_| ())?;
    let ArchivedProposalGround::MutantKilled(external_ground) = external.ground() else {
        return Err(());
    };
    assert!(
        matches!(external_ground.target().identity(), ArchivedMutationIdentity::External(claim) if claim.as_bytes() == &[17; 32])
    );
    assert!(external_ground.target().owner().is_none());
    assert!(external_ground.target().family().is_none());
    assert!(
        matches!(external_ground.target().site(), ArchivedMutationSite::Reported(site) if site.file() == "src/étranger.rs" && site.line() == 43 && site.column() == 7)
    );
    Ok(())
}

#[test]
fn pin_and_discharge_keep_portable_counts_and_the_live_owners_actual_join_ceilings()
-> Result<(), ()> {
    let pin = proposal_vector::pin(u64::MAX - 1, u64::MAX);
    let pin_record = read_proposal(&pin.encoded(), LIMITS).map_err(|_| ())?;
    let ArchivedProposalGround::ClaimPinned(pin_ground) = pin_record.ground() else {
        return Err(());
    };
    assert_ne!(pin_record.candidate().claim(), pin_ground.claim());
    assert_eq!(pin_ground.claim().namespace(), "another-owner");
    assert_eq!(pin_ground.before(), u64::MAX - 1);
    assert_eq!(pin_ground.after(), u64::MAX);
    assert_eq!(
        ArchivedPinGround::comparison(),
        NoComparisonReason::GroundCarriesNoFailure
    );
    assert_eq!(pin_ground.capsule().key().trial().as_bytes(), &[1; 32]);
    for (slot, lane) in [
        (0, ObligationLane::TestRow),
        (1, ObligationLane::FuzzSeed),
        (2, ObligationLane::ChaosScenario),
    ] {
        for input in [InputKind::Unit, InputKind::Bound] {
            let bytes = proposal_vector::discharge("manque-λ".as_bytes(), slot, input).encoded();
            let record = read_proposal(&bytes, LIMITS).map_err(|_| ())?;
            let ArchivedProposalGround::ObligationDischarged(ground) = record.ground() else {
                return Err(());
            };
            assert_eq!(ground.owed().namespace(), "owed-owner");
            assert_eq!(ground.owed().stem(), "obligation");
            assert_eq!(ground.compared_owed(), ground.owed());
            assert_eq!(ground.opening_condition(), "manque-λ");
            assert_eq!(ground.lane(), lane);
            assert_eq!(ground.trial().as_bytes(), &[88; 32]);
            assert_eq!(ground.key().trial().as_bytes(), &[1; 32]);
            assert_eq!(
                ground.key().input().is_some(),
                matches!(input, InputKind::Bound)
            );
        }
    }
    Ok(())
}

#[test]
fn evidence_movement_changes_archive_integrity_without_renaming_the_offer() -> Result<(), ()> {
    let original = proposal_vector::pin(1, 3);
    let first = read_proposal(&original.encoded(), LIMITS).map_err(|_| ())?;
    let revised = proposal_vector::pin(2, 4);
    let second = read_proposal(&revised.encoded(), LIMITS).map_err(|_| ())?;
    assert_eq!(first.identity(), second.identity());
    assert_ne!(first.address(), second.address());
    let mut moved = revised;
    moved.destination = (b"another", b"suite");
    let third = read_proposal(&moved.encoded(), LIMITS).map_err(|_| ())?;
    assert_ne!(second.identity(), third.identity());
    moved.stated_identity = Some(original.identity());
    assert_eq!(
        read_proposal(&moved.encoded(), LIMITS),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::IdentityJoinMismatch
        ))
    );
    Ok(())
}

#[test]
fn byte_and_each_population_ceiling_are_independent() {
    let vector = KillVector::declared();
    let proposal = vector.proposal();
    let encoded = proposal.encoded();
    let field = vector.run.encoded().len();
    let exact =
        ProposalArchiveLimits::declared(ArchiveLimits::declared(encoded.len(), field), 2, 3, 2);
    assert!(read_proposal(&encoded, exact).is_ok());
    for (limits, refusal) in [
        (
            ProposalArchiveLimits::declared(
                ArchiveLimits::declared(encoded.len() - 1, field),
                2,
                3,
                2,
            ),
            ProposalArchiveRefusal::Historical(ArchiveRefusal::EnvelopeTooLarge),
        ),
        (
            ProposalArchiveLimits::declared(
                ArchiveLimits::declared(encoded.len(), field - 1),
                2,
                3,
                2,
            ),
            ProposalArchiveRefusal::Historical(ArchiveRefusal::FieldTooLarge),
        ),
        (
            ProposalArchiveLimits::declared(LIMITS.bytes(), 1, 3, 2),
            ProposalArchiveRefusal::Candidate(CandidateArchiveRefusal::TooManyLabels),
        ),
        (
            ProposalArchiveLimits::declared(LIMITS.bytes(), 2, 2, 2),
            ProposalArchiveRefusal::Historical(ArchiveRefusal::TooManyRows),
        ),
        (
            ProposalArchiveLimits::declared(LIMITS.bytes(), 2, 3, 1),
            ProposalArchiveRefusal::TooManyKnownFailures,
        ),
    ] {
        assert_eq!(read_proposal(&encoded, limits), Err(refusal));
    }
    let mut no_known = vector;
    no_known.count = 0;
    no_known.known.clear();
    assert!(
        read_proposal(
            &no_known.proposal().encoded(),
            ProposalArchiveLimits::declared(LIMITS.bytes(), 2, 3, 0)
        )
        .is_ok()
    );
    no_known.count = 9;
    assert_eq!(
        read_proposal(&no_known.proposal().encoded(), LIMITS),
        Err(ProposalArchiveRefusal::TooManyKnownFailures)
    );
    no_known.count = u64::MAX;
    assert!(matches!(
        read_proposal(&no_known.proposal().encoded(), LIMITS),
        Err(ProposalArchiveRefusal::TooManyKnownFailures
            | ProposalArchiveRefusal::Historical(ArchiveRefusal::LengthOutsidePlatform {
                declared: u64::MAX
            }))
    ));
}

#[test]
fn rehashed_kill_records_cannot_hide_missing_demonstration_or_wrong_capsule_joins() -> Result<(), ()>
{
    let mut vector = KillVector::declared();
    vector.run.posture = vec![0];
    assert_eq!(
        read_proposal(&vector.proposal().encoded(), LIMITS),
        Err(ProposalArchiveRefusal::DemonstrationRequired)
    );
    vector = KillVector::declared();
    vector.trial = [9; 32];
    assert_eq!(
        read_proposal(&vector.proposal().encoded(), LIMITS),
        Err(ProposalArchiveRefusal::DemonstrationRequired)
    );
    vector = KillVector::declared();
    let passing = trial_vector::envelope(&trial_vector::body(&[0], &[1]));
    *vector.run.rows.get_mut(1).ok_or(())? = run_vector::row(1, 8, 0, &passing);
    assert_eq!(
        read_proposal(&vector.proposal().encoded(), LIMITS),
        Err(ProposalArchiveRefusal::DemonstrationRequired)
    );
    vector = KillVector::declared();
    *vector.capsule.key.get_mut(120).ok_or(())? = 1;
    let changed_key = hash("input-execution-key/v1", &vector.capsule.key);
    vector
        .capsule
        .capsule
        .get_mut(8..40)
        .ok_or(())?
        .copy_from_slice(&changed_key);
    assert_eq!(
        read_proposal(&vector.proposal().encoded(), LIMITS),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::IdentityJoinMismatch
        ))
    );
    vector = KillVector::declared();
    vector.known = vec![vector.capsule.fingerprint.clone()];
    vector.count = 1;
    assert_eq!(
        read_proposal(&vector.proposal().encoded(), LIMITS),
        Err(ProposalArchiveRefusal::FailureAlreadyKnown)
    );
    let mut mismatch = KillVector::declared().proposal();
    let mut candidate = CandidateVector::declared(Opening::Survivor);
    candidate.origin = vec![3, 1];
    proposal_vector::name((b"another", b"point"), &mut candidate.origin);
    mismatch.candidate = candidate.encoded();
    assert_eq!(
        read_proposal(&mismatch.encoded(), LIMITS),
        Err(ProposalArchiveRefusal::SurvivorPointMismatch)
    );
    Ok(())
}

#[test]
fn targets_activation_and_concrete_grounds_refuse_unsupported_or_empty_members() {
    for seat in 0u8..4 {
        let mut vector = KillVector::declared();
        match seat {
            0 => vector.target.identity = vec![9],
            1 => vector.target.family = vec![9],
            2 => vector.target.site = vec![9],
            _ => vector.target.owner = vec![9],
        }
        assert_eq!(
            read_proposal(&vector.proposal().encoded(), LIMITS),
            Err(ProposalArchiveRefusal::Historical(
                ArchiveRefusal::InvalidSlot
            ))
        );
    }
    for activation in [vec![1], vec![9], proposal_vector::activation(0)] {
        let mut vector = KillVector::declared();
        vector.activation = activation;
        assert_eq!(
            read_proposal(&vector.proposal().encoded(), LIMITS),
            Err(ProposalArchiveRefusal::Historical(
                ArchiveRefusal::InvalidSlot
            ))
        );
    }
    for (before, after) in [(0, 0), (5, 4)] {
        assert_eq!(
            read_proposal(&proposal_vector::pin(before, after).encoded(), LIMITS),
            Err(ProposalArchiveRefusal::InvalidProofDelta)
        );
    }
    assert_eq!(
        read_proposal(
            &proposal_vector::discharge(b"", 0, InputKind::Unit).encoded(),
            LIMITS
        ),
        Err(ProposalArchiveRefusal::MissingOpeningCondition)
    );
    assert_eq!(
        read_proposal(
            &proposal_vector::discharge(&[255], 0, InputKind::Unit).encoded(),
            LIMITS
        ),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::InvalidText
        ))
    );
    assert_eq!(
        read_proposal(
            &proposal_vector::discharge(b"owed", 9, InputKind::Unit).encoded(),
            LIMITS
        ),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::InvalidSlot
        ))
    );
}

#[test]
fn framing_identity_custody_and_all_truncated_prefixes_refuse_independently() -> Result<(), ()> {
    let mut header_vector = proposal_vector::pin(1, 2);
    for (version, kind, custody, expected) in [
        (2, 1, 0, ArchiveRefusal::UnsupportedFormat { found: 2 }),
        (1, 2, 0, ArchiveRefusal::WrongKind { found: 2 }),
        (1, 1, 1, ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        header_vector.version = version;
        header_vector.kind = kind;
        header_vector.custody = custody;
        assert_eq!(
            read_proposal(&header_vector.encoded(), LIMITS),
            Err(ProposalArchiveRefusal::Historical(expected))
        );
    }
    header_vector = proposal_vector::pin(1, 2);
    header_vector.ground = 9;
    assert_eq!(
        read_proposal(&header_vector.encoded(), LIMITS),
        Err(ProposalArchiveRefusal::InvalidGround)
    );
    for vector in [
        KillVector::declared().proposal(),
        proposal_vector::pin(1, 2),
        proposal_vector::discharge(b"owed", 0, InputKind::Bound),
    ] {
        let body = vector.body();
        for length in 0..body.len() {
            assert!(
                read_proposal(
                    &proposal_vector::envelope(body.get(..length).ok_or(())?),
                    LIMITS
                )
                .is_err(),
                "accepted prefix {length}"
            );
        }
        let mut trailing = body;
        trailing.push(0);
        assert_eq!(
            read_proposal(&proposal_vector::envelope(&trailing), LIMITS),
            Err(ProposalArchiveRefusal::Historical(
                ArchiveRefusal::TrailingBytes
            ))
        );
        let mut corrupt = vector.encoded();
        *corrupt.first_mut().ok_or(())? ^= 1;
        assert_eq!(
            read_proposal(&corrupt, LIMITS),
            Err(ProposalArchiveRefusal::Historical(
                ArchiveRefusal::AddressMismatch
            ))
        );
    }
    let mut short_claim = proposal_vector::pin(1, 2);
    short_claim.destination = (b"", b"suite");
    assert_eq!(
        read_proposal(&short_claim.encoded(), LIMITS),
        Err(ProposalArchiveRefusal::Historical(
            ArchiveRefusal::InvalidText
        ))
    );
    let mut malicious = vec![0; 12];
    malicious
        .get_mut(..4)
        .ok_or(())?
        .copy_from_slice(&1u32.to_be_bytes());
    malicious
        .get_mut(4..8)
        .ok_or(())?
        .copy_from_slice(&1u32.to_be_bytes());
    malicious.extend_from_slice(&u64::MAX.to_be_bytes());
    assert!(read_proposal(&proposal_vector::envelope(&malicious), LIMITS).is_err());
    Ok(())
}

#[test]
#[ignore = "driven by the historical proposal process-boundary claim"]
fn child_loads_proposal() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(65537)
        .read_to_end(&mut encoded)?;
    let record = read_proposal(&encoded, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(encoded);
    process::publish(record.encoded())
}

#[test]
fn every_independent_ground_survives_a_fresh_reader_process() -> Result<(), Box<dyn Error>> {
    for vector in [
        KillVector::declared().proposal(),
        proposal_vector::pin(1, 2),
        proposal_vector::discharge(b"owed", 2, InputKind::Bound),
    ] {
        let encoded = vector.encoded();
        assert_eq!(
            process::round_trip(&encoded, "archive::proposals::child_loads_proposal")?,
            encoded
        );
    }
    Ok(())
}

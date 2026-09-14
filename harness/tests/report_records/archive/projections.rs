//! Independent complete projection readback, identity, report and mutation joins.

use super::projection_vector::{ProjectionVector, envelope};
use super::vector::hash;
use macroonz_harness::descriptor::archive::BindingArchiveLimits;
use macroonz_harness::muterprater::interpretation_archive::ParityArchiveLimits;
use macroonz_harness::muterprater::specimen_archive::{
    ProjectionArchiveLimits, ProjectionArchiveRefusal, read_projection,
};
use macroonz_harness::muterprater::verdict_archive::{ArchivedMutationOutcome, ArchivedRejection};
use macroonz_harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, ArchivedAttempt, ArchivedConclusion,
};
use std::error::Error;
use std::io::Read as _;

const BYTES: ArchiveLimits = ArchiveLimits::declared(65536, 32768);
const PARITY: ParityArchiveLimits = ParityArchiveLimits::declared(
    ArchiveLimits::declared(32768, 8192),
    BindingArchiveLimits::declared(8192, 256, 8),
    ArchiveLimits::declared(8192, 4096),
    256,
    8,
);
const LIMITS: ProjectionArchiveLimits =
    ProjectionArchiveLimits::declared(BYTES, PARITY, BYTES, BYTES, 4096);

#[test]
fn independent_projection_retains_all_available_material_and_derives_only_bytes_identity()
-> Result<(), ()> {
    let vector = ProjectionVector::declared();
    let bytes = vector.encoded();
    let record = read_projection(&bytes, LIMITS).map_err(|_| ())?;
    assert_eq!(record.encoded(), bytes);
    drop(bytes);
    assert_eq!(record.parity().encoded(), vector.parity.encoded());
    assert_eq!(record.baseline_content().bytes(), vector.baseline);
    assert_eq!(record.selected_content().bytes(), vector.selected);
    for (content, source) in [
        (record.baseline_content(), &vector.baseline),
        (record.selected_content(), &vector.selected),
    ] {
        assert_eq!(
            content.identity().address().as_bytes(),
            &hash("compiled-artifact-content/v1", source)
        );
    }
    assert_eq!(
        record.standing().artifact(),
        record.selected_content().identity()
    );
    assert_eq!(record.standing().pair(), record.parity().pair());
    assert_eq!(record.standing().check(), record.parity().witness().check());
    assert_eq!(
        record.standing().execution(),
        record.parity().production_report().key()
    );
    assert_eq!(record.standing().selection().surface().as_bytes(), &[7; 32]);
    assert_eq!(
        record.standing().selection().point().namespace(),
        "mutation-owner"
    );
    assert_eq!(record.standing().selection().point().stem(), "point");
    assert_eq!(
        record.standing().selection().alternative().as_bytes(),
        &[11; 32]
    );
    assert_eq!(
        record.baseline_report().encoded(),
        vector.baseline_report.encoded()
    );
    assert_eq!(
        record.selected_report().encoded(),
        vector.selected_report.encoded()
    );
    assert_eq!(record.mutation().encoded(), vector.mutation.encoded());
    assert_eq!(
        record.mutation().target().family(),
        Some("historical-family-outside-current-bank")
    );
    let ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)) =
        record.selected_report().attempt()
    else {
        return Err(());
    };
    let ArchivedMutationOutcome::Killed(ArchivedRejection::Demonstrated(rejection)) =
        record.mutation().outcome()
    else {
        return Err(());
    };
    assert_eq!(finding, rejection);
    assert_eq!(finding.foreign().ok_or(())?.bytes(), &[b'x', 0xff]);
    Ok(())
}

#[test]
fn compiled_phase_site_may_differ_from_parity_and_measurement_does_not_decide() -> Result<(), ()> {
    let mut vector = ProjectionVector::declared();
    *vector.baseline_report.site.last_mut().ok_or(())? = b'X';
    vector
        .selected_report
        .site
        .clone_from(&vector.baseline_report.site);
    vector.baseline_report.measurement = vec![2, 0];
    vector.selected_report.measurement = vec![0];
    vector
        .selected_report
        .measurement
        .extend_from_slice(&97u64.to_be_bytes());
    let record = read_projection(&vector.encoded(), LIMITS).map_err(|_| ())?;
    assert_ne!(
        record.baseline_report().site(),
        record.parity().production_report().site()
    );
    assert_ne!(
        record.baseline_report().measurement(),
        record.selected_report().measurement()
    );
    assert_ne!(
        record.standing().pair().production_revision().revision(),
        record.parity().witness().subject_revision().revision()
    );
    Ok(())
}

#[test]
fn each_artifact_identity_and_the_changed_source_requirement_is_independent() {
    for baseline in [true, false] {
        let mut vector = ProjectionVector::declared();
        if baseline {
            vector.baseline_id = Some(vec![9; 32]);
        } else {
            vector.selected_id = Some(vec![9; 32]);
        }
        assert_eq!(
            read_projection(&vector.encoded(), LIMITS),
            Err(ProjectionArchiveRefusal::ArtifactIdentityMismatch)
        );
    }
    let mut same = ProjectionVector::declared();
    same.selected.clone_from(&same.baseline);
    assert_eq!(
        read_projection(&same.encoded(), LIMITS),
        Err(ProjectionArchiveRefusal::ArtifactDidNotChange)
    );
    let mut width = ProjectionVector::declared();
    width.selected_id = Some(vec![9; 31]);
    assert_eq!(
        read_projection(&width.encoded(), LIMITS),
        Err(ProjectionArchiveRefusal::Record(
            ArchiveRefusal::InvalidAddressWidth
        ))
    );
}

#[test]
fn each_compiled_report_key_site_posture_and_outcome_join_is_observed() -> Result<(), ()> {
    for baseline in [true, false] {
        for offset in [48usize, 88, 123, 131, 139, 148] {
            let mut vector = ProjectionVector::declared();
            let report = if baseline {
                &mut vector.baseline_report
            } else {
                &mut vector.selected_report
            };
            *report.key.get_mut(offset).ok_or(())? ^= 1;
            assert!(
                read_projection(&vector.encoded(), LIMITS).is_err(),
                "role={baseline} offset={offset}"
            );
        }
        for moved in [0u8, 1, 2] {
            let mut vector = ProjectionVector::declared();
            let report = if baseline {
                &mut vector.baseline_report
            } else {
                &mut vector.selected_report
            };
            match moved {
                0 => *report.site.last_mut().ok_or(())? ^= 1,
                1 => report.posture = 2,
                _ => report.attempt = vec![3],
            }
            assert_eq!(
                read_projection(&vector.encoded(), LIMITS),
                Err(ProjectionArchiveRefusal::ReportJoinMismatch)
            );
        }
    }
    let mut reversed = ProjectionVector::declared();
    core::mem::swap(&mut reversed.baseline_report, &mut reversed.selected_report);
    assert_eq!(
        read_projection(&reversed.encoded(), LIMITS),
        Err(ProjectionArchiveRefusal::ReportJoinMismatch)
    );
    Ok(())
}

#[test]
fn raw_or_rejected_parity_cannot_be_promoted_by_the_projection_envelope() {
    for disposition in [vec![0], vec![2, 3]] {
        let mut vector = ProjectionVector::declared();
        if disposition == [2, 3] {
            vector.parity.conclusion =
                super::parity_vector::finding(&vector.parity.witness.trial());
        }
        vector.parity.disposition = disposition;
        assert_eq!(
            read_projection(&vector.encoded(), LIMITS),
            Err(ProjectionArchiveRefusal::ParityNotQualified)
        );
    }
}

#[test]
fn selection_surface_point_and_alternative_joins_refuse_separately() -> Result<(), ()> {
    for (offset, cause) in [
        (8usize, ProjectionArchiveRefusal::StandingMismatch),
        (48, ProjectionArchiveRefusal::MutationJoinMismatch),
        (85, ProjectionArchiveRefusal::MutationJoinMismatch),
    ] {
        let mut vector = ProjectionVector::declared();
        *vector.selection.get_mut(offset).ok_or(())? ^= 1;
        assert_eq!(read_projection(&vector.encoded(), LIMITS), Err(cause));
    }
    Ok(())
}

#[test]
fn mutation_road_owner_axes_and_full_finding_cannot_hide_behind_recomputed_integrity()
-> Result<(), ()> {
    for moved in 0u8..8 {
        let mut vector = ProjectionVector::declared();
        match moved {
            0 => *vector.mutation.target.first_mut().ok_or(())? = 1,
            1 => *vector.mutation.target.last_mut().ok_or(())? ^= 1,
            2 => vector.mutation.activation = super::proposal_vector::activation(7),
            3 => vector.mutation.equivalence = 2,
            4 => vector.mutation.outcome = vec![2, 3],
            5 => {
                let at = vector.mutation.outcome.len().saturating_sub(4);
                *vector.mutation.outcome.get_mut(at).ok_or(())? = b'y';
            }
            6 => {
                vector.mutation.outcome = vec![0, 1];
                vector
                    .mutation
                    .outcome
                    .extend(super::trial_vector::foreign(b"backend", &[0, 0]));
            }
            _ => {
                let mut target = super::proposal_vector::TargetVector::selected(2);
                target.family = vec![0];
                vector.mutation.target = target.body();
            }
        }
        assert_eq!(
            read_projection(&vector.encoded(), LIMITS),
            Err(ProjectionArchiveRefusal::MutationJoinMismatch),
            "move={moved}"
        );
    }
    Ok(())
}

#[test]
fn projection_headers_integrity_lengths_trailing_bytes_and_every_body_prefix_refuse()
-> Result<(), ()> {
    for (slot, value, cause) in [
        (0usize, 2u32, ArchiveRefusal::UnsupportedFormat { found: 2 }),
        (1, 2, ArchiveRefusal::WrongKind { found: 2 }),
        (2, 1, ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        let mut vector = ProjectionVector::declared();
        *vector.header.get_mut(slot).ok_or(())? = value;
        assert_eq!(
            read_projection(&vector.encoded(), LIMITS),
            Err(ProjectionArchiveRefusal::Record(cause))
        );
    }
    let vector = ProjectionVector::declared();
    let body = vector.body();
    for length in 0..body.len() {
        assert!(
            read_projection(&envelope(body.get(..length).ok_or(())?), LIMITS).is_err(),
            "prefix={length}"
        );
    }
    let mut corrupt = vector.encoded();
    *corrupt.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_projection(&corrupt, LIMITS),
        Err(ProjectionArchiveRefusal::Record(
            ArchiveRefusal::AddressMismatch
        ))
    );
    let mut trailing = body.clone();
    trailing.push(0);
    assert_eq!(
        read_projection(&envelope(&trailing), LIMITS),
        Err(ProjectionArchiveRefusal::Record(
            ArchiveRefusal::TrailingBytes
        ))
    );
    let mut huge = body.get(..12).ok_or(())?.to_vec();
    huge.extend_from_slice(&u64::MAX.to_be_bytes());
    assert_eq!(
        read_projection(&envelope(&huge), LIMITS),
        Err(ProjectionArchiveRefusal::Record(ArchiveRefusal::Truncated))
    );
    Ok(())
}

#[test]
fn projection_source_and_each_nested_resource_ceiling_is_independent() {
    let bytes = ProjectionVector::declared().encoded();
    let exact = ProjectionArchiveLimits::declared(
        ArchiveLimits::declared(bytes.len(), BYTES.field()),
        PARITY,
        BYTES,
        BYTES,
        3,
    );
    assert!(read_projection(&bytes, exact).is_ok());
    for limits in [
        ProjectionArchiveLimits::declared(
            ArchiveLimits::declared(bytes.len().saturating_sub(1), BYTES.field()),
            PARITY,
            BYTES,
            BYTES,
            3,
        ),
        ProjectionArchiveLimits::declared(
            ArchiveLimits::declared(BYTES.envelope(), 31),
            PARITY,
            BYTES,
            BYTES,
            3,
        ),
        ProjectionArchiveLimits::declared(
            BYTES,
            ParityArchiveLimits::declared(
                ArchiveLimits::declared(1, 8192),
                PARITY.binding(),
                PARITY.trial(),
                256,
                8,
            ),
            BYTES,
            BYTES,
            3,
        ),
        ProjectionArchiveLimits::declared(
            BYTES,
            PARITY,
            ArchiveLimits::declared(1, 32768),
            BYTES,
            3,
        ),
        ProjectionArchiveLimits::declared(
            BYTES,
            PARITY,
            BYTES,
            ArchiveLimits::declared(1, 32768),
            3,
        ),
        ProjectionArchiveLimits::declared(BYTES, PARITY, BYTES, BYTES, 2),
    ] {
        assert!(read_projection(&bytes, limits).is_err());
    }
}

#[test]
#[ignore = "driven by the historical projection process-boundary claim"]
fn child_loads_projection() -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(65537)
        .read_to_end(&mut bytes)?;
    let record = read_projection(&bytes, LIMITS)
        .map_err(|cause| std::io::Error::other(format!("{cause:?}")))?;
    drop(bytes);
    super::process::publish(record.encoded())
}

#[test]
fn historical_projection_survives_owned_readback_in_a_fresh_process() -> Result<(), Box<dyn Error>>
{
    let bytes = ProjectionVector::declared().encoded();
    assert_eq!(
        super::process::round_trip(&bytes, "archive::projections::child_loads_projection")?,
        bytes
    );
    Ok(())
}

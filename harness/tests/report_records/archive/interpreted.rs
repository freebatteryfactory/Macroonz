//! Complete historical data, distinct lawful contexts and owned process crossings.

use super::interpreted_vector::InterpretedVector;
use macroonz_harness::descriptor::archive::BindingArchiveLimits;
use macroonz_harness::muterprater::discovery_archive::SurfaceArchiveLimits;
use macroonz_harness::muterprater::interpretation_archive::{
    InterpretedArchiveLimits, ParityArchiveLimits, read_interpreted,
};
use macroonz_harness::muterprater::specimen_archive::ProjectionArchiveLimits;
use macroonz_harness::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutationOutcome, ArchivedRejection,
};
use macroonz_harness::report::archive::{ArchiveLimits, ArchivedAttempt, ArchivedConclusion};
use std::error::Error;
use std::io::Read as _;

const BYTES: ArchiveLimits = ArchiveLimits::declared(262_144, 131_072);
const PARITY: ParityArchiveLimits = ParityArchiveLimits::declared(
    BYTES,
    BindingArchiveLimits::declared(8192, 256, 8),
    BYTES,
    256,
    8,
);
pub(super) const LIMITS: InterpretedArchiveLimits = InterpretedArchiveLimits::declared(
    BYTES,
    SurfaceArchiveLimits::declared(BYTES, 8, 8),
    super::suites::LIMITS,
    ProjectionArchiveLimits::declared(BYTES, PARITY, BYTES, BYTES, 4096),
    BYTES,
    BYTES,
    256,
);

#[test]
fn independent_interpreted_envelope_retains_each_complete_member() -> Result<(), ()> {
    let vector = InterpretedVector::declared()?;
    let bytes = vector.encoded();
    let record = read_interpreted(&bytes, &LIMITS).map_err(|_| ())?;
    assert_eq!(record.encoded(), bytes);
    drop(bytes);
    assert_eq!(record.trust().surface().encoded(), vector.surface.encoded());
    assert_eq!(record.trust().suite().encoded(), vector.suite.encoded());
    assert_eq!(
        record.trust().projection().encoded(),
        vector.projection.encoded()
    );
    assert_eq!(record.meaning().bytes(), vector.meaning);
    assert_eq!(record.report().encoded(), vector.report.encoded());
    assert_eq!(record.mutation().encoded(), vector.mutation.encoded());
    assert_eq!(
        record.address().as_bytes(),
        &super::vector::hash("historical-interpreted-evidence/v1", &vector.body())
    );
    assert_eq!(
        record.meaning().convention(),
        record
            .trust()
            .projection()
            .parity()
            .production()
            .convention()
    );
    let ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding)) = record.report().attempt()
    else {
        return Err(());
    };
    let ArchivedMutationOutcome::Killed(ArchivedRejection::Demonstrated(rejection)) =
        record.mutation().outcome()
    else {
        return Err(());
    };
    assert_eq!(finding, rejection);
    assert_eq!(finding.foreign().ok_or(())?.bytes(), &[b'x', 255]);
    let ArchivedActivation::Observed(activation) = record.mutation().activation() else {
        return Err(());
    };
    assert_eq!(activation.firings(), 7);
    assert_eq!(activation.witness(), record.report().key().trial());
    let suite = record.trust().suite();
    let [_missed, first, later] = suite.manifest().run().reports() else {
        return Err(());
    };
    assert_eq!(suite.kill_ordinal(), 1);
    assert_eq!(suite.kill(), first);
    assert_ne!(first, later);
    Ok(())
}

#[test]
fn suite_context_pair_revisions_phase_sites_and_measurements_remain_independent() -> Result<(), ()>
{
    let mut vector = InterpretedVector::declared()?;
    vector.report.measurement = vec![2, 0];
    let record = read_interpreted(&vector.encoded(), &LIMITS).map_err(|_| ())?;
    let projection = record.trust().projection();
    assert_ne!(record.report().site(), projection.selected_report().site());
    assert_ne!(
        record.report().site(),
        projection.parity().production_report().site()
    );
    assert_ne!(
        projection.selected_report().site(),
        projection.parity().production_report().site()
    );
    assert_ne!(
        record.report().measurement(),
        projection.selected_report().measurement()
    );
    assert_ne!(
        projection.parity().pair().production_revision().revision(),
        projection.parity().witness().subject_revision().revision()
    );
    assert_ne!(
        record.trust().suite().kill().target(),
        projection.mutation().target()
    );
    for material in [
        super::backend_vector::Material::Absent,
        super::backend_vector::Material::Complete,
    ] {
        vector.suite = super::suite_vector::SuiteVector::declared(material);
        let material_record = read_interpreted(&vector.encoded(), &LIMITS).map_err(|_| ())?;
        assert_eq!(
            material_record
                .trust()
                .suite()
                .manifest()
                .original_console()
                .is_some(),
            matches!(material, super::backend_vector::Material::Complete)
        );
    }
    vector.meaning.clear();
    assert!(
        read_interpreted(&vector.encoded(), &LIMITS)
            .map_err(|_| ())?
            .meaning()
            .bytes()
            .is_empty()
    );
    Ok(())
}

#[test]
#[ignore = "driven by the complete interpreted process-boundary claim"]
fn child_loads_interpreted() -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(262_145)
        .read_to_end(&mut bytes)?;
    let record = read_interpreted(&bytes, &LIMITS)
        .map_err(|cause| std::io::Error::other(format!("{cause:?}")))?;
    drop(bytes);
    super::process::publish(record.encoded())
}

#[test]
fn complete_interpreted_bytes_cross_a_fresh_process_without_live_bindings()
-> Result<(), Box<dyn Error>> {
    let vector = InterpretedVector::declared().map_err(|()| std::io::Error::other("vector"))?;
    let bytes = vector.encoded();
    drop(vector);
    assert_eq!(
        super::process::round_trip(&bytes, "archive::interpreted::child_loads_interpreted")?,
        bytes
    );
    Ok(())
}

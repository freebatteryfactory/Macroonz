//! Historical suite pressure preserves its full run, exact selected kill and owned originals.

use super::backend_vector::{BackendVector, CONSOLE, FILE, Material, SOURCE};
use super::suite_vector::SuiteVector;
use macroonz_harness::muterprater::backend_archive::{
    SuitePressureArchiveLimits, read_backend, read_suite_pressure, retain_suite_pressure,
    retain_suite_pressure_with_material,
};
use macroonz_harness::muterprater::verdict_archive::{retain_mutation, retain_mutation_run};
use macroonz_harness::muterprater::{
    AdapterQualification, AnnouncedRoster, BackendVersion, CompiledSuiteArtifactCustody,
    CompiledSuiteArtifactStanding, CompiledSuitePressure, GrammarStanding, MutationSourceRevision,
    MutationVerdict,
};
use macroonz_harness::report::archive::ArchiveLimits;
use std::error::Error;
use std::io::Read as _;

pub(super) const LIMITS: SuitePressureArchiveLimits = SuitePressureArchiveLimits::declared(
    ArchiveLimits::declared(131_072, 65_536),
    super::backends::LIMITS,
);

pub(super) fn fixture(
    console: &str,
    sources: &[(&str, &[u8])],
) -> Result<CompiledSuitePressure, ()> {
    let manifest = super::backends::fixture(console, sources)?;
    let version = BackendVersion::stated("27-test").map_err(|_| ())?;
    let qualification =
        AdapterQualification::of(manifest.reading(), GrammarStanding::Checked(version))
            .map_err(|_| ())?;
    let current = sources
        .iter()
        .map(|(file, bytes)| MutationSourceRevision::from_content(file, bytes))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let custody = CompiledSuiteArtifactCustody::current(manifest, current).map_err(|_| ())?;
    CompiledSuitePressure::demonstrated(
        CompiledSuiteArtifactStanding::Reported(&custody),
        &qualification,
    )
    .map_err(|_| ())
}

#[test]
fn independent_suite_envelope_matches_live_retention_with_and_without_originals() -> Result<(), ()>
{
    let pressure = fixture(CONSOLE, &[(FILE, SOURCE)])?;
    for material in [Material::Absent, Material::Complete] {
        let vector = SuiteVector::declared(material);
        let record = read_suite_pressure(&vector.encoded(), LIMITS).map_err(|_| ())?;
        let retained = if matches!(material, Material::Complete) {
            retain_suite_pressure_with_material(&pressure, CONSOLE, &[(FILE, SOURCE)], LIMITS)
        } else {
            retain_suite_pressure(&pressure, LIMITS)
        }
        .map_err(|_| ())?;
        assert_eq!(retained, record);
        assert_eq!(record.encoded(), vector.encoded());
        assert_eq!(
            record.address().as_bytes(),
            &super::vector::hash("historical-compiled-suite-pressure/v1", &vector.body())
        );
        assert_eq!(record.checked_version(), "27-test");
        assert_eq!(record.qualification_profile(), record.manifest().profile());
        assert_eq!(record.kill_ordinal(), 0);
        assert_eq!(
            record.manifest(),
            &read_backend(
                &BackendVector::declared(material).encoded(),
                LIMITS.backend()
            )
            .map_err(|_| ())?
        );
        assert_eq!(
            record.kill(),
            &retain_mutation(pressure.kill(), LIMITS.backend().bytes()).map_err(|_| ())?
        );
        assert_eq!(
            record.manifest().original_console(),
            matches!(material, Material::Complete).then_some(CONSOLE.as_bytes())
        );
    }
    Ok(())
}

#[test]
fn independent_mixed_run_retains_inconclusive_and_later_kill_beside_the_first_kill()
-> Result<(), ()> {
    let vector = SuiteVector::mixed();
    let record = read_suite_pressure(&vector.encoded(), LIMITS).map_err(|_| ())?;
    let [missed, first, later] = record.manifest().run().reports() else {
        return Err(());
    };
    assert_eq!(
        MutationVerdict::from(missed.outcome()),
        MutationVerdict::Inconclusive
    );
    assert_eq!(
        MutationVerdict::from(first.outcome()),
        MutationVerdict::Killed
    );
    assert_eq!(
        MutationVerdict::from(later.outcome()),
        MutationVerdict::Killed
    );
    assert_ne!(first, later);
    assert_eq!(record.kill_ordinal(), 1);
    assert_eq!(record.kill(), first);
    assert_eq!(record.manifest().run().census().pressed(), 3);
    assert_eq!(record.manifest().run().census().killed(), 2);
    assert_eq!(record.manifest().run().census().inconclusive(), 1);
    assert_eq!(record.manifest().announced(), AnnouncedRoster::Stated(19));
    Ok(())
}

#[test]
fn synthetic_suite_retains_all_outcome_roads_and_complete_original_rosters() -> Result<(), ()> {
    let console = "Found 99 mutants\nok Unmutated baseline\nmissed z.rs:1:1: change\ncaught a.rs:2:2: first kill\nunviable a.rs:3:3: change\ntimeout z.rs:4:4: change\nfailed a.rs:5:5: change\ncaught z.rs:6:6: later kill\nsummary\n";
    let sources = [("z.rs", b"z".as_slice()), ("a.rs", &[255u8])];
    let pressure = fixture(console, &sources)?;
    let record = retain_suite_pressure_with_material(&pressure, console, &sources, LIMITS)
        .map_err(|_| ())?;
    assert_eq!(record.kill_ordinal(), 1);
    assert_eq!(record.manifest().run().census().pressed(), 6);
    assert_eq!(record.manifest().run().census().killed(), 2);
    assert_eq!(record.manifest().run().census().inconclusive(), 4);
    assert_eq!(record.manifest().announced(), AnnouncedRoster::Stated(99));
    assert_eq!(
        record.manifest().run(),
        &retain_mutation_run(
            pressure.custody().manifest().reading().run(),
            LIMITS.backend().run()
        )
        .map_err(|_| ())?
    );
    assert_eq!(
        record.kill(),
        &retain_mutation(pressure.kill(), LIMITS.backend().bytes()).map_err(|_| ())?
    );
    let [a, z] = record.manifest().sources() else {
        return Err(());
    };
    assert_eq!((a.file(), a.original()), ("a.rs", Some([255u8].as_slice())));
    assert_eq!((z.file(), z.original()), ("z.rs", Some(b"z".as_slice())));
    let [z_source, a_source] = sources;
    assert_eq!(
        retain_suite_pressure_with_material(&pressure, console, &[a_source, z_source], LIMITS)
            .map_err(|_| ())?,
        record
    );
    let [unread] = record.manifest().unparsed() else {
        return Err(());
    };
    assert_eq!(unread.ordinal(), 8);
    assert_eq!(unread.text().bytes(), b"summary");
    drop(pressure);
    assert_eq!(
        read_suite_pressure(record.encoded(), LIMITS).map_err(|_| ())?,
        record
    );
    Ok(())
}

#[test]
fn suite_records_cross_a_fresh_process_after_the_live_source_is_dropped()
-> Result<(), Box<dyn Error>> {
    let pressure =
        fixture(CONSOLE, &[(FILE, SOURCE)]).map_err(|()| std::io::Error::other("fixture"))?;
    let live = retain_suite_pressure_with_material(&pressure, CONSOLE, &[(FILE, SOURCE)], LIMITS)
        .map_err(|cause| std::io::Error::other(format!("{cause:?}")))?;
    drop(pressure);
    for bytes in [
        SuiteVector::declared(Material::Absent).encoded(),
        SuiteVector::declared(Material::Complete).encoded(),
        SuiteVector::mixed().encoded(),
        live.encoded().to_vec(),
    ] {
        assert_eq!(
            crate::archive_process::round_trip(&bytes, "archive::suites::suite_archive_child")?,
            bytes
        );
    }
    Ok(())
}

#[test]
#[ignore = "invoked by the parent with bounded complete historical suite pressure"]
fn suite_archive_child() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(131_073)
        .read_to_end(&mut encoded)?;
    let record = read_suite_pressure(&encoded, LIMITS)
        .map_err(|cause| std::io::Error::other(format!("{cause:?}")))?;
    drop(encoded);
    crate::archive_process::publish(record.encoded())
}

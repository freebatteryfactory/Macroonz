//! Backend archive fields, complete originals and fresh-process ownership.

use super::backend_vector::{BackendVector, CONSOLE, FILE, Material, SOURCE};
use macroonz_harness::muterprater::backend_archive::{
    BackendArchiveLimits, BackendArchiveRefusal, read_backend, retain_backend,
    retain_backend_with_material,
};
use macroonz_harness::muterprater::verdict_archive::{
    MutationRunArchiveLimits, retain_mutation_run,
};
use macroonz_harness::muterprater::{
    AnnouncedRoster, BackendCommand, BackendVersion, ClaimCeiling, CompiledSuiteArtifactManifest,
    MutationBackendInvocation, MutationSourceRevision, ReadingSource, WrappedBackend,
};
use macroonz_harness::report::archive::ArchiveLimits;
use macroonz_harness::report::{TargetBinding, TargetTriple, ToolchainIdentity};
use std::error::Error;
use std::io::Read as _;

pub(super) const LIMITS: BackendArchiveLimits = BackendArchiveLimits::declared(
    MutationRunArchiveLimits::declared(ArchiveLimits::declared(65_536, 32_768), 8),
    8,
    8,
    8,
);

pub(super) fn fixture(
    console: &str,
    sources: &[(&str, &[u8])],
) -> Result<CompiledSuiteArtifactManifest, ()> {
    let invocation = MutationBackendInvocation::declared(
        WrappedBackend::CargoMutants,
        BackendVersion::stated("27-test").map_err(|_| ())?,
        BackendCommand::declared("cargo path", &["mutants", "", "two words"]).map_err(|_| ())?,
        TargetBinding::bound(
            TargetTriple::declared("native-test"),
            ToolchainIdentity::declared("rust-test"),
        ),
    );
    let sources = sources
        .iter()
        .map(|(file, bytes)| MutationSourceRevision::from_content(file, bytes))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    macroonz_harness::muterprater::wrap::read_artifact(
        console,
        invocation,
        sources,
        |_| None,
        |_, _| None,
    )
    .map_err(|_| ())
}

#[test]
fn independent_backend_bytes_match_every_live_retained_field() -> Result<(), ()> {
    let manifest = fixture(CONSOLE, &[(FILE, SOURCE)])?;
    for material in [Material::Absent, Material::Complete] {
        let originals = matches!(material, Material::Complete);
        let vector = BackendVector::declared(material);
        let record = read_backend(&vector.encoded(), LIMITS).map_err(|_| ())?;
        let retained = if originals {
            retain_backend_with_material(&manifest, CONSOLE, &[(FILE, SOURCE)], LIMITS)
        } else {
            retain_backend(&manifest, LIMITS)
        }
        .map_err(|_| ())?;
        assert_eq!(retained, record);
        assert_eq!(record.invocation().backend(), WrappedBackend::CargoMutants);
        assert_eq!(record.invocation().version(), "27-test");
        assert_eq!(record.invocation().executable(), "cargo path");
        assert_eq!(
            record.invocation().arguments(),
            ["mutants", "", "two words"]
        );
        assert_eq!(record.invocation().target(), manifest.invocation().target());
        assert_eq!(record.profile().backend(), WrappedBackend::CargoMutants);
        assert_eq!(record.profile().version(), "27-test");
        assert_eq!(record.profile().source(), ReadingSource::ConsoleStream);
        assert_eq!(record.profile().grammar().number(), 1);
        assert_eq!(record.profile().ceiling(), ClaimCeiling::WitnessRejection);
        assert_eq!(
            record.output().as_bytes(),
            manifest.output().address().as_bytes()
        );
        assert_eq!(record.announced(), AnnouncedRoster::Stated(19));
        assert_eq!(record.run().census().pressed(), 1);
        assert_eq!(
            record.run(),
            &retain_mutation_run(manifest.reading().run(), LIMITS.run()).map_err(|_| ())?
        );
        let [source] = record.sources() else {
            return Err(());
        };
        assert_eq!(source.file(), FILE);
        assert_eq!(
            source.revision().as_bytes(),
            &super::vector::hash("mutation-source-revision/v1", SOURCE)
        );
        assert_eq!(source.original(), originals.then_some(SOURCE));
        assert_eq!(
            record.original_console(),
            originals.then_some(CONSOLE.as_bytes())
        );
        let [line] = record.unparsed() else {
            return Err(());
        };
        assert_eq!(line.ordinal(), 3);
        assert_eq!(line.text().bytes(), b"summary");
        assert_eq!(record.encoded(), vector.encoded());
    }
    Ok(())
}

/// Synthetic text exercises every outcome and canonical source joining without claiming a backend execution.
#[test]
fn synthetic_multifile_console_preserves_all_outcomes_and_original_order() -> Result<(), ()> {
    let console = "Found 7 mutants\nmissed z.rs:0:0: replace x with y\nok Unmutated baseline\ncaught a.rs:1:1: replace x with y\nunviable a.rs:1:1: replace x with y\ntimeout z.rs:2:2: replace x with y\nfailed a.rs:3:3: replace x with y\nfailed Unmutated baseline\nFound 99 mutants\n";
    let sources = [("z.rs", b"z".as_slice()), ("a.rs", b"a".as_slice())];
    let manifest = fixture(console, &sources)?;
    let record =
        retain_backend_with_material(&manifest, console, &sources, LIMITS).map_err(|_| ())?;
    assert_eq!(record.announced(), AnnouncedRoster::Stated(99));
    assert_eq!(record.run().census().killed(), 1);
    assert_eq!(record.run().census().inconclusive(), 4);
    assert_eq!(record.run().census().survived(), 0);
    assert_eq!(
        record.run(),
        &retain_mutation_run(manifest.reading().run(), LIMITS.run()).map_err(|_| ())?
    );
    let [a, z] = record.sources() else {
        return Err(());
    };
    assert_eq!((a.file(), a.original()), ("a.rs", Some(b"a".as_slice())));
    assert_eq!((z.file(), z.original()), ("z.rs", Some(b"z".as_slice())));
    let [z_source, a_source] = sources;
    let sorted = [a_source, z_source];
    assert_eq!(
        retain_backend_with_material(&manifest, console, &sorted, LIMITS).map_err(|_| ())?,
        record
    );
    let empty = fixture("ok Unmutated baseline\n", &[])?;
    let empty = retain_backend_with_material(&empty, "ok Unmutated baseline\n", &[], LIMITS)
        .map_err(|_| ())?;
    assert!(empty.sources().is_empty());
    assert!(empty.run().reports().is_empty());
    assert_eq!(empty.announced(), AnnouncedRoster::Unstated);
    Ok(())
}

#[test]
fn supplied_material_refuses_every_roster_and_content_mismatch() -> Result<(), ()> {
    let manifest = fixture(CONSOLE, &[(FILE, SOURCE)])?;
    for (sources, expected) in [
        (
            vec![],
            BackendArchiveRefusal::SourceMissing(FILE.to_owned()),
        ),
        (
            vec![(FILE, SOURCE), (FILE, SOURCE)],
            BackendArchiveRefusal::DuplicateMaterialSource(FILE.to_owned()),
        ),
        (
            vec![(FILE, SOURCE), ("extra", SOURCE)],
            BackendArchiveRefusal::SourceUnexpected("extra".to_owned()),
        ),
        (
            vec![(FILE, b"changed".as_slice())],
            BackendArchiveRefusal::SourceMaterialMismatch(FILE.to_owned()),
        ),
    ] {
        assert_eq!(
            retain_backend_with_material(&manifest, CONSOLE, &sources, LIMITS),
            Err(expected)
        );
    }
    assert_eq!(
        retain_backend_with_material(
            &manifest,
            &format!("{CONSOLE}\n"),
            &[(FILE, SOURCE)],
            LIMITS
        ),
        Err(BackendArchiveRefusal::OutputMaterialMismatch)
    );
    Ok(())
}

#[test]
fn independent_and_live_originals_own_their_data_in_a_fresh_process() -> Result<(), Box<dyn Error>>
{
    let manifest =
        fixture(CONSOLE, &[(FILE, SOURCE)]).map_err(|()| std::io::Error::other("fixture"))?;
    let live = retain_backend_with_material(&manifest, CONSOLE, &[(FILE, SOURCE)], LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    for bytes in [
        BackendVector::declared(Material::Absent).encoded(),
        BackendVector::declared(Material::Complete).encoded(),
        live.encoded().to_vec(),
    ] {
        assert_eq!(
            crate::archive_process::round_trip(&bytes, "archive::backends::backend_archive_child")?,
            bytes
        );
    }
    Ok(())
}

#[test]
#[ignore = "invoked by the parent with a bounded complete historical backend manifest"]
fn backend_archive_child() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(65_537)
        .read_to_end(&mut encoded)?;
    let record = read_backend(&encoded, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(encoded);
    crate::archive_process::publish(record.encoded())
}

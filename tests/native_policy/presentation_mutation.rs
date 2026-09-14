//! Complete backend populations and weaker historical readings through the root facade.

use crate::presentation_formats::{field, parsed};
use macroonz::harness::muterprater::{
    BackendCommand, BackendVersion, BackendVersionPosture, CompiledSuiteArtifactManifest,
    MutationBackendInvocation, MutationSourceRevision, WrappedBackend, backend_archive,
    verdict_archive, wrap,
};
use macroonz::harness::report::{
    TargetBinding, TargetTriple, ToolchainIdentity, archive::ArchiveLimits,
};
use macroonz::presentation;
use serde_json::{Value, json};

const LIMITS: backend_archive::BackendArchiveLimits =
    backend_archive::BackendArchiveLimits::declared(
        verdict_archive::MutationRunArchiveLimits::declared(
            ArchiveLimits::declared(65_536, 16_384),
            16,
        ),
        8,
        8,
        8,
    );

pub(super) fn manifest(console: &str) -> Result<CompiledSuiteArtifactManifest, String> {
    let invocation = MutationBackendInvocation::declared(
        WrappedBackend::CargoMutants,
        BackendVersion::stated("presentation-test").map_err(|error| format!("{error:?}"))?,
        BackendCommand::declared("cargo path", &["mutants", "", "<argument>|two words"])
            .map_err(|error| format!("{error:?}"))?,
        TargetBinding::bound(
            TargetTriple::declared("declared-target"),
            ToolchainIdentity::declared("declared-tool"),
        ),
    );
    let source = MutationSourceRevision::from_content("fixture.rs", b"\0\xffsource")
        .map_err(|error| format!("{error:?}"))?;
    wrap::read_artifact(console, invocation, vec![source], |_| None, |_, _| None)
        .map_err(|error| format!("{error:?}"))
}

#[test]
fn mutation_presentation_keeps_duplicate_targets_all_axes_and_unparsed_lines() -> Result<(), String>
{
    let console = "Found 99 mutants\nok Unmutated baseline\ncaught fixture.rs:0:0: replace <x>| with y\ncaught fixture.rs:0:0: replace <x>| with y\nmissed fixture.rs:1:2: replace a with b\nunviable fixture.rs:3:4: replace a with b\ntimeout fixture.rs:5:6: replace a with b\nfailed fixture.rs:7:8: replace a with b\nunread <tag>|&\n";
    let manifest = manifest(console)?;
    let reading = parsed(&presentation::backend_reading(manifest.reading()))?;
    let run = parsed(&presentation::mutation_run(manifest.reading().run()))?;
    assert_eq!(field(&reading, "/record/run")?, field(&run, "/record")?);
    assert_eq!(
        field(&reading, "/record/announced")?,
        &json!({"kind":"stated", "value":99u32})
    );
    assert_eq!(field(&run, "/record/denominator")?, &json!(6usize));
    assert_eq!(
        field(&run, "/record/census")?,
        &json!({"pressed":6u32, "killed":2u32, "survived":0u32, "inconclusive":4u32})
    );
    let reports = field(&run, "/record/reports")?
        .as_array()
        .ok_or("missing reports")?;
    assert_eq!(reports.len(), 6);
    assert_eq!(reports.first(), reports.get(1));
    let expected = [
        ("built", "completed", "killed"),
        ("built", "completed", "killed"),
        ("built", "completed", "inconclusive"),
        ("unviable", "infrastructure-failed", "inconclusive"),
        ("built", "timed-out", "inconclusive"),
        ("tool-failed", "infrastructure-failed", "inconclusive"),
    ];
    for (record, (materialization, execution, outcome)) in reports.iter().zip(expected) {
        assert_eq!(field(record, "/baseline")?, "qualified");
        assert_eq!(field(record, "/materialization")?, materialization);
        assert_eq!(field(record, "/execution")?, execution);
        assert_eq!(field(record, "/outcome/kind")?, outcome);
        assert_eq!(
            field(record, "/activation/kind")?,
            "unobservable-under-backend"
        );
        assert_eq!(field(record, "/equivalence")?, "not-assessed");
        assert_eq!(field(record, "/target/owner/kind")?, "owner-unmapped");
    }
    assert_eq!(
        field(&run, "/record/reports/2/outcome/value")?,
        "unobservable-and-unrejected"
    );
    assert_eq!(
        field(&run, "/record/reports/0/outcome/value/kind")?,
        "reported-by-backend"
    );
    assert!(
        run.pointer("/record/reports/0/outcome/value/value/fingerprint")
            .is_none()
    );
    assert_eq!(
        field(&reading, "/record/unparsed/0/ordinal")?,
        &json!(8usize)
    );
    assert_eq!(
        field(&reading, "/record/unparsed/0/text/shown")?,
        "unread <tag>|&"
    );
    historical(&manifest, console)?;
    Ok(())
}

fn historical(manifest: &CompiledSuiteArtifactManifest, console: &str) -> Result<(), String> {
    let current = parsed(&presentation::backend_manifest(manifest))?;
    for originals in [false, true] {
        let archive = if originals {
            backend_archive::retain_backend_with_material(
                manifest,
                console,
                &[("fixture.rs", b"\0\xffsource")],
                LIMITS,
            )
        } else {
            backend_archive::retain_backend(manifest, LIMITS)
        }
        .map_err(|error| format!("{error:?}"))?;
        let archive = backend_archive::read_backend(archive.encoded(), LIMITS)
            .map_err(|error| format!("{error:?}"))?;
        let historical = parsed(&presentation::archived_backend(&archive))?;
        assert_eq!(
            field(&historical, "/standing")?,
            "historical-unauthenticated"
        );
        for path in [
            "/record/invocation",
            "/record/output",
            "/record/reading/unparsed",
            "/record/reading/profile",
            "/record/reading/announced",
        ] {
            assert_eq!(field(&current, path)?, field(&historical, path)?);
        }
        assert_eq!(
            field(&historical, "/record/sources/0/original")?,
            &json!(originals.then_some("00ff736f75726365"))
        );
        assert_eq!(
            field(&historical, "/record/original_console")?.is_string(),
            originals
        );
        let run = parsed(&presentation::archived_mutation_run(archive.run()))?;
        assert_eq!(
            field(&run, "/record")?,
            field(&historical, "/record/reading/run")?
        );
        assert_eq!(
            archive.run().reports().len(),
            manifest.reading().run().reports().len()
        );
        for (record, retained) in manifest
            .reading()
            .run()
            .reports()
            .iter()
            .zip(archive.run().reports())
        {
            let shown = parsed(&presentation::mutation_record(record))?;
            let history = parsed(&presentation::archived_mutation(retained))?;
            assert_eq!(field(&history, "/standing")?, "historical-unauthenticated");
            for key in [
                "target",
                "baseline",
                "materialization",
                "activation",
                "execution",
                "equivalence",
                "outcome",
            ] {
                let path = format!("/record/{key}");
                assert_eq!(field(&shown, &path)?, field(&history, &path)?);
            }
        }
    }
    Ok(())
}

#[test]
fn mutation_empty_reading_is_distinct_from_missing_or_failed_baseline() -> Result<(), String> {
    let reading = wrap::read_output(
        "ok Unmutated baseline\n",
        BackendVersionPosture::Unstated,
        |_| None,
        |_, _| None,
    )
    .map_err(|error| format!("{error:?}"))?;
    let empty = parsed(&presentation::backend_reading(&reading))?;
    assert_eq!(field(&empty, "/record/run/reports")?, &json!([]));
    assert_eq!(field(&empty, "/record/announced/value")?, &Value::Null);
    assert_eq!(field(&empty, "/record/profile/version/kind")?, "unstated");
    for (console, expected) in [
        (
            "Found 9 mutants\n",
            json!({"kind":"baseline-not-stated", "value":null}),
        ),
        (
            "failed Unmutated baseline\n",
            json!({"kind":"baseline-not-qualified", "value":"baseline-failed"}),
        ),
    ] {
        let refusal = wrap::read_output(
            console,
            BackendVersionPosture::Unstated,
            |_| None,
            |_, _| None,
        )
        .err()
        .ok_or("baseline unexpectedly admitted")?;
        let shown = parsed(&presentation::backend_reading_refusal(&refusal))?;
        assert_eq!(field(&shown, "/record/cause")?, &expected);
        assert!(shown.pointer("/record/run").is_none());
    }
    let current = manifest("ok Unmutated baseline\ncaught fixture.rs:1:1: replace x with y\n")?;
    let refusal = wrap::read_artifact(
        "ok Unmutated baseline\ncaught absent.rs:1:1: replace x with y\n",
        current.invocation().clone(),
        current.sources().to_vec(),
        |_| None,
        |_, _| None,
    )
    .err()
    .ok_or("missing source admitted")?;
    let shown = parsed(&presentation::backend_manifest_refusal(&refusal))?;
    assert_eq!(field(&shown, "/record/phase")?, "artifact-join");
    assert_eq!(
        field(&shown, "/record/cause")?,
        &json!({"kind":"reported-source-missing", "value":"absent.rs"})
    );
    Ok(())
}

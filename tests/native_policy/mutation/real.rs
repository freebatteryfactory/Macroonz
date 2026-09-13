use super::configure::{archives, backend, finish, request};
use crate::compiler::configure::{host, root};
use macroonz::harness::muterprater::backend_archive::{BackendSourceRefusal, read_backend};
use macroonz::harness::muterprater::{ArtifactCustodyRefusal, MutationVerdict};
use macroonz::native_mutation::{self, MutationCustodyError, MutationObservationError};
use macroonz::native_process::{ProcessLimits, ProcessStop};
use std::time::Duration;

const SOURCE: &str =
    "pub fn step(value: u32) -> u32 { value + 1 }\n#[path = \"other/fixture.rs\"] pub mod other;\n";

#[test]
fn actual_backend_retains_command_sources_and_historical_currency() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    fixture(&root)?;
    let limits = ProcessLimits::informed(
        Duration::from_secs(180),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    let backend = backend()?;
    let selected = request(&host, &root, &backend, limits, "real-output")?;
    let executed = selected.clone();
    let output = finish(
        native_mutation::run(selected, |_| None, |_, _| None).map_err(|error| error.to_string())?,
    )?;
    let manifest = output
        .manifest()
        .map_err(|error| format!("{error}: {:?}", output.process()))?;
    super::presentation::accepted(&output)?;
    assert_eq!(output.process().stop(), &ProcessStop::Exited);
    assert_eq!(output.process().status().code(), Some(2_i32));
    assert!(
        manifest
            .reading()
            .run()
            .reports()
            .iter()
            .any(|report| report.verdict() == MutationVerdict::Killed)
    );
    assert!(
        manifest
            .reading()
            .run()
            .reports()
            .iter()
            .any(|report| report.verdict() == MutationVerdict::Inconclusive)
    );
    assert!(
        manifest
            .reading()
            .run()
            .reports()
            .iter()
            .all(|report| report.verdict() != MutationVerdict::Survived)
    );
    assert_eq!(manifest.invocation().version().spelling(), "27.0.0");
    assert_eq!(
        manifest
            .sources()
            .iter()
            .map(macroonz::harness::muterprater::MutationSourceRevision::file)
            .collect::<Vec<_>>(),
        ["fixture.rs"]
    );
    assert_eq!(
        manifest.invocation().command().arguments(),
        executed.process().arguments()
    );
    assert_eq!(
        manifest.invocation().command().executable(),
        backend.to_str().ok_or("backend path")?
    );
    assert_eq!(
        manifest.invocation().target().target().spelling(),
        host.triple
    );
    assert!(
        manifest
            .invocation()
            .target()
            .toolchain()
            .spelling()
            .starts_with("rustc 1.98.1 ")
    );
    assert_eq!(
        output.original_sources().collect::<Vec<_>>(),
        [("fixture.rs", SOURCE.as_bytes())]
    );
    compare_sources(&output, &root)?;
    assert!(native_mutation::run(executed, |_| None, |_, _| None).is_err());
    failed_baseline(&host, &root, &backend, limits)
}

fn failed_baseline(
    host: &crate::compiler::types::Host,
    root: &std::path::Path,
    backend: &std::path::Path,
    limits: ProcessLimits,
) -> Result<(), String> {
    let failed = request(host, root, backend, limits, "failed-baseline")?;
    let failed = finish(
        native_mutation::run(failed, |_| None, |_, _| None).map_err(|error| error.to_string())?,
    )?;
    assert_eq!(failed.process().status().code(), Some(4_i32));
    super::presentation::observation_failure(&failed, "process")?;
    assert_eq!(
        failed.manifest().err(),
        Some(&MutationObservationError::Process)
    );
    Ok(())
}

fn compare_sources(
    output: &native_mutation::MutationOutput,
    root: &std::path::Path,
) -> Result<(), String> {
    assert!(output.current_sources(root, 1024).is_ok());
    let retained = output
        .retain(archives())
        .map_err(|error| error.to_string())?;
    let saved = root.join("retained.bin");
    std::fs::write(&saved, retained.encoded()).map_err(|error| error.to_string())?;
    let archived = read_backend(
        &std::fs::read(saved).map_err(|error| error.to_string())?,
        archives(),
    )
    .map_err(|error| format!("{error:?}"))?;
    let compared = native_mutation::compare_historical(&archived, root, 1024)
        .map_err(|error| error.to_string())?;
    super::presentation::archived(output, &archived)?;
    let before = macroonz::presentation::native_mutation(output);
    assert_eq!(compared.manifest().encoded(), retained.encoded());
    assert_eq!(compared.current_sources().len(), 1);
    std::fs::write(
        root.join("fixture.rs"),
        "pub fn step(value: u32) -> u32 { value + 2 }\n",
    )
    .map_err(|error| error.to_string())?;
    assert!(matches!(
        output.current_sources(root, 1024),
        Err(MutationCustodyError::Current(
            ArtifactCustodyRefusal::CurrentSourceMoved { .. }
        ))
    ));
    assert!(
        matches!(native_mutation::compare_historical(&archived, root, 1024), Err(MutationCustodyError::Historical(BackendSourceRefusal::Moved(file))) if file == "fixture.rs")
    );
    assert_eq!(
        output.original_sources().collect::<Vec<_>>(),
        [("fixture.rs", SOURCE.as_bytes())]
    );
    assert_eq!(before, macroonz::presentation::native_mutation(output));
    super::presentation::archived(output, &archived)?;
    Ok(())
}

fn fixture(root: &std::path::Path) -> Result<(), String> {
    drop(crate::compiler::real::package(
        root,
        "[lib]\nname = \"mutation_fixture\"\npath = \"fixture.rs\"\n[[test]]\nname = \"independent\"\npath = \"independent.rs\"\n",
    )?);
    std::fs::write(root.join("fixture.rs"), SOURCE).map_err(|error| error.to_string())?;
    std::fs::create_dir(root.join("other")).map_err(|error| error.to_string())?;
    std::fs::write(
        root.join("other/fixture.rs"),
        "pub fn unrelated(value: u32) -> u32 { value + 9 }\n",
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        root.join("independent.rs"),
        "#[test]\nfn zero_advances() { assert_eq!(mutation_fixture::step(0), 1); }\n",
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

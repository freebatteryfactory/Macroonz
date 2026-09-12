use crate::compiler::{configure, types::Host};
use macroonz::harness::muterprater::backend_archive::BackendArchiveLimits;
use macroonz::harness::muterprater::verdict_archive::MutationRunArchiveLimits;
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::native_mutation::{
    MutationOutput, MutationRequest, MutationRun, MutationSources, MutationToolchain,
};
use macroonz::native_process::ProcessLimits;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub(super) fn request(
    host: &Host,
    root: &Path,
    backend: &Path,
    limits: ProcessLimits,
    name: &str,
) -> Result<MutationRequest, String> {
    let selected = configure::tool(host, backend, root, limits)?;
    let toolchain = MutationToolchain::declared(host.cargo.clone(), host.rustc.clone())
        .map_err(|error| error.to_string())?;
    let source =
        RelativeSourcePath::informed("fixture.rs").map_err(|error| format!("{error:?}"))?;
    let sources =
        MutationSources::declared(vec![source], 1_048_576).map_err(|error| error.to_string())?;
    MutationRequest::cargo_mutants(
        &selected,
        toolchain,
        sources,
        root.join(name),
        &root.join("mutation-target"),
        &host.triple,
    )
    .map_err(|error| error.to_string())
}

pub(super) fn finish(run: MutationRun) -> Result<MutationOutput, String> {
    match run {
        MutationRun::Finished(output) => Ok(*output),
        MutationRun::Pending(pending) => {
            let detail = format!("unexpected mutation cleanup: {:?}", pending.process());
            drop(pending.finish(Duration::from_secs(5)));
            Err(detail)
        }
    }
}

pub(super) fn backend() -> Result<PathBuf, String> {
    let output = std::process::Command::new(if cfg!(windows) { "where.exe" } else { "which" })
        .arg("cargo-mutants")
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let text = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
    text.lines()
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "missing cargo-mutants installation".to_owned())
}

pub(super) const fn archives() -> BackendArchiveLimits {
    BackendArchiveLimits::declared(
        MutationRunArchiveLimits::declared(ArchiveLimits::declared(4_194_304, 1_048_576), 1024),
        128,
        16,
        1024,
    )
}

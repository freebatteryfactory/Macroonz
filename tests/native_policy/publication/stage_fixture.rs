use super::configure::{configuration, finish, publication, scratch};
use super::fixture::{LIMITS, path};
use super::types::Example;
use crate::compiler::configure::{bounds, locus, tool};
use crate::compiler::types::Host;
use macroonz::native_compiler::{CompilerRequest, DependencyLimits};
use macroonz::native_process::ProcessLimits;
use macroonz::native_publication::{
    AuthoredFile, CompiledPublication, Formatter, PreparedPublication, StagedPublication,
    StagingPlan, StagingRun,
};
use macroonz::native_storage::StorageName;
use std::path::Path;
use std::time::Duration;

pub(super) const DRIVER: &str = "#[path = \"generated/definition.rs\"] mod definition;\n#[path = \"generated/alpha.rs\"] mod alpha;\n#[path = \"generated/beta.rs\"] mod beta;\n#[path = \"generated/other.rs\"] mod other;\nuse std::io::Write;\nfn main() -> std::io::Result<()> { assert_eq!(alpha::VALUE, 7); assert_eq!(beta::VALUE, 9); writeln!(std::io::stdout(), \"{}\", other::OTHER) }\n";
pub(super) const DEPENDENCIES: DependencyLimits = DependencyLimits {
    bytes: 65536,
    files: 16,
};

pub(super) fn authored(name: &str, source: &[u8]) -> Result<AuthoredFile, String> {
    Ok(AuthoredFile {
        path: path(name)?,
        bytes: source.to_vec(),
    })
}

pub(super) fn plan(source: &str) -> Result<StagingPlan<Example>, String> {
    StagingPlan::declared(
        PreparedPublication::unformatted(publication()?),
        vec![authored("fixture.rs", source.as_bytes())?],
        LIMITS,
    )
    .map_err(|error| error.to_string())
}

pub(super) fn staged(
    root: &Path,
    name: &str,
    source: &str,
) -> Result<StagedPublication<Example>, String> {
    plan(source)?
        .stage(
            root,
            &StorageName::informed(name).map_err(|error| format!("{error:?}"))?,
        )
        .map_err(|error| error.to_string())
}

pub(super) fn formatted(host: &Host, root: &Path) -> Result<PreparedPublication<Example>, String> {
    let executable = host
        .rustc
        .parent()
        .ok_or("no toolchain directory")?
        .join(format!("rustfmt{}", std::env::consts::EXE_SUFFIX));
    let formatter = Formatter::qualified(
        &tool(host, &executable, root, bounds()?)?,
        configuration(root)?,
    )
    .map_err(|error| error.to_string())?;
    let publication = publication()?;
    let outputs = publication
        .files()
        .map(|file| {
            finish(
                formatter
                    .format(&file, scratch(root)?)
                    .map_err(|error| error.to_string())?,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    PreparedPublication::formatted(publication, outputs, 65536).map_err(|error| error.to_string())
}

pub(super) fn request(
    host: &Host,
    executable: &Path,
    staged: &Path,
    output: &Path,
    limits: ProcessLimits,
) -> Result<CompilerRequest, String> {
    CompilerRequest::rustc(
        &tool(host, executable, staged, limits)?,
        locus()?,
        output.to_path_buf(),
        &host.triple,
    )
    .and_then(|request| request.with_dependencies(output.with_extension("d"), DEPENDENCIES))
    .map_err(|error| error.to_string())
}

pub(super) fn compiled(run: StagingRun<Example>) -> Result<CompiledPublication<Example>, String> {
    match run {
        StagingRun::Compiled(output) => Ok(*output),
        StagingRun::Refused(output) => Err(format!(
            "{:?}: {}",
            output.reason(),
            String::from_utf8_lossy(output.compiler().process().stderr().bytes())
        )),
        StagingRun::Pending(pending) => {
            drop(pending.finish(Duration::from_secs(5)));
            Err("unexpected pending staged compilation".to_owned())
        }
    }
}

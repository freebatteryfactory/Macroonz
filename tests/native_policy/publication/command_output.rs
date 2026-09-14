use super::command_fixture::{generate, preparation};
use super::configure::publication;
use super::destination_fixture::snapshot;
use super::stage_fixture::DEPENDENCIES;
use crate::compiler::configure::{bounds, host, locus, root, tool};
use macroonz::native_compiler::{CargoFixture, CargoTarget, CompilerRequest};
use macroonz::native_process::ProcessTool;
use macroonz::native_publication::{BakeCause, BakeCommand, DestinationError, bake};
use std::path::{Path, PathBuf};

#[test]
fn compiler_outputs_cannot_write_into_the_destination_before_installation_preflight()
-> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let output = root()?;
    let aliases = root()?;
    let sentinel = output.join("authored");
    std::fs::write(&sentinel, b"authored bytes").map_err(|error| error.to_string())?;
    let directory_alias = aliases.join("destination");
    super::stage_refusal::link(&output, &directory_alias)?;
    let file_alias = aliases.join("artifact");
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&sentinel, &file_alias)
        .map_err(|error| error.to_string())?;
    #[cfg(unix)]
    std::os::unix::fs::symlink(&sentinel, &file_alias).map_err(|error| error.to_string())?;
    let before = snapshot(&output)?;
    let missing = source.join("nonexistent-compiler");
    let selected = tool(&host, &missing, &source, bounds()?)?;
    for case in [
        ("rustc", sentinel.clone(), source.join("observed.d")),
        ("rustc", file_alias.clone(), source.join("observed.d")),
        ("rustc", source.join("artifact"), sentinel.clone()),
        (
            "rustc",
            source.join("artifact"),
            directory_alias.join("new.d"),
        ),
        (
            "cargo",
            output.join("new/target"),
            source.join("observed.d"),
        ),
        (
            "cargo",
            directory_alias.join("new/target"),
            source.join("observed.d"),
        ),
        (
            "cargo",
            output
                .parent()
                .ok_or("destination parent absent")?
                .to_path_buf(),
            source.join("observed.d"),
        ),
    ] {
        let mut command = generate(
            preparation(&source, &work, &host)?,
            &source,
            &output,
            &host,
            bounds()?,
        )?;
        let BakeCommand::Generate(selection) = &mut command else {
            return Err("generation selection absent".to_owned());
        };
        selection.compiler = compiler(case, &selected, &source, &host.triple)?;
        let failure = bake(command, publication)
            .err()
            .ok_or("compiler overlap accepted")?;
        assert!(
            matches!(
                failure.cause(),
                BakeCause::Destination {
                    error: DestinationError::Conflict(_),
                    compiled: None,
                }
            ),
            "{failure}"
        );
        assert_eq!(snapshot(&output)?, before);
        assert!(snapshot(&work)?.is_empty());
    }
    std::fs::remove_file(file_alias).map_err(|error| error.to_string())?;
    super::stage_refusal::unlink(&directory_alias)
}

fn compiler(
    (kind, primary, dependency): (&str, PathBuf, PathBuf),
    selected: &ProcessTool,
    source: &Path,
    triple: &str,
) -> Result<CompilerRequest, String> {
    let request = if kind == "rustc" {
        CompilerRequest::rustc(selected, locus()?, primary, triple)
    } else {
        CompilerRequest::cargo(
            selected,
            CargoFixture::informed(
                source.join("Cargo.toml"),
                primary,
                "fixture".to_owned(),
                CargoTarget::Library,
                triple.to_owned(),
            )
            .map_err(|error| error.to_string())?,
            locus()?,
        )
    };
    request
        .and_then(|request| request.with_dependencies(dependency, DEPENDENCIES))
        .map_err(|error| error.to_string())
}

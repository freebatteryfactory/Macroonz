//! The fixed cross-platform launcher for the hosted mutation campaign.
//!
//! cargo-mutants compiles changed code in a VCS-free scratch copy. The
//! repository-law tests that code executes must still judge one clean committed
//! subject, or the unmutated baseline refuses for having no Git authority. This
//! launcher reads and validates the ordinary production repository snapshot
//! first, then supplies its absolute root, commit, and tree as one test-only
//! subject basis inherited by the scratch test processes.
//!
//! The command shape is closed here: workspace scope, concurrency, output
//! directory, and report admission cannot be changed by workflow arguments. A
//! copied VCS directory or in-place mutation would make the mutation itself look
//! like tracked dirt and counterfeit a caught result, so neither road exists.

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::mutation_report;
use crate::repository::snapshot::{
    MUTATION_SUBJECT_COMMIT, MUTATION_SUBJECT_ROOT, MUTATION_SUBJECT_TREE, RepositorySnapshot,
    cargo_binary,
};

/// The campaign's ignored output coordinate beneath the repository root.
const OUTPUT_DIRECTORY: &str = "target/mutation-run";

/// Derives the typed report adapter's input from the campaign output owner.
fn report_path() -> PathBuf {
    Path::new(OUTPUT_DIRECTORY).join("mutants.out")
}

/// Runs the one admitted mutation campaign and validates its finalized report.
pub(crate) fn run(
    repository_root: &Path,
    arguments: impl Iterator<Item = String>,
) -> Result<(), Box<dyn Error>> {
    require_no_arguments(arguments)?;
    if !repository_root.is_absolute() {
        return Err(format!(
            "mutation-campaign requires an absolute repository root; got {}",
            repository_root.display()
        )
        .into());
    }

    let subject = RepositorySnapshot::read(repository_root)?;
    let output_directory = repository_root.join(OUTPUT_DIRECTORY);
    let output_parent = output_directory.parent().ok_or_else(|| {
        format!(
            "mutation output coordinate has no parent: {}",
            output_directory.display()
        )
    })?;
    fs::create_dir_all(output_parent).map_err(|source| {
        format!(
            "cannot create mutation output parent {}: {source}",
            output_parent.display()
        )
    })?;
    let mut campaign = command(
        repository_root,
        subject.committed().commit(),
        subject.committed().tree(),
    );
    let status = campaign
        .status()
        .map_err(|source| format!("cannot start cargo-mutants campaign: {source}"))?;
    let exit_code = status.code().ok_or_else(|| {
        String::from("cargo-mutants campaign ended by a signal before it could report an outcome")
    })?;
    let report_path = report_path();
    let report_path = report_path
        .to_str()
        .ok_or_else(|| {
            format!(
                "mutation report path is not Unicode: {}",
                report_path.display()
            )
        })?
        .to_owned();
    mutation_report::run(
        repository_root,
        [String::from("run"), exit_code.to_string(), report_path].into_iter(),
    )
}

/// Refuses a second command surface beside the one fixed below.
fn require_no_arguments(mut arguments: impl Iterator<Item = String>) -> Result<(), String> {
    match arguments.next() {
        None => Ok(()),
        Some(argument) => Err(format!(
            "mutation-campaign accepts no arguments; unexpected {argument:?}"
        )),
    }
}

/// Constructs the exact cargo-mutants command and its test-only subject basis.
fn command(repository_root: &Path, commit: &str, tree: &str) -> Command {
    let mut command = Command::new(cargo_binary());
    command
        .current_dir(repository_root)
        .args([
            OsStr::new("mutants"),
            OsStr::new("--workspace"),
            OsStr::new("-j"),
            OsStr::new("2"),
            OsStr::new("--output"),
            OsStr::new(OUTPUT_DIRECTORY),
        ])
        .env(MUTATION_SUBJECT_ROOT, repository_root)
        .env(MUTATION_SUBJECT_COMMIT, commit)
        .env(MUTATION_SUBJECT_TREE, tree);
    command
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::ffi::{OsStr, OsString};
    use std::path::Path;

    use super::{OUTPUT_DIRECTORY, command, report_path, require_no_arguments};
    use crate::repository::snapshot::{
        MUTATION_SUBJECT_COMMIT, MUTATION_SUBJECT_ROOT, MUTATION_SUBJECT_TREE,
    };

    /// Positive control: the launcher owns one exact command and one complete
    /// subject triple, with no copied-VCS, in-place, filtering, or baseline-skip
    /// road hidden among its arguments.
    #[test]
    fn the_campaign_command_shape_is_closed() -> Result<(), String> {
        let root = std::env::temp_dir().join("threadpak-mutation-command-subject");
        let campaign = command(&root, "expected-commit", "expected-tree");
        assert_eq!(campaign.get_current_dir(), Some(root.as_path()));
        assert_eq!(report_path().parent(), Some(Path::new(OUTPUT_DIRECTORY)));
        assert_eq!(report_path().file_name(), Some(OsStr::new("mutants.out")));
        let arguments = campaign.get_args().map(OsString::from).collect::<Vec<_>>();
        assert_eq!(
            arguments,
            [
                OsString::from("mutants"),
                OsString::from("--workspace"),
                OsString::from("-j"),
                OsString::from("2"),
                OsString::from("--output"),
                OsString::from(OUTPUT_DIRECTORY),
            ]
        );
        let environment = campaign
            .get_envs()
            .map(|(name, value)| {
                let value = value.ok_or_else(|| {
                    format!(
                        "campaign removed environment input {}",
                        name.to_string_lossy()
                    )
                })?;
                Ok((OsString::from(name), OsString::from(value)))
            })
            .collect::<Result<BTreeMap<_, _>, String>>()?;
        assert_eq!(environment.len(), 3);
        assert_eq!(
            environment.get(OsStr::new(MUTATION_SUBJECT_ROOT)),
            Some(&root.into_os_string())
        );
        assert_eq!(
            environment.get(OsStr::new(MUTATION_SUBJECT_COMMIT)),
            Some(&OsString::from("expected-commit"))
        );
        assert_eq!(
            environment.get(OsStr::new(MUTATION_SUBJECT_TREE)),
            Some(&OsString::from("expected-tree"))
        );
        Ok(())
    }

    /// Planted reversal: workflow text cannot add a second campaign grammar.
    #[test]
    fn campaign_arguments_are_not_a_pass_through() {
        assert!(
            require_no_arguments([String::from("--baseline=skip")].into_iter())
                .is_err_and(|reason| reason.contains("accepts no arguments"))
        );
    }
}

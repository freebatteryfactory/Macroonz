//! The `cargo xtask` command shell.
//!
//! Two commands, and the second contains the first.
//!
//! `cargo xtask check` reads the repository ONCE and runs every day-zero
//! repository law over that one reading, reporting each result; any broken law
//! fails the run. Checks grow one at a time as each written rule gains something
//! to enforce — the repository never carries a rule that nothing checks.
//!
//! `cargo xtask qualify` runs the complete entry bar, and the ordered stage
//! table in [`qualification`] is the only definition of what that bar is —
//! naming its stages a second time here would be the same battery written
//! twice, drifting the first time either copy was edited alone. It is the whole
//! bar and the only spelling of it, so the road a hosted runner takes and the
//! road a working machine takes cannot differ.
//!
//! This file is the shell and nothing else. It resolves the command, builds the
//! one reading, holds the one table that names every law beside the function
//! that checks it, and runs that table in order. The laws live in [`checks`];
//! the reading they all stand on lives in [`repository`]; the ordered battery
//! `qualify` runs lives in [`qualification`], which is handed the law table's
//! runner rather than reaching back for it. Keeping the table alone here is what
//! makes the registered set readable in one screen: the array below is the
//! roster, its length is the only statement of how many laws there are, and
//! adding a law is one more line in it — so a law added without a name, or a
//! name registered twice, is visible at a glance rather than buried among the
//! checks themselves.

mod checks;
mod repository;
mod qualification;

use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::checks::coupling::check_collection_bodies_are_coupled;
use crate::checks::dependency::check_no_core_tooling_edge;
use crate::checks::hygiene::{
    check_lf_and_no_symlinks, check_no_python, check_underscore_fields_are_phantom,
};
use crate::checks::mint::check_refusal_mints_are_inside_the_plane;
use crate::checks::obligations::check_obligations_join;
use crate::checks::parity::check_agents_claude_parity;
use crate::checks::placement::{check_band_map, check_tooling_module_order};
use crate::checks::seal::check_stamped_guards_seal_their_position;
use crate::checks::supply_chain::check_dependency_gate_artifacts;
use crate::checks::toolchain::{check_lint_wall, check_toolchain_pin, check_workspace_members};
use crate::checks::vocabulary::{check_banned_vocabulary, check_no_personal_names};
use crate::repository::snapshot::{RepositorySnapshot, repo_root};
use crate::repository::types::{Check, Read};

/// The command a bare `cargo xtask` means.
const DEFAULT_COMMAND: &str = "check";

fn main() -> Result<(), Box<dyn Error>> {
    let root = repo_root()?;
    let command = match std::env::args().nth(1) {
        Some(named) => named,
        None => String::from(DEFAULT_COMMAND),
    };
    match command.as_str() {
        "check" => run_checks(&root),
        "qualify" => qualification::qualify(&root, run_checks),
        other => Err(format!("unknown xtask command: {other}").into()),
    }
}

/// Reads the repository once and runs every repository law over that reading,
/// printing one PASS or FAIL line per law.
///
/// The reading comes first and is shared, which is the whole of the typed
/// repository model: no law walks the tree, opens a file, or starts a process,
/// so two laws cannot be judging two different trees. The run opens by naming
/// what it read — how many files, and the commit those files were committed at —
/// because a verdict that cannot be attached to a tree is a verdict about
/// nothing in particular, and this campaign has already produced one false green
/// from a restore that preserved a modification time.
fn run_checks(root: &Path) -> Result<(), Box<dyn Error>> {
    let snapshot = RepositorySnapshot::read(root)?;
    println!(
        "read {} files at commit {} (committed tree {})",
        snapshot.files().count(),
        spelled(snapshot.commit()),
        spelled(snapshot.tree())
    );
    let checks: [Check; 17] = [
        ("agents-claude-parity", check_agents_claude_parity),
        ("lf-and-no-symlinks", check_lf_and_no_symlinks),
        ("no-python", check_no_python),
        ("one-toolchain-floor", check_toolchain_pin),
        ("workspace-members-match-readme", check_workspace_members),
        ("lint-wall-inherited", check_lint_wall),
        ("no-core-tooling-edge", check_no_core_tooling_edge),
        (
            "dependency-gate-artifacts-are-present-and-distinct",
            check_dependency_gate_artifacts,
        ),
        (
            "underscore-fields-are-phantom",
            check_underscore_fields_are_phantom,
        ),
        ("band-map-matches-lib", check_band_map),
        ("tooling-module-order", check_tooling_module_order),
        ("readme-obligations-join", check_obligations_join),
        (
            "collection-bodies-are-coupled",
            check_collection_bodies_are_coupled,
        ),
        (
            "refusal-mints-are-inside-the-plane",
            check_refusal_mints_are_inside_the_plane,
        ),
        (
            "stamped-guards-seal-their-position",
            check_stamped_guards_seal_their_position,
        ),
        ("no-personal-names", check_no_personal_names),
        ("banned-vocabulary", check_banned_vocabulary),
    ];
    let mut failures = Vec::new();
    for (name, check) in checks {
        match check(&snapshot) {
            Ok(()) => println!("PASS {name}"),
            Err(reason) => {
                println!("FAIL {name}: {reason}");
                failures.push(name);
            }
        }
    }
    if failures.is_empty() {
        println!("all repository laws hold");
        Ok(())
    } else {
        Err(format!("{} repository law(s) broken", failures.len()).into())
    }
}

/// How one read fact is spelled in the line a run opens with.
///
/// An unknown says it is unknown. A run that printed a blank where a commit
/// belongs would be a run claiming to have judged something it cannot name.
fn spelled<T: fmt::Display>(read: &Read<T>) -> String {
    match *read {
        Read::Known(ref fact) => fact.to_string(),
        Read::DeclaredAbsent(reason) => format!("unknown ({reason})"),
        Read::Unreadable(ref failure) => format!("unknown ({failure})"),
    }
}

//! The `cargo xtask` command shell.
//!
//! Two commands, and the second contains the first.
//!
//! `cargo xtask check` runs every day-zero repository law and reports each
//! result; any broken law fails the run. Checks grow one at a time as each
//! written rule gains something to enforce — the repository never carries a rule
//! that nothing checks.
//!
//! `cargo xtask qualify` runs the complete entry bar: formatting, the lint wall,
//! the tests, those same repository laws, the wasm build, the documentation
//! build, and a closing read of the checkout that refuses a worktree the run
//! itself dirtied. It is the whole bar and the only spelling of it, so the road a
//! hosted runner takes and the road a working machine takes cannot differ.
//!
//! This file is the shell and nothing else. It resolves the command, holds the
//! one table that names every law beside the function that checks it, and runs
//! that table in order. The laws live in [`checks`]; the reading they do lives
//! in [`repository`]; the ordered battery `qualify` runs lives in
//! [`qualification`], which is handed the law table's runner rather than
//! reaching back for it. Keeping the table alone here is what makes the
//! registered set readable in one screen: adding a law is one line beside
//! thirteen others, so a law added without a name, or a name registered twice,
//! is visible at a glance rather than buried among the checks themselves.

mod checks;
mod repository;
mod qualification;

use std::error::Error;
use std::path::Path;

use crate::checks::coupling::check_collection_bodies_are_coupled;
use crate::checks::dependency::check_no_core_tooling_edge;
use crate::checks::hygiene::{
    check_lf_and_no_symlinks, check_no_python, check_underscore_fields_are_phantom,
};
use crate::checks::obligations::check_obligations_join;
use crate::checks::parity::check_agents_claude_parity;
use crate::checks::placement::{check_band_map, check_tooling_module_order};
use crate::checks::toolchain::{check_lint_wall, check_toolchain_pin, check_workspace_members};
use crate::checks::vocabulary::{check_banned_vocabulary, check_no_personal_names};
use crate::repository::types::Check;
use crate::repository::walk::repo_root;

fn main() -> Result<(), Box<dyn Error>> {
    let root = repo_root()?;
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("check"));
    match command.as_str() {
        "check" => run_checks(&root),
        "qualify" => qualification::qualify(&root, run_checks),
        other => Err(format!("unknown xtask command: {other}").into()),
    }
}

/// Runs every repository law, printing one PASS or FAIL line per law.
fn run_checks(root: &Path) -> Result<(), Box<dyn Error>> {
    let checks: [Check; 14] = [
        ("agents-claude-parity", check_agents_claude_parity),
        ("lf-and-no-symlinks", check_lf_and_no_symlinks),
        ("no-python", check_no_python),
        ("one-toolchain-floor", check_toolchain_pin),
        ("workspace-members-match-readme", check_workspace_members),
        ("lint-wall-inherited", check_lint_wall),
        ("no-core-tooling-edge", check_no_core_tooling_edge),
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
        ("no-personal-names", check_no_personal_names),
        ("banned-vocabulary", check_banned_vocabulary),
    ];
    let mut failures = Vec::new();
    for (name, check) in checks {
        match check(root) {
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

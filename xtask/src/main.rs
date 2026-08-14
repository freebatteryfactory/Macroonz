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
use std::path::Path;

use crate::checks::alarms::check_alarm_artifacts;
use crate::checks::coupling::check_collection_bodies_are_coupled;
use crate::checks::dependency::check_no_core_tooling_edge;
use crate::checks::hygiene::{
    check_lf_and_no_symlinks, check_no_python, check_underscore_fields_are_phantom,
};
use crate::checks::obligations::check_obligations_join;
use crate::checks::parity::check_agents_claude_parity;
use crate::checks::placement::{check_band_map, check_tooling_module_order};
use crate::checks::positivity::check_inhabitant_promising_limits;
use crate::checks::seat::check_seat_modules_carry_nothing_else;
use crate::checks::supply_chain::check_dependency_gate_artifacts;
use crate::checks::toolchain::{check_lint_wall, check_toolchain_pin, check_workspace_members};
use crate::checks::vocabulary::{check_banned_vocabulary, check_no_personal_names};
use crate::repository::snapshot::{RepositorySnapshot, repo_root};
use crate::repository::types::Check;

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
/// what it read — how many files, and WHETHER those files are a committed tree —
/// because a verdict that cannot be attached to a tree is a verdict about
/// nothing in particular, and this campaign has already produced one false green
/// from a restore that preserved a modification time.
///
/// The opening line no longer names a commit it has not established. It used to:
/// the reading walked the disk and then asked git what `HEAD` was, and printed
/// the two side by side as though one were about the other, so every run on a
/// dirty checkout stated a commit-bound result it had never bound. The binding
/// is established around the read now, and where there is none the sentence says
/// so instead of naming a commit anyway.
fn run_checks(root: &Path) -> Result<(), Box<dyn Error>> {
    let snapshot = RepositorySnapshot::read(root)?;
    println!(
        "read {} files {}",
        snapshot.files().count(),
        snapshot.binding()
    );
    // EIGHTEEN, and the number is decided here rather than counted from the
    // lines below, because this array's length is the only statement of how many
    // repository laws there are and a wrong one is `E0308` rather than a law
    // that quietly stopped running. Where it came from, exactly: the two lanes
    // this merge joins branched from a roster of SEVENTEEN. One lane made a
    // defect class unrepresentable and retired the two readers that had been
    // re-implementing name resolution to hunt it — the refusal-mint law and the
    // stamped-guard law — replacing both with the one syntax-only law
    // `seat-modules-carry-nothing-else`, which is sixteen. Two legs the lanes
    // named honestly rather than shipping around land here:
    // `alarm-artifacts-are-present-and-distinct` and
    // `inhabitant-promising-limits-are-witnessed`. Sixteen and two is eighteen.
    let checks: [Check; 18] = [
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
            "alarm-artifacts-are-present-and-distinct",
            check_alarm_artifacts,
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
            "seat-modules-carry-nothing-else",
            check_seat_modules_carry_nothing_else,
        ),
        (
            "inhabitant-promising-limits-are-witnessed",
            check_inhabitant_promising_limits,
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

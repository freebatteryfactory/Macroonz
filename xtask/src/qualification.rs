//! The qualification road: one command, the complete battery, in order.
//!
//! `cargo xtask qualify` is the entry bar and the only spelling of it. The
//! stages live here rather than in a hosted runner's configuration because a
//! battery written in a workflow file exists only where that workflow runs: a
//! contributor could not rehearse the bar before pushing, and the two spellings
//! would drift the first time one of them was edited alone. A hosted runner
//! supplies the one thing a working machine cannot — a clean host nobody
//! configured, and a refusal that stands between a change and the branch — and
//! it supplies that by CALLING this road, never by restating it.
//!
//! # Why the road stops at the first red
//!
//! Four of the seven stages compile the workspace. A source that does not build
//! makes each of them print the same errors again, so a run that continued past
//! the first failure would bury the one thing to fix under three copies of its
//! consequences. Stopping names it once, on the last line of the log.
//!
//! Inside a stage the opposite rule holds: the repository laws report every
//! offence at once. They are independent of one another, so reporting them one
//! at a time would cost a round trip per law to learn what one run already
//! knows. Fail fast across stages that share a cause; report everything within
//! a stage whose findings do not.
//!
//! # Why every resolving stage is `--locked`
//!
//! `Cargo.lock` is checked in, so the exact dependency versions a build uses are
//! a decision the repository already made and a reader can see. Left to itself,
//! cargo repairs a lock file it finds stale and carries on, which qualifies a
//! dependency set nobody chose and no second machine reproduces. `--locked`
//! turns that silent repair into a refusal: the checked lock file is an INPUT to
//! qualification rather than something the run may rewrite on its way past.
//! Formatting resolves nothing and therefore carries no such flag — a flag on a
//! stage that never reads a manifest would be decoration.
//!
//! # Why the road ends by reading the checkout
//!
//! The last stage refuses a worktree that qualification itself dirtied. Every
//! stage before it READS the source: compiling, testing, and building
//! documentation must leave the tree byte-for-byte as they found it, and a stage
//! that instead generated or rewrote repository material would otherwise report
//! PASS about a tree that no longer exists. Committed bytes and qualified bytes
//! have to be the same bytes, and this is where that is checked rather than
//! assumed.
//!
//! # Why the stages shell out the way they do
//!
//! Five stages are cargo invocations, one reads the checkout through git, and
//! one is a function call. The six spawned stages are given an explicit argument
//! list and an explicit working directory, so no shell parses them, no
//! platform's quoting rules apply, and no path separator is ever spelled — the
//! same table runs on a Windows working machine and on a Linux runner. The one
//! stage that spawns nothing is the repository laws: they are already linked
//! into this binary, so spawning a second cargo to reach them would buy nothing
//! and cost a rebuild.

use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, Stdio};

/// One qualification stage: what the log calls it, and the work it is.
struct Stage {
    /// The name printed when the stage opens and when it settles.
    name: &'static str,
    /// What running the stage means.
    work: Work,
}

/// What a stage runs.
enum Work {
    /// A cargo invocation.
    Cargo {
        /// The arguments after the cargo binary, passed as a list rather than a
        /// command line so nothing between here and the child re-parses them.
        args: &'static [&'static str],
        /// Environment set for this child alone, so a flag one stage needs
        /// never leaks into another stage's build.
        env: &'static [(&'static str, &'static str)],
    },
    /// The in-process repository laws.
    Repository,
    /// A read of the checkout, spawned the same way a cargo stage is.
    Worktree,
}

/// The complete battery, in the order it runs.
///
/// The order is cheapest-refusal-first among the stages that can refuse for
/// unrelated reasons: formatting costs no compilation, the lint wall settles
/// what compiles at all, and the next four stand on a workspace already known to
/// build. The wall is deny-by-configuration in the workspace manifest, so the
/// clippy stage carries no lint flags of its own — a flag here would be a second
/// place the wall is stated. The worktree check is last for the opposite reason
/// to all of them: its subject is what the other six did, so it has nothing to
/// read until they are over.
const STAGES: [Stage; 7] = [
    Stage {
        name: "formatting",
        work: Work::Cargo {
            args: &["fmt", "--all", "--", "--check"],
            env: &[],
        },
    },
    Stage {
        name: "lint wall",
        work: Work::Cargo {
            args: &["clippy", "--locked", "--workspace", "--all-targets"],
            env: &[],
        },
    },
    Stage {
        name: "tests",
        work: Work::Cargo {
            args: &["test", "--locked", "--workspace"],
            env: &[],
        },
    },
    Stage {
        name: "repository laws",
        work: Work::Repository,
    },
    Stage {
        name: "wasm build",
        work: Work::Cargo {
            args: &[
                "build",
                "--locked",
                "--package",
                "threadpak",
                "--target",
                "wasm32-unknown-unknown",
            ],
            env: &[],
        },
    },
    Stage {
        name: "documentation",
        work: Work::Cargo {
            args: &["doc", "--locked", "--workspace", "--no-deps"],
            env: &[("RUSTDOCFLAGS", "-D warnings")],
        },
    },
    Stage {
        name: "worktree clean",
        work: Work::Worktree,
    },
];

/// Runs the complete battery, printing each stage's name, the exact work it
/// runs, and how it settled.
///
/// The repository laws arrive as an argument rather than as a call back into
/// the shell that dispatched this command. The shell owns the wiring between a
/// command name and the work it means, and this road runs a table it was
/// handed; that way neither reaches into the other and the registered set stays
/// readable in one place.
pub(crate) fn qualify(
    root: &Path,
    repository_laws: fn(&Path) -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    let total = STAGES.len();
    for (index, stage) in STAGES.iter().enumerate() {
        let position = index.saturating_add(1);
        println!(
            "==> qualify {position}/{total} {}: {}",
            stage.name,
            spelling(&stage.work)
        );
        let settled = match stage.work {
            Work::Cargo { args, env } => run_cargo(root, args, env),
            Work::Repository => repository_laws(root).map_err(|error| error.to_string()),
            Work::Worktree => run_worktree_clean(root),
        };
        match settled {
            Ok(()) => println!("<== PASS {}", stage.name),
            Err(reason) => {
                println!("<== FAIL {}: {reason}", stage.name);
                return Err(format!(
                    "qualification stopped at stage {position}/{total} ({}): {reason}",
                    stage.name
                )
                .into());
            }
        }
    }
    println!("all {total} qualification stages hold");
    Ok(())
}

/// How one stage's work is spelled in the log.
///
/// A cargo stage prints the command a reader can paste into their own shell,
/// environment first, so what a hosted run did is reproducible from the log
/// alone rather than by reading this file.
fn spelling(work: &Work) -> String {
    match *work {
        Work::Cargo { args, env } => {
            let mut line = String::new();
            for &(key, value) in env {
                line.push_str(key);
                line.push_str("=\"");
                line.push_str(value);
                line.push_str("\" ");
            }
            line.push_str("cargo");
            for &argument in args {
                line.push(' ');
                line.push_str(argument);
            }
            line
        }
        Work::Repository => String::from("in-process, linked into this binary"),
        Work::Worktree => String::from("git status --porcelain"),
    }
}

/// Runs one cargo stage to completion, its output streaming straight through to
/// this process's own streams.
///
/// Inheriting the streams rather than capturing them is what makes a hosted log
/// readable while it runs: a captured stage would print nothing for minutes and
/// then everything at once, and a run cancelled mid-stage would print nothing at
/// all.
fn run_cargo(root: &Path, args: &[&str], env: &[(&str, &str)]) -> Result<(), String> {
    let mut command = Command::new(cargo_binary());
    command.current_dir(root);
    command.args(args);
    for &(key, value) in env {
        command.env(key, value);
    }
    let status = command
        .status()
        .map_err(|error| format!("cargo could not be started: {error}"))?;
    if status.success() {
        return Ok(());
    }
    match status.code() {
        Some(code) => Err(format!("cargo exited {code}")),
        None => Err(String::from("cargo was ended by a signal")),
    }
}

/// Refuses a checkout that does not match what is committed.
///
/// `git status --porcelain` is the machine-readable spelling of the same
/// question a person asks before committing: it prints one line per path that
/// differs from `HEAD` or is untracked, and prints NOTHING when the checkout is
/// clean. Empty output is therefore the entire pass condition — no parsing, no
/// interpretation, no list of paths this stage forgives.
///
/// The dirty paths are named rather than counted, because the two failures this
/// stage catches need different repairs and only the paths tell them apart: a
/// stage that wrote into the tree is a defect in qualification, while an
/// uncommitted local edit is a run that was started too early.
///
/// stdout is captured because it is the verdict; stderr is inherited so that
/// git's own complaint about a checkout it cannot read lands in the log next to
/// the stage that asked.
fn run_worktree_clean(root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["status", "--porcelain"])
        .stderr(Stdio::inherit())
        .output()
        .map_err(|error| format!("git could not be started: {error}"))?;
    if !output.status.success() {
        return match output.status.code() {
            Some(code) => Err(format!("git exited {code}")),
            None => Err(String::from("git was ended by a signal")),
        };
    }
    let listing = String::from_utf8_lossy(&output.stdout);
    let dirty = dirty_entries(&listing);
    if dirty.is_empty() {
        return Ok(());
    }
    Err(format!(
        "the checkout does not match what is committed: {}",
        dirty.join("; ")
    ))
}

/// Every entry a porcelain listing reports, each carrying git's two-column
/// status ahead of the path so the log says both what differs and how.
///
/// Pure over the listing, which is what lets the verdict this stage turns on be
/// proven against fixture text: a law about a clean checkout that could only be
/// tested by dirtying one would be proving itself false to run.
fn dirty_entries(listing: &str) -> Vec<&str> {
    listing.lines().filter(|line| !line.is_empty()).collect()
}

/// The cargo binary a stage is spawned with.
///
/// Cargo sets `CARGO` for every process it starts, so a nested invocation
/// reaches the exact binary that started this one — the pinned toolchain's
/// cargo, not whatever a machine's search path resolves today. The fallback
/// covers the case where the xtask binary is run directly, where no pin has
/// been resolved and the search path is all there is.
fn cargo_binary() -> OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"))
}

/// Planted reversals for the closing stage's verdict.
///
/// The verdict is pure over git's listing, so every case below is a fixture
/// string. Nothing on disk is written, read, or moved: the stage that refuses a
/// dirty checkout is never proven by dirtying one.
#[cfg(test)]
mod tests {
    use super::dirty_entries;

    /// The pass condition, stated exactly: a clean checkout prints nothing, and
    /// a trailing newline is still nothing.
    #[test]
    fn an_empty_listing_is_the_whole_pass_condition() {
        assert!(dirty_entries("").is_empty());
        assert!(dirty_entries("\n").is_empty());
    }

    /// Planted reversal: a stage that generated repository material and a stage
    /// that rewrote it. Every entry survives to the caller with its status
    /// column intact, so the failure names what to look at rather than counting
    /// it — and a verdict that dropped entries would report a smaller mess than
    /// the one it found.
    #[test]
    fn every_reported_entry_reaches_the_verdict() {
        let found = dirty_entries("?? xtask/generated.rs\n M src/lib.rs\n");
        assert_eq!(
            found,
            vec!["?? xtask/generated.rs", " M src/lib.rs"],
            "{found:?}"
        );
    }

    /// The positive control: one dirty path is a failure, so a verdict that
    /// only fired on many would pass the single-file case this stage exists to
    /// catch.
    #[test]
    fn one_dirty_path_is_already_a_failure() {
        let found = dirty_entries(" M Cargo.lock\n");
        assert_eq!(found.len(), 1, "{found:?}");
    }
}

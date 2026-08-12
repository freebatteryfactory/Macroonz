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
//! Four of the six stages compile the workspace. A source that does not build
//! makes each of them print the same errors again, so a run that continued past
//! the first failure would bury the one thing to fix under three copies of its
//! consequences. Stopping names it once, on the last line of the log.
//!
//! Inside a stage the opposite rule holds: the repository laws report every
//! offence at once. Those thirteen are independent of one another, so reporting
//! them one at a time would cost thirteen round trips to learn what one run
//! already knows. Fail fast across stages that share a cause; report everything
//! within a stage whose findings do not.
//!
//! # Why the stages shell out the way they do
//!
//! Five stages are cargo invocations and one is a function call. The five are
//! spawned with an explicit argument list and an explicit working directory, so
//! no shell parses them, no platform's quoting rules apply, and no path
//! separator is ever spelled — the same table runs on a Windows working machine
//! and on a Linux runner. The sixth is called directly because the repository
//! laws are already linked into this binary; spawning a second cargo to reach
//! them would buy nothing and cost a rebuild.

use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

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
}

/// The complete battery, in the order it runs.
///
/// The order is cheapest-refusal-first among the stages that can refuse for
/// unrelated reasons: formatting costs no compilation, the lint wall settles
/// what compiles at all, and the remaining four stand on a workspace already
/// known to build. The wall is deny-by-configuration in the workspace manifest,
/// so the clippy stage carries no lint flags of its own — a flag here would be a
/// second place the wall is stated.
const STAGES: [Stage; 6] = [
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
            args: &["clippy", "--workspace", "--all-targets"],
            env: &[],
        },
    },
    Stage {
        name: "tests",
        work: Work::Cargo {
            args: &["test", "--workspace"],
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
            args: &["doc", "--workspace", "--no-deps"],
            env: &[("RUSTDOCFLAGS", "-D warnings")],
        },
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

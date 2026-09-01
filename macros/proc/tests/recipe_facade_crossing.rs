//! A package-shaped adopter reaches the root recipe through a renamed facade and invokes its evidence carrier.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static SCRATCH_ORDINAL: AtomicU32 = AtomicU32::new(0);

fn scratch_root() -> Result<PathBuf, String> {
    let parent = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    for _attempt in 0u16..1_024u16 {
        let ordinal = SCRATCH_ORDINAL.fetch_add(1, Ordering::SeqCst);
        let candidate = parent.join(format!(
            "macroonz_recipe_facade_{}_{ordinal}",
            std::process::id()
        ));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Err("no unoccupied recipe-facade scratch seat remained".to_owned())
}

fn manifest_path(path: &Path) -> Result<String, String> {
    let spelling = path
        .to_str()
        .ok_or_else(|| format!("package path is not UTF-8: {}", path.display()))?;
    let mut escaped = String::new();
    for character in spelling.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\u{0008}' => escaped.push_str("\\b"),
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            '\u{000c}' => escaped.push_str("\\f"),
            '\r' => escaped.push_str("\\r"),
            '\u{0000}'..='\u{001f}' | '\u{007f}' => {
                push_toml_unicode_escape(character, &mut escaped)?;
            }
            _ => escaped.push(character),
        }
    }
    Ok(escaped)
}

fn push_toml_unicode_escape(character: char, into: &mut String) -> Result<(), String> {
    let code = u32::from(character);
    into.push_str("\\u");
    for shift in [12_u32, 8_u32, 4_u32, 0_u32] {
        into.push(hexadecimal_digit((code >> shift) & 0x0f)?);
    }
    Ok(())
}

fn hexadecimal_digit(value: u32) -> Result<char, String> {
    match value {
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('2'),
        3 => Ok('3'),
        4 => Ok('4'),
        5 => Ok('5'),
        6 => Ok('6'),
        7 => Ok('7'),
        8 => Ok('8'),
        9 => Ok('9'),
        10 => Ok('A'),
        11 => Ok('B'),
        12 => Ok('C'),
        13 => Ok('D'),
        14 => Ok('E'),
        15 => Ok('F'),
        _ => Err(format!("{value} is not a four-bit value")),
    }
}

fn cargo(scratch: &Path, arguments: &[&str]) -> Result<Output, String> {
    Command::new("cargo")
        .arg("+1.98.0")
        .args(arguments)
        .arg("--manifest-path")
        .arg(scratch.join("Cargo.toml"))
        .current_dir(scratch)
        .env("CARGO_TARGET_DIR", scratch.join("target"))
        .output()
        .map_err(|error| error.to_string())
}

fn command_refusal(label: &str, output: &Output) -> String {
    format!(
        "{label} refused with status {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn write_specimen(
    scratch: &Path,
    facade_features: &str,
    producer: &str,
    consumer: &str,
) -> Result<(), String> {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "the proc package is not below the repository root".to_owned())?;
    let facade = manifest_path(repository)?;
    std::fs::create_dir(scratch.join("src")).map_err(|error| error.to_string())?;
    std::fs::create_dir(scratch.join("tests")).map_err(|error| error.to_string())?;
    let manifest = format!(
        r#"[package]
name = "renamed-recipe-adopter"
version = "0.0.0"
edition = "2024"
rust-version = "1.98.0"
publish = false
autobins = false
autoexamples = false
autotests = false
autobenches = false
build = false

[lib]
path = "src/lib.rs"

[[test]]
name = "recipe"
path = "tests/recipe.rs"

[dependencies]
bakery = {{ package = "macroonz", path = "{facade}", default-features = false{facade_features} }}

[lints.rust]
warnings = "deny"
unsafe_code = "forbid"

[workspace]
"#
    );
    std::fs::write(scratch.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("src/lib.rs"), producer).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("tests/recipe.rs"), consumer).map_err(|error| error.to_string())
}

#[test]
fn a_renamed_facade_bakes_and_delivers_one_recipe_without_recipe_topology_ceremony()
-> Result<(), String> {
    observed_in_scratch(observe_crossing)
}

#[test]
fn the_root_recipe_remains_available_without_the_optional_harness() -> Result<(), String> {
    observed_in_scratch(observe_without_harness)
}

#[test]
fn a_harness_projection_is_typed_unavailable_without_the_optional_harness() -> Result<(), String> {
    observed_in_scratch(observe_harness_refusal)
}

#[test]
fn the_irreducible_proc_carrier_is_unique_and_hidden_from_generated_docs() {
    const PROC_SOURCE: &str = include_str!("../src/lib.rs");
    const CARRIER: &str = "pub fn __macroonz_recipe_carrier";
    const HIDDEN_CARRIER: &str = "#[doc(hidden)]\n#[proc_macro]\npub fn __macroonz_recipe_carrier";

    assert_eq!(PROC_SOURCE.matches(CARRIER).count(), 1usize);
    assert_eq!(PROC_SOURCE.matches(HIDDEN_CARRIER).count(), 1usize);
}

fn observed_in_scratch(observe: impl FnOnce(&Path) -> Result<(), String>) -> Result<(), String> {
    let scratch = scratch_root()?;
    let observed = observe(&scratch);
    let removed = std::fs::remove_dir_all(&scratch).map_err(|error| error.to_string());
    match (observed, removed) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(refusal), Ok(())) => Err(refusal),
        (Ok(()), Err(cleanup)) => Err(format!(
            "recipe-facade qualification passed but scratch cleanup refused at {}: {cleanup}",
            scratch.display()
        )),
        (Err(refusal), Err(cleanup)) => Err(format!(
            "{refusal}\nrecipe-facade scratch cleanup also refused at {}: {cleanup}",
            scratch.display()
        )),
    }
}

fn observe_crossing(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch, ", features = [\"harness\"]", PRODUCER, CONSUMER)?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("scratch lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal("renamed recipe qualification", &tested));
    }
    let wasm = cargo(
        scratch,
        &[
            "check",
            "--lib",
            "--locked",
            "--offline",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    if !wasm.status.success() {
        return Err(command_refusal("renamed recipe Wasm posture", &wasm));
    }
    Ok(())
}

fn observe_without_harness(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch, "", NO_HARNESS_PRODUCER, NO_HARNESS_CONSUMER)?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("no-harness lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal("no-harness recipe qualification", &tested));
    }
    Ok(())
}

fn observe_harness_refusal(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch, "", HARNESS_REFUSAL_PRODUCER, EMPTY_CONSUMER)?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("harness-refusal lock generation", &locked));
    }
    let checked = cargo(scratch, &["check", "--lib", "--locked", "--offline"])?;
    if checked.status.success() {
        return Err("a harness-owned bake compiled without the facade harness feature".to_owned());
    }
    let stderr = String::from_utf8_lossy(&checked.stderr);
    if !stderr
        .contains("projection `trials` requires the facade harness feature, which is unavailable")
    {
        return Err(command_refusal(
            "harness-owned projection produced the wrong refusal",
            &checked,
        ));
    }
    Ok(())
}

const PRODUCER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

use core::sync::atomic::{AtomicUsize, Ordering};

static OPENED: AtomicUsize = AtomicUsize::new(0);

fn record_open() {
    OPENED.fetch_add(1, Ordering::Relaxed);
}

bakery::recipe! {
    /// A package-shaped adopter recipe.
    pub mod door {
        /// The caller-owned state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The closed state.
            Closed,
            /// The open state.
            Open,
        }

        /// The caller-owned event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// The request to open.
            OpenDoor,
        }

        bake! {
            vocabularies(State, Event);
            transitions {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                compile_contract;
                property;
            };
            evidence {
                trials {
                    support = recipe_trials_support,
                    module = recipe_trials,
                    table = named("recipe", "trial-table"),
                    suite checks = named("recipe", "unit") {
                        transition_answers {
                            claim = named("recipe", "transition-answers"),
                            subject = named("recipe", "dispatch"),
                            check = named("recipe", "exact"),
                            population = named("recipe", "declared-rows"),
                        },
                    },
                };
                mutation(states) {
                    module = recipe_mutations,
                    refusal = RecipeMutationRefusal,
                    support = recipe_mutation_support,
                    family = named("recipe", "refusals"),
                    point = named("recipe", "state-order"),
                    fact = named("recipe", "state-order"),
                    map named("recipe", "state-order") = named("recipe", "order-held"),
                    permit named("recipe", "order-held") = ["declared-order-permutation"],
                };
                benchmarks {
                    support = recipe_bench_support,
                    table_function = recipe_bench_table,
                    table = named("recipe", "bench-table"),
                    reporter = recipe_bench_reporter,
                    dispatch_pace {
                        workload = named("recipe", "dispatch"),
                        preflight = named("recipe", "dispatch-correct"),
                        planted_worse = named("recipe", "dispatch-worse"),
                        complexity = named("recipe", "linear"),
                        axis = [2, 4, 8],
                        samples = 16,
                        warmups = 4,
                        ratio_numerator = 3,
                        ratio_denominator = 1,
                        observe = [named("recipe", "rows-touched")],
                    },
                };
                network {
                    harness = bakery::harness,
                    module = recipe_network,
                    namespace = "recipe",
                    nodes = [client, server],
                    link forward = client to server,
                    schedule quiet = [],
                };
                concurrency {
                    harness = bakery::harness,
                    module = recipe_concurrency,
                    namespace = "recipe",
                    transitions_hold {
                        population = "transition-orders",
                        interleavings = 16,
                        samples = 32,
                        seed = 11,
                    },
                };
            };
            support(door_recipe_support);
        }
    }
}

/// Reads how many admitted transitions invoked their caller-owned effect.
pub fn opened() -> usize {
    OPENED.load(Ordering::Relaxed)
}
"#;

const CONSUMER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

use renamed_recipe_adopter::{door_recipe_support, recipe_mutation_support};

door_recipe_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
}

recipe_mutation_support! {
    declaring: renamed_recipe_adopter,
    harness: bakery::harness,
}

#[test]
fn the_generated_recipe_and_independent_carrier_are_callable() {
    assert_eq!(
        renamed_recipe_adopter::door::baked::apply(
            renamed_recipe_adopter::door::State::Closed,
            renamed_recipe_adopter::door::Event::OpenDoor,
        ),
        Ok(renamed_recipe_adopter::door::State::Open)
    );
    assert!(renamed_recipe_adopter::opened() > 0);
    assert_eq!(recipe_mutations::production(&()), ["Closed", "Open"]);
    assert_eq!(
        recipe_mutations::candidate_orders(),
        [["Open", "Closed"]]
    );
    assert!(recipe_mutations::lowering().is_ok());
    assert_ne!(recipe_mutations::production(&()), ["Open", "Closed"]);
}
"#;

const NO_HARNESS_PRODUCER: &str = r"#![forbid(unsafe_code)]
#![deny(warnings)]

fn record_open() {}

bakery::recipe! {
    /// A no-harness facade recipe.
    pub mod door {
        /// The caller-owned state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The closed state.
            Closed,
            /// The open state.
            Open,
        }

        /// The caller-owned event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// The request to open.
            OpenDoor,
        }

        bake! {
            vocabularies(State, Event);
            transitions {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
            };
            evidence {
                trials unavailable;
                mutation unavailable;
                benchmarks unavailable;
                network unavailable;
                concurrency unavailable;
            };
        }
    }
}
";

const NO_HARNESS_CONSUMER: &str = r"#![forbid(unsafe_code)]
#![deny(warnings)]

#[test]
fn the_no_harness_recipe_is_callable() {
    assert_eq!(
        renamed_recipe_adopter::door::baked::apply(
            renamed_recipe_adopter::door::State::Closed,
            renamed_recipe_adopter::door::Event::OpenDoor,
        ),
        Ok(renamed_recipe_adopter::door::State::Open)
    );
}
";

const HARNESS_REFUSAL_PRODUCER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

fn record_open() {}

bakery::recipe! {
    pub mod door {
        pub enum State {
            Closed,
            Open,
        }

        pub enum Event {
            OpenDoor,
        }

        bake! {
            vocabularies(State, Event);
            transitions {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                dispatch(apply);
            };
            evidence {
                trials {
                    support = recipe_trials_support,
                    module = recipe_trials,
                    table = named("recipe", "trial-table"),
                    suite checks = named("recipe", "unit") {
                        transition_answers {
                            claim = named("recipe", "transition-answers"),
                            subject = named("recipe", "dispatch"),
                            check = named("recipe", "exact"),
                            population = named("recipe", "declared-rows"),
                        },
                    },
                };
            };
        }
    }
}
"#;

const EMPTY_CONSUMER: &str = "#![forbid(unsafe_code)]\n#![deny(warnings)]\n";

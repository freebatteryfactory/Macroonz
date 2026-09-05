//! Disposable package-shaped scratch custody and Rust 1.98 Cargo invocation shared by the proc integration lanes.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static SCRATCH_ORDINAL: AtomicU32 = AtomicU32::new(0);

/// The repository root above the proc package, where the facade and sibling packages live.
pub(crate) fn repository_root() -> Result<&'static Path, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "the proc package is not below the repository root".to_owned())
}

/// Atomically claim one empty scratch root for this lane beneath Cargo's target-owned temporary directory.
fn scratch_root(lane: &str) -> Result<PathBuf, String> {
    let parent = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    for _attempt in 0u16..1_024u16 {
        let ordinal = SCRATCH_ORDINAL.fetch_add(1, Ordering::SeqCst);
        let candidate = parent.join(format!("macroonz_{lane}_{}_{ordinal}", std::process::id()));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Err(format!("no unoccupied {lane} scratch seat remained"))
}

/// Observe inside one exclusively owned scratch root and remove it whether the observation passes or refuses.
pub(crate) fn observed_in_scratch_for(
    lane: &str,
    observe: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<(), String> {
    let scratch = scratch_root(lane)?;
    let observed = observe(&scratch);
    let removed = std::fs::remove_dir_all(&scratch).map_err(|error| error.to_string());
    match (observed, removed) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(refusal), Ok(())) => Err(refusal),
        (Ok(()), Err(cleanup)) => Err(format!(
            "{lane} qualification passed but scratch cleanup refused at {}: {cleanup}",
            scratch.display()
        )),
        (Err(refusal), Err(cleanup)) => Err(format!(
            "{refusal}\n{lane} scratch cleanup also refused at {}: {cleanup}",
            scratch.display()
        )),
    }
}

/// One exact UTF-8 path spelling escaped as the body of a TOML basic string.
pub(crate) fn manifest_path(path: &Path) -> Result<String, String> {
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

/// Append one four-digit TOML Unicode escape for a control character.
fn push_toml_unicode_escape(character: char, into: &mut String) -> Result<(), String> {
    let code = u32::from(character);
    into.push_str("\\u");
    for shift in [12_u32, 8_u32, 4_u32, 0_u32] {
        into.push(hexadecimal_digit((code >> shift) & 0x0f)?);
    }
    Ok(())
}

/// Render one four-bit value without a fallible formatting road.
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

/// Run one Rust 1.98 Cargo subcommand against the scratch package under its own target directory.
///
/// The subcommand leads and the manifest path follows it, so trailing tool arguments after `--` stay where the subcommand expects them.
pub(crate) fn cargo(scratch: &Path, arguments: &[&str]) -> Result<Output, String> {
    cargo_with_target(scratch, &scratch.join("target"), arguments)
}

/// Reconcile a standalone fixture against the repository's existing dependency pins.
pub(crate) fn lock_from_repository(scratch: &Path) -> Result<Output, String> {
    std::fs::copy(
        repository_root()?.join("Cargo.lock"),
        scratch.join("Cargo.lock"),
    )
    .map_err(|error| error.to_string())?;
    cargo(scratch, &["update", "--workspace", "--offline"])
}

/// Run a Cargo observation whose manifest and disposable build root have separate declared custody.
pub(crate) fn cargo_with_target(
    manifest_root: &Path,
    target: &Path,
    arguments: &[&str],
) -> Result<Output, String> {
    cargo_command(manifest_root, target, arguments)?
        .output()
        .map_err(|error| error.to_string())
}

/// Construct one scratch Cargo command without inheriting another workspace's Nextest profile.
pub(crate) fn cargo_command(
    manifest_root: &Path,
    target: &Path,
    arguments: &[&str],
) -> Result<Command, String> {
    let (subcommand, rest) = arguments
        .split_first()
        .ok_or_else(|| "a Cargo observation requires one subcommand".to_owned())?;
    let mut command = Command::new("cargo");
    command.arg("+1.98.1").arg(subcommand);
    let remaining = if *subcommand == "nextest" {
        let (verb, remaining) = rest
            .split_first()
            .ok_or_else(|| "a Nextest observation requires its subcommand".to_owned())?;
        command.arg(verb);
        remaining
    } else {
        rest
    };
    command
        .arg("--manifest-path")
        .arg(manifest_root.join("Cargo.toml"))
        .args(remaining)
        .current_dir(manifest_root)
        .env("CARGO_TARGET_DIR", target)
        .env_remove("NEXTEST_PROFILE");
    Ok(command)
}

/// Render one unsuccessful subprocess as an actionable refusal.
pub(crate) fn command_refusal(label: &str, output: &Output) -> String {
    format!(
        "{label} refused with status {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

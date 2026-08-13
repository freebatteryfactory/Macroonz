//! Reading a home README.
//!
//! A home README is markdown prose plus fenced yaml blocks and obligation rows
//! that tooling parses. These readers turn those blocks and rows into values and
//! stop there — the joins that decide whether the values agree with the tree
//! live in `crate::checks::obligations` and `crate::checks::toolchain`.

use std::fs;
use std::path::{Path, PathBuf};

/// The lines inside the README's fenced yaml block.
pub(crate) fn readme_yaml_block(root: &Path) -> Result<Vec<String>, String> {
    let readme =
        fs::read_to_string(root.join("README.md")).map_err(|e| format!("README.md: {e}"))?;
    let mut lines = Vec::new();
    let mut inside = false;
    for line in readme.lines() {
        if inside {
            if line.trim() == "```" {
                return Ok(lines);
            }
            lines.push(line.to_string());
        } else if line.trim() == "```yaml" {
            inside = true;
        }
    }
    Err(String::from("README.md has no fenced yaml block"))
}

/// Every home README the join reads: the root one, and one per numbered band.
pub(crate) fn home_readmes(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut readmes = vec![root.join("README.md")];
    let src = root.join("src");
    let entries = fs::read_dir(&src).map_err(|e| format!("{}: {e}", src.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", src.display()))?;
        let candidate = entry.path().join("README.md");
        if candidate.is_file() {
            readmes.push(candidate);
        }
    }
    Ok(readmes)
}

/// Every green law one README claims, as `(module, law, declaring README)`.
pub(crate) fn claimed_green_laws(
    readmes: &[PathBuf],
) -> Result<Vec<(String, String, PathBuf)>, String> {
    let mut claimed = Vec::new();
    for readme in readmes {
        let text = fs::read_to_string(readme).map_err(|e| format!("{}: {e}", readme.display()))?;
        for line in text.lines() {
            let Some(rest) = line.trim().strip_prefix("green: laws.rs ") else {
                continue;
            };
            let target: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
                .collect();
            let Some((module, law)) = target.split_once("::") else {
                return Err(format!(
                    "{}: green target `{target}` is not module::law",
                    readme.display()
                ));
            };
            claimed.push((module.to_string(), law.to_string(), readme.clone()));
        }
    }
    Ok(claimed)
}

/// The value of every `green:` obligation row in one README that states a
/// ROUTE — a path to a file — rather than a law or a sentence, in file order.
///
/// A green row is written one of a few ways. `laws.rs module::law` names a
/// compile-time law and is joined against `laws.rs` itself; `none — …`,
/// `owed — …`, and `structural (…)` are sentences stating that no executable
/// green exists and why. Anything whose first word ends in `.rs` is a file the
/// row says holds the positive control, and this reader is what hands those to
/// the join so the file can be required to exist. `laws.rs` is excluded because
/// the other leg already reads it, and reading it here as well would answer one
/// claim twice.
pub(crate) fn claimed_green_routes(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("green: "))
        .filter_map(|value| value.split_whitespace().next())
        .filter(|named| is_rust_route(named) && *named != "laws.rs")
        .map(String::from)
        .collect()
}

/// Whether one green row's first word is a path to a Rust file.
///
/// Read through `Path` rather than off the end of the string: a row states a
/// repository-relative path with forward slashes, and asking the path type for
/// its extension is the reading that stays right on either platform.
fn is_rust_route(named: &str) -> bool {
    Path::new(named)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rs"))
}

/// The value of every `red:` obligation row in one README, in file order.
///
/// The prefix is matched on the TRIMMED line, so a word merely ending in `red`
/// followed by a colon — `unnumbered:`, `authored:`, `Shred:` — is never a row.
pub(crate) fn red_twin_rows(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("red: "))
        .map(|value| value.trim().to_string())
        .collect()
}

/// The value of every `tooling-red:` obligation row in one README, in file
/// order.
///
/// Read exactly like a core `red:` row and counted on its own ledger. An
/// `owed-to-…` row is a lawful debt; any other row NAMES a reversal that must
/// resolve to a real testpak test or compile-fail fixture, and the check refuses
/// it if it does not.
pub(crate) fn tooling_red_rows(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("tooling-red: "))
        .map(|value| value.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{claimed_green_routes, red_twin_rows};

    /// A row is read off the trimmed line, so an ordinary word ending in `red`
    /// followed by a colon is never mistaken for one.
    #[test]
    fn only_a_red_row_is_a_red_row() {
        let text = "unnumbered: first-class, not a row\n\
                    Shred: the four progress facts, not a row\n\
                    ## The connectives (authored: a heading, not a row)\n\
                    \x20   red: owed-to-testpak\n";
        assert_eq!(red_twin_rows(text), vec![String::from("owed-to-testpak")]);
    }

    /// A green ROUTE is read apart from every other spelling a green row takes.
    ///
    /// The positive control for the reader the green leg stands on: a reader
    /// that also returned the `laws.rs` rows would make the join answer one
    /// claim twice, and one that swallowed the prose forms would demand a file
    /// from a row whose whole content is that no file exists.
    #[test]
    fn only_a_path_shaped_green_row_is_a_route() {
        let text = "    green: laws.rs root::a_law_that_exists\n\
                    \x20   green: none — the type's nonexistence is the law\n\
                    \x20   green: owed — executable when the roster lands\n\
                    \x20   green: structural (a phantom makes the handle !Send)\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.rs\n";
        assert_eq!(
            claimed_green_routes(text),
            vec![String::from("testpak/tests/stamp_row_ceiling.rs")]
        );
    }
}

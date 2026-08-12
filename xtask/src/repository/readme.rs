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
    use super::red_twin_rows;

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
}

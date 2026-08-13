//! Reading a home README.
//!
//! A home README is markdown prose plus fenced yaml blocks and obligation rows
//! that tooling parses. These readers turn those blocks and rows into values and
//! stop there — the joins that decide whether the values agree with the tree
//! live in `crate::checks::obligations` and `crate::checks::toolchain`.

use std::fs;
use std::path::{Path, PathBuf};

use crate::repository::types::GreenRow;

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

/// The spelling a green row opens with when its positive control is a
/// compile-time seat.
const COMPILE_TIME_SEAT: &str = "laws.rs";

/// The separator a `none` or `owed` disposition states its account after, as
/// every such row in this repository is written.
const DISPOSITION_DASH: char = '—';

/// The opener a `structural` disposition states its account inside.
const DISPOSITION_PAREN: char = '(';

/// Every `green:` obligation row in one README, classified, in file order.
///
/// The prefix is matched WITHOUT its trailing space and the value is trimmed
/// after, so a row whose value was emptied is read as a row with an empty value
/// rather than vanishing from the population.
pub(crate) fn classify_green_rows(readme_text: &str) -> Vec<GreenRow> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("green:"))
        .map(|value| classify_green_row(value.trim()))
        .collect()
}

/// One green row's value, classified. Nothing is dropped: a value no lawful
/// spelling reads comes back as [`GreenRow::Unreadable`] carrying itself.
fn classify_green_row(value: &str) -> GreenRow {
    let Some(first) = value.split_whitespace().next() else {
        return GreenRow::Unreadable(value.to_string());
    };
    let account = value.strip_prefix(first).unwrap_or_default().trim();
    let read = if first == COMPILE_TIME_SEAT {
        names_a_target(account).then_some(GreenRow::CompileTimeSeat)
    } else if first == "none" || first == "owed" {
        accounts_after(account, DISPOSITION_DASH).then_some(GreenRow::Disposition)
    } else if first == "structural" {
        accounts_after(account, DISPOSITION_PAREN).then_some(GreenRow::Disposition)
    } else if is_rust_route(first) {
        Some(GreenRow::Route(first.to_string()))
    } else {
        None
    };
    read.unwrap_or_else(|| GreenRow::Unreadable(value.to_string()))
}

/// Whether what follows `laws.rs` on a row names a `module::name` target.
///
/// The target itself is resolved by [`claimed_green_laws`] against `laws.rs`.
/// All that is read here is whether the row named one at all, because a row that
/// names none is answered by neither leg.
fn names_a_target(account: &str) -> bool {
    account
        .split_whitespace()
        .next()
        .is_some_and(|target| target.contains("::"))
}

/// Whether a disposition opens its account with `opener` and states something
/// after it.
///
/// Something means a word: an opener followed by nothing, by its own closing
/// bracket, or by punctuation states the absence and accounts for none of it,
/// which is the half of the form that carries the whole meaning.
fn accounts_after(account: &str, opener: char) -> bool {
    account
        .strip_prefix(opener)
        .is_some_and(|why| why.chars().any(char::is_alphanumeric))
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
/// It is matched WITHOUT its trailing space for the reason the green side is: a
/// row whose value was emptied is still a row, and a reader that stopped seeing
/// it would quietly shrink the denominator this repository publishes.
pub(crate) fn red_twin_rows(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("red:"))
        .map(|value| value.trim().to_string())
        .collect()
}

/// The value of every `tooling-red:` obligation row in one README, in file
/// order.
///
/// Read exactly like a core `red:` row — same prefix discipline, same emptied
/// row still read — and counted on its own ledger. An `owed-to-…` row is a
/// lawful debt; any other row NAMES a reversal that must resolve to a real
/// testpak test or compile-fail fixture, and the check refuses it if it does
/// not.
pub(crate) fn tooling_red_rows(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("tooling-red:"))
        .map(|value| value.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{classify_green_rows, red_twin_rows, tooling_red_rows};
    use crate::repository::types::GreenRow;

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

    /// Planted reversal: a `red:` and a `tooling-red:` row whose value was
    /// emptied. Read on the trailing space, neither line was a row at all, and
    /// the obligation left the denominator without anything refusing.
    ///
    /// The red side's twin of the green side's dropped route, and the more
    /// expensive of the two: the core and tooling red counts are PUBLISHED on
    /// every run, so a row that stops being read shrinks a number the campaign
    /// reports rather than merely going unchecked.
    #[test]
    fn an_emptied_red_row_is_still_a_row() {
        let text = "    red:\n\
                    \x20   tooling-red:\n\
                    \x20   red: owed-to-testpak\n";
        assert_eq!(
            red_twin_rows(text),
            vec![String::new(), String::from("owed-to-testpak")]
        );
        assert_eq!(tooling_red_rows(text), vec![String::new()]);
    }

    /// The positive control: every spelling this repository actually writes is
    /// read as the spelling it is, and the route population is exactly the rows
    /// naming a file.
    ///
    /// A reader that also returned the `laws.rs` rows would make the join answer
    /// one claim twice, and one that swallowed the prose dispositions would
    /// demand a file from a row whose whole content is that no file exists.
    #[test]
    fn every_lawful_green_spelling_is_read_as_itself() {
        let text = "    green: laws.rs root::a_seat_that_exists\n\
                    \x20   green: none — the type's nonexistence is what refuses\n\
                    \x20   green: owed — executable when the roster lands\n\
                    \x20   green: structural (a phantom makes the handle !Send)\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.rs\n";
        assert_eq!(
            classify_green_rows(text),
            vec![
                GreenRow::CompileTimeSeat,
                GreenRow::Disposition,
                GreenRow::Disposition,
                GreenRow::Disposition,
                GreenRow::Route(String::from("testpak/tests/stamp_row_ceiling.rs")),
            ]
        );
    }

    /// Planted reversal: five green rows no lawful spelling reads. Under a
    /// reader that FILTERED to the path-shaped rows, every one of them was
    /// dropped — and a dropped row is an obligation that qualifies while the
    /// positive control it names is never looked for.
    #[test]
    fn a_green_row_no_spelling_reads_is_named_not_dropped() {
        let text = "    green: testpak/tests/stamp_row_ceiling.r\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.md\n\
                    \x20   green: structural\n\
                    \x20   green:\n\
                    \x20   green: laws.rs\n";
        assert_eq!(
            classify_green_rows(text),
            vec![
                GreenRow::Unreadable(String::from("testpak/tests/stamp_row_ceiling.r")),
                GreenRow::Unreadable(String::from("testpak/tests/stamp_row_ceiling.md")),
                GreenRow::Unreadable(String::from("structural")),
                GreenRow::Unreadable(String::new()),
                GreenRow::Unreadable(String::from("laws.rs")),
            ]
        );
    }

    /// Planted reversal: a disposition that states the absence and withholds
    /// the account of it, in all three of its words.
    ///
    /// The word alone is the half of the form that says nothing. `none` with no
    /// reason is indistinguishable from a row somebody gave up on, and the
    /// separator is what this repository's rows are actually written with.
    #[test]
    fn a_disposition_without_its_account_is_unreadable() {
        let text = "    green: none\n\
                    \x20   green: owed\n\
                    \x20   green: none - a hyphen is not the declared separator\n\
                    \x20   green: structural ()\n\
                    \x20   green: owed —\n";
        let read = classify_green_rows(text);
        assert!(
            read.iter()
                .all(|row| matches!(*row, GreenRow::Unreadable(_))),
            "{read:?}"
        );
        assert_eq!(read.len(), 5, "{read:?}");
    }
}

//! Source-roster collection and exact file matching.
//!
//! A roster is collected by file, refusing a repeated file, and matched against the files a reading or a manifest expects.
//! Each caller supplies its own refusals for duplicate, missing and unexpected files.

use std::collections::{BTreeMap, BTreeSet};

/// Collect supplied source revisions by file.
///
/// # Errors
///
/// Refuses the first repeated file, through the caller's refusal.
pub(super) fn collected<Value, Refusal>(
    supplied: impl IntoIterator<Item = Value>,
    file_of: fn(&Value) -> &str,
    duplicate: fn(String) -> Refusal,
) -> Result<BTreeMap<String, Value>, Refusal> {
    let mut roster = BTreeMap::new();
    for source in supplied {
        let file = file_of(&source).to_owned();
        if roster.insert(file.clone(), source).is_some() {
            return Err(duplicate(file));
        }
    }
    Ok(roster)
}

/// Match a collected roster against the files that are expected of it.
///
/// # Errors
///
/// Refuses the first expected file the roster lacks in file order, then the first roster file nothing expects in file order, each through the caller's refusal.
pub(super) fn matched<Value, Refusal>(
    roster: &BTreeMap<String, Value>,
    expected: &BTreeSet<&str>,
    missing: fn(String) -> Refusal,
    unexpected: fn(String) -> Refusal,
) -> Result<(), Refusal> {
    for file in expected.iter().copied() {
        if !roster.contains_key(file) {
            return Err(missing(file.to_owned()));
        }
    }
    for file in roster.keys() {
        if !expected.contains(file.as_str()) {
            return Err(unexpected(file.to_owned()));
        }
    }
    Ok(())
}

//! The one source-roster join both custody roads stand on.
//!
//! A roster of source revisions is collected by file, refusing a repeated file, and then matched against the files a reading or a manifest expects, refusing an expected file the roster lacks and a roster file nothing expects.
//! Each road supplies its own refusal for each of the three disagreements, so the two public refusal vocabularies stay distinct while the walk is written once.

use super::types::MutationSourceRevision;
use std::collections::{BTreeMap, BTreeSet};

/// Collect supplied source revisions by file.
///
/// # Errors
///
/// Refuses the first repeated file, through the caller's refusal.
pub(super) fn collected<Refusal>(
    supplied: Vec<MutationSourceRevision>,
    duplicate: fn(String) -> Refusal,
) -> Result<BTreeMap<String, MutationSourceRevision>, Refusal> {
    let mut roster = BTreeMap::new();
    for source in supplied {
        let file = source.file().to_owned();
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
pub(super) fn matched<Refusal>(
    roster: &BTreeMap<String, MutationSourceRevision>,
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

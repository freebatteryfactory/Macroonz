//! Reading a Cargo manifest.
//!
//! Cargo admits several spellings of the same declaration, and a reader that
//! knew only one of them would let a renamed, test-only, platform-conditional,
//! or dotted entry through unread. These readers report what a manifest
//! DECLARES and nothing more; whether a declaration is lawful is decided in
//! `crate::checks`.
//!
//! # A TOML key is a path, not a name
//!
//! That single fact is what the dependency reader below is built on, and
//! ignoring it is what let a prohibited edge hide. `serde = "1"` under
//! `[dependencies]`, `serde.version = "1"` under the same header,
//! `[dependencies.serde]` with its fields beneath it, and
//! `dependencies.serde.version = "1"` written before any header are four
//! spellings of ONE declaration; Cargo resolves all four to the same key path
//! and so does this reader. A reader that instead cut a line at its first `=`
//! saw the key `threadpak-macros.workspace` where Cargo saw the package
//! `threadpak-macros`, and a name matching no package matched no law either.
//!
//! # The line is the unit, and that is two ceilings, each with a direction
//!
//! Every reader here is line-oriented, which is exact for the manifests this
//! repository commits and for every spelling named above, and which cannot see
//! two constructs.
//!
//! **A multi-line basic string whose body reads like a manifest.** Its lines
//! are read as the header and entries they resemble, so a table quoted inside
//! a `description` is read as a table. That answer is wrong in the direction
//! this law can afford — a lawful manifest is REFUSED, never a prohibited one
//! passed — and the reversal that would run the other way does not survive
//! cargo. MEASURED on cargo 1.97.1: a decoy entry in a dependency table whose
//! value is a multi-line string, which is the one shape that could close a
//! table early and hide the edge beneath it, fails with `failed to parse the
//! version requirement`, so it never reaches a build.
//!
//! **A dependency table written as an INLINE table**, whose entries live
//! inside one line's value rather than on lines of their own. This one could
//! hide an edge, so it is not left to be missed: [`dependency_declarations`]
//! reports it by name in [`ManifestDependencies::unread`] and the topology law
//! refuses a manifest that carries one. Reading it properly is the typed
//! repository model's migration, and a second parser seated here would be the
//! duplicate authority this repository is eliminating.

/// Every Cargo dependency-edge kind, each of which the topology law covers.
const DEPENDENCY_TABLE_KINDS: [&str; 3] =
    ["dependencies", "dev-dependencies", "build-dependencies"];

/// The table a platform-conditional dependency table hangs beneath.
const TARGET_TABLE: &str = "target";

/// Extracts the double-quoted value of a `key = "value"` line.
pub(crate) fn quoted_value(text: &str, key: &str) -> Result<String, String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(key)
            && let Some(rest) = rest.trim_start().strip_prefix('=')
        {
            return Ok(rest.trim().trim_matches('"').to_string());
        }
    }
    Err(format!("no `{key}` line found"))
}

/// Extracts the items of a `key = ["a", "b"]` bracket list.
pub(crate) fn bracket_list(text: &str, key: &str) -> Result<Vec<String>, String> {
    let start = text
        .find(&format!("{key} = ["))
        .ok_or_else(|| format!("no `{key}` list found"))?;
    let rest = text.get(start..).ok_or_else(|| String::from("bad slice"))?;
    let open = rest.find('[').ok_or_else(|| String::from("no bracket"))?;
    let close = rest
        .find(']')
        .ok_or_else(|| format!("unterminated `{key}` list"))?;
    let inner = rest
        .get(open.saturating_add(1)..close)
        .ok_or_else(|| String::from("bad slice"))?;
    Ok(inner
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .filter(|item| !item.is_empty())
        .collect())
}

/// One dependency entry, as `(edge kind, entry key, declared package, declared
/// path)`.
pub(crate) type DependencyEntry = (&'static str, String, Option<String>, Option<String>);

/// What one manifest declares about its dependencies: the entries a reader
/// resolved, and the tables it could not enter.
///
/// The second field exists because a reader that returned only what it managed
/// to read would answer "no prohibited edge" and "no reading happened" with the
/// same empty list. A caller gets both facts or neither.
pub(crate) struct ManifestDependencies {
    /// Every dependency entry the manifest declares, one per entry rather than
    /// one per line: a dotted entry spelled across several lines is one entry,
    /// which is what Cargo resolves it to.
    pub(crate) entries: Vec<DependencyEntry>,
    /// Every dependency table the manifest writes as an inline table, spelled
    /// as the key path it sits at. Its entries live inside a value this
    /// line-oriented reader does not enter, so they are reported UNREAD rather
    /// than reported absent.
    pub(crate) unread: Vec<String>,
}

/// What a manifest declares about its dependencies.
///
/// Every line is resolved to the full key path it sits at — the enclosing
/// table header's path, then its own key's — and that path is a dependency
/// declaration when it reads `[target, SPEC,] KIND, NAME, FIELD…`. Ordinary,
/// renamed, dev, build, target-specific, quoted, dotted, and sub-table
/// dependencies therefore arrive by one road rather than by a spelling each,
/// and a spelling nobody thought of is read correctly if Cargo resolves it to
/// that shape.
///
/// Entries are keyed by `(kind, name)` within one table block, so the several
/// lines of a dotted entry accumulate into the one entry they declare. Blocks
/// do not merge across headers: a package named in a bare table and again under
/// a `target.'…'` prefix is two declarations and stays two entries.
pub(crate) fn dependency_declarations(manifest_text: &str) -> ManifestDependencies {
    let mut entries: Vec<DependencyEntry> = Vec::new();
    let mut unread: Vec<String> = Vec::new();
    let mut table: Vec<String> = Vec::new();
    let mut block_start = 0usize;
    for raw in manifest_text.lines() {
        let line = strip_comment(raw);
        if line.is_empty() {
            continue;
        }
        if let Some(header) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            table = key_path(header);
            block_start = entries.len();
            if let Some((kind, name, fields)) = dependency_position(&table)
                && fields.is_empty()
            {
                let _seated = seat(&mut entries, block_start, kind, name);
            }
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let mut place = table.clone();
        place.extend(key_path(key));
        let value = value.trim();
        let Some((kind, name, fields)) = dependency_position(&place) else {
            if value.starts_with('{')
                && let Some(spelling) = unenterable_table(&place)
            {
                unread.push(spelling);
            }
            continue;
        };
        let index = seat(&mut entries, block_start, kind, name);
        let sole = if fields.len() == 1 {
            fields.first().map(String::as_str)
        } else {
            None
        };
        let Some((_, _, package, path)) = entries.get_mut(index) else {
            continue;
        };
        if fields.is_empty() {
            *package = quoted_assignment(value, "package");
            *path = quoted_assignment(value, "path");
        } else if sole == Some("package") {
            *package = quoted_text(value);
        } else if sole == Some("path") {
            *path = quoted_text(value);
        }
    }
    ManifestDependencies { entries, unread }
}

/// Where in `entries` the `(kind, name)` entry of the current table block sits,
/// seating a fresh one when the block has not named it yet.
///
/// The search starts at the block's first entry, so seating is scoped to the
/// block: the dotted lines of one entry find each other, and the same package
/// named under a second header seats a second entry.
fn seat(
    entries: &mut Vec<DependencyEntry>,
    block_start: usize,
    kind: &'static str,
    name: &str,
) -> usize {
    let existing = entries
        .iter()
        .enumerate()
        .skip(block_start)
        .find(|(_, (entry_kind, entry_key, _, _))| {
            *entry_kind == kind && entry_key.as_str() == name
        })
        .map(|(index, _)| index);
    if let Some(index) = existing {
        return index;
    }
    let index = entries.len();
    entries.push((kind, name.to_string(), None, None));
    index
}

/// The dependency declaration one key path names: its edge kind, the entry it
/// names, and the fields addressed beneath that entry.
///
/// A path that names a dependency TABLE without naming an entry in it — the
/// `[dependencies]` header itself — is not a declaration and returns nothing;
/// the lines beneath it arrive here with their own key appended.
fn dependency_position(place: &[String]) -> Option<(&'static str, &str, &[String])> {
    let rest = after_target(place);
    let first = rest.first()?;
    let kind = DEPENDENCY_TABLE_KINDS
        .into_iter()
        .find(|kind| *kind == first.as_str())?;
    let name = rest.get(1)?;
    if name.is_empty() {
        return None;
    }
    Some((kind, name.as_str(), rest.get(2..).unwrap_or_default()))
}

/// The key path with a `target.'…'` prefix removed, so a platform-conditional
/// declaration is read exactly like the unconditional one it conditions.
fn after_target(place: &[String]) -> &[String] {
    if place
        .first()
        .is_some_and(|first| first.as_str() == TARGET_TABLE)
    {
        return place.get(2..).unwrap_or_default();
    }
    place
}

/// The key path this reader cannot enter, where an inline value sits at one:
/// a whole dependency table written as `KIND = { … }`, or a whole `target`
/// tree written the same way.
///
/// The entries would be inside the value, and this reader reads lines. Naming
/// the path is what lets the topology law refuse the manifest instead of
/// reporting an absence it never established.
fn unenterable_table(place: &[String]) -> Option<String> {
    let rest = after_target(place);
    let names_target = place
        .first()
        .is_some_and(|first| first.as_str() == TARGET_TABLE);
    if rest.is_empty() && names_target {
        return Some(place.join("."));
    }
    if rest.len() == 1
        && rest
            .first()
            .is_some_and(|first| DEPENDENCY_TABLE_KINDS.contains(&first.as_str()))
    {
        return Some(place.join("."));
    }
    None
}

/// The segments of one TOML key path, quotes removed and unquoted whitespace
/// dropped.
///
/// A dot separates segments only outside a quoted segment, so a target
/// predicate keeps its own dots and its own inner quotes. What is out of reach
/// by construction is an escape sequence inside a basic string: a key needing
/// one cannot name a Cargo package, whose characters are alphanumerics, `-`,
/// and `_`.
fn key_path(key: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    for character in key.chars() {
        if let Some(open) = quote {
            if character == open {
                quote = None;
            } else {
                current.push(character);
            }
        } else if character == '"' || character == '\'' {
            quote = Some(character);
        } else if character == '.' {
            segments.push(std::mem::take(&mut current));
        } else if !character.is_whitespace() {
            current.push(character);
        }
    }
    segments.push(current);
    segments
}

/// One line with its comment removed and its ends trimmed.
///
/// A `#` opens a comment only outside a string, so a path or a predicate
/// carrying one survives. Removing it here is what keeps a header with a
/// comment after it a header, and what stops the word `path` inside a comment
/// from being read as a declaration.
fn strip_comment(line: &str) -> &str {
    let mut quote: Option<char> = None;
    for (index, character) in line.char_indices() {
        match quote {
            Some(open) if character == open => quote = None,
            Some(_) => {}
            None => {
                if character == '"' || character == '\'' {
                    quote = Some(character);
                } else if character == '#' {
                    return line.get(..index).unwrap_or_default().trim();
                }
            }
        }
    }
    line.trim()
}

/// The quoted value assigned to `key` anywhere in one line of manifest text,
/// whether the line is a table entry or an inline table body. The key is
/// matched whole, so `package` never matches inside a longer key.
fn quoted_assignment(text: &str, key: &str) -> Option<String> {
    let mut from = 0usize;
    loop {
        let rest = text.get(from..)?;
        let offset = rest.find(key)?;
        let start = from.saturating_add(offset);
        let end = start.saturating_add(key.len());
        let before_is_key = text
            .get(..start)
            .and_then(|head| head.chars().next_back())
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if !before_is_key
            && let Some(tail) = text.get(end..)
            && let Some(value) = tail.trim_start().strip_prefix('=')
            && let Some(quoted) = quoted_text(value)
        {
            return Some(quoted);
        }
        from = end;
    }
}

/// The contents of the quoted string one value opens with, under either TOML
/// quote. A literal string is a spelling of the same value a basic string
/// carries, so a path or a rename written in single quotes is read.
fn quoted_text(value: &str) -> Option<String> {
    let mut characters = value.trim_start().chars();
    let open = characters.next()?;
    if open != '"' && open != '\'' {
        return None;
    }
    let rest = characters.as_str();
    let end = rest.find(open)?;
    rest.get(..end).map(str::to_string)
}

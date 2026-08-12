//! Reading a Cargo manifest.
//!
//! Cargo admits several spellings of the same declaration, and a reader that
//! knew only one of them would let a renamed, test-only, or platform-conditional
//! entry through unread. These readers report what a manifest DECLARES and
//! nothing more; whether a declaration is lawful is decided in `crate::checks`.

/// Every Cargo dependency-edge kind, each of which the topology law covers.
const DEPENDENCY_TABLE_KINDS: [&str; 3] =
    ["dependencies", "dev-dependencies", "build-dependencies"];

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

/// Every dependency entry a manifest declares, as
/// `(edge kind, entry key, declared package, declared path)`.
///
/// Ordinary, renamed, dev, build, and target-specific dependencies are all read
/// the same way, across the table spellings Cargo admits: the bare table, the
/// `[KIND.entry]` sub-table, and either under a `target.'…'` prefix.
pub(crate) fn dependency_entries(
    manifest_text: &str,
) -> Vec<(&'static str, String, Option<String>, Option<String>)> {
    let mut entries = Vec::new();
    let mut table: Option<&'static str> = None;
    let mut pending: Option<(&'static str, String, Option<String>, Option<String>)> = None;
    for raw in manifest_text.lines() {
        let line = raw.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(header) = line.strip_prefix('[').and_then(|h| h.strip_suffix(']')) {
            if let Some(entry) = pending.take() {
                entries.push(entry);
            }
            match dependency_table(header.trim()) {
                Some((kind, None)) => table = Some(kind),
                Some((kind, Some(key))) => {
                    table = None;
                    pending = Some((kind, key, None, None));
                }
                None => table = None,
            }
            continue;
        }
        if let Some((_, _, package, path)) = pending.as_mut() {
            if let Some(value) = quoted_assignment(line, "package") {
                *package = Some(value);
            }
            if let Some(value) = quoted_assignment(line, "path") {
                *path = Some(value);
            }
            continue;
        }
        let Some(kind) = table else {
            continue;
        };
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().trim_matches('"');
        if key.is_empty() {
            continue;
        }
        let value = value.trim();
        entries.push((
            kind,
            key.to_string(),
            quoted_assignment(value, "package"),
            quoted_assignment(value, "path"),
        ));
    }
    if let Some(entry) = pending {
        entries.push(entry);
    }
    entries
}

/// The dependency-edge kind a table header declares, plus the single entry the
/// header names when it is the `[dependencies.name]` sub-table form.
///
/// Recognized: the three bare kinds, each `[target.'…'.KIND]` form, and either
/// spelled with a trailing entry name.
fn dependency_table(header: &str) -> Option<(&'static str, Option<String>)> {
    for kind in DEPENDENCY_TABLE_KINDS {
        if header == kind {
            return Some((kind, None));
        }
        if let Some(prefix) = header.strip_suffix(kind)
            && let Some(prefix) = prefix.strip_suffix('.')
            && prefix.starts_with("target.")
        {
            return Some((kind, None));
        }
        if let Some(entry) = header.strip_prefix(&format!("{kind}.")) {
            return Some((kind, Some(entry.trim().trim_matches('"').to_string())));
        }
        if let Some((prefix, entry)) = header.split_once(&format!(".{kind}."))
            && prefix.starts_with("target.")
        {
            return Some((kind, Some(entry.trim().trim_matches('"').to_string())));
        }
    }
    None
}

/// The double-quoted value assigned to `key` anywhere in one line of manifest
/// text, whether the line is a table entry or an inline table body. The key is
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
            && let Some(quoted) = value.trim_start().strip_prefix('"')
            && let Some(index) = quoted.find('"')
        {
            return quoted.get(..index).map(str::to_string);
        }
        from = end;
    }
}

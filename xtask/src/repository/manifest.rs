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
//! # The line is the unit, so a declaration that is not one line is unread
//!
//! Every reader here is line-oriented, which is exact for the manifests this
//! repository commits and for every spelling named above. What a line-oriented
//! reader cannot do is tell structure from data once a value stops closing on
//! the line that opened it. The lines that value spans are read as the headers
//! and keys they resemble, so the reader believes a table ENDED where a string
//! merely continued, and every key after it lands somewhere else.
//!
//! That was not a ceiling. It was a hole, and it was executed. MEASURED on
//! cargo 1.97.1, against the real root manifest, with the gate run as its own
//! compiled binary: a `[dev-dependencies.helpers]` sub-table carrying a
//! multi-line string whose body holds one line shaped like a table header is
//! accepted by cargo, resolves `threadpak → threadpak-macroc`, and passed the
//! topology law — because by the time the reader reached `package` and `path`
//! it no longer believed it was inside a dependency table at all. Three more
//! spellings buy the same forgetting and were measured accepted and resolving
//! the same edge: the multi-line LITERAL string, the bracket list whose
//! elements sit on their own lines with one of them shaped like a header, and
//! the inline table spread across lines, whose continuation lines are read as
//! fresh entries of the enclosing table. A fourth needs no dependency table at
//! all: a spanning value anywhere in the file leaves the reader inside whatever
//! header its body quoted, so a dotted `dev-dependencies.helpers.package` key
//! written afterwards resolves for cargo and reaches no law here.
//!
//! So this reader refuses rather than guesses. [`dependency_declarations`]
//! reports what it did not enter in [`ManifestDependencies::unread`], and the
//! topology law refuses a manifest carrying any of three shapes:
//!
//! **A value that does not close on the line that opens it** — a multi-line
//! basic string, a multi-line literal string, a bracket list broken across
//! lines, or an inline table broken across lines. The refusal is on the SHAPE
//! and never on what the body says, because a rule that read the body to decide
//! whether the body was dangerous would be reading the body.
//!
//! **A basic string carrying a `\` escape**, which is the second half of the
//! same honesty and is described under its own heading below.
//!
//! **A dependency table written as an INLINE table**, whose entries live inside
//! one line's value rather than on lines of their own.
//!
//! Reading any of them properly is the typed repository model's migration, and a
//! second parser seated here would be the duplicate authority this repository is
//! eliminating.
//!
//! # Which way the refusal is allowed to be wrong
//!
//! Both delimiters are handled, `"""` and `'''`, and so are `[` and `{`. A value
//! that closes where it opens is READ, not refused, so `notes = """one line"""`
//! is self-contained and passes, and so does every other single-line use of
//! either delimiter. The closing delimiter of a basic string is still found with
//! `\` escapes honoured the way TOML honours them, and that has not stopped
//! mattering now that an escape is refused on its own account: it is what makes
//! `notes = "a \" b"` a value that CLOSES and carries an escape, rather than one
//! reported as spanning lines it does not span. Two shapes, two repairs, and a
//! reader that confused them would send an author to the wrong one.
//!
//! What is refused bluntly is the spanning shape itself, whatever it holds — a
//! `description` spanning three lines of harmless prose is refused exactly like
//! one quoting a table, and so is a workspace `members` list nobody could hide
//! an edge inside. That costs this repository one real line, `workspace.members`
//! in the root manifest, which is written on one line and says there why. It is
//! the affordable direction: a manifest is REFUSED where it might have been
//! read, and never READ where it should have been refused. The reverse rule —
//! refuse only when the spanned body looks dangerous — would put the reader back
//! inside the value it just admitted it cannot enter.
//!
//! # What a value SAYS is not what it MEANS
//!
//! The second refusal, in a different mechanism, found while measuring the
//! first. It is not a ceiling: it was a hole, MEASURED on cargo 1.97.1 against
//! the real root manifest with the gate run as its own compiled binary. Cargo
//! decodes a basic string's escapes before it resolves anything, and this reader
//! compares the TEXT of a value against the names Cargo compares the DECODED
//! value against, so this manifest was accepted, resolved
//! `threadpak -> threadpak-macroc`, and the topology law printed PASS:
//!
//! ```toml
//! [dev-dependencies]
//! helpers = { package = "threadpak\u002Dmacroc", path = "\u006Dacros/macroc" }
//! ```
//!
//! `\u002D` is six characters here and one `-` to Cargo. So a basic string
//! carrying a `\` is reported UNREAD, whatever the escape spells: TOML admits
//! `\u`, `\U`, `\n`, `\t`, `\\`, `\"` and the rest, all of them decoded by Cargo
//! and none of them decoded here, and a rule that refused only the escapes
//! somebody had thought of would be the same guess this file exists to stop.
//! Decoding them instead is the typed repository model's job; refusing is what
//! an undecoded reader may honestly do in the meantime.
//!
//! A LITERAL string is a different value and is left READ, which is measured
//! rather than assumed. TOML decodes nothing inside `'…'`, and cargo agrees:
//! `package = 'threadpak\u002Dmacroc'` is refused by CARGO ITSELF with `invalid
//! character \ in package name`, because nothing turned those six characters
//! into a `-`, so there is no edge for an escape to hide there. What a `\` in a
//! literal string IS, is a Windows path separator: `path = 'macros\macroc'` is
//! accepted by cargo, resolves the edge, and is caught here — refusing it would
//! be a false refusal on a lawful spelling, so it is not refused.
//!
//! The distinction is exactly TOML's, and it is why the escape scan runs inside
//! the string skipper rather than over the line: a `\` is an escape where the
//! string it sits in decodes escapes, and a character where it does not.

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

/// One declaration this reader did not enter, spelled as the key path it sits
/// at.
///
/// Three shapes rather than one word, because they take three repairs: one is
/// spelled out as a table with its entries on their own lines, one is put back
/// on the single line it belongs on, and one is written with the characters it
/// means. A reader that reported them the same way would tell an author to fix
/// the wrong thing.
pub(crate) enum Unread {
    /// A whole dependency table written as an inline table. Its entries sit
    /// inside one line's value, and this reader does not enter a value.
    InlineTable(String),
    /// A value that does not close on the line that opens it. The lines it
    /// spans are data, and this reader reads lines — so what those lines look
    /// like is not what they are.
    MultiLineValue(String),
    /// A basic string carrying a `\` escape. Cargo decodes it before resolving
    /// anything and this reader does not, so what the value says here is not
    /// what it means there. A literal string decodes nothing and is not this.
    EscapedValue(String),
}

/// What one manifest declares about its dependencies: the entries a reader
/// resolved, and the declarations it could not enter.
///
/// The second field exists because a reader that returned only what it managed
/// to read would answer "no prohibited edge" and "no reading happened" with the
/// same empty list. A caller gets both facts or neither.
pub(crate) struct ManifestDependencies {
    /// Every dependency entry the manifest declares, one per entry rather than
    /// one per line: a dotted entry spelled across several lines is one entry,
    /// which is what Cargo resolves it to.
    pub(crate) entries: Vec<DependencyEntry>,
    /// Every declaration this reader could not enter, so that what it did not
    /// read is reported UNREAD rather than reported absent.
    pub(crate) unread: Vec<Unread>,
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
///
/// A line whose value does not close on it, or whose basic string carries an
/// escape, is reported UNREAD and is never resolved to an entry, wherever in the
/// file it sits. The first check is on every such line rather than only on the
/// ones inside a dependency table, because a value that spans lines takes the
/// reader's idea of which table it is in with it, and the keys that idea then
/// mis-seats are written after the value, not inside it. The second is on every
/// line for a plainer reason: a reader that does not decode has no business
/// deciding which undecoded values were the important ones.
pub(crate) fn dependency_declarations(manifest_text: &str) -> ManifestDependencies {
    let mut entries: Vec<DependencyEntry> = Vec::new();
    let mut unread: Vec<Unread> = Vec::new();
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
        match scan_value(value) {
            ScannedValue::Read => {}
            ScannedValue::SpansLines => {
                unread.push(Unread::MultiLineValue(place.join(".")));
                continue;
            }
            ScannedValue::Escaped => {
                unread.push(Unread::EscapedValue(place.join(".")));
                continue;
            }
        }
        let Some((kind, name, fields)) = dependency_position(&place) else {
            if value.starts_with('{')
                && let Some(spelling) = unenterable_table(&place)
            {
                unread.push(Unread::InlineTable(spelling));
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

/// What one line's value is, as far as a reader that never decodes it can tell.
enum ScannedValue {
    /// The value closes on its line and passed through nothing this reader
    /// cannot resolve: it is the value it appears to be.
    Read,
    /// The value does not close on the line that opens it, so the next line is
    /// inside it rather than after it.
    SpansLines,
    /// The value closes, and a basic string inside it carries a `\` escape, so
    /// the characters here are not the characters Cargo resolves against.
    Escaped,
}

/// What a string this reader stepped over carried that it cannot resolve.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Carried {
    /// Nothing. Every character in the body means itself, here and to Cargo.
    Nothing,
    /// A `\` escape, which Cargo decodes before it resolves anything and this
    /// reader does not decode at all.
    Escape,
}

/// One string stepped over whole: what follows it, and what it carried.
struct SkippedString<'a> {
    /// The text after the closing delimiter.
    rest: &'a str,
    /// What the body carried. Never [`Carried::Escape`] for a literal string,
    /// which has no escapes to carry.
    carried: Carried,
}

/// Both questions this reader has to answer about a value before it trusts
/// anything it reads off the line, or off the next one.
///
/// A value that closes here means the next line is structure, and a value that
/// does not means the next line is somebody's data wearing structure's clothes.
/// Strings are skipped whole under either quote and either width, so a bracket
/// or a brace inside one counts for nothing, and brackets and braces outside one
/// are counted in and out. What is left open at the end of the line — a string
/// still running, or a list or an inline table still nested — spans lines. What
/// was passed on the way is reported too: a `\` inside a BASIC string is an
/// escape Cargo decodes and this reader does not, and a `\` inside a literal
/// string is a character in both, which is why the answer comes back from the
/// string skipper rather than from a scan of the line.
///
/// This answers where a value ENDS and what it PASSED THROUGH, never what it
/// means. It resolves no key, turns no escape into the character it stands for,
/// and produces no value; a `#` outside a string ends the line here for the same
/// reason it does in [`strip_comment`], and nothing else about the body is
/// looked at.
fn scan_value(value: &str) -> ScannedValue {
    let mut depth = 0usize;
    let mut carried = Carried::Nothing;
    let mut rest = value;
    loop {
        let Some(character) = rest.chars().next() else {
            return settled(depth, carried);
        };
        if character == '#' {
            return settled(depth, carried);
        }
        if character == '"' || character == '\'' {
            let Some(skipped) = string_end(rest, character) else {
                return ScannedValue::SpansLines;
            };
            if skipped.carried == Carried::Escape {
                carried = Carried::Escape;
            }
            rest = skipped.rest;
            continue;
        }
        if character == '[' || character == '{' {
            depth = depth.saturating_add(1);
        } else if character == ']' || character == '}' {
            depth = depth.saturating_sub(1);
        }
        let Some(after) = rest.get(character.len_utf8()..) else {
            return settled(depth, carried);
        };
        rest = after;
    }
}

/// The verdict once the line runs out.
///
/// Still nested is a value that spans lines; an escape passed on the way is a
/// value whose characters are not the ones Cargo resolves against. Nesting is
/// answered first because a value that has not ended yet has not finished saying
/// what it carries either.
fn settled(depth: usize, carried: Carried) -> ScannedValue {
    if depth != 0 {
        return ScannedValue::SpansLines;
    }
    match carried {
        Carried::Nothing => ScannedValue::Read,
        Carried::Escape => ScannedValue::Escaped,
    }
}
/// The string a value opens with, stepped over whole, or nothing when it does
/// not close on the same line.
///
/// The delimiter is whichever the value opened with — one quote or three, basic
/// or literal — and the closing delimiter is the same one, which is what makes
/// `"""` a different string from `"` rather than three of them in a row.
///
/// Whether the string DECODES is the difference TOML draws between its two
/// kinds, and it decides both answers here. In a basic string a `\` escapes the
/// character after it, so `"a \" b"` closes at its last quote and not its
/// second, and the escape is reported because Cargo will turn it into a
/// character this reader never will. In a literal string there are no escapes at
/// all: a `\` is a backslash to TOML, to Cargo, and here, so it is stepped over
/// like any other character and reported as nothing. That is measured rather
/// than assumed — `package = 'threadpak\u002Dmacroc'` is refused by cargo itself
/// as an invalid package name, because nothing decoded it.
fn string_end(text: &str, quote: char) -> Option<SkippedString<'_>> {
    let triple = if quote == '"' { "\"\"\"" } else { "'''" };
    let decodes = quote == '"';
    let (delimiter, mut rest) = if let Some(body) = text.strip_prefix(triple) {
        (triple, body)
    } else {
        (text.get(..quote.len_utf8())?, text.get(quote.len_utf8()..)?)
    };
    let mut carried = Carried::Nothing;
    loop {
        if let Some(after) = rest.strip_prefix(delimiter) {
            return Some(SkippedString {
                rest: after,
                carried,
            });
        }
        let character = rest.chars().next()?;
        let mut step = character.len_utf8();
        if decodes && character == '\\' {
            carried = Carried::Escape;
            if let Some(after) = rest.get(step..).and_then(|tail| tail.chars().next()) {
                step = step.saturating_add(after.len_utf8());
            }
        }
        rest = rest.get(step..)?;
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

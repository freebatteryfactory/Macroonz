//! What the repository may be called and what it may call things.
//!
//! Two laws over the same scanning shape: no personal name anywhere, and no
//! construction-lifecycle word in the specification tree. Both are laws about
//! WORDS, so both have to survive the ways a word hides — a different casing, a
//! plural, a `camelCase` seam — and both have to say so out loud, because a
//! word-scan that only reads prose is a word-scan somebody will route around.

use std::fs;
use std::path::Path;

use crate::repository::walk::{
    JUDGE_DIRECTORY, TOOLING_DIRECTORY, relative_slash_path, visit_files,
};

/// The construction-lifecycle vocabulary the working law bans in prose and in
/// identifiers. This checker spells the words plainly because `xtask` sits
/// outside the tree it scans; `AGENTS.md` and `CLAUDE.md` state the ban itself
/// and are likewise outside it.
const BANNED_VOCABULARY: [&str; 4] = ["factory", "candidate", "promotion", "self-hosting"];

/// Lawful survivals: `(repository-relative path, word, why it stands)`. A term
/// stands only where it is named to FORBID it, to record a kill, or to
/// document a rename — never as live vocabulary.
const BANNED_VOCABULARY_ALLOWLIST: [(&str, &str, &str); 3] = [
    (
        "src/23_evidence/README.md",
        "candidate",
        "the executed-rename record: the dead word is named once to record that \
         `proposal` replaced it",
    ),
    (
        "src/23_evidence/README.md",
        "promotion",
        "the same record: `adoption` replaced it",
    ),
    (
        "src/23_evidence/README.md",
        "factory",
        "the same record: `realization owner` replaced it",
    ),
];

/// No personal name appears in any repository file — role terms only. The
/// banned spellings are assembled from bytes so this checker never contains
/// what it forbids.
pub(crate) fn check_no_personal_names(root: &Path) -> Result<(), String> {
    let banned: [Vec<u8>; 2] = [
        vec![0x65, 0x61, 0x73, 0x73, 0x61],
        vec![0x61, 0x79, 0x6f, 0x75, 0x62],
    ];
    let banned: Vec<String> = banned
        .iter()
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .collect();
    let mut offenders = Vec::new();
    visit_files(root, &mut |path| {
        let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let text = String::from_utf8_lossy(&bytes).to_lowercase();
        for name in &banned {
            if text.contains(name.as_str()) {
                offenders.push(path.display().to_string());
                break;
            }
        }
        Ok(())
    })?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "personal name present in: {}",
            offenders.join(", ")
        ))
    }
}

/// No banned construction-lifecycle word appears in the specification tree.
///
/// Two scans run over every file, and a hit from either is an offence:
///
/// 1. Whole-word, case-insensitive over the whole text: word edges are ASCII
///    alphanumerics, so `snake_case`, `SCREAMING_SNAKE`, kebab-case strings,
///    and plain prose all count, while a longer word merely containing the
///    term does not.
/// 2. Split-identifier: every identifier-like token is cut on `camelCase` and
///    `snake_case` boundaries and each resulting word is compared
///    case-insensitively against the banned list AND its simple plural, so
///    a `CamelCase` type name ending in the plural, a `mixedCase` field, and
///    the plural in plain prose are all caught. A hyphenated banned term
///    matches a consecutive run of split words inside one token, so
///    `SelfHosting` and `self_hosting` are caught too.
///
/// Both scans report the banned ROOT word, so one allowlist entry covers a
/// file for either scan. The scanned tree is the machine (`src/`), the root
/// `README.md`, the metaprogramming subsystem (`macros/`), and the
/// qualification plane (`testpak/`): the tools and the judge speak the
/// machine's vocabulary or they speak none.
pub(crate) fn check_banned_vocabulary(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    let mut read: Vec<(String, String)> = Vec::new();
    let mut inspect = |path: &Path| -> Result<(), String> {
        let scanned = path
            .extension()
            .is_some_and(|extension| extension == "rs" || extension == "md");
        if !scanned {
            return Ok(());
        }
        let relative = relative_slash_path(root, path);
        let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        offenders.extend(banned_vocabulary_offences(&relative, &text));
        read.push((relative, text));
        Ok(())
    };
    visit_files(&root.join("src"), &mut inspect)?;
    visit_files(&root.join(TOOLING_DIRECTORY), &mut inspect)?;
    visit_files(&root.join(JUDGE_DIRECTORY), &mut inspect)?;
    inspect(&root.join("README.md"))?;
    // The allowlist is joined against the same scan: every allowance has to
    // still be excusing something.
    offenders.extend(stale_allowlist_offences(&read));
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "banned vocabulary present: {}",
            offenders.join(", ")
        ))
    }
}

/// Every allowlist entry whose named file no longer spells the word it excuses,
/// one offence per stale entry.
///
/// An allowance is a claim about a file: this word survives HERE, for this
/// reason. When the word leaves the file the claim is no longer about anything,
/// and what is left is a standing hole nobody is watching — the next edit that
/// reintroduces the word to that file passes silently, and the reason line still
/// reads as if somebody had looked. So an entry that matches nothing is refused
/// exactly as a red row naming a reversal nobody wrote is refused: both read as
/// discharged and are not.
///
/// The scan is the same one the ban uses, so an entry is stale by the check's own
/// standard rather than by a second, looser reading of the file.
///
/// Pure over its inputs — `(repository-relative path, that file's text)` pairs
/// for every scanned file — so the law is proven against fixture text.
fn stale_allowlist_offences(scanned: &[(String, String)]) -> Vec<String> {
    let mut offences = Vec::new();
    for (file, word, reason) in BANNED_VOCABULARY_ALLOWLIST {
        let matched = scanned
            .iter()
            .any(|(path, text)| path == file && banned_words_in(text).contains(&word));
        if !matched {
            offences.push(format!(
                "stale allowlist entry: {file} no longer spells `{word}` ({reason})"
            ));
        }
    }
    offences
}

/// Every banned root word one text spells, by either scan, each reported once.
///
/// Both scans are pure over the text, which is what makes the law provable
/// against fixture strings rather than against the tree it guards.
fn banned_words_in(text: &str) -> Vec<&'static str> {
    let lowered = text.to_lowercase();
    let mut hits: Vec<&'static str> = Vec::new();
    for word in BANNED_VOCABULARY {
        if contains_whole_word(&lowered, word) && !hits.contains(&word) {
            hits.push(word);
        }
    }
    for banned in split_scan_hits(text) {
        if !hits.contains(&banned) {
            hits.push(banned);
        }
    }
    hits
}

/// The offences one file commits, its allowlisted survivals removed.
fn banned_vocabulary_offences(relative: &str, text: &str) -> Vec<String> {
    banned_words_in(text)
        .into_iter()
        .filter(|word| {
            !BANNED_VOCABULARY_ALLOWLIST
                .iter()
                .any(|(file, allowed, _)| *file == relative && allowed == word)
        })
        .map(|word| format!("{relative}: {word}"))
        .collect()
}

/// Whether `haystack` contains `needle` bounded by non-alphanumerics on both
/// sides.
fn contains_whole_word(haystack: &str, needle: &str) -> bool {
    let mut from = 0usize;
    loop {
        let Some(rest) = haystack.get(from..) else {
            return false;
        };
        let Some(offset) = rest.find(needle) else {
            return false;
        };
        let start = from.saturating_add(offset);
        let end = start.saturating_add(needle.len());
        let before_is_word = haystack
            .get(..start)
            .and_then(|head| head.chars().next_back())
            .is_some_and(|c| c.is_ascii_alphanumeric());
        let after_is_word = haystack
            .get(end..)
            .and_then(|tail| tail.chars().next())
            .is_some_and(|c| c.is_ascii_alphanumeric());
        if !before_is_word && !after_is_word {
            return true;
        }
        from = end;
    }
}

/// Every banned root word spelled by an identifier-like token in `text` once
/// that token is cut on `camelCase` and `snake_case` boundaries.
fn split_scan_hits(text: &str) -> Vec<&'static str> {
    let mut hits: Vec<&'static str> = Vec::new();
    let tokens = text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'));
    for token in tokens {
        if token.is_empty() {
            continue;
        }
        let words = split_identifier_words(token);
        for word in &words {
            if let Some(banned) = spells_banned_word(word)
                && !hits.contains(&banned)
            {
                hits.push(banned);
            }
        }
        for banned in BANNED_VOCABULARY {
            let parts: Vec<&str> = banned.split('-').collect();
            if parts.len() < 2 || words.len() < parts.len() {
                continue;
            }
            let spelled = words.windows(parts.len()).any(|run| {
                run.iter()
                    .zip(parts.iter())
                    .all(|(word, part)| word == part)
            });
            if spelled && !hits.contains(&banned) {
                hits.push(banned);
            }
        }
    }
    hits
}

/// Cuts one identifier-like token into its lowercase words on `snake_case`
/// separators, `camelCase` boundaries, and acronym-to-word boundaries
/// (`SELFHosting` cuts before `Hosting`).
fn split_identifier_words(token: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut previous_is_lower_or_digit = false;
    let mut chars = token.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '_' {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            previous_is_lower_or_digit = false;
            continue;
        }
        let next_is_lower = chars.peek().is_some_and(char::is_ascii_lowercase);
        if c.is_ascii_uppercase()
            && !current.is_empty()
            && (previous_is_lower_or_digit || next_is_lower)
        {
            words.push(std::mem::take(&mut current));
        }
        current.push(c.to_ascii_lowercase());
        previous_is_lower_or_digit = c.is_ascii_lowercase() || c.is_ascii_digit();
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

/// The banned root word a single lowercase split word spells, counting the
/// simple plural (`candidates`, `promotions`, `factories`). Hyphenated banned
/// terms are matched as word runs, never here.
fn spells_banned_word(word: &str) -> Option<&'static str> {
    BANNED_VOCABULARY.into_iter().find(|banned| {
        if banned.contains('-') {
            return false;
        }
        let plural = banned
            .strip_suffix('y')
            .map_or_else(|| format!("{banned}s"), |stem| format!("{stem}ies"));
        word == *banned || word == plural
    })
}

/// Planted reversals for the ban and for its allowlist.
///
/// The scans are pure over text, so every reversal below is a fixture string.
/// Nothing on disk is written, read, or mutated: the law that guards the tree is
/// never proven by dirtying the tree.
#[cfg(test)]
mod tests {
    use super::{
        BANNED_VOCABULARY_ALLOWLIST, banned_vocabulary_offences, banned_words_in,
        stale_allowlist_offences,
    };
    use crate::repository::walk::repo_root;
    use std::fs;
    use std::path::PathBuf;

    /// Planted reversal: the term smuggled into a `camelCase` identifier, where
    /// no whole-word scan of the text would ever find it.
    #[test]
    fn a_camel_case_smuggle_is_caught() {
        let found = banned_words_in("let selectedFactoryFloor = 1;");
        assert_eq!(found, vec!["factory"], "{found:?}");
    }

    /// Planted reversal: the plural, in prose and in a `CamelCase` type name.
    #[test]
    fn a_plural_is_caught() {
        let prose = banned_words_in("the surviving candidates were counted");
        assert_eq!(prose, vec!["candidate"], "{prose:?}");
        let irregular = banned_words_in("struct RegisteredFactories;");
        assert_eq!(irregular, vec!["factory"], "{irregular:?}");
    }

    /// Planted reversal: a hyphenated term spelled as a consecutive run of
    /// words inside one identifier, in both casings.
    #[test]
    fn a_hyphen_run_is_caught() {
        let camel = banned_words_in("enum SelfHosting { No }");
        assert_eq!(camel, vec!["self-hosting"], "{camel:?}");
        let snake = banned_words_in("const SELF_HOSTING_POSTURE: u8 = 0;");
        assert_eq!(snake, vec!["self-hosting"], "{snake:?}");
    }

    /// Planted reversal: plain `snake_case`, and the kebab-case string a
    /// README row would carry.
    #[test]
    fn a_snake_case_or_kebab_spelling_is_caught() {
        let snake = banned_words_in("fn promotion_route() {}");
        assert_eq!(snake, vec!["promotion"], "{snake:?}");
        let kebab = banned_words_in("id: gate.promotion-ladder");
        assert_eq!(kebab, vec!["promotion"], "{kebab:?}");
    }

    /// The positive control: clean text passes, and a longer word merely
    /// CONTAINING a banned root is not a hit. A checker that flagged
    /// everything would satisfy every reversal above and be worthless.
    #[test]
    fn clean_text_passes() {
        let found = banned_words_in(
            "The proposal was adopted by its realization owner. \
             Manufactured goods and refactoring are ordinary words.",
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// An allowlisted path keeps its one named survival and nothing else: the
    /// allowance is per file AND per word, never a blanket pass.
    #[test]
    fn an_allowlisted_path_keeps_only_its_named_survival() {
        let allowed = banned_vocabulary_offences(
            "src/23_evidence/README.md",
            "`proposal` replaced the dead word candidate",
        );
        assert!(allowed.is_empty(), "{allowed:?}");
        let elsewhere =
            banned_vocabulary_offences("src/00_refusal/README.md", "the dead word candidate");
        assert_eq!(elsewhere.len(), 1, "{elsewhere:?}");
        let unallowed = banned_vocabulary_offences(
            "src/23_evidence/README.md",
            "a self-hosting posture is not allowlisted anywhere",
        );
        assert_eq!(unallowed.len(), 1, "{unallowed:?}");
        assert!(
            unallowed
                .first()
                .is_some_and(|offence| offence.contains("self-hosting"))
        );
    }

    /// Planted reversal: an allowlist entry whose named file no longer spells
    /// the word it excuses. The allowance reads as if somebody had looked at
    /// that file, and the hole it leaves open is unwatched.
    #[test]
    fn a_stale_allowlist_entry_is_a_violation() {
        // The lawful state: every entry's file still spells the word it excuses.
        let live: Vec<(String, String)> = BANNED_VOCABULARY_ALLOWLIST
            .iter()
            .map(|(file, word, _)| {
                (
                    (*file).to_string(),
                    format!("the dead word {word} is recorded here once"),
                )
            })
            .collect();
        assert!(stale_allowlist_offences(&live).is_empty());

        // One entry's word gone from its file, the other two still there.
        let partial: Vec<(String, String)> = live
            .iter()
            .filter(|(_, text)| !text.contains("promotion"))
            .cloned()
            .collect();
        let found = stale_allowlist_offences(&partial);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.first().is_some_and(|offence| {
            offence.contains("stale allowlist entry") && offence.contains("promotion")
        }));

        // The file gone entirely: every entry naming it is stale, and each one
        // is reported on its own rather than folded into one line.
        let gone = stale_allowlist_offences(&[]);
        assert_eq!(gone.len(), BANNED_VOCABULARY_ALLOWLIST.len(), "{gone:?}");
    }

    /// The real allowlist holds: every entry still excuses a word its named
    /// file spells, read through the ban's own scan.
    #[test]
    fn the_real_allowlist_still_excuses_something() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let scanned: Vec<(String, String)> = BANNED_VOCABULARY_ALLOWLIST
            .iter()
            .map(|(file, _, _)| {
                (
                    (*file).to_string(),
                    fs::read_to_string(root.join(file)).unwrap_or_default(),
                )
            })
            .collect();
        let found = stale_allowlist_offences(&scanned);
        assert!(found.is_empty(), "{found:?}");
    }
}

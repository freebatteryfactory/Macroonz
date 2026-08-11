//! Repository law checks for the `ThreadPak` workspace.
//!
//! `cargo xtask check` runs every day-zero repository law and reports each result;
//! any broken law fails the run. Checks grow one at a time as each written rule
//! gains something to enforce — the repository never carries a rule that nothing
//! checks.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Directories never visited by repository-wide file checks.
const SKIP_DIRS: [&str; 2] = [".git", "target"];

/// One repository law: a name and the function that checks it.
type Check = (&'static str, fn(&Path) -> Result<(), String>);

fn main() -> Result<(), Box<dyn Error>> {
    let root = repo_root()?;
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("check"));
    match command.as_str() {
        "check" => run_checks(&root),
        other => Err(format!("unknown xtask command: {other}").into()),
    }
}

/// The workspace root: the parent of the xtask crate directory.
fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let parent = manifest_dir
        .parent()
        .ok_or("xtask crate directory has no parent")?;
    Ok(parent.to_path_buf())
}

/// Runs every repository law, printing one PASS or FAIL line per law.
fn run_checks(root: &Path) -> Result<(), Box<dyn Error>> {
    let checks: [Check; 11] = [
        ("agents-claude-parity", check_agents_claude_parity),
        ("lf-and-no-symlinks", check_lf_and_no_symlinks),
        ("no-python", check_no_python),
        ("toolchain-pin-matches-readme", check_toolchain_pin),
        ("workspace-members-match-readme", check_workspace_members),
        ("lint-wall-inherited", check_lint_wall),
        (
            "underscore-fields-are-phantom",
            check_underscore_fields_are_phantom,
        ),
        ("band-map-matches-lib", check_band_map),
        ("readme-obligations-join", check_obligations_join),
        ("no-personal-names", check_no_personal_names),
        ("banned-vocabulary", check_banned_vocabulary),
    ];
    let mut failures = Vec::new();
    for (name, check) in checks {
        match check(root) {
            Ok(()) => println!("PASS {name}"),
            Err(reason) => {
                println!("FAIL {name}: {reason}");
                failures.push(name);
            }
        }
    }
    if failures.is_empty() {
        println!("all repository laws hold");
        Ok(())
    } else {
        Err(format!("{} repository law(s) broken", failures.len()).into())
    }
}

/// `AGENTS.md` and `CLAUDE.md` carry the same working law and must stay
/// byte-identical.
fn check_agents_claude_parity(root: &Path) -> Result<(), String> {
    let agents = fs::read(root.join("AGENTS.md")).map_err(|e| format!("AGENTS.md: {e}"))?;
    let claude = fs::read(root.join("CLAUDE.md")).map_err(|e| format!("CLAUDE.md: {e}"))?;
    if agents == claude {
        Ok(())
    } else {
        Err(String::from("AGENTS.md and CLAUDE.md differ"))
    }
}

/// Every file in the repository is LF-only and nothing is a symlink.
fn check_lf_and_no_symlinks(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    visit_files(root, &mut |path| {
        let metadata =
            fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if metadata.file_type().is_symlink() {
            offenders.push(format!("symlink: {}", path.display()));
            return Ok(());
        }
        let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if bytes.contains(&b'\r') {
            offenders.push(format!("CRLF: {}", path.display()));
        }
        Ok(())
    })?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// No Python exists in this repository, ever.
fn check_no_python(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    visit_files(root, &mut |path| {
        if path.extension().is_some_and(|ext| ext == "py") {
            offenders.push(path.display().to_string());
        }
        Ok(())
    })?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!("python files present: {}", offenders.join(", ")))
    }
}

/// The toolchain pinned in `rust-toolchain.toml` matches the toolchain the README
/// yaml block declares.
fn check_toolchain_pin(root: &Path) -> Result<(), String> {
    let toolchain_text = fs::read_to_string(root.join("rust-toolchain.toml"))
        .map_err(|e| format!("rust-toolchain.toml: {e}"))?;
    let pinned = quoted_value(&toolchain_text, "channel")?;
    let yaml = readme_yaml_block(root)?;
    let declared = yaml
        .iter()
        .find_map(|line| line.strip_prefix("toolchain: "))
        .map(|value| value.trim_matches('"').to_string())
        .ok_or_else(|| String::from("README yaml block has no toolchain line"))?;
    if pinned == declared {
        Ok(())
    } else {
        Err(format!(
            "rust-toolchain.toml pins {pinned} but README declares {declared}"
        ))
    }
}

/// The workspace members in `Cargo.toml` match the members the README yaml block
/// declares.
fn check_workspace_members(root: &Path) -> Result<(), String> {
    let manifest =
        fs::read_to_string(root.join("Cargo.toml")).map_err(|e| format!("Cargo.toml: {e}"))?;
    let actual = bracket_list(&manifest, "members")?;
    let yaml = readme_yaml_block(root)?;
    let mut declared = Vec::new();
    let mut in_members = false;
    for line in &yaml {
        if in_members {
            if let Some(item) = line.trim().strip_prefix("- ") {
                declared.push(item.trim().to_string());
            } else {
                in_members = false;
            }
        }
        if line.trim() == "workspace_members:" {
            in_members = true;
        }
    }
    if actual == declared {
        Ok(())
    } else {
        Err(format!(
            "Cargo.toml members {actual:?} but README declares {declared:?}"
        ))
    }
}

/// The root manifest declares the one lint wall and every member inherits it.
fn check_lint_wall(root: &Path) -> Result<(), String> {
    let manifest =
        fs::read_to_string(root.join("Cargo.toml")).map_err(|e| format!("Cargo.toml: {e}"))?;
    if !manifest.contains("[workspace.lints.rust]") {
        return Err(String::from(
            "root Cargo.toml has no [workspace.lints.rust] wall",
        ));
    }
    let members = bracket_list(&manifest, "members")?;
    let mut missing = Vec::new();
    for member in members {
        let path = root.join(&member).join("Cargo.toml");
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if !text.contains("[lints]\nworkspace = true") {
            missing.push(member);
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("members not inheriting the lint wall: {missing:?}"))
    }
}

/// Every numbered band directory is complete (README.md, mod.rs, types.rs) and
/// `lib.rs` declares every band via its `#[path]` attribute in ascending band
/// order — the band map and the crate never drift apart.
fn check_band_map(root: &Path) -> Result<(), String> {
    let src = root.join("src");
    let mut bands = Vec::new();
    let entries = fs::read_dir(&src).map_err(|e| format!("{}: {e}", src.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", src.display()))?;
        if !entry
            .file_type()
            .map_err(|e| format!("{}: {e}", src.display()))?
            .is_dir()
        {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some((number, _)) = name.split_once('_') else {
            continue;
        };
        if number.len() == 2 && number.chars().all(|c| c.is_ascii_digit()) {
            bands.push(name);
        }
    }
    bands.sort();
    let mut offenders = Vec::new();
    for band in &bands {
        for file in ["README.md", "mod.rs", "types.rs"] {
            if !src.join(band).join(file).is_file() {
                offenders.push(format!("{band} missing {file}"));
            }
        }
    }
    let lib = fs::read_to_string(src.join("lib.rs")).map_err(|e| format!("lib.rs: {e}"))?;
    let mut declared_positions = Vec::new();
    for band in &bands {
        let needle = format!("#[path = \"{band}/mod.rs\"]");
        match lib.find(&needle) {
            Some(position) => declared_positions.push((position, band.clone())),
            None => offenders.push(format!("lib.rs does not declare {band}")),
        }
    }
    let mut sorted = declared_positions.clone();
    sorted.sort();
    if sorted != declared_positions {
        offenders.push(String::from(
            "lib.rs band declarations are out of band order",
        ));
    }
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// The obligations join: every README obligation naming a `laws.rs` green law
/// points at a law that exists, and every law in `laws.rs` is claimed by some
/// obligation — the READMEs and the laws never drift apart. (The third leg —
/// the owed red twin — joins when testpak lands.)
fn check_obligations_join(root: &Path) -> Result<(), String> {
    let mut claimed = Vec::new();
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
    for readme in &readmes {
        let text = fs::read_to_string(readme).map_err(|e| format!("{}: {e}", readme.display()))?;
        for line in text.lines() {
            let Some(rest) = line.trim().strip_prefix("green: laws.rs ") else {
                continue;
            };
            let target: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
                .collect();
            match target.split_once("::") {
                Some((module, law)) => {
                    claimed.push((module.to_string(), law.to_string(), readme.clone()));
                }
                None => {
                    return Err(format!(
                        "{}: green target `{target}` is not module::law",
                        readme.display()
                    ));
                }
            }
        }
    }
    let laws_path = src.join("laws.rs");
    let laws = fs::read_to_string(&laws_path).map_err(|e| format!("laws.rs: {e}"))?;
    let mut existing = Vec::new();
    let mut current_module = String::new();
    let mut previous_was_test = false;
    for line in laws.lines() {
        if let Some(rest) = line.strip_prefix("mod ")
            && let Some(module) = rest.strip_suffix(" {")
        {
            current_module = module.to_string();
        }
        if previous_was_test
            && let Some(rest) = line.trim().strip_prefix("fn ")
            && let Some(law) = rest.split('(').next()
        {
            existing.push((current_module.clone(), law.to_string()));
        }
        previous_was_test = line.trim() == "#[test]";
    }
    let mut offenders = Vec::new();
    for (module, law, readme) in &claimed {
        if !existing.iter().any(|(m, l)| m == module && l == law) {
            offenders.push(format!(
                "{} claims {module}::{law} but laws.rs has no such law",
                readme.display()
            ));
        }
    }
    for (module, law) in &existing {
        if !claimed.iter().any(|(m, l, _)| m == module && l == law) {
            offenders.push(format!(
                "laws.rs {module}::{law} is claimed by no obligation"
            ));
        }
    }
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// No personal name appears in any repository file — role terms only. The
/// banned spellings are assembled from bytes so this checker never contains
/// what it forbids.
fn check_no_personal_names(root: &Path) -> Result<(), String> {
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
/// An underscore-prefixed field is lawful only when it is a `PhantomData`
/// type-level law. Real data behind an underscore is the suppressor idiom —
/// "ignore this mess" — and the repository refuses it: the only honest `_`
/// is one with nothing to read.
fn check_underscore_fields_are_phantom(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    visit_files(&root.join("src"), &mut |path| {
        if path.extension().is_none_or(|extension| extension != "rs") {
            return Ok(());
        }
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        for (index, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            let field = trimmed
                .strip_prefix("pub(crate) ")
                .or_else(|| trimmed.strip_prefix("pub "))
                .unwrap_or(trimmed);
            if field.starts_with('_')
                && !field.starts_with("_ ")
                && field.contains(": ")
                && !trimmed.starts_with("//")
                && !line.contains("PhantomData")
            {
                offenders.push(format!(
                    "{}:{}: underscore field without PhantomData",
                    path.display(),
                    index.saturating_add(1)
                ));
            }
        }
        Ok(())
    })?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// Both scans report the banned ROOT word, so one allowlist entry covers a
/// file for either scan.
fn check_banned_vocabulary(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
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
        let lowered = text.to_lowercase();
        let mut hits: Vec<&'static str> = Vec::new();
        for word in BANNED_VOCABULARY {
            if contains_whole_word(&lowered, word) && !hits.contains(&word) {
                hits.push(word);
            }
        }
        for banned in split_scan_hits(&text) {
            if !hits.contains(&banned) {
                hits.push(banned);
            }
        }
        for word in hits {
            let allowed = BANNED_VOCABULARY_ALLOWLIST
                .iter()
                .any(|(file, allowed, _)| *file == relative && *allowed == word);
            if !allowed {
                offenders.push(format!("{relative}: {word}"));
            }
        }
        Ok(())
    };
    visit_files(&root.join("src"), &mut inspect)?;
    inspect(&root.join("README.md"))?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "banned vocabulary present: {}",
            offenders.join(", ")
        ))
    }
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
            if let Some(banned) = spells_banned_word(word) {
                if !hits.contains(&banned) {
                    hits.push(banned);
                }
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

/// The repository-relative path, slash-separated on every platform.
fn relative_slash_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Visits every file under `dir`, skipping [`SKIP_DIRS`].
fn visit_files(
    dir: &Path,
    visit: &mut dyn FnMut(&Path) -> Result<(), String>,
) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|e| format!("{}: {e}", path.display()))?;
        if file_type.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !SKIP_DIRS.contains(&name.as_ref()) {
                visit_files(&path, visit)?;
            }
        } else {
            visit(&path)?;
        }
    }
    Ok(())
}

/// Extracts the double-quoted value of a `key = "value"` line.
fn quoted_value(text: &str, key: &str) -> Result<String, String> {
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
fn bracket_list(text: &str, key: &str) -> Result<Vec<String>, String> {
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

/// The lines inside the README's fenced yaml block.
fn readme_yaml_block(root: &Path) -> Result<Vec<String>, String> {
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

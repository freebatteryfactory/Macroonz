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
    let checks: [Check; 9] = [
        ("agents-claude-parity", check_agents_claude_parity),
        ("lf-and-no-symlinks", check_lf_and_no_symlinks),
        ("no-python", check_no_python),
        ("toolchain-pin-matches-readme", check_toolchain_pin),
        ("workspace-members-match-readme", check_workspace_members),
        ("lint-wall-inherited", check_lint_wall),
        ("band-map-matches-lib", check_band_map),
        ("readme-obligations-join", check_obligations_join),
        ("no-personal-names", check_no_personal_names),
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

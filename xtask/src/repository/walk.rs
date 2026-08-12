//! Walking the tree: where the repository root is, the order its files are
//! visited in, how a path is spelled back, and what one declared module
//! contributes.
//!
//! Every law that judges the tree rather than a text reads it through here, so
//! no two laws can be judging different trees. The named subsystem directories
//! sit here as well: three separate laws name them, so none of those laws owns
//! the name, and it belongs beside the walker they all reach for.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::repository::types::ModuleLayout;

/// Directories never visited by repository-wide file checks.
const SKIP_DIRS: [&str; 2] = [".git", "target"];

/// The metaprogramming subsystem's directory.
///
/// The topology law refuses a dependency path into it, and both scanning laws
/// walk it, so the name answers to no single law and stands here instead.
pub(crate) const TOOLING_DIRECTORY: &str = "macros";

/// The directory the judge lives in, standing here for the same reason: the
/// topology law, both scanning laws, and the reversal inventory all name it.
pub(crate) const JUDGE_DIRECTORY: &str = "testpak";

/// The workspace root: the parent of the xtask crate directory.
pub(crate) fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let parent = manifest_dir
        .parent()
        .ok_or("xtask crate directory has no parent")?;
    Ok(parent.to_path_buf())
}

/// Visits every file under `dir`, skipping [`SKIP_DIRS`].
///
/// Entries are visited in file-name order rather than in the order the
/// filesystem happens to return them, so every check's traversal — and so every
/// diagnostic it emits — is the same on every machine and every run.
pub(crate) fn visit_files(
    dir: &Path,
    visit: &mut dyn FnMut(&Path) -> Result<(), String>,
) -> Result<(), String> {
    let read = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut entries = Vec::new();
    for entry in read {
        let entry = entry.map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = entry.path();
        let is_dir = entry
            .file_type()
            .map_err(|e| format!("{}: {e}", path.display()))?
            .is_dir();
        entries.push((entry.file_name(), path, is_dir));
    }
    entries.sort_by(|(left, _, _), (right, _, _)| left.cmp(right));
    for (name, path, is_dir) in entries {
        if is_dir {
            if !SKIP_DIRS.contains(&name.to_string_lossy().as_ref()) {
                visit_files(&path, visit)?;
            }
        } else {
            visit(&path)?;
        }
    }
    Ok(())
}

/// The repository-relative path, slash-separated on every platform.
pub(crate) fn relative_slash_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// The source text one declared module contributes, and the layout it is in.
///
/// This is the stage that turns a declared NAME into the text an order is read
/// off. `name.rs` is its own text; `name/` is every `.rs` file under it joined
/// together, because a submodule reaching forward is its parent reaching
/// forward. The check and the law that judges the real tree both read a module
/// through here, so neither can be judging a different tree than the other.
pub(crate) fn module_source(src: &Path, name: &str) -> Result<(String, ModuleLayout), String> {
    let flat = src.join(format!("{name}.rs"));
    if flat.is_file() {
        let text = fs::read_to_string(&flat).map_err(|e| format!("{}: {e}", flat.display()))?;
        return Ok((text, ModuleLayout::Flat));
    }
    let directory = src.join(name);
    if !directory.is_dir() {
        return Err(format!(
            "{name} is declared and is neither {} nor {}/",
            flat.display(),
            directory.display()
        ));
    }
    let mut collected = String::new();
    visit_files(&directory, &mut |path| {
        if path.extension().is_some_and(|extension| extension == "rs") {
            let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
            collected.push_str(&text);
            collected.push('\n');
        }
        Ok(())
    })?;
    Ok((collected, ModuleLayout::Directory))
}

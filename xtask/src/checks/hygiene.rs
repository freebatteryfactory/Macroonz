//! What the tree may spell.
//!
//! Three laws that judge files and lines by what they are rather than by what
//! they agree with: LF line endings with no symlinks, no Python anywhere ever,
//! and no underscore-prefixed field carrying real data. They share one shape —
//! walk a tree, collect every offender, name them all in one refusal — because a
//! rule about every file is worth nothing if some corner of the tree is exempt
//! and worth little if it reports only the first hit.

use std::fs;
use std::path::Path;

use crate::repository::walk::{JUDGE_DIRECTORY, TOOLING_DIRECTORY, visit_files};

/// Every file in the repository is LF-only and nothing is a symlink.
pub(crate) fn check_lf_and_no_symlinks(root: &Path) -> Result<(), String> {
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
pub(crate) fn check_no_python(root: &Path) -> Result<(), String> {
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

/// An underscore-prefixed field is lawful only when it is a `PhantomData`
/// type-level law. Real data behind an underscore is the suppressor idiom —
/// "ignore this mess" — and the repository refuses it: the only honest `_`
/// is one with nothing to read.
///
/// The scan covers the machine (`src/`), the metaprogramming subsystem
/// (`macros/`), and the qualification plane (`testpak/`): the tools that project
/// the machine's contracts, and the plane that judges them, are held to the
/// machine's own honesty about what a field carries.
pub(crate) fn check_underscore_fields_are_phantom(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    let mut inspect = |path: &Path| -> Result<(), String> {
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
    };
    visit_files(&root.join("src"), &mut inspect)?;
    visit_files(&root.join(TOOLING_DIRECTORY), &mut inspect)?;
    visit_files(&root.join(JUDGE_DIRECTORY), &mut inspect)?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// Planted reversals for the laws whose subject is a tree rather than a text.
///
/// A fixture string cannot reach a check that reads a directory, so these are
/// planted against a scratch root outside the repository. Nothing is written
/// inside the repository — the laws that guard the tree are never proven by
/// dirtying the tree.
#[cfg(test)]
mod tests {
    use super::{check_lf_and_no_symlinks, check_no_python, check_underscore_fields_are_phantom};
    use crate::checks::scratch::Scratch;
    use std::fs;

    /// Planted reversal: a file carrying CRLF.
    ///
    /// The symlink half of this law is NOT planted. Creating a symlink is a
    /// privileged operation on one of the supported platforms, so a fixture
    /// that planted one would pass or fail on who ran it rather than on the
    /// law. That half stands on the check's own code and on nothing executed
    /// here, and this doc line is where that is admitted rather than implied.
    #[test]
    fn a_crlf_file_is_a_violation() {
        let scratch = Scratch::named("lf-only");
        scratch.write("clean.md", "one line\nanother\n");
        assert!(check_lf_and_no_symlinks(scratch.root()).is_ok());

        scratch.write("drifted.md", "one line\r\nanother\r\n");
        let found = check_lf_and_no_symlinks(scratch.root());
        assert!(found.is_err_and(|reason| reason.contains("CRLF") && reason.contains("drifted")));
    }

    /// Planted reversal: a Python file anywhere in the tree.
    #[test]
    fn a_python_file_is_a_violation() {
        let scratch = Scratch::named("no-python");
        scratch.write("tool.rs", "fn main() {}\n");
        scratch.write("notes/readme.md", "prose\n");
        assert!(check_no_python(scratch.root()).is_ok());

        scratch.write("notes/helper.py", "the file's presence is the offence\n");
        let found = check_no_python(scratch.root());
        assert!(found.is_err_and(|reason| reason.contains("helper.py")));
    }

    /// Planted reversal: real data behind an underscore — the suppressor idiom
    /// this law exists to refuse — planted in each of the three trees the scan
    /// covers, so no tree is scanned in name only.
    #[test]
    fn an_underscore_field_carrying_data_is_a_violation() {
        let scratch = Scratch::named("underscore-fields");
        let lawful = "struct Demo {\n    _law: PhantomData<*const ()>,\n}\n";
        scratch.write("src/lawful.rs", lawful);
        scratch.write("macros/lawful.rs", lawful);
        scratch.write("testpak/lawful.rs", lawful);
        assert!(check_underscore_fields_are_phantom(scratch.root()).is_ok());

        for tree in ["src", "macros", "testpak"] {
            scratch.write(
                &format!("{tree}/smuggled.rs"),
                "struct Demo {\n    _hidden: u64,\n}\n",
            );
            let found = check_underscore_fields_are_phantom(scratch.root());
            assert!(
                found.is_err_and(|reason| reason.contains("smuggled.rs")
                    && reason.contains("underscore field without PhantomData")),
                "{tree} tree is not scanned"
            );
            let _removed = fs::remove_file(scratch.root().join(tree).join("smuggled.rs"));
        }
    }
}

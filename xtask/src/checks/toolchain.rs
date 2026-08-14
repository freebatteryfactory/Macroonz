//! The build the README describes is the build that runs.
//!
//! Three laws join what the repository's configuration files say about the
//! build: the toolchain floor, the workspace members, and the one lint wall
//! every member inherits. A document that is wrong about the build is worse than
//! no document, because a reader trusts it — and so does a resolver, and so does
//! clippy, which is why the floor is read out of every file that states it
//! rather than out of the two that happen to be prose.

use std::fs;
use std::path::Path;

use crate::repository::manifest::{bracket_list, quoted_value};
use crate::repository::readme::readme_yaml_block;

/// Every statement of the toolchain floor names the same version.
///
/// Four files say what this workspace builds on: `rust-toolchain.toml` pins the
/// channel, the root manifest declares `rust-version`, `clippy.toml` tells
/// clippy which MSRV its suggestions must not reach past, and the README tells a
/// reader. Only the pin decides what actually runs, so the other three are
/// claims ABOUT the pin — and a claim about the pin that disagrees with it is
/// worse than no claim, because each of the three is read by something that
/// then behaves differently. The join makes the disagreement unrepresentable
/// rather than merely absent.
pub(crate) fn check_toolchain_pin(root: &Path) -> Result<(), String> {
    let toolchain_text = fs::read_to_string(root.join("rust-toolchain.toml"))
        .map_err(|e| format!("rust-toolchain.toml: {e}"))?;
    let pinned = quoted_value(&toolchain_text, "channel")?;
    let yaml = readme_yaml_block(root)?;
    let declared = yaml
        .iter()
        .find_map(|line| line.strip_prefix("toolchain: "))
        .map(|value| value.trim_matches('"').to_string())
        .ok_or_else(|| String::from("README yaml block has no toolchain line"))?;
    let manifest =
        fs::read_to_string(root.join("Cargo.toml")).map_err(|e| format!("Cargo.toml: {e}"))?;
    let floor = quoted_value(&manifest, "rust-version")?;
    let clippy_text =
        fs::read_to_string(root.join("clippy.toml")).map_err(|e| format!("clippy.toml: {e}"))?;
    let suggested = quoted_value(&clippy_text, "msrv")?;
    let mut disagreements = Vec::new();
    if declared != pinned {
        disagreements.push(format!("README declares {declared}"));
    }
    if floor != pinned {
        disagreements.push(format!("Cargo.toml rust-version is {floor}"));
    }
    if suggested != pinned {
        disagreements.push(format!("clippy.toml msrv is {suggested}"));
    }
    if disagreements.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "rust-toolchain.toml pins {pinned} but {}",
            disagreements.join("; ")
        ))
    }
}

/// The workspace members in `Cargo.toml` match the members the README yaml block
/// declares.
pub(crate) fn check_workspace_members(root: &Path) -> Result<(), String> {
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
pub(crate) fn check_lint_wall(root: &Path) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    use super::{check_lint_wall, check_toolchain_pin, check_workspace_members};
    use crate::checks::scratch::Scratch;

    /// A README carrying the fenced yaml block the joins read.
    const FIXTURE_README: &str = "# Fixture\n\n```yaml\nphase: architecture-closure\n\
                                  toolchain: \"1.97.1\"\nworkspace_members:\n  - one\n  - two\n\
                                  ```\n";

    /// Planted reversal, one per statement of the floor. The pin is what builds
    /// run under; the README is what a reader believes, the manifest floor is
    /// what a downstream resolver believes, and the clippy MSRV is what a lint
    /// suggestion believes. Each drifts on its own, so each is reversed on its
    /// own — and the last case reverses the exact state this join was written
    /// for, where the pin said 1.97.1 while two files said 1.97.
    #[test]
    fn a_floor_that_drifts_from_the_pin_is_a_violation() {
        let scratch = Scratch::named("toolchain-pin");
        scratch.write("README.md", FIXTURE_README);
        scratch.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.97.1\"\n");
        scratch.write(
            "Cargo.toml",
            "[workspace.package]\nrust-version = \"1.97.1\"\n",
        );
        scratch.write("clippy.toml", "msrv = \"1.97.1\"\n");
        assert!(check_toolchain_pin(scratch.root()).is_ok());

        scratch.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.98.0\"\n");
        let drifted_readme = check_toolchain_pin(scratch.root());
        assert!(drifted_readme.is_err_and(
            |reason| reason.contains("1.98.0") && reason.contains("README declares 1.97.1")
        ));

        scratch.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.97.1\"\n");
        scratch.write(
            "Cargo.toml",
            "[workspace.package]\nrust-version = \"1.97\"\n",
        );
        let drifted_floor = check_toolchain_pin(scratch.root());
        assert!(
            drifted_floor.is_err_and(|reason| reason.contains("Cargo.toml rust-version is 1.97"))
        );

        scratch.write(
            "Cargo.toml",
            "[workspace.package]\nrust-version = \"1.97.1\"\n",
        );
        scratch.write("clippy.toml", "msrv = \"1.97\"\n");
        let drifted_suggestions = check_toolchain_pin(scratch.root());
        assert!(
            drifted_suggestions.is_err_and(|reason| reason.contains("clippy.toml msrv is 1.97"))
        );
    }

    /// Planted reversal: a workspace member the README does not declare, in
    /// both directions — a member added to the manifest alone, and one removed
    /// from the manifest while the README still lists it.
    #[test]
    fn a_member_set_that_drifts_from_the_readme_is_a_violation() {
        let scratch = Scratch::named("workspace-members");
        scratch.write("README.md", FIXTURE_README);
        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\", \"two\"]\n");
        assert!(check_workspace_members(scratch.root()).is_ok());

        scratch.write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"one\", \"two\", \"three\"]\n",
        );
        let added = check_workspace_members(scratch.root());
        assert!(added.is_err_and(|reason| reason.contains("three")));

        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\"]\n");
        let removed = check_workspace_members(scratch.root());
        assert!(removed.is_err_and(|reason| reason.contains("two")));
    }

    /// Planted reversal: a member that does not inherit the lint wall, and
    /// separately a root that declares no wall at all. The two are different
    /// failures — one member walking out, and the wall never existing — and the
    /// check names them apart.
    #[test]
    fn a_member_outside_the_lint_wall_is_a_violation() {
        let scratch = Scratch::named("lint-wall");
        let inheriting = "[package]\nname = \"member\"\n\n[lints]\nworkspace = true\n";
        scratch.write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"one\", \"two\"]\n\n[workspace.lints.rust]\n\
             warnings = { level = \"deny\", priority = -1 }\n",
        );
        scratch.write("one/Cargo.toml", inheriting);
        scratch.write("two/Cargo.toml", inheriting);
        assert!(check_lint_wall(scratch.root()).is_ok());

        scratch.write("two/Cargo.toml", "[package]\nname = \"member\"\n");
        let escaped = check_lint_wall(scratch.root());
        assert!(escaped.is_err_and(|reason| reason.contains("two") && !reason.contains("\"one\"")));

        scratch.write("two/Cargo.toml", inheriting);
        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\", \"two\"]\n");
        let wall_free = check_lint_wall(scratch.root());
        assert!(wall_free.is_err_and(|reason| reason.contains("no [workspace.lints.rust] wall")));
    }
}

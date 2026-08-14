//! The build the README describes is the build that runs.
//!
//! Three laws join what the repository's configuration files say about the
//! build: the toolchain floor, the workspace members, and the one lint wall
//! every member inherits. A document that is wrong about the build is worse than
//! no document, because a reader trusts it — and so does a resolver, and so does
//! clippy, which is why the floor is read out of every file that states it
//! rather than out of the two that happen to be prose.
//!
//! Every one of those files is read by the decoder that owns its language. The
//! reader this replaced cut a `key = "value"` line at its first `=` and matched
//! `"[lints]\nworkspace = true"` as a substring, which meant a member could
//! carry those exact bytes inside a comment and inherit nothing — a law
//! answering about characters when its subject was a declaration.

use crate::repository::cargo::{
    Declaration, MANIFEST_FILE, declares_table, declares_yes, string_at, strings_at,
};
use crate::repository::markdown::phase_declaration;
use crate::repository::snapshot::RepositorySnapshot;
use crate::repository::types::CanonicalPath;

/// The file that pins the channel every build runs under.
const TOOLCHAIN_PIN: &str = "rust-toolchain.toml";

/// The file that tells clippy which floor its suggestions must not reach past.
const LINT_CONFIGURATION: &str = "clippy.toml";

/// The document a reader is told the floor by.
const ROOT_README: &str = "README.md";

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
pub(crate) fn check_toolchain_pin(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let pin = snapshot
        .cargo()
        .document(TOOLCHAIN_PIN)
        .taken(TOOLCHAIN_PIN)?;
    let pinned = string_at(pin, &["toolchain", "channel"]).taken("the pinned channel")?;
    let readme = snapshot
        .markdown()
        .document(&CanonicalPath::spelled(ROOT_README))
        .taken(ROOT_README)?;
    let phase = phase_declaration(readme).taken("the README phase declaration")?;
    let manifest = snapshot
        .cargo()
        .document(MANIFEST_FILE)
        .taken(MANIFEST_FILE)?;
    let floor = string_at(manifest, &["workspace", "package", "rust-version"])
        .taken("the workspace rust-version")?;
    let configuration = snapshot
        .cargo()
        .document(LINT_CONFIGURATION)
        .taken(LINT_CONFIGURATION)?;
    let suggested = string_at(configuration, &["msrv"]).taken("the clippy msrv")?;
    let mut disagreements = Vec::new();
    if phase.toolchain() != pinned {
        disagreements.push(format!("README declares {}", phase.toolchain()));
    }
    if *floor != *pinned {
        disagreements.push(format!("{MANIFEST_FILE} rust-version is {floor}"));
    }
    if *suggested != *pinned {
        disagreements.push(format!("{LINT_CONFIGURATION} msrv is {suggested}"));
    }
    if disagreements.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{TOOLCHAIN_PIN} pins {pinned} but {}",
            disagreements.join("; ")
        ))
    }
}

/// The workspace members in the root manifest match the members the README's
/// phase declaration states.
pub(crate) fn check_workspace_members(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let manifest = snapshot
        .cargo()
        .document(MANIFEST_FILE)
        .taken(MANIFEST_FILE)?;
    let actual = strings_at(manifest, &["workspace", "members"]).taken("the workspace members")?;
    let readme = snapshot
        .markdown()
        .document(&CanonicalPath::spelled(ROOT_README))
        .taken(ROOT_README)?;
    let phase = phase_declaration(readme).taken("the README phase declaration")?;
    if actual.as_slice() == phase.members() {
        Ok(())
    } else {
        Err(format!(
            "{MANIFEST_FILE} members {actual:?} but README declares {:?}",
            phase.members()
        ))
    }
}

/// The root manifest declares the one lint wall and every member inherits it.
///
/// Inheritance is a DECLARATION — `[lints] workspace = true` — so it is asked of
/// the decoded document rather than matched as text. A member carrying those
/// bytes inside a comment declares nothing, and used to pass.
pub(crate) fn check_lint_wall(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let manifest = snapshot
        .cargo()
        .document(MANIFEST_FILE)
        .taken(MANIFEST_FILE)?;
    if declares_table(manifest, &["workspace", "lints", "rust"]).known() != Some(&Declaration::Yes)
    {
        return Err(format!(
            "root {MANIFEST_FILE} has no [workspace.lints.rust] wall"
        ));
    }
    let members = strings_at(manifest, &["workspace", "members"]).taken("the workspace members")?;
    let mut missing = Vec::new();
    for member in members {
        let path = format!("{member}/{MANIFEST_FILE}");
        let document = snapshot.cargo().document(&path).taken(&path)?;
        if declares_yes(document, &["lints", "workspace"]).known() != Some(&Declaration::Yes) {
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

    /// A README carrying the fenced data block the joins read.
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
    fn a_floor_that_drifts_from_the_pin_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("toolchain-pin");
        scratch.write("README.md", FIXTURE_README);
        scratch.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.97.1\"\n");
        scratch.write(
            "Cargo.toml",
            "[workspace.package]\nrust-version = \"1.97.1\"\n",
        );
        scratch.write("clippy.toml", "msrv = \"1.97.1\"\n");
        assert!(check_toolchain_pin(&scratch.read()?).is_ok());

        scratch.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.98.0\"\n");
        let drifted_readme = check_toolchain_pin(&scratch.read()?);
        assert!(drifted_readme.is_err_and(
            |reason| reason.contains("1.98.0") && reason.contains("README declares 1.97.1")
        ));

        scratch.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.97.1\"\n");
        scratch.write(
            "Cargo.toml",
            "[workspace.package]\nrust-version = \"1.97\"\n",
        );
        let drifted_floor = check_toolchain_pin(&scratch.read()?);
        assert!(
            drifted_floor.is_err_and(|reason| reason.contains("Cargo.toml rust-version is 1.97"))
        );

        scratch.write(
            "Cargo.toml",
            "[workspace.package]\nrust-version = \"1.97.1\"\n",
        );
        scratch.write("clippy.toml", "msrv = \"1.97\"\n");
        let drifted_suggestions = check_toolchain_pin(&scratch.read()?);
        assert!(
            drifted_suggestions.is_err_and(|reason| reason.contains("clippy.toml msrv is 1.97"))
        );
        Ok(())
    }

    /// Planted reversal: a workspace member the README does not declare, in
    /// both directions — a member added to the manifest alone, and one removed
    /// from the manifest while the README still lists it.
    #[test]
    fn a_member_set_that_drifts_from_the_readme_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("workspace-members");
        scratch.write("README.md", FIXTURE_README);
        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\", \"two\"]\n");
        assert!(check_workspace_members(&scratch.read()?).is_ok());

        scratch.write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"one\", \"two\", \"three\"]\n",
        );
        let added = check_workspace_members(&scratch.read()?);
        assert!(added.is_err_and(|reason| reason.contains("three")));

        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\"]\n");
        let removed = check_workspace_members(&scratch.read()?);
        assert!(removed.is_err_and(|reason| reason.contains("two")));
        Ok(())
    }

    /// Planted reversal: a member that does not inherit the lint wall, and
    /// separately a root that declares no wall at all. The two are different
    /// failures — one member walking out, and the wall never existing — and the
    /// check names them apart.
    #[test]
    fn a_member_outside_the_lint_wall_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("lint-wall");
        let inheriting = "[package]\nname = \"member\"\n\n[lints]\nworkspace = true\n";
        scratch.write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"one\", \"two\"]\n\n[workspace.lints.rust]\n\
             warnings = { level = \"deny\", priority = -1 }\n",
        );
        scratch.write("one/Cargo.toml", inheriting);
        scratch.write("two/Cargo.toml", inheriting);
        assert!(check_lint_wall(&scratch.read()?).is_ok());

        scratch.write("two/Cargo.toml", "[package]\nname = \"member\"\n");
        let escaped = check_lint_wall(&scratch.read()?);
        assert!(escaped.is_err_and(|reason| reason.contains("two") && !reason.contains("\"one\"")));

        scratch.write("two/Cargo.toml", inheriting);
        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\", \"two\"]\n");
        let wall_free = check_lint_wall(&scratch.read()?);
        assert!(wall_free.is_err_and(|reason| reason.contains("no [workspace.lints.rust] wall")));
        Ok(())
    }

    /// Planted reversal: a member carrying the inheritance bytes inside a
    /// COMMENT.
    ///
    /// The substring reader this replaced passed it. The member declares no
    /// `[lints]` table at all, so it inherits nothing and builds outside the one
    /// wall this workspace declares — while a law about that inheritance read
    /// clean, because the characters it was matching were in the file.
    #[test]
    fn inheritance_written_in_a_comment_inherits_nothing() -> Result<(), String> {
        let scratch = Scratch::named("lint-wall-comment");
        scratch.write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"one\"]\n\n[workspace.lints.rust]\n\
             warnings = { level = \"deny\", priority = -1 }\n",
        );
        scratch.write(
            "one/Cargo.toml",
            "[package]\nname = \"member\"\n# [lints]\n# workspace = true\n",
        );
        let found = check_lint_wall(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("one")),
            "a member that declares no inheritance passed on the strength of a comment"
        );
        Ok(())
    }
}

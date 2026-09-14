//! The shipped skill's first complete recipe executes unchanged against an independent four-pair policy expectation.

use crate::scratch::{
    cargo, command_refusal, lock_from_repository, manifest_path, repository_root,
};
use std::path::Path;

pub(super) fn prepare(root: &Path, destination: &Path) -> Result<(), String> {
    let skill = std::fs::read_to_string(root.join("skills/macroonz/SKILL.md"))
        .map_err(|error| error.to_string())?;
    let recipe = first_recipe(&skill)?;
    std::fs::write(destination, consumer(recipe)).map_err(|error| error.to_string())
}

fn consumer(recipe: &str) -> String {
    with_policy(recipe, "matches!(capability, access::Capability::Read)")
}

fn with_policy(recipe: &str, expected: &str) -> String {
    format!(
        r"//! The packaged skill's recipe and a caller-owned policy expectation.
{recipe}
fn main() {{
    for stage in [access::Stage::Draft, access::Stage::Published] {{
        for capability in [access::Capability::Read, access::Capability::Write] {{
            let expected = {expected};
            assert_eq!(access::baked::policy::contains(&stage, &capability), expected);
        }}
    }}
}}
"
    )
}

pub(super) fn observe_adoption(scratch: &Path) -> Result<(), String> {
    let root = repository_root()?;
    let skill = std::fs::read_to_string(root.join("skills/macroonz/SKILL.md"))
        .map_err(|error| error.to_string())?;
    let recipe = first_recipe(&skill)?;
    assert_eq!(recipe.matches("(Draft, Read);").count(), 1usize);
    assert_eq!(recipe.matches("typestate(Stage);").count(), 1usize);
    let source = consumer(recipe);
    let manifest = format!(
        r#"[package]
name = "documented-policy-adopter"
version = "0.0.0"
edition = "2024"
publish = false
build = false

[[bin]]
name = "documented-policy-adopter"
path = "main.rs"

[dependencies]
macroonz = {{ path = "{}", default-features = false }}

[lints.rust]
warnings = "deny"
unsafe_code = "forbid"

[workspace]
"#,
        manifest_path(root)?,
    );
    std::fs::write(scratch.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("main.rs"), &source).map_err(|error| error.to_string())?;
    let locked = lock_from_repository(scratch)?;
    if !locked.status.success() {
        return Err(command_refusal("documented adopter lock", &locked));
    }
    let first = cargo(scratch, &["run", "--locked", "--offline"])?;
    if !first.status.success() {
        return Err(command_refusal("documented adopter execution", &first));
    }
    let wrong = consumer(&recipe.replace("(Draft, Read);", "(Draft, Write);"));
    std::fs::write(scratch.join("main.rs"), wrong).map_err(|error| error.to_string())?;
    let disagreed = cargo(scratch, &["run", "--locked", "--offline"])?;
    if disagreed.status.success()
        || !String::from_utf8_lossy(&disagreed.stderr).contains("assertion `left == right` failed")
    {
        return Err(command_refusal(
            "independent policy disagreement",
            &disagreed,
        ));
    }
    let missing = consumer(&recipe.replace("typestate(Stage);", "typestate(Missing);"));
    std::fs::write(scratch.join("main.rs"), missing).map_err(|error| error.to_string())?;
    let refused = cargo(scratch, &["check", "--locked", "--offline"])?;
    if refused.status.success()
        || !String::from_utf8_lossy(&refused.stderr).contains("names no authored enum `Missing`")
    {
        return Err(command_refusal("documented selection refusal", &refused));
    }
    std::fs::write(scratch.join("main.rs"), source).map_err(|error| error.to_string())?;
    let repaired = cargo(scratch, &["run", "--locked", "--offline"])?;
    if !repaired.status.success() {
        return Err(command_refusal("documented adopter repair", &repaired));
    }
    Ok(())
}

pub(super) fn extension(root: &Path) -> Result<[String; 2], String> {
    let skill = std::fs::read_to_string(root.join("skills/macroonz/SKILL.md"))
        .map_err(|error| error.to_string())?;
    let recipe = first_recipe(&skill)?;
    assert_eq!(recipe.matches("(Draft, Read);").count(), 1usize);
    let changed = recipe.replace("(Draft, Read);", "(Draft, Write);");
    Ok([
        consumer(&changed),
        with_policy(
            &changed,
            "matches!((&stage, &capability), (access::Stage::Draft, access::Capability::Write) | (access::Stage::Published, access::Capability::Read))",
        ),
    ])
}

pub(super) fn first_recipe(skill: &str) -> Result<&str, String> {
    let (_, section) = skill
        .split_once("## Write one generic recipe\n")
        .ok_or_else(|| "the packaged skill has no first-recipe section".to_owned())?;
    let section = if section.starts_with("## ") {
        ""
    } else {
        section
            .split_once("\n## ")
            .map_or(section, |(first, _)| first)
    };
    let (_, rust) = section
        .split_once("```rust\n")
        .ok_or_else(|| "the packaged skill has no first-recipe Rust block".to_owned())?;
    let (recipe, _) = rust
        .split_once("\n```")
        .ok_or_else(|| "the packaged skill's first recipe has no closing fence".to_owned())?;
    if !recipe.trim_start().starts_with("macroonz::recipe!") {
        return Err("the packaged skill's first Rust block is not its recipe".to_owned());
    }
    Ok(recipe)
}

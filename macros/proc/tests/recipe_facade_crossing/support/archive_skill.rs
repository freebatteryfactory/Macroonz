//! The shipped skill's first complete recipe executes unchanged against an independent four-pair policy expectation.

use crate::scratch::command_refusal;
use std::path::Path;
use std::process::Command;

pub(super) fn prepare(root: &Path) -> Result<(), String> {
    let skill = std::fs::read_to_string(root.join("skills/macroonz/SKILL.md"))
        .map_err(|error| error.to_string())?;
    let recipe = first_recipe(&skill)?;
    let consumer = format!(
        r"//! The packaged skill's recipe and a caller-owned policy expectation.
{recipe}
fn main() {{
    for stage in [access::Stage::Draft, access::Stage::Published] {{
        for capability in [access::Capability::Read, access::Capability::Write] {{
            let expected = matches!(capability, access::Capability::Read);
            assert_eq!(access::baked::policy::contains(&stage, &capability), expected);
        }}
    }}
}}
"
    );
    std::fs::write(root.join("archive_skill.rs"), consumer).map_err(|error| error.to_string())
}

pub(super) fn execute(root: &Path, target: &Path) -> Result<(), String> {
    let executable = target.join(format!("archive-skill{}", std::env::consts::EXE_SUFFIX));
    let compiled = Command::new("rustc")
        .args(["+1.98.1", "--edition=2024", "-Dwarnings", "-Funsafe_code"])
        .arg(root.join("archive_skill.rs"))
        .arg("--extern")
        .arg(format!(
            "macroonz={}",
            target.join("debug/libmacroonz.rlib").display()
        ))
        .arg("-L")
        .arg(format!(
            "dependency={}",
            target.join("debug/deps").display()
        ))
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| error.to_string())?;
    if !compiled.status.success() {
        return Err(command_refusal("packaged skill compilation", &compiled));
    }
    let executed = Command::new(executable)
        .output()
        .map_err(|error| error.to_string())?;
    if !executed.status.success() {
        return Err(command_refusal(
            "packaged skill policy expectation",
            &executed,
        ));
    }
    Ok(())
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

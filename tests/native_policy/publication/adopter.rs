use super::example::{executable_path, files, report, run};
use crate::compiler::configure::{bounds, host, root, spelling, target, tool};
use crate::compiler::types::Host;
use crate::presentation_formats::field;
use macroonz::native_process::{self, ProcessLimits};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[test]
fn fresh_adopter_runs_the_command_through_renamed_and_reexported_facade_paths() -> Result<(), String>
{
    let source = root()?;
    let host = host(&source)?;
    let executable = build(&source, &host)?;
    let destination = root()?;
    let workspace = root()?;
    let rustfmt = host
        .rustc
        .parent()
        .ok_or("toolchain directory absent")?
        .join(format!("rustfmt{}", std::env::consts::EXE_SUFFIX));
    let mut configuration = json!({
        "action": "prepare", "value": 42_u64,
        "workspace": spelling(&workspace)?, "destination": spelling(&destination)?,
        "rustc": spelling(&host.rustc)?, "target": host.triple, "environment": host.environment,
        "rustfmt": spelling(&rustfmt)?, "format_configuration": spelling(&super::configure::configuration(&source)?)?,
    });
    let prepared = report(&run(&source, &host, &executable, &configuration)?, 0)?;
    let mut first_files = None;
    for selected_workspace in [&workspace, &root()?] {
        *configuration
            .get_mut("workspace")
            .ok_or("workspace absent")? = json!(spelling(selected_workspace)?);
        for action in ["inspect", "generate", "generate", "check"] {
            *configuration.get_mut("action").ok_or("action absent")? = json!(action);
            let observed = report(&run(&source, &host, &executable, &configuration)?, 0)?;
            assert_eq!(
                field(files(&prepared)?, "/0/canonical_digest")?,
                field(files(&observed)?, "/0/canonical_digest")?
            );
            if let Some(first_files) = &first_files {
                assert_eq!(files(&observed)?, first_files);
            } else {
                first_files = Some(files(&observed)?.clone());
            }
            if action == "generate" {
                read_back(&source, &host, &observed)?;
            }
            if action == "check" {
                assert_eq!(
                    field(&observed, "/record/value/comparison/is_current")?,
                    &json!(true)
                );
            }
        }
    }
    let before = super::destination_fixture::snapshot(&destination)?;
    *configuration.get_mut("value").ok_or("value absent")? = json!(43_u64);
    let stale = report(&run(&source, &host, &executable, &configuration)?, 1)?;
    assert_eq!(
        field(&stale, "/record/value/comparison/issues/0/problem")?,
        &json!("stale")
    );
    assert_eq!(super::destination_fixture::snapshot(&destination)?, before);
    Ok(())
}

fn read_back(source: &Path, host: &Host, observed: &Value) -> Result<(), String> {
    let executable = executable_path(observed)?;
    let request = tool(host, &executable, source, bounds()?)?
        .invocation(Vec::new())
        .map_err(|error| error.to_string())?;
    let output = crate::process::completed(
        native_process::run(&request, None).map_err(|error| error.to_string())?,
    )?;
    assert!(output.status().success());
    assert_eq!(output.stdout().bytes(), b"42\n");
    Ok(())
}

fn build(source: &Path, host: &Host) -> Result<PathBuf, String> {
    let consumer = source.join("consumer");
    let bridge = source.join("bridge");
    let example = consumer.join("examples/publication_workflow");
    let input = consumer.join("examples/support/native_input");
    let configuration = consumer.join("examples/support/publication_configuration");
    for directory in [&bridge, &example, &input, &configuration] {
        std::fs::create_dir_all(directory).map_err(|error| error.to_string())?;
    }
    material(&example, &input, &configuration)?;
    let suffix = source
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("adopter suffix absent")?;
    let name = format!("publication-adopter-{suffix}");
    let repository = serde_json::to_string(&spelling(Path::new(env!("CARGO_MANIFEST_DIR")))?)
        .map_err(|error| error.to_string())?;
    std::fs::write(bridge.join("Cargo.toml"), format!("[package]\nname = \"publication-bridge-{suffix}\"\nversion = \"0.0.0\"\nedition = \"2024\"\nbuild = false\n[lib]\npath = \"lib.rs\"\n[dependencies]\nbakery = {{ package = \"macroonz\", path = {repository}, default-features = false, features = [\"native-tooling\"] }}\n[workspace]\n")).map_err(|error| error.to_string())?;
    std::fs::write(bridge.join("lib.rs"), "#![deny(warnings)]\n#![forbid(unsafe_code)]\npub use bakery::{compiler, harness, native_compiler, native_process, native_publication, presentation};\n").map_err(|error| error.to_string())?;
    std::fs::write(consumer.join("Cargo.toml"), format!("[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\nbuild = false\n[[bin]]\nname = \"{name}\"\npath = \"examples/publication_workflow/main.rs\"\n[lints.rust]\nwarnings = \"deny\"\nunsafe_code = \"forbid\"\n[dependencies]\nfacade_bridge = {{ package = \"publication-bridge-{suffix}\", path = \"../bridge\" }}\nserde_json = {{ version = \"=1.0.151\", default-features = false, features = [\"alloc\"] }}\n[workspace]\n")).map_err(|error| error.to_string())?;
    std::fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
        consumer.join("Cargo.lock"),
    )
    .map_err(|error| error.to_string())?;
    cargo(&consumer, host, &["update", "--workspace", "--offline"])?;
    cargo(
        &consumer,
        host,
        &[
            "build",
            "--locked",
            "--offline",
            "-j1",
            "--bin",
            &name,
            "--target-dir",
            &spelling(&target()?)?,
        ],
    )?;
    Ok(target()?
        .join("debug")
        .join(format!("{name}{}", std::env::consts::EXE_SUFFIX)))
}

fn material(example: &Path, input: &Path, configuration: &Path) -> Result<(), String> {
    for (name, source) in [
        (
            "main.rs",
            include_str!("../../../examples/publication_workflow/main.rs"),
        ),
        (
            "configuration.rs",
            include_str!("../../../examples/publication_workflow/configuration.rs"),
        ),
        (
            "generate.rs",
            include_str!("../../../examples/publication_workflow/generate.rs"),
        ),
        (
            "types.rs",
            include_str!("../../../examples/publication_workflow/types.rs"),
        ),
        (
            "type_contract.rs",
            include_str!("../../../examples/publication_workflow/type_contract.rs"),
        ),
    ] {
        std::fs::write(
            example.join(name),
            source.replace("macroonz::", "facade_bridge::"),
        )
        .map_err(|error| error.to_string())?;
    }
    for (name, source) in [
        (
            "mod.rs",
            include_str!("../../../examples/support/publication_configuration/mod.rs"),
        ),
        (
            "command.rs",
            include_str!("../../../examples/support/publication_configuration/command.rs"),
        ),
        (
            "prepare.rs",
            include_str!("../../../examples/support/publication_configuration/prepare.rs"),
        ),
        (
            "generate.rs",
            include_str!("../../../examples/support/publication_configuration/generate.rs"),
        ),
    ] {
        std::fs::write(
            configuration.join(name),
            source.replace("macroonz::", "facade_bridge::"),
        )
        .map_err(|error| error.to_string())?;
    }
    for (name, source) in [
        (
            "mod.rs",
            include_str!("../../../examples/support/native_input/mod.rs"),
        ),
        (
            "read.rs",
            include_str!("../../../examples/support/native_input/read.rs"),
        ),
        (
            "environment.rs",
            include_str!("../../../examples/support/native_input/environment.rs"),
        ),
    ] {
        std::fs::write(input.join(name), source).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn cargo(source: &Path, host: &Host, arguments: &[&str]) -> Result<(), String> {
    let limits = ProcessLimits::informed(
        Duration::from_secs(180),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    let request = tool(host, &host.cargo, source, limits)?
        .invocation(arguments.iter().map(|value| (*value).to_owned()).collect())
        .map_err(|error| error.to_string())?;
    let output = crate::process::completed(
        native_process::run(&request, None).map_err(|error| error.to_string())?,
    )?;
    if !output.status().success() {
        return Err(format!(
            "{arguments:?}: {}",
            String::from_utf8_lossy(output.stderr().bytes())
        ));
    }
    Ok(())
}

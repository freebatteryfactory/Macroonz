use super::configure::{archives, backend};
use crate::compiler::configure::{host, root, spelling, target, tool};
use macroonz::harness::muterprater::backend_archive::read_backend;
use macroonz::native_process::{self, ProcessLimits};
use macroonz::native_storage::{StorageLimits, StorageName, StorageRoot};
use std::path::Path;
use std::time::Duration;

#[test]
fn public_example_executes_retains_and_compares_an_independent_subject() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    drop(crate::compiler::real::package(
        &root,
        "[lib]\nname = \"mutation_example_fixture\"\npath = \"fixture.rs\"\n[[test]]\nname = \"separate\"\npath = \"separate.rs\"\n",
    )?);
    std::fs::write(
        root.join("fixture.rs"),
        "pub fn double(value: u32) -> u32 { value * 2 }\n",
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(
        root.join("separate.rs"),
        "#[test] fn observes_double() { assert_eq!(mutation_example_fixture::double(3), 6); }\n",
    )
    .map_err(|error| error.to_string())?;
    let storage = root.join("storage");
    std::fs::create_dir(&storage).map_err(|error| error.to_string())?;
    let configuration = serde_json::json!({
        "backend": spelling(&backend()?)?, "cargo": spelling(&host.cargo)?, "rustc": spelling(&host.rustc)?,
        "directory": spelling(&root)?, "sources": ["fixture.rs"], "target": host.triple,
        "output": spelling(&root.join("backend-output"))?, "target_directory": spelling(&root.join("mutation-target"))?,
        "storage": spelling(&storage)?, "batch": "current", "environment": host.environment,
    });
    let input = root.join("configuration.json");
    std::fs::write(
        &input,
        serde_json::to_vec(&configuration).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let limits = ProcessLimits::informed(
        Duration::from_secs(240),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    let selected = tool(
        &host,
        &host.cargo,
        Path::new(env!("CARGO_MANIFEST_DIR")),
        limits,
    )?;
    let mut arguments = [
        "run",
        "--example",
        "mutation_workflow",
        "--features",
        "native-tooling",
        "--locked",
        "--offline",
        "-j1",
        "--target-dir",
    ]
    .map(str::to_owned)
    .to_vec();
    arguments.push(spelling(&target()?)?);
    let request = selected
        .invocation(arguments)
        .map_err(|error| error.to_string())?;
    let output = crate::process::completed(
        native_process::run(
            &request,
            Some(std::fs::File::open(input).map_err(|error| error.to_string())?),
        )
        .map_err(|error| error.to_string())?,
    )?;
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    let text = std::str::from_utf8(output.stdout().bytes()).map_err(|error| error.to_string())?;
    assert!(
        text.contains("0 inconclusive; 1 historical source claims match current files"),
        "{text}"
    );
    retained(&storage)
}

fn retained(storage: &Path) -> Result<(), String> {
    let root = StorageRoot::open(storage).map_err(|error| format!("{error:?}"))?;
    let loaded = root
        .load(
            &StorageName::informed("current").map_err(|error| format!("{error:?}"))?,
            StorageLimits {
                artifacts: 1,
                bytes: 4_194_304,
            },
        )
        .map_err(|error| format!("{error:?}"))?;
    let [artifact] = loaded.as_slice() else {
        return Err("example storage roster differs".to_owned());
    };
    let historical =
        read_backend(&artifact.bytes, archives()).map_err(|error| format!("{error:?}"))?;
    assert!(!historical.run().reports().is_empty());
    assert_eq!(
        historical
            .sources()
            .first()
            .and_then(|source| source.original()),
        Some(b"pub fn double(value: u32) -> u32 { value * 2 }\n".as_slice())
    );
    Ok(())
}

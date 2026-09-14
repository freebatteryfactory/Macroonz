use super::configure::debug;
use crate::compiler::configure::{host, root, spelling, target, tool};
use macroonz::native_process::{self, ProcessLimits};
use std::path::Path;
use std::time::Duration;

#[test]
fn public_example_reports_actual_profile_and_replays_coverage_earned_seeds() -> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    std::fs::write(
        run.join("subject.rs"),
        include_bytes!("../../../examples/support/rustc_coverage_subject.rs"),
    )
    .map_err(debug)?;
    let configuration = serde_json::json!({
        "rustc": spelling(&host.rustc)?, "directory": spelling(&run)?, "source": "subject.rs",
        "artifact": spelling(&run.join(format!("subject{}", std::env::consts::EXE_SUFFIX)))?,
        "scratch": spelling(&run.join("cases"))?, "target": host.triple, "environment": host.environment,
        "candidates": [[0_u8], [1_u8, 2_u8, 3_u8], [0_u8]],
    });
    let input = run.join("configuration.json");
    std::fs::write(&input, serde_json::to_vec(&configuration).map_err(debug)?).map_err(debug)?;
    let limits = ProcessLimits::informed(
        Duration::from_secs(180),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(debug)?;
    let selected = tool(
        &host,
        &host.cargo,
        Path::new(env!("CARGO_MANIFEST_DIR")),
        limits,
    )?;
    let mut arguments = [
        "run",
        "--example",
        "coverage_workflow",
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
    let request = selected.invocation(arguments).map_err(debug)?;
    let output = crate::process::completed(
        native_process::run(&request, Some(std::fs::File::open(input).map_err(debug)?))
            .map_err(debug)?,
    )?;
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    let text = std::str::from_utf8(output.stdout().bytes()).map_err(debug)?;
    let lines = text.lines().collect::<Vec<_>>();
    let [
        compiler,
        target_line,
        versions,
        profdata,
        cov,
        first,
        second,
        repeated,
        replay,
    ] = lines.as_slice()
    else {
        return Err(format!("unexpected profile and candidate report: {text}"));
    };
    assert_eq!(
        *compiler,
        format!("compiler {}: 1.98.1", host.rustc.display())
    );
    assert_eq!(
        *target_line,
        format!("host {}; target {}", host.triple, host.triple)
    );
    assert!(versions.starts_with("compiler LLVM ") && versions.contains("; matching tools "));
    assert!(
        profdata.starts_with("Profdata: ")
            && profdata.ends_with(&format!("llvm-profdata{}", std::env::consts::EXE_SUFFIX))
    );
    assert!(
        cov.starts_with("Cov: ")
            && cov.ends_with(&format!("llvm-cov{}", std::env::consts::EXE_SUFFIX))
    );
    assert_eq!(
        [*first, *second, *repeated, *replay],
        [
            "candidate [0]: interesting",
            "candidate [1, 2, 3]: interesting",
            "candidate [0]: known",
            "replayed 2 retained seeds through fresh target processes"
        ]
    );
    Ok(())
}

//! Execute the public retention example across independent processes and hostile storage inputs.

use crate::compiler::configure::{host, root, spelling, target, tool};
use crate::presentation_formats::field;
use macroonz::native_process::{self, ProcessOutput, ProcessTool};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

#[test]
fn public_retained_example_reduces_loads_and_replays_without_reducing_again() -> Result<(), String>
{
    let run = root()?;
    let selected = executable(&run)?;
    let retained = success(&invoke(&selected, &run, &configuration(&run, "retain")?)?)?;
    assert_eq!(field(&retained, "/kind")?, "input-run");
    assert_eq!(field(&retained, "/record/input/payload")?, "070109");
    assert_eq!(field(&retained, "/record/report/denominator")?, 1usize);
    assert_eq!(
        field(
            &retained,
            "/record/report/census/0/disposition/value/attempt/value/kind"
        )?,
        "refused"
    );

    let loaded = invoke(&selected, &run, &configuration(&run, "inspect")?)?;
    let historical = success(&loaded)?;
    assert_eq!(
        field(&historical, "/standing")?,
        "historical-unauthenticated"
    );
    assert_eq!(field(&historical, "/record/input/payload")?, "070109");
    assert_eq!(field(&historical, "/record/capsules/0/row")?, 0usize);
    assert_eq!(
        field(&historical, "/record/capsules/0/capsule/input")?,
        "01"
    );
    assert!(text(&loaded)?.ends_with("loaded historical records; decodes=0 checks=0 probes=0\n"));

    for (action, expected, movement, summary) in [
        ("replay", "defect-reproduced", "same", "DefectReproduced"),
        (
            "replay-fixed",
            "fixed-on-witness",
            "moved",
            "FixedOnWitness",
        ),
    ] {
        let observed = invoke(&selected, &run, &configuration(&run, action)?)?;
        let replayed = success(&observed)?;
        assert_eq!(field(&replayed, "/record/outcome/kind")?, expected);
        assert_eq!(field(&replayed, "/record/movement/subject")?, movement);
        assert_eq!(field(&replayed, "/record/movement/check")?, "same");
        assert_eq!(field(&replayed, "/record/lineage")?, "reached-witness");
        assert!(text(&observed)?.ends_with(&format!("{summary}; decodes=1 checks=1 probes=0\n")));
    }
    hostile(&selected, &run)
}

fn hostile(selected: &ProcessTool, run: &Path) -> Result<(), String> {
    let original = std::fs::read(run.join("item-count/item-input")).map_err(debug)?;
    refused(
        &invoke(selected, run, &configuration(run, "retain")?)?,
        "fresh batch",
    )?;
    assert_eq!(
        std::fs::read(run.join("item-count/item-input")).map_err(debug)?,
        original
    );
    for (request, repair) in [
        (
            json!({"action":"inspect", "storage":spelling(run)?, "batch":"../escape"}),
            "portable batch name",
        ),
        (
            json!({"action":"inspect", "storage":"relative", "batch":"count"}),
            "absolute path",
        ),
        (
            json!({"action":"unknown", "storage":spelling(run)?, "batch":"count"}),
            "action must be",
        ),
    ] {
        refused(&invoke(selected, run, &request)?, repair)?;
    }
    std::fs::write(run.join("item-count/item-input"), b"not an input archive").map_err(debug)?;
    refused(
        &invoke(selected, run, &configuration(run, "inspect")?)?,
        "original input convention",
    )?;
    Ok(())
}

fn executable(run: &Path) -> Result<ProcessTool, String> {
    let host = host(run)?;
    let limits = crate::process::limits(
        Duration::from_secs(180),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )?;
    let cargo = tool(
        &host,
        &host.cargo,
        Path::new(env!("CARGO_MANIFEST_DIR")),
        limits,
    )?;
    let output_root = target()?;
    let mut arguments = [
        "build",
        "--example",
        "retained_workflow",
        "--no-default-features",
        "--features",
        "native-tooling",
        "--locked",
        "--offline",
        "-j1",
        "--target-dir",
    ]
    .map(str::to_owned)
    .to_vec();
    arguments.push(spelling(&output_root)?);
    let build = crate::process::completed(
        native_process::run(&cargo.invocation(arguments).map_err(debug)?, None).map_err(debug)?,
    )?;
    if !build.status().success() {
        return Err(String::from_utf8_lossy(build.stderr().bytes()).into_owned());
    }
    let executable = output_root
        .join("debug/examples")
        .join(format!("retained_workflow{}", std::env::consts::EXE_SUFFIX));
    ProcessTool::informed(
        executable,
        run.to_path_buf(),
        Vec::new(),
        crate::process::limits(
            Duration::from_secs(10),
            Duration::from_secs(5),
            1_048_576,
            65_536,
        )?,
        &[],
    )
    .map_err(debug)
}

fn configuration(run: &Path, action: &str) -> Result<Value, String> {
    Ok(json!({"action":action, "storage":spelling(run)?, "batch":"count"}))
}

fn invoke(
    selected: &ProcessTool,
    run: &Path,
    configuration: &Value,
) -> Result<ProcessOutput, String> {
    let input = run.join("configuration.json");
    std::fs::write(&input, serde_json::to_vec(configuration).map_err(debug)?).map_err(debug)?;
    crate::process::completed(
        native_process::run(
            &selected.invocation(Vec::new()).map_err(debug)?,
            Some(std::fs::File::open(input).map_err(debug)?),
        )
        .map_err(debug)?,
    )
}

fn success(output: &ProcessOutput) -> Result<Value, String> {
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    let mut lines = text(output)?.lines();
    let value = serde_json::from_str(lines.next().ok_or("missing record")?).map_err(debug)?;
    assert!(lines.next().is_some());
    assert!(lines.next().is_none());
    Ok(value)
}

fn refused(output: &ProcessOutput, repair: &str) -> Result<(), String> {
    assert!(!output.status().success(), "unexpected example success");
    assert!(text(output)?.is_empty());
    assert!(
        String::from_utf8_lossy(output.stderr().bytes()).contains(repair),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    Ok(())
}

fn text(output: &ProcessOutput) -> Result<&str, String> {
    std::str::from_utf8(output.stdout().bytes()).map_err(debug)
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

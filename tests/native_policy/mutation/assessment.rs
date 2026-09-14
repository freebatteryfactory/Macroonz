//! Public assessment across real compiled executions, independent witnesses and a fresh historical reader.

use crate::compiler::configure::{host, root, spelling, target, tool};
use crate::compiler::types::Host;
use crate::presentation_formats::field;
use macroonz::native_process::{self, ProcessOutput, ProcessTool};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

#[test]
fn public_assessment_keeps_real_survival_catch_and_historical_loading_separate()
-> Result<(), String> {
    let run = root()?;
    let host = host(&run)?;
    let caller = caller(&host)?;
    let work = run.join("subject");
    let storage = run.join("records");
    std::fs::create_dir(&work).map_err(debug)?;
    std::fs::create_dir(&storage).map_err(debug)?;
    let configuration = configuration(&host, &work, &storage)?;
    let output = invoke(&caller, &run, &configuration)?;
    let records = records(&output, 3)?;
    for (record, verdict) in records.iter().zip(["survived", "killed"]) {
        assert_eq!(field(record, "/standing")?, "historical-unauthenticated");
        assert_eq!(field(record, "/record/difference")?, "differs");
        assert_eq!(field(record, "/record/input/bytes")?, "00000007");
        assert_eq!(field(record, "/record/mutation/outcome/kind")?, verdict);
        let roads = field(record, "/record/roads")?
            .as_array()
            .ok_or("roads missing")?;
        assert_eq!(roads.len(), 5);
        for (road, expected) in roads
            .iter()
            .zip(["00000001", "00000001", "00000001", "00000000", "00000000"])
        {
            assert_eq!(field(road, "/meaning/bytes")?, expected);
            assert_eq!(field(road, "/report/attempt/kind")?, "executed");
            let conclusion = if verdict == "killed" && expected == "00000000" {
                "refused"
            } else {
                "passed"
            };
            assert_eq!(field(road, "/report/attempt/value/kind")?, conclusion);
        }
    }
    assert!(text(&output)?.ends_with(
        "weak=Survived strong=Killed equivalence=Refuted; evaluations=2 compiled-executions=2 admissions=0\n"
    ));
    proposal(
        records.last().ok_or("proposal missing")?,
        records.get(1).ok_or("strong assessment missing")?,
    )?;
    actual_programs(&host, &work)?;
    let original = std::fs::read(storage.join("item-assessment/item-weak")).map_err(debug)?;
    refused(&invoke(&caller, &run, &configuration)?)?;
    assert_eq!(
        std::fs::read(storage.join("item-assessment/item-weak")).map_err(debug)?,
        original
    );

    std::fs::write(work.join("baseline/predicate.rs"), b"this is not Rust").map_err(debug)?;
    std::fs::write(
        work.join("selected/predicate.rs"),
        b"this is not Rust either",
    )
    .map_err(debug)?;
    let inspection = json!({"action":"inspect", "storage":spelling(&storage)?});
    let loaded = invoke(&caller, &run, &inspection)?;
    assert_eq!(records, self::records(&loaded, 3)?);
    assert!(text(&loaded)?.ends_with(
        "loaded historical assessments and proposal; evaluations=0 compiled-executions=0 witnesses=0 admissions=0\n"
    ));
    replay(&caller, &host, &run, &storage)?;
    hostile(&caller, &host, &run, &storage)
}

fn proposal(record: &Value, strong: &Value) -> Result<(), String> {
    assert_eq!(field(record, "/standing")?, "historical-unauthenticated");
    assert_eq!(
        field(record, "/record/candidate/synthesis/kind")?,
        "survivor"
    );
    assert_eq!(field(record, "/record/ground/kind")?, "mutant-killed");
    let ground = field(record, "/record/ground/value")?;
    assert_eq!(field(ground, "/report/denominator")?, 2u64);
    assert_eq!(
        field(ground, "/trial_report/attempt/value/kind")?,
        "refused"
    );
    let witness = field(ground, "/capsule/input")?
        .as_str()
        .ok_or("witness absent")?;
    let source = field(strong, "/record/selected_content/identity")?
        .as_str()
        .ok_or("selected source missing")?;
    assert_eq!(witness, format!("{source}0000000700000000"));
    Ok(())
}

fn replay(caller: &ProcessTool, host: &Host, run: &Path, storage: &Path) -> Result<(), String> {
    for (action, conclusion, outcome, check) in [
        ("replay", "refused", "defect-reproduced", "same"),
        ("replay-weakened", "passed", "not-reproduced", "moved"),
    ] {
        let output = invoke(
            caller,
            run,
            &json!({
                "action":action, "storage":spelling(storage)?, "target":host.triple,
                "toolchain":"rustc 1.98.1",
            }),
        )?;
        let records = records(&output, 2)?;
        let [trial, comparison] = records.as_slice() else {
            return Err("replay readings absent".to_owned());
        };
        assert_eq!(field(trial, "/record/attempt/kind")?, "executed");
        assert_eq!(field(trial, "/record/attempt/value/kind")?, conclusion);
        assert_eq!(field(comparison, "/record/outcome/kind")?, outcome);
        assert_eq!(field(comparison, "/record/movement/check")?, check);
        assert_eq!(field(comparison, "/record/movement/subject")?, "same");
        if action == "replay-weakened" {
            assert_eq!(
                field(comparison, "/record/outcome/value")?,
                "passed-without-repair-standing"
            );
        }
        assert!(
            text(&output)?.ends_with("decodes=1 checks=1 compiled-executions=0 admissions=0\n")
        );
    }
    let proposal_path = storage.join("item-assessment/item-proposal");
    let original = std::fs::read(&proposal_path).map_err(debug)?;
    std::fs::write(&proposal_path, b"corrupted proposal").map_err(debug)?;
    refused(&invoke(
        caller,
        run,
        &json!({
            "action":"replay", "storage":spelling(storage)?, "target":host.triple,
            "toolchain":"rustc 1.98.1",
        }),
    )?)?;
    std::fs::write(&proposal_path, original).map_err(debug)?;
    Ok(())
}

fn hostile(caller: &ProcessTool, host: &Host, run: &Path, storage: &Path) -> Result<(), String> {
    for (case, key, value) in [
        ("zero", "sample", json!(0u32)),
        (
            "missing-tool",
            "rustc",
            json!(spelling(&run.join("absent-rustc"))?),
        ),
        ("relative-storage", "storage", json!("relative")),
    ] {
        let work = run.join(case);
        std::fs::create_dir(&work).map_err(debug)?;
        let mut request = configuration(host, &work, storage)?;
        request
            .as_object_mut()
            .ok_or("request object")?
            .insert(key.to_owned(), value);
        refused(&invoke(caller, run, &request)?)?;
    }
    std::fs::write(
        storage.join("item-assessment/item-weak"),
        b"corrupted historical record",
    )
    .map_err(debug)?;
    refused(&invoke(
        caller,
        run,
        &json!({"action":"inspect", "storage":spelling(storage)?}),
    )?)?;
    Ok(())
}

fn actual_programs(host: &Host, directory: &Path) -> Result<(), String> {
    let input = directory.join("outside-input.txt");
    std::fs::write(&input, b"0\n").map_err(debug)?;
    for (role, expected) in [("baseline", b"0\n"), ("selected", b"1\n")] {
        let executable = directory
            .join(role)
            .join(format!("predicate{}", std::env::consts::EXE_SUFFIX));
        let request = tool(
            host,
            &executable,
            directory,
            crate::compiler::configure::bounds()?,
        )?
        .invocation(Vec::new())
        .map_err(debug)?;
        let output = crate::process::completed(
            native_process::run(&request, Some(std::fs::File::open(&input).map_err(debug)?))
                .map_err(debug)?,
        )?;
        assert!(output.status().success());
        assert_eq!(output.stdout().bytes(), expected);
    }
    Ok(())
}

fn caller(host: &Host) -> Result<ProcessTool, String> {
    tool(
        host,
        &host.cargo,
        Path::new(env!("CARGO_MANIFEST_DIR")),
        crate::process::limits(
            Duration::from_secs(180),
            Duration::from_secs(5),
            2_097_152,
            1_048_576,
        )?,
    )
}

fn configuration(host: &Host, work: &Path, storage: &Path) -> Result<Value, String> {
    Ok(json!({
        "action":"assess", "rustc":spelling(&host.rustc)?, "directory":spelling(work)?,
        "target":host.triple, "toolchain":"rustc 1.98.1", "environment":host.environment,
        "storage":spelling(storage)?, "sample":7,
    }))
}

fn invoke(
    caller: &ProcessTool,
    run: &Path,
    configuration: &Value,
) -> Result<ProcessOutput, String> {
    let input = run.join("request.json");
    std::fs::write(&input, serde_json::to_vec(configuration).map_err(debug)?).map_err(debug)?;
    let mut arguments = [
        "run",
        "--quiet",
        "--example",
        "mutation_assessment",
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
    arguments.push(spelling(&target()?)?);
    crate::process::completed(
        native_process::run(
            &caller.invocation(arguments).map_err(debug)?,
            Some(std::fs::File::open(input).map_err(debug)?),
        )
        .map_err(debug)?,
    )
}

fn records(output: &ProcessOutput, count: usize) -> Result<Vec<Value>, String> {
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    let mut lines = text(output)?.lines();
    let records = lines
        .by_ref()
        .take(count)
        .map(|line| serde_json::from_str(line).map_err(debug))
        .collect::<Result<Vec<Value>, String>>()?;
    assert_eq!(records.len(), count);
    assert!(lines.next().is_some());
    assert!(lines.next().is_none());
    Ok(records)
}

fn refused(output: &ProcessOutput) -> Result<(), String> {
    assert!(!output.status().success());
    assert!(text(output)?.is_empty());
    assert!(!output.stderr().bytes().is_empty());
    Ok(())
}

fn text(output: &ProcessOutput) -> Result<&str, String> {
    std::str::from_utf8(output.stdout().bytes()).map_err(debug)
}
fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

//! The public Job caller executes its checks and benchmark while refusing absent suites and undistinguished work.

use std::path::Path;

#[test]
fn public_job_example_executes_complete_and_selected_tables() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = crate::check::scratch()?;
    let output = crate::check::cargo(
        root,
        root,
        &scratch,
        "job-example",
        &[
            "run",
            "--example",
            "job_workflow",
            "--no-default-features",
            "--features",
            "harness",
            "--locked",
            "--offline",
            "-j1",
        ],
    )?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::str::from_utf8(&output.stdout).map_err(|error| error.to_string())?;
    assert_eq!(text.lines().count(), 11);
    for expected in [
        "all: Ok(EveryTrialConcluded { selected: 2, denominator: 2 })",
        "codec: Ok(EveryTrialConcluded { selected: 1, denominator: 2 })",
        "required-lane control: Err(\"required declared trials were not selected\")",
        "absent: Some(Unknown { position: 0 })",
        "benchmark: Ok(())",
        "same-work control: Some(PlantedWorseNotDistinguished)",
    ] {
        assert!(
            text.lines().any(|line| line == expected),
            "missing {expected}"
        );
    }
    declaration(&document(text, "declaration: ")?)?;
    census(&document(text, "all report: ")?, &["selected", "selected"])?;
    census(
        &document(text, "codec report: ")?,
        &["not-selected", "selected"],
    )?;
    benchmark(
        &document(text, "benchmark report: ")?,
        &[16, 64, 256],
        "qualified",
    )?;
    benchmark(
        &document(text, "same-work report: ")?,
        &[8, 16, 32],
        "planted-worse-not-distinguished",
    )?;
    Ok(())
}

fn document(text: &str, prefix: &str) -> Result<serde_json::Value, String> {
    let mut matching = text.lines().filter_map(|line| line.strip_prefix(prefix));
    let source = matching.next().ok_or_else(|| format!("missing {prefix}"))?;
    assert!(matching.next().is_none(), "duplicate {prefix}");
    serde_json::from_str(source).map_err(|error| error.to_string())
}

fn declaration(document: &serde_json::Value) -> Result<(), String> {
    assert_eq!(
        document["declaration"],
        include_str!("../../../examples/job_workflow/declaration.rs")
    );
    assert_eq!(
        document["independent_callables"],
        include_str!("../../../examples/job_workflow/checks.rs")
    );
    assert_eq!(
        document["independent_vectors"],
        include_str!("../../../examples/job_workflow/ordinary.rs")
    );
    assert_eq!(
        document["benchmark_work"],
        include_str!("../../../examples/job_workflow/benchmark/work.rs")
    );
    assert_eq!(
        document["benchmark_judge"],
        include_str!("../../../examples/job_workflow/benchmark/judge.rs")
    );
    assert_eq!(
        document["benchmark_expectations"],
        include_str!("../../../examples/job_workflow/benchmark/read.rs")
    );
    let generated = document["compiler_output"]
        .as_str()
        .ok_or("missing compiler output")?;
    for symbol in [
        "STAGE_VARIANTS",
        "EVENT_VARIANTS",
        "apply",
        "encode_canonical",
        "decode_canonical",
        "RecordDecodeError",
        "job_support",
        "job_bench_support",
    ] {
        assert!(generated.contains(symbol), "missing generated {symbol}");
    }
    assert!(!generated.contains("bake !"));
    Ok(())
}

fn census(document: &serde_json::Value, dispositions: &[&str]) -> Result<(), String> {
    assert_eq!(document["kind"], "run");
    assert_eq!(
        document
            .pointer("/record/denominator")
            .and_then(serde_json::Value::as_u64),
        Some(2)
    );
    let rows = document
        .pointer("/record/census")
        .and_then(serde_json::Value::as_array)
        .ok_or("missing complete census")?;
    assert_eq!(rows.len(), dispositions.len());
    for (row, expected) in rows.iter().zip(dispositions) {
        assert_eq!(
            row.pointer("/disposition/kind")
                .and_then(serde_json::Value::as_str),
            Some(*expected)
        );
    }
    Ok(())
}

fn benchmark(document: &serde_json::Value, control: &[u64], stage: &str) -> Result<(), String> {
    assert_eq!(document["kind"], "benchmark");
    assert_eq!(
        document
            .pointer("/record/denominator")
            .and_then(serde_json::Value::as_u64),
        Some(1)
    );
    let rows = document
        .pointer("/record/readings")
        .and_then(serde_json::Value::as_array)
        .ok_or("missing benchmark readings")?;
    assert_eq!(rows.len(), 1);
    let reading = rows.first().ok_or("missing benchmark reading")?;
    let outcome = &reading["outcome"];
    assert_eq!(outcome["stage"], stage);
    for (name, expected) in [
        ("measured", [8, 16, 32].as_slice()),
        ("planted_worse", control),
    ] {
        let curve = outcome[name].as_array().ok_or("missing work curve")?;
        assert_eq!(curve.len(), 3);
        for ((point, count), size) in curve.iter().zip(expected).zip([2_u64, 4, 8]) {
            assert_eq!(point["input_size"], size);
            let counts = point["counts"].as_array().ok_or("missing work counts")?;
            assert_eq!(counts.len(), 1);
            assert_eq!(counts.first().ok_or("missing work count")?["count"], *count);
        }
    }
    Ok(())
}

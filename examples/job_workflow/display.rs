//! Display the authored Job, its callable compiler output and the separately executed records.

use crate::debug;
use macroonz::compiler::recipe::HarnessPosture;
use macroonz::compiler::{CrateBinding, Door, Producer, TextCapture};
use macroonz::harness::report::RunReport;
use std::io::Write;

const DOOR: Door = Door::declared(
    "job-display",
    "job-display.recipe",
    "job-display::bake",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "job-display",
        name: "compiler",
    },
);

pub(super) fn declaration() -> Result<(), String> {
    let source = include_str!("declaration.rs");
    let (_, invocation) = source
        .split_once("macroonz::recipe!")
        .ok_or("missing the Job recipe invocation")?;
    let module = invocation
        .trim()
        .strip_prefix('{')
        .and_then(|body| body.strip_suffix('}'))
        .ok_or("the Job declaration must end with its recipe invocation")?;
    let capture = TextCapture::read(module).map_err(debug)?;
    let baked = macroonz::compiler::recipe::bake(capture.input(), HarnessPosture::Available, &DOOR)
        .map_err(|error| error.summary().to_owned())?;
    let output = baked.emit();
    let tokens = output.tokens().ok_or("the Job recipe emitted no tokens")?;
    let document = serde_json::json!({
        "declaration": source,
        "compiler_output": tokens.inspected(),
        "compiler_door": "job-display.recipe",
        "independent_callables": include_str!("checks.rs"),
        "independent_vectors": include_str!("ordinary.rs"),
        "benchmark_work": include_str!("benchmark/work.rs"),
        "benchmark_judge": include_str!("benchmark/judge.rs"),
        "benchmark_expectations": include_str!("benchmark/read.rs"),
    });
    writeln!(std::io::stdout(), "declaration: {document}").map_err(debug)
}

pub(super) fn run(label: &str, report: &RunReport) -> Result<(), String> {
    writeln!(
        std::io::stdout(),
        "{label} report: {}",
        macroonz::presentation::run(report).json()
    )
    .map_err(debug)
}

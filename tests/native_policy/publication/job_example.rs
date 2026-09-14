//! A shared stamp and its adopted values are independently checked against actual installed files.

use super::destination_fixture::snapshot;
use super::example;
use crate::compiler::configure::{bounds, finish, host, locus, root, spelling, tool};
use crate::compiler::job_example::invoke;
use crate::compiler::types::Host;
use crate::presentation_formats::field;
use macroonz::harness::oracle::{CompilationVerdict, DeclaredCompilation};
use macroonz::native_compiler::{self, CompilerRequest};
use macroonz::native_process::{ProcessLimits, ProcessOutput, ProcessTool};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

#[test]
fn job_publication_keeps_stale_site_and_independent_tampered_value_controls() -> Result<(), String>
{
    let source = root()?;
    let host = host(&source)?;
    let executable = example::build(&source, &host, "job_publication")?;
    let work = root()?;
    let destination = root()?;
    let fixture = include_bytes!("../../../examples/job_publication/fixture.rs");
    std::fs::write(destination.join("fixture.rs"), fixture).map_err(debug)?;
    let caller = tool(
        &host,
        &executable,
        &source,
        ProcessLimits::informed(
            Duration::from_secs(360),
            Duration::from_secs(5),
            1_048_576,
            1_048_576,
        )
        .map_err(debug)?,
    )?;
    let mut configuration = json!({
        "action":"generate", "values":[7_u8,9_u8],
        "workspace":spelling(&work)?, "destination":spelling(&destination)?,
        "rustc":spelling(&host.rustc)?, "target":host.triple, "environment":host.environment,
    });
    let generated = example::report(&invoke(&caller, &source, "generated", &configuration)?, 0)?;
    let files = example::files(&generated)?
        .as_array()
        .ok_or("file roster absent")?;
    let paths = files
        .iter()
        .map(|file| field(file, "/path"))
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(
        paths,
        vec![
            &json!("definition.rs"),
            &json!("first.rs"),
            &json!("second.rs")
        ]
    );
    assert_eq!(
        std::fs::read(destination.join("fixture.rs")).map_err(debug)?,
        fixture
    );
    let installed = snapshot(&destination)?;
    *configuration.get_mut("action").ok_or("action absent")? = json!("check");
    let current = example::report(&invoke(&caller, &source, "current", &configuration)?, 0)?;
    assert_eq!(
        field(&current, "/record/value/comparison/is_current")?,
        true
    );
    assert_eq!(snapshot(&destination)?, installed);
    let lawful = read_back(&host, &destination, &source, "lawful")?;
    assert!(
        lawful.status().success(),
        "{}",
        String::from_utf8_lossy(lawful.stderr().bytes())
    );
    assert!(lawful.stdout().bytes().is_empty());
    *configuration.get_mut("values").ok_or("values absent")? = json!([7_u8, 10_u8]);
    let stale = example::report(&invoke(&caller, &source, "stale", &configuration)?, 1)?;
    discrepancies(&stale, &[("second.rs", "stale")])?;
    assert_eq!(snapshot(&destination)?, installed);
    *configuration.get_mut("values").ok_or("values absent")? = json!([7_u8, 9_u8]);
    damaged(&host, &destination, &source, &caller, configuration)
}

fn damaged(
    host: &Host,
    destination: &Path,
    source: &Path,
    caller: &ProcessTool,
    mut configuration: Value,
) -> Result<(), String> {
    let first = std::fs::read(destination.join("first.rs")).map_err(debug)?;
    std::fs::write(
        destination.join("first.rs"),
        b"crate::declared_value!(8);\n",
    )
    .map_err(debug)?;
    let damaged = snapshot(destination)?;
    let tampered = example::report(&invoke(caller, source, "tampered", &configuration)?, 1)?;
    discrepancies(
        &tampered,
        &[("first.rs", "tampered"), ("first.rs", "stale")],
    )?;
    assert_eq!(snapshot(destination)?, damaged);
    let failed = read_back(host, destination, source, "damaged")?;
    assert!(!failed.status().success());
    let diagnostic = String::from_utf8_lossy(failed.stderr().bytes());
    assert!(
        diagnostic.contains("left: 8") && diagnostic.contains("right: 7"),
        "{diagnostic}"
    );
    *configuration.get_mut("action").ok_or("action absent")? = json!("generate");
    let refused = example::report(
        &invoke(caller, source, "preserved-damage", &configuration)?,
        1,
    )?;
    assert_eq!(field(&refused, "/record/cause/kind")?, "destination");
    assert_eq!(snapshot(destination)?, damaged);
    std::fs::write(destination.join("first.rs"), first).map_err(debug)?;
    *configuration.get_mut("action").ok_or("action absent")? = json!("check");
    example::report(&invoke(caller, source, "restored", &configuration)?, 0)?;
    Ok(())
}

fn discrepancies(record: &Value, expected: &[(&str, &str)]) -> Result<(), String> {
    let issues = field(record, "/record/value/comparison/issues")?
        .as_array()
        .ok_or("issues absent")?;
    assert_eq!(issues.len(), expected.len());
    for (issue, (path, problem)) in issues.iter().zip(expected) {
        assert_eq!(field(issue, "/path")?, path);
        assert_eq!(field(issue, "/problem")?, problem);
    }
    Ok(())
}

fn read_back(
    host: &Host,
    destination: &Path,
    output: &Path,
    name: &str,
) -> Result<ProcessOutput, String> {
    let artifact = output.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let request = CompilerRequest::rustc(
        &tool(host, &host.rustc, destination, bounds()?)?,
        locus()?,
        artifact.clone(),
        &host.triple,
    )
    .map_err(debug)?;
    let compiled = finish(native_compiler::compile(&request).map_err(debug)?)?;
    assert_eq!(
        compiled.compared(&DeclaredCompilation::compiles()),
        Ok(CompilationVerdict::Conforms)
    );
    let reader = tool(host, &artifact, destination, bounds()?)?
        .invocation(Vec::new())
        .map_err(debug)?;
    let observed = crate::process::completed(compiled.read_back(&reader, None).map_err(debug)?)?;
    std::fs::write(
        output.join(format!("{name}-read.stderr.log")),
        observed.stderr().bytes(),
    )
    .map_err(debug)?;
    std::fs::write(
        output.join(format!("{name}-read.stdout.log")),
        observed.stdout().bytes(),
    )
    .map_err(debug)?;
    Ok(observed)
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

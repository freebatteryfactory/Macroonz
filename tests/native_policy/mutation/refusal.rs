use super::configure::{finish, request};
use crate::compiler::{
    configure::{bounds, host, root},
    refusal::standin,
};
use macroonz::native_mutation::{self, MutationError, MutationObservationError, MutationPhase};
use macroonz::native_process::{ProcessLimits, ProcessStop};
use std::time::Duration;

const CONSOLE: &str =
    "Found 1 mutants\nok Unmutated baseline\ncaught fixture.rs:1:1: replace step -> u32 with 0\n";

pub(super) fn subject(body: &str) -> String {
    format!(
        "use std::io::Write;\nfn main() -> Result<(), Box<dyn std::error::Error>> {{ if std::env::args().any(|argument| argument == \"--version\") {{ std::io::stdout().write_all(b\"cargo-mutants 27.0.0\\n\")?; return Ok(()); }} {body} Ok(()) }}\n"
    )
}

#[test]
fn console_roster_source_movement_and_infrastructure_failures_refuse() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let bodies = [
        (
            format!(
                "std::io::stderr().write_all({:?}.as_bytes())?;",
                CONSOLE.replace("Found 1", "Found 2")
            ),
            "roster",
        ),
        (
            "std::io::stderr().write_all(&[255])?;".to_owned(),
            "console",
        ),
        (
            format!(
                "std::io::stderr().write_all({CONSOLE:?}.as_bytes())?; std::fs::write(\"fixture.rs\", b\"changed\")?;"
            ),
            "source",
        ),
        (
            "std::io::stderr().write_all(&vec![b'x'; 131072])?;".to_owned(),
            "limit",
        ),
        (
            "std::thread::sleep(std::time::Duration::from_secs(10));".to_owned(),
            "deadline",
        ),
    ];
    for (at, (body, expected)) in bodies.iter().enumerate() {
        std::fs::write(root.join("fixture.rs"), b"original").map_err(|error| error.to_string())?;
        let backend = standin(&root, &host, &format!("backend-{at}"), &subject(body))?;
        let limits = ProcessLimits::informed(
            Duration::from_millis(750),
            Duration::from_secs(3),
            4096,
            4096,
        )
        .map_err(|error| error.to_string())?;
        let output = finish(
            native_mutation::run(
                request(&host, &root, &backend, limits, &format!("output-{at}"))?,
                |_| None,
                |_, _| None,
            )
            .map_err(|error| error.to_string())?,
        )?;
        let error = output.manifest().err().ok_or("hostile output admitted")?;
        let matched = match *expected {
            "roster" => error == &MutationObservationError::Roster,
            "console" => matches!(error, MutationObservationError::Console(_)),
            "source" => matches!(error, MutationObservationError::Sources(_)),
            "limit" => {
                error == &MutationObservationError::Process
                    && output.process().stop() == &ProcessStop::OutputLimit
            }
            "deadline" => {
                error == &MutationObservationError::Process
                    && output.process().stop() == &ProcessStop::Deadline
            }
            _ => false,
        };
        assert!(matched, "{expected}: {error:?}");
        super::presentation::observation_failure(
            &output,
            match *expected {
                "source" => "sources",
                "limit" | "deadline" => "process",
                other => other,
            },
        )?;
    }
    Ok(())
}

#[test]
fn unsupported_backend_version_refuses_before_source_or_mutation_execution() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let backend = standin(
        &root,
        &host,
        "unsupported",
        "use std::io::Write; fn main() -> std::io::Result<()> { std::io::stdout().write_all(b\"cargo-mutants 99.0.0\\n\") }",
    )?;
    let result = native_mutation::run(
        request(&host, &root, &backend, bounds()?, "output")?,
        |_| None,
        |_, _| None,
    );
    super::presentation::query(
        result.as_ref().err().ok_or("missing refusal")?,
        "backend-version",
        "finished",
    )?;
    assert!(matches!(
        result,
        Err(MutationError::Query {
            phase: MutationPhase::BackendVersion,
            ..
        })
    ));
    assert!(!root.join("output").exists());
    Ok(())
}

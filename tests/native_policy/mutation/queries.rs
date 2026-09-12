use super::configure::{finish, request};
use super::refusal::subject;
use crate::compiler::{
    configure::{bounds, host, root},
    refusal::standin,
};
use macroonz::native_mutation::{self, MutationError, MutationPhase, MutationRun};
use macroonz::native_process::{ProcessLimits, ProcessRun, ProcessStop};
use std::time::Duration;

#[test]
fn query_and_backend_cleanup_keep_their_distinct_execution_contexts() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let backend = standin(
        &root,
        &host,
        "cleanup-backend",
        &subject(
            "std::io::stderr().write_all(b\"Found 1 mutants\\nok Unmutated baseline\\ncaught fixture.rs:1:1: replace value with 0\\n\")?;",
        ),
    )?;
    std::fs::write(root.join("fixture.rs"), b"source A").map_err(|error| error.to_string())?;
    let zero_cleanup = ProcessLimits::informed(Duration::from_secs(5), Duration::ZERO, 4096, 4096)
        .map_err(|error| error.to_string())?;
    let refusal = native_mutation::run(
        request(&host, &root, &backend, zero_cleanup, "query-output")?,
        |_| None,
        |_, _| None,
    )
    .err()
    .ok_or("query cleanup not retained")?;
    assert!(
        matches!(&refusal, MutationError::Query { phase: MutationPhase::BackendVersion, run, .. } if matches!(run.as_ref(), ProcessRun::Pending(_)))
    );
    let refusal = refusal.finish_cleanup(Duration::from_secs(3));
    assert!(
        matches!(refusal, MutationError::Query { phase: MutationPhase::BackendVersion, run, .. } if matches!(run.as_ref(), ProcessRun::Finished(_)))
    );
    assert!(!root.join("query-output").exists());
    let selected = request(&host, &root, &backend, zero_cleanup, "backend-output")?
        .version_queries(bounds()?)
        .map_err(|error| error.to_string())?;
    let run =
        native_mutation::run(selected, |_| None, |_, _| None).map_err(|error| error.to_string())?;
    let MutationRun::Pending(pending) = run else {
        return Err("backend cleanup not retained".to_owned());
    };
    assert_eq!(pending.process().stop(), &ProcessStop::Exited);
    let output = finish(pending.finish(Duration::from_secs(3)))?;
    assert!(output.manifest().is_ok());
    assert_eq!(
        output.original_sources().collect::<Vec<_>>(),
        [("fixture.rs", b"source A".as_slice())]
    );
    Ok(())
}

#[test]
fn compiler_query_and_backend_query_failures_keep_phase_and_capture() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let backend = standin(&root, &host, "version-ok", &subject(""))?;
    let wrong = standin(
        &root,
        &host,
        "wrong-rustc",
        "use std::io::Write; fn main() -> std::io::Result<()> { std::io::stdout().write_all(b\"rustc 99.0.0 (unsupported)\\n\") }",
    )?;
    let wrong_host = crate::compiler::types::Host {
        cargo: host.cargo.clone(),
        rustc: wrong.clone(),
        triple: host.triple.clone(),
        environment: host
            .environment
            .iter()
            .filter(|(key, _)| key != "RUSTC")
            .cloned()
            .collect(),
    };
    let compiler_refusal = native_mutation::run(
        request(&wrong_host, &root, &backend, bounds()?, "wrong-output")?,
        |_| None,
        |_, _| None,
    )
    .err()
    .ok_or("unsupported compiler admitted")?;
    assert!(
        matches!(compiler_refusal, MutationError::Query { phase: MutationPhase::CompilerVersion, request, .. } if request.executable() == wrong)
    );
    for (name, body, stop) in [
        (
            "query-flood",
            "use std::io::Write; fn main() -> std::io::Result<()> { std::io::stderr().write_all(&vec![b'x'; 131072]) }",
            ProcessStop::OutputLimit,
        ),
        (
            "query-timeout",
            "fn main() { std::thread::sleep(std::time::Duration::from_secs(10)); }",
            ProcessStop::Deadline,
        ),
    ] {
        let executable = standin(&root, &host, name, body)?;
        let limits = ProcessLimits::informed(
            Duration::from_millis(750),
            Duration::from_secs(3),
            4096,
            4096,
        )
        .map_err(|error| error.to_string())?;
        let refusal = native_mutation::run(
            request(&host, &root, &executable, limits, name)?,
            |_| None,
            |_, _| None,
        )
        .err()
        .ok_or("query failure admitted")?;
        let MutationError::Query { run, phase, .. } = refusal else {
            return Err("query evidence lost".to_owned());
        };
        let ProcessRun::Finished(output) = *run else {
            return Err("query cleanup unfinished".to_owned());
        };
        assert_eq!(phase, MutationPhase::BackendVersion);
        assert_eq!(output.stop(), &stop);
        assert!(!root.join(name).is_dir());
    }
    Ok(())
}

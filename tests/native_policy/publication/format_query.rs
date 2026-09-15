use super::configure::configuration;
use crate::compiler::configure::{bounds, host, root, standin, tool};
use macroonz::native_process::{ProcessError, ProcessLimits, ProcessRun, ProcessStop};
use macroonz::native_publication::{FormatError, Formatter};
use std::time::Duration;

#[test]
fn failed_or_unsupported_version_queries_cannot_qualify_a_formatter() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let missing = tool(&host, &root.join("missing-rustfmt"), &root, bounds()?)?;
    assert!(matches!(
        Formatter::qualified(&missing, configuration(&root)?),
        Err(FormatError::Process(ProcessError::Start(_)))
    ));
    for (name, source, stop) in [
        (
            "future-version",
            "use std::io::Write; fn main() -> std::io::Result<()> { std::io::stdout().write_all(b\"rustfmt 99.0.0-stable (future)\\n\") }",
            ProcessStop::Exited,
        ),
        (
            "query-failure",
            "fn main() -> std::process::ExitCode { std::process::ExitCode::from(7u8) }",
            ProcessStop::Exited,
        ),
        (
            "query-flood",
            "use std::io::Write; fn main() -> std::io::Result<()> { std::io::stdout().write_all(&vec![b'x';131072]) }",
            ProcessStop::OutputLimit,
        ),
        (
            "query-timeout",
            "fn main() { std::thread::sleep(std::time::Duration::from_secs(10)); }",
            ProcessStop::Deadline,
        ),
    ] {
        let executable = standin(&root, &host, name, source)?;
        let limits = ProcessLimits::informed(
            Duration::from_millis(750),
            Duration::from_secs(3),
            1024,
            1024,
        )
        .map_err(|error| error.to_string())?;
        let refusal = Formatter::qualified(
            &tool(&host, &executable, &root, limits)?,
            configuration(&root)?,
        )
        .err()
        .ok_or("query was admitted")?;
        let FormatError::Query { request, run } = refusal else {
            return Err("query evidence lost".to_owned());
        };
        assert_eq!(request.arguments(), ["--version"]);
        assert_eq!(request.executable(), executable);
        let ProcessRun::Finished(output) = *run else {
            return Err("query cleanup unfinished".to_owned());
        };
        assert_eq!(output.stop(), &stop);
    }
    Ok(())
}

use super::configure::{configuration, finish, publication, scratch, standin as body};
use crate::compiler::configure::{bounds, host, root, tool};
use crate::compiler::refusal::standin;
use crate::presentation_formats::{field, parsed};
use macroonz::native_process::{ProcessLimits, ProcessRun, ProcessStop};
use macroonz::native_publication::{FormatError, FormatObservationError, FormatRun, Formatter};
use macroonz::presentation::{publication_format_output, publication_format_run};
use std::time::Duration;

#[test]
fn formatter_failures_floods_and_deadlines_preserve_actual_process_standing() -> Result<(), String>
{
    let root = root()?;
    let host = host(&root)?;
    let publication = publication()?;
    let file = publication.files().last().ok_or("no file")?;
    for (name, source, stop) in [
        (
            "failed",
            "std::io::stderr().write_all(b\"formatter failed\\n\")?; std::fs::read(\"absent-input\")?;",
            ProcessStop::Exited,
        ),
        (
            "stdout-flood",
            "std::io::stdout().write_all(&vec![b'x';131072])?;",
            ProcessStop::OutputLimit,
        ),
        (
            "stderr-flood",
            "std::io::stderr().write_all(&vec![b'x';131072])?;",
            ProcessStop::OutputLimit,
        ),
        (
            "timeout",
            "std::thread::sleep(std::time::Duration::from_secs(10));",
            ProcessStop::Deadline,
        ),
    ] {
        let executable = standin(&root, &host, name, &body(source))?;
        let selected = Formatter::qualified(
            &tool(&host, &executable, &root, bounds()?)?,
            configuration(&root)?,
        )
        .map_err(|error| format!("{error:?}"))?
        .execution_limits(
            ProcessLimits::informed(
                Duration::from_millis(750),
                Duration::from_secs(3),
                4096,
                4096,
            )
            .map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;
        let output = finish(
            selected
                .format(&file, scratch(&root)?)
                .map_err(|error| error.to_string())?,
        )?;
        assert_eq!(output.process().stop(), &stop);
        assert_eq!(output.source(), Err(FormatObservationError::Process));
        assert!(output.published_digest().is_err());
    }
    Ok(())
}

#[test]
fn invalid_text_or_changed_configuration_never_establishes_formatted_source() -> Result<(), String>
{
    let root = root()?;
    let host = host(&root)?;
    let publication = publication()?;
    let file = publication.files().last().ok_or("no file")?;
    for (name, source, expected) in [
        (
            "invalid-utf8",
            "std::io::stdout().write_all(&[255, 10])?;",
            FormatObservationError::Text,
        ),
        (
            "no-newline",
            "std::io::stdout().write_all(b\"pub const V:u8=1;\")?;",
            FormatObservationError::Text,
        ),
        (
            "windows-lines",
            "std::io::stdout().write_all(b\"pub const V:u8=1;\\r\\n\")?;",
            FormatObservationError::Text,
        ),
        (
            "nul-output",
            "std::io::stdout().write_all(b\"\\0\\n\")?;",
            FormatObservationError::Text,
        ),
        (
            "changed-config",
            "std::fs::write(\"declared.toml\",b\"max_width=80\\n\")?; std::io::stdout().write_all(b\"pub const V:u8=1;\\n\")?;",
            FormatObservationError::Configuration(
                "rustfmt configuration changed during execution".to_owned(),
            ),
        ),
    ] {
        let executable = standin(&root, &host, name, &body(source))?;
        let selected = Formatter::qualified(
            &tool(&host, &executable, &root, bounds()?)?,
            configuration(&root)?,
        )
        .map_err(|error| format!("{error:?}"))?;
        let output = finish(
            selected
                .format(&file, scratch(&root)?)
                .map_err(|error| error.to_string())?,
        )?;
        assert_eq!(output.source(), Err(expected));
        let shown = parsed(&publication_format_output(&output))?;
        assert_eq!(field(&shown, "/record/source/kind")?, "refused");
        assert_eq!(field(&shown, "/record/process/status/success")?, true);
        assert_eq!(
            field(&shown, "/record/source/value/kind")?,
            if name == "changed-config" {
                "configuration"
            } else {
                "text"
            }
        );
    }
    Ok(())
}

#[test]
fn query_and_format_cleanup_remain_distinct_and_retryable() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let executable = standin(
        &root,
        &host,
        "pending-formatter",
        &body("std::io::stdout().write_all(b\"pub const OTHER: u8 = 42;\\n\")?;"),
    )?;
    let zero = ProcessLimits::informed(Duration::from_secs(5), Duration::ZERO, 4096, 4096)
        .map_err(|error| error.to_string())?;
    let refusal = Formatter::qualified(
        &tool(&host, &executable, &root, zero)?,
        configuration(&root)?,
    )
    .err()
    .ok_or("query cleanup not retained")?;
    let FormatError::Query { request, run } = refusal else {
        return Err("query context lost".to_owned());
    };
    assert_eq!(request.arguments(), ["--version"]);
    let ProcessRun::Pending(query) = *run else {
        return Err("query was not pending".to_owned());
    };
    assert!(matches!(
        query.finish(Duration::from_secs(3)),
        ProcessRun::Finished(_)
    ));
    let selected = Formatter::qualified(
        &tool(&host, &executable, &root, bounds()?)?,
        configuration(&root)?,
    )
    .map_err(|error| error.to_string())?
    .execution_limits(zero)
    .map_err(|error| error.to_string())?;
    let publication = publication()?;
    let file = publication.files().last().ok_or("no file")?;
    let formatting = selected
        .format(&file, scratch(&root)?)
        .map_err(|error| error.to_string())?;
    let shown = parsed(&publication_format_run(&formatting))?;
    assert_eq!(field(&shown, "/record/kind")?, "pending-cleanup");
    let FormatRun::Pending(pending) = formatting else {
        return Err("format cleanup not retained".to_owned());
    };
    let output = finish(pending.finish(Duration::from_secs(3)))?;
    assert_eq!(output.source(), Ok("pub const OTHER: u8 = 42;\n"));
    assert_eq!(output.canonical_digest(), file.canonical_digest());
    Ok(())
}

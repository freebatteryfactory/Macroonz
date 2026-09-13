use super::types::FormatContext;
use super::{FormatError, FormatObservationError};
use crate::native_process::{CaptureEnd, ProcessOutput, ProcessStop};
use std::io::Read;
use std::path::Path;

pub(super) fn configuration(path: &Path) -> Result<Vec<u8>, FormatError> {
    let file = std::fs::File::open(path).map_err(FormatError::Filesystem)?;
    if !file.metadata().map_err(FormatError::Filesystem)?.is_file() {
        return Err(FormatError::Configuration(
            "rustfmt configuration is not a regular file".to_owned(),
        ));
    }
    let mut bytes = Vec::new();
    file.take(65537)
        .read_to_end(&mut bytes)
        .map_err(FormatError::Filesystem)?;
    if bytes.len() > 65536usize {
        return Err(FormatError::Configuration(
            "rustfmt configuration exceeds 65536 bytes".to_owned(),
        ));
    }
    Ok(bytes)
}

fn successful(process: &ProcessOutput) -> bool {
    process.stop() == &ProcessStop::Exited
        && process.status().success()
        && process.stdout().end() == &CaptureEnd::Eof
        && process.stderr().end() == &CaptureEnd::Eof
}

pub(super) fn supported(process: &ProcessOutput) -> bool {
    successful(process)
        && std::str::from_utf8(process.stdout().bytes()).is_ok_and(|text| {
            let version = text.trim_end_matches(['\r', '\n']);
            version.starts_with("rustfmt 1.9.0-stable (")
                && version.ends_with(')')
                && !version.contains(['\r', '\n'])
        })
}

pub(super) fn text(
    context: &FormatContext,
    process: &ProcessOutput,
) -> Result<(), FormatObservationError> {
    if !successful(process) {
        return Err(FormatObservationError::Process);
    }
    let source = std::str::from_utf8(process.stdout().bytes())
        .map_err(|_error| FormatObservationError::Text)?;
    if source.contains(['\r', '\0']) || !source.ends_with('\n') {
        return Err(FormatObservationError::Text);
    }
    let current = configuration(&context.config)
        .map_err(|error| FormatObservationError::Configuration(error.to_string()))?;
    if current != context.configuration {
        return Err(FormatObservationError::Configuration(
            "rustfmt configuration changed during execution".to_owned(),
        ));
    }
    Ok(())
}

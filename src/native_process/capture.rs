use super::types::{CaptureEnd, CaptureTask, CapturedOutput};
use std::io::Read;

pub(super) fn reserve(limit: usize) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(limit)
        .map_err(std::io::Error::other)?;
    Ok(bytes)
}

pub(super) fn start(
    reader: Option<impl Read + Send + 'static>,
    bytes: Vec<u8>,
    limit: usize,
) -> CaptureTask {
    let Some(reader) = reader else {
        return CaptureTask::Finished(CapturedOutput {
            bytes,
            end: CaptureEnd::Failed("missing requested output pipe".to_owned()),
        });
    };
    match std::thread::Builder::new()
        .name("macroonz-pipe".to_owned())
        .spawn(move || drain(reader, bytes, limit))
    {
        Ok(handle) => CaptureTask::Reading(handle),
        Err(error) => CaptureTask::Finished(CapturedOutput {
            bytes: Vec::new(),
            end: CaptureEnd::Failed(error.to_string()),
        }),
    }
}

fn drain(mut reader: impl Read, mut bytes: Vec<u8>, limit: usize) -> CapturedOutput {
    let mut buffer = [0u8; 8192];
    let end = loop {
        match reader.read(&mut buffer) {
            Ok(0) => break CaptureEnd::Eof,
            Ok(count) => {
                let Some(chunk) = buffer.get(..count) else {
                    break CaptureEnd::Failed("reader exceeded supplied buffer".to_owned());
                };
                let remaining = limit.saturating_sub(bytes.len());
                bytes.extend(chunk.iter().take(remaining));
                if count > remaining {
                    break CaptureEnd::LimitExceeded;
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
            Err(error) => break CaptureEnd::Failed(error.to_string()),
        }
    };
    CapturedOutput { bytes, end }
}

pub(super) fn poll(task: &mut CaptureTask) {
    if matches!(task, CaptureTask::Reading(handle) if handle.is_finished()) {
        let empty = CaptureTask::Finished(CapturedOutput {
            bytes: Vec::new(),
            end: CaptureEnd::Failed("reader panicked".to_owned()),
        });
        if let CaptureTask::Reading(handle) = std::mem::replace(task, empty)
            && let Ok(captured) = handle.join()
        {
            *task = CaptureTask::Finished(captured);
        }
    }
}

pub(super) fn stop(task: &CaptureTask) -> Option<super::ProcessStop> {
    match task {
        CaptureTask::Finished(captured) => end_stop(captured),
        CaptureTask::Reading(_) => None,
    }
}

pub(super) fn end_stop(captured: &CapturedOutput) -> Option<super::ProcessStop> {
    match &captured.end {
        CaptureEnd::LimitExceeded => Some(super::ProcessStop::OutputLimit),
        CaptureEnd::Failed(error) => Some(super::ProcessStop::ObservationFailed(error.clone())),
        CaptureEnd::Eof => None,
    }
}

use super::source::{boolean, field, text};
use super::types::{
    Artifact, CargoFixture, CargoTarget, CompilerObservationError, CompilerRequest, Observation,
    Protocol, SelectedCargoMessage, Stream,
};
use crate::harness::oracle::ObservedCompilation;
use crate::native_process::{ProcessOutput, ProcessStop};
use serde_json::Value;
use std::path::Path;

pub(super) fn observe(
    request: &CompilerRequest,
    process: &ProcessOutput,
) -> Result<Observation, CompilerObservationError> {
    if process.stop() != &ProcessStop::Exited {
        return Err(CompilerObservationError::Interrupted(
            process.stop().clone(),
        ));
    }
    let ordinary_refusal = match request.protocol {
        Protocol::Rustc { .. } => 1_i32,
        Protocol::Cargo(_) => 101_i32,
    };
    if process.status().code() != Some(0_i32) && process.status().code() != Some(ordinary_refusal) {
        return Err(CompilerObservationError::ProcessFailure);
    }
    let bytes = match request.protocol {
        Protocol::Rustc { .. } => process.stderr().bytes(),
        Protocol::Cargo(_) => process.stdout().bytes(),
    };
    let source =
        std::str::from_utf8(bytes).map_err(|error| CompilerObservationError::InvalidJson {
            line: 1,
            detail: error.to_string(),
        })?;
    let mut stream = Stream::default();
    for (at, line) in source
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
    {
        let value =
            super::decode::value(line).map_err(|error| CompilerObservationError::InvalidJson {
                line: at.saturating_add(1),
                detail: error.to_string(),
            })?;
        match &request.protocol {
            Protocol::Rustc { artifact } => rustc(&mut stream, &value, request, artifact)?,
            Protocol::Cargo(fixture) => cargo(&mut stream, &value, request, fixture)?,
        }
    }
    conclude(stream, process.status(), &request.protocol)
}

fn rustc(
    stream: &mut Stream,
    value: &Value,
    request: &CompilerRequest,
    artifact: &Path,
) -> Result<(), CompilerObservationError> {
    match text(value, "$message_type")? {
        "diagnostic" => diagnostic(stream, value, request),
        "artifact" if text(value, "emit")? == "link" => {
            let path =
                super::source::physical(request.process.directory(), text(value, "artifact")?)?;
            if path != artifact {
                return Err(CompilerObservationError::Protocol(
                    "unexpected rustc artifact".to_owned(),
                ));
            }
            artifact_once(
                stream,
                Artifact {
                    executable: Some(path),
                    cargo_fresh: None,
                },
            )
        }
        _ => Ok(()),
    }
}

fn cargo(
    stream: &mut Stream,
    value: &Value,
    request: &CompilerRequest,
    fixture: &CargoFixture,
) -> Result<(), CompilerObservationError> {
    if stream.finished.is_some() {
        return Err(CompilerObservationError::Protocol(
            "output after build-finished".to_owned(),
        ));
    }
    match text(value, "reason")? {
        "build-finished" => {
            stream.finished = Some(boolean(value, "success")?);
            Ok(())
        }
        "compiler-message" => {
            let Some(message) = SelectedCargoMessage::for_fixture(value, fixture)? else {
                return Ok(());
            };
            diagnostic(stream, field(message.value(), "message")?, request)
        }
        "compiler-artifact" => {
            let Some(message) = SelectedCargoMessage::for_fixture(value, fixture)? else {
                return Ok(());
            };
            let artifact_value = message.value();
            if boolean(field(artifact_value, "profile")?, "test")? {
                return Err(CompilerObservationError::Protocol(
                    "unexpected test artifact".to_owned(),
                ));
            }
            let executable = match field(artifact_value, "executable")? {
                Value::Null => None,
                Value::String(path) => {
                    let path = super::source::physical(request.process.directory(), path)?;
                    if !path.starts_with(fixture.target_directory()) {
                        return Err(CompilerObservationError::Protocol(
                            "artifact outside the declared target directory".to_owned(),
                        ));
                    }
                    Some(path)
                }
                Value::Bool(_) | Value::Number(_) | Value::Array(_) | Value::Object(_) => {
                    return Err(CompilerObservationError::Protocol(
                        "invalid executable field".to_owned(),
                    ));
                }
            };
            if matches!(fixture.target(), CargoTarget::Binary(_)) && executable.is_none() {
                return Err(CompilerObservationError::Protocol(
                    "binary artifact has no executable".to_owned(),
                ));
            }
            artifact_once(
                stream,
                Artifact {
                    executable,
                    cargo_fresh: Some(boolean(artifact_value, "fresh")?),
                },
            )
        }
        _ => Ok(()),
    }
}

fn diagnostic(
    stream: &mut Stream,
    value: &Value,
    request: &CompilerRequest,
) -> Result<(), CompilerObservationError> {
    match text(value, "level")? {
        "error: internal compiler error" => Err(CompilerObservationError::ProcessFailure),
        "error" => {
            stream.errors = stream.errors.saturating_add(1);
            if let Some(anchor) =
                super::source::anchor(value, request.process.directory(), &request.locus)?
            {
                stream.anchors.push(anchor);
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn artifact_once(stream: &mut Stream, artifact: Artifact) -> Result<(), CompilerObservationError> {
    if stream.artifact.is_some() {
        return Err(CompilerObservationError::Protocol(
            "multiple selected artifacts".to_owned(),
        ));
    }
    stream.artifact = Some(artifact);
    Ok(())
}

fn conclude(
    stream: Stream,
    status: std::process::ExitStatus,
    protocol: &Protocol,
) -> Result<Observation, CompilerObservationError> {
    if matches!(protocol, Protocol::Cargo(_)) && stream.finished != Some(status.success()) {
        return Err(CompilerObservationError::Protocol(
            "build-finished and exit status disagree or completion is missing".to_owned(),
        ));
    }
    if status.success() {
        if stream.errors != 0 || stream.artifact.is_none() {
            return Err(CompilerObservationError::Protocol(
                "successful build lacks its artifact or carries errors".to_owned(),
            ));
        }
        return Ok(Observation {
            compilation: ObservedCompilation::compiled(),
            artifact: stream.artifact,
        });
    }
    if stream.artifact.is_some() {
        return Err(CompilerObservationError::Protocol(
            "failed selected build also reported an artifact".to_owned(),
        ));
    }
    if stream.errors == 0 {
        return Err(CompilerObservationError::ProcessFailure);
    }
    let [anchor] = stream.anchors.as_slice() else {
        return Err(CompilerObservationError::DiagnosticCount(
            stream.anchors.len(),
        ));
    };
    Ok(Observation {
        compilation: ObservedCompilation::refused(anchor.clone()),
        artifact: None,
    })
}

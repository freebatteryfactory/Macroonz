use super::CompilerObservationError;
use crate::harness::oracle::{
    DiagnosticAnchor, PrimarySourceSpan, RelativeSourcePath, RustcErrorCode, SourcePosition,
};
use serde_json::Value;
use std::path::{Component, Path, PathBuf};

pub(super) fn anchor(
    diagnostic: &Value,
    root: &Path,
    locus: &RelativeSourcePath,
) -> Result<Option<DiagnosticAnchor>, CompilerObservationError> {
    let mut primary = Vec::new();
    for span in array(diagnostic, "spans")? {
        if boolean(span, "is_primary")? {
            primary.push((span, logical(root, text(span, "file_name")?)?));
        }
    }
    if !primary.iter().any(|(_, source)| source == locus) {
        return Ok(None);
    }
    let [(span, source)] = primary.as_slice() else {
        return Err(CompilerObservationError::PrimarySpanCount(primary.len()));
    };
    let code = diagnostic
        .get("code")
        .and_then(|code| code.get("code"))
        .and_then(Value::as_str)
        .ok_or(CompilerObservationError::UncodedDiagnostic)?;
    let code =
        RustcErrorCode::informed(code).map_err(|_| CompilerObservationError::UncodedDiagnostic)?;
    let start = position(span, "line_start", "column_start")?;
    let end = position(span, "line_end", "column_end")?;
    let primary_span = PrimarySourceSpan::informed(source.clone(), start, end)
        .map_err(|error| CompilerObservationError::Source(format!("invalid span: {error:?}")))?;
    Ok(Some(DiagnosticAnchor::at(code, primary_span)))
}

fn position(
    span: &Value,
    line: &str,
    column: &str,
) -> Result<SourcePosition, CompilerObservationError> {
    SourcePosition::informed(number(span, line)?, number(span, column)?)
        .map_err(|error| CompilerObservationError::Source(format!("invalid position: {error:?}")))
}

fn logical(root: &Path, offered: &str) -> Result<RelativeSourcePath, CompilerObservationError> {
    let physical = physical(root, offered)?;
    let relative = physical.strip_prefix(root).map_err(|_| {
        CompilerObservationError::Source("primary source is outside the declared root".to_owned())
    })?;
    let segments = relative
        .components()
        .map(|component| match component {
            Component::Normal(segment) => segment.to_str().ok_or_else(|| {
                CompilerObservationError::Source("non-Unicode source segment".to_owned())
            }),
            Component::Prefix(_)
            | Component::RootDir
            | Component::CurDir
            | Component::ParentDir => Err(CompilerObservationError::Source(
                "non-normal source segment".to_owned(),
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;
    RelativeSourcePath::informed(&segments.join("/")).map_err(|error| {
        CompilerObservationError::Source(format!("invalid logical source: {error:?}"))
    })
}

pub(super) fn physical(root: &Path, offered: &str) -> Result<PathBuf, CompilerObservationError> {
    let path = Path::new(offered);
    if offered.contains('\0')
        || offered
            .split(std::path::is_separator)
            .any(|part| matches!(part, "." | ".."))
    {
        return Err(CompilerObservationError::Source(
            "ambiguous physical path".to_owned(),
        ));
    }
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else if path
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
        && !offered.split(std::path::is_separator).any(str::is_empty)
    {
        Ok(root.join(path))
    } else {
        Err(CompilerObservationError::Source(
            "ambiguous physical path".to_owned(),
        ))
    }
}

pub(super) fn field<'a>(
    value: &'a Value,
    name: &str,
) -> Result<&'a Value, CompilerObservationError> {
    value
        .get(name)
        .ok_or_else(|| CompilerObservationError::Protocol(format!("missing {name}")))
}
pub(super) fn text<'a>(value: &'a Value, name: &str) -> Result<&'a str, CompilerObservationError> {
    field(value, name)?
        .as_str()
        .ok_or_else(|| CompilerObservationError::Protocol(format!("{name} is not text")))
}
pub(super) fn array<'a>(
    value: &'a Value,
    name: &str,
) -> Result<&'a [Value], CompilerObservationError> {
    field(value, name)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| CompilerObservationError::Protocol(format!("{name} is not an array")))
}
pub(super) fn boolean(value: &Value, name: &str) -> Result<bool, CompilerObservationError> {
    field(value, name)?
        .as_bool()
        .ok_or_else(|| CompilerObservationError::Protocol(format!("{name} is not boolean")))
}
fn number(value: &Value, name: &str) -> Result<u64, CompilerObservationError> {
    field(value, name)?.as_u64().ok_or_else(|| {
        CompilerObservationError::Protocol(format!("{name} is not an unsigned integer"))
    })
}

//! Structural reading of the rustc JSON facts emitted for one compiler challenge.

#[path = "diagnostic_json.rs"]
mod json;
#[path = "diagnostic_protocol.rs"]
mod protocol;

use json::cargo_diagnostic;
use macroonz_harness::oracle::{
    CompilationVerdict, DeclaredCompilation, DiagnosticAnchor, ObservedCompilation,
    PrimarySourceSpan, PrimarySourceSpanRefusal, RelativeSourcePath, RelativeSourcePathRefusal,
    RustcErrorCode, RustcErrorCodeRefusal, SourcePosition, SourcePositionRefusal,
};
use macroonz_harness::report::{FindingLocation, InfrastructureFault};
use std::path::{Component, Path};
use std::process::Output;

#[derive(Default)]
struct Diagnostic {
    code: Option<String>,
    level: Option<String>,
    spans: Vec<Span>,
}

#[derive(Default)]
struct Span {
    file_name: Option<String>,
    line_start: Option<u64>,
    line_end: Option<u64>,
    column_start: Option<u64>,
    column_end: Option<u64>,
    is_primary: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceField {
    FileName,
    LineStart,
    LineEnd,
    ColumnStart,
    ColumnEnd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ObservationRefusal {
    NoRelevantDiagnostic,
    MultipleRelevantDiagnostics { observed: usize },
    MultiplePrimarySpans { observed: usize },
    MissingCode,
    MissingSourceField(SourceField),
    RootNotAbsolute,
    SourceOutsideRoot,
    NonUnicodePath,
    NonNormalPath,
    LogicalPath(RelativeSourcePathRefusal),
    Code(RustcErrorCodeRefusal),
    Position(SourcePositionRefusal),
    Span(PrimarySourceSpanRefusal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DiagnosticReadFailure {
    Observation(ObservationRefusal),
    Infrastructure {
        fault: InfrastructureFault,
        detail: String,
    },
}

pub(crate) fn require_compilation(
    output: &Output,
    root: &Path,
    locus: &RelativeSourcePath,
    declared: &DeclaredCompilation,
) -> Result<(), String> {
    let observed = observed_compilation(output, root, locus)?;
    let verdict = macroonz_harness::oracle::compiled::compared_compilation(&observed, declared);
    if verdict == CompilationVerdict::Conforms {
        return Ok(());
    }
    let conclusion = verdict.concluded(FindingLocation::at(file!(), line!()));
    Err(format!(
        "compiled-oracle comparison disagreed: {verdict:?}; conclusion: {conclusion:?}"
    ))
}

pub(crate) fn observed_compilation(
    output: &Output,
    root: &Path,
    locus: &RelativeSourcePath,
) -> Result<ObservedCompilation, String> {
    let observed = if output.status.success() {
        ObservedCompilation::compiled()
    } else if output.status.code().is_none() {
        return Err(format!(
            "compiler host did not resolve its process: {:?}",
            DiagnosticReadFailure::Infrastructure {
                fault: InfrastructureFault::BackendExecutionUnresolved,
                detail: String::from_utf8_lossy(&output.stderr).into_owned(),
            }
        ));
    } else {
        observed_refusal(&output.stdout, root, locus)
            .map_err(|failure| format!("compiler observation was not established: {failure:?}"))?
    };
    Ok(observed)
}

fn observed_refusal(
    output: &[u8],
    root: &Path,
    locus: &RelativeSourcePath,
) -> Result<ObservedCompilation, DiagnosticReadFailure> {
    let text =
        std::str::from_utf8(output).map_err(|error| DiagnosticReadFailure::Infrastructure {
            fault: InfrastructureFault::CaptureFailed,
            detail: format!("Cargo JSON was not UTF-8: {error}"),
        })?;
    let mut relevant = Vec::new();
    let mut error_diagnostics = 0usize;

    for line in text.split('\n').filter(|line| {
        !line
            .bytes()
            .all(|byte| matches!(byte, b' ' | b'\t' | b'\r'))
    }) {
        let diagnostic =
            cargo_diagnostic(line).map_err(|detail| DiagnosticReadFailure::Infrastructure {
                fault: InfrastructureFault::CaptureFailed,
                detail,
            })?;
        let Some(diagnostic) = diagnostic else {
            continue;
        };
        if diagnostic.level.as_deref() != Some("error") {
            continue;
        }
        error_diagnostics = error_diagnostics.saturating_add(1usize);
        if let Some(anchor) = relevant_anchor(&diagnostic, root, locus)? {
            relevant.push(anchor);
        }
    }

    match relevant.as_slice() {
        [anchor] => Ok(ObservedCompilation::refused(anchor.clone())),
        [] if error_diagnostics == 0usize => Err(DiagnosticReadFailure::Infrastructure {
            fault: InfrastructureFault::BackendInitializationFailed,
            detail: "the failed Cargo process emitted no structured compiler error".to_owned(),
        }),
        [] => Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::NoRelevantDiagnostic,
        )),
        many => Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::MultipleRelevantDiagnostics {
                observed: many.len(),
            },
        )),
    }
}

fn relevant_anchor(
    diagnostic: &Diagnostic,
    root: &Path,
    locus: &RelativeSourcePath,
) -> Result<Option<DiagnosticAnchor>, DiagnosticReadFailure> {
    let primary: Vec<&Span> = diagnostic
        .spans
        .iter()
        .filter(|span| span.is_primary == Some(true))
        .collect();
    if primary.is_empty() {
        return Ok(None);
    }

    let mut mapped = Vec::with_capacity(primary.len());
    for span in primary {
        mapped.push(mapped_primary(span, root)?);
    }
    if !mapped.iter().any(|span| span.source() == locus) {
        return Ok(None);
    }
    if mapped.len() != 1usize {
        return Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::MultiplePrimarySpans {
                observed: mapped.len(),
            },
        ));
    }
    let mapped_primary = mapped
        .first()
        .cloned()
        .ok_or(DiagnosticReadFailure::Observation(
            ObservationRefusal::NoRelevantDiagnostic,
        ))?;
    let code = diagnostic
        .code
        .as_deref()
        .ok_or(DiagnosticReadFailure::Observation(
            ObservationRefusal::MissingCode,
        ))?;
    let code = RustcErrorCode::informed(code)
        .map_err(|refusal| DiagnosticReadFailure::Observation(ObservationRefusal::Code(refusal)))?;
    Ok(Some(DiagnosticAnchor::at(code, mapped_primary)))
}

fn mapped_primary(span: &Span, root: &Path) -> Result<PrimarySourceSpan, DiagnosticReadFailure> {
    let file_name = required(span.file_name.as_deref(), SourceField::FileName)?;
    let source = normalized_source(root, Path::new(file_name))?;
    let line_start = required(span.line_start, SourceField::LineStart)?;
    let line_end = required(span.line_end, SourceField::LineEnd)?;
    let column_start = required(span.column_start, SourceField::ColumnStart)?;
    let column_end = required(span.column_end, SourceField::ColumnEnd)?;
    let start = SourcePosition::informed(line_start, column_start).map_err(|refusal| {
        DiagnosticReadFailure::Observation(ObservationRefusal::Position(refusal))
    })?;
    let end = SourcePosition::informed(line_end, column_end).map_err(|refusal| {
        DiagnosticReadFailure::Observation(ObservationRefusal::Position(refusal))
    })?;
    PrimarySourceSpan::informed(source, start, end)
        .map_err(|refusal| DiagnosticReadFailure::Observation(ObservationRefusal::Span(refusal)))
}

fn required<T: Copy>(value: Option<T>, field: SourceField) -> Result<T, DiagnosticReadFailure> {
    value.ok_or(DiagnosticReadFailure::Observation(
        ObservationRefusal::MissingSourceField(field),
    ))
}

fn normalized_source(
    root: &Path,
    offered: &Path,
) -> Result<RelativeSourcePath, DiagnosticReadFailure> {
    if !root.is_absolute() {
        return Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::RootNotAbsolute,
        ));
    }
    if !offered.is_absolute() {
        reject_ambiguous_physical_relative(offered)?;
    }
    let physical = if offered.is_absolute() {
        offered.to_path_buf()
    } else {
        root.join(offered)
    };
    let relative = physical
        .strip_prefix(root)
        .map_err(|_| DiagnosticReadFailure::Observation(ObservationRefusal::SourceOutsideRoot))?;
    let mut segments = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(segment) => segments.push(
                segment
                    .to_str()
                    .ok_or(DiagnosticReadFailure::Observation(
                        ObservationRefusal::NonUnicodePath,
                    ))?
                    .to_owned(),
            ),
            Component::Prefix(_)
            | Component::RootDir
            | Component::CurDir
            | Component::ParentDir => {
                return Err(DiagnosticReadFailure::Observation(
                    ObservationRefusal::NonNormalPath,
                ));
            }
        }
    }
    RelativeSourcePath::informed(&segments.join("/")).map_err(|refusal| {
        DiagnosticReadFailure::Observation(ObservationRefusal::LogicalPath(refusal))
    })
}

fn reject_ambiguous_physical_relative(offered: &Path) -> Result<(), DiagnosticReadFailure> {
    let spelling = offered.to_str().ok_or(DiagnosticReadFailure::Observation(
        ObservationRefusal::NonUnicodePath,
    ))?;
    if spelling
        .split(std::path::is_separator)
        .any(|segment| segment.is_empty() || matches!(segment, "." | ".."))
    {
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::NonNormalPath,
        ))
    } else {
        Ok(())
    }
}

const E0308_AT_LOCUS: &[u8] = br#"{"reason":"compiler-message","message":{"level":"error","code":{"code":"E0308"},"spans":[{"file_name":"src/main.rs","line_start":1,"line_end":1,"column_start":1,"column_end":2,"is_primary":true}]}}"#;
const E0277_AT_LOCUS: &[u8] = br#"{"reason":"compiler-message","message":{"level":"error","code":{"code":"E0277"},"spans":[{"file_name":"src/main.rs","line_start":1,"line_end":1,"column_start":1,"column_end":2,"is_primary":true}]}}"#;
const CODELESS_AT_LOCUS: &[u8] = br#"{"reason":"compiler-message","message":{"level":"error","code":null,"spans":[{"file_name":"src/main.rs","line_start":1,"line_end":1,"column_start":1,"column_end":2,"is_primary":true}]}}"#;
const NO_PRIMARY: &[u8] =
    br#"{"reason":"compiler-message","message":{"level":"error","code":null,"spans":[]}}"#;
const TWO_PRIMARY: &[u8] = br#"{"reason":"compiler-message","message":{"level":"error","code":{"code":"E0308"},"spans":[{"file_name":"src/main.rs","line_start":1,"line_end":1,"column_start":1,"column_end":2,"is_primary":true},{"file_name":"src/main.rs","line_start":1,"line_end":1,"column_start":3,"column_end":4,"is_primary":true}]}}"#;

fn test_root(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("qualification")
        .join(name)
}

fn test_locus() -> Result<RelativeSourcePath, String> {
    RelativeSourcePath::informed("src/main.rs")
        .map_err(|refusal| format!("test locus was refused: {refusal:?}"))
}

#[test]
fn root_relative_mapping_is_stable_across_challenge_relocation() -> Result<(), String> {
    let first = test_root("diagnostic-map-first");
    let second = test_root("diagnostic-map-second");
    let first_source = first.join("src").join("main.rs");
    let second_source = second.join("src").join("main.rs");
    let first_mapped = normalized_source(&first, &first_source)
        .map_err(|failure| format!("first mapping failed: {failure:?}"))?;
    let second_mapped = normalized_source(&second, &second_source)
        .map_err(|failure| format!("second mapping failed: {failure:?}"))?;
    assert_eq!(first_mapped, second_mapped);
    assert_eq!(first_mapped.spelling(), "src/main.rs");

    let outside = first
        .parent()
        .ok_or_else(|| "test root carried no parent".to_owned())?
        .join("outside.rs");
    assert_eq!(
        normalized_source(&first, &outside),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::SourceOutsideRoot,
        ))
    );
    assert_eq!(
        normalized_source(&first, Path::new("../outside.rs")),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::NonNormalPath,
        ))
    );
    assert_eq!(
        normalized_source(&first, Path::new("src/./main.rs")),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::NonNormalPath,
        ))
    );
    let physical_relative = Path::new("src").join("main.rs");
    assert_eq!(
        normalized_source(&first, &physical_relative),
        Ok(first_mapped)
    );
    Ok(())
}

#[test]
fn diagnostic_selection_does_not_filter_by_the_declared_error_code() -> Result<(), String> {
    let observed = observed_refusal(
        E0277_AT_LOCUS,
        &test_root("diagnostic-code-control"),
        &test_locus()?,
    )
    .map_err(|failure| format!("diagnostic selection failed: {failure:?}"))?;
    let Some(anchor) = observed.refusal() else {
        return Err("diagnostic selection reported acceptance".to_owned());
    };
    assert_eq!(anchor.code().spelling(), "E0277");
    Ok(())
}

#[test]
fn zero_multiple_and_ambiguous_anchors_refuse_observation_establishment() -> Result<(), String> {
    let root = test_root("diagnostic-ambiguity-controls");
    let locus = test_locus()?;
    assert_eq!(
        observed_refusal(NO_PRIMARY, &root, &locus),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::NoRelevantDiagnostic,
        ))
    );
    assert_eq!(
        observed_refusal(CODELESS_AT_LOCUS, &root, &locus),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::MissingCode,
        ))
    );
    let repeated = [E0308_AT_LOCUS, b"\n", E0308_AT_LOCUS].concat();
    assert_eq!(
        observed_refusal(&repeated, &root, &locus),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::MultipleRelevantDiagnostics { observed: 2usize },
        ))
    );
    assert_eq!(
        observed_refusal(TWO_PRIMARY, &root, &locus),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::MultiplePrimarySpans { observed: 2usize },
        ))
    );
    Ok(())
}

#[test]
fn malformed_structured_output_is_capture_infrastructure() -> Result<(), String> {
    let failure = observed_refusal(
        b"not-json",
        &test_root("diagnostic-malformed-control"),
        &test_locus()?,
    );
    assert!(matches!(
        failure,
        Err(DiagnosticReadFailure::Infrastructure {
            fault: InfrastructureFault::CaptureFailed,
            detail: _,
        })
    ));
    Ok(())
}

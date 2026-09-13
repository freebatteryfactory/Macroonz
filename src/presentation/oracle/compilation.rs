//! Exact compilation observations and owner-produced differences.

use crate::harness::oracle::{
    CompilationDisagreement, CompilationVerdict, DiagnosticAnchor, ObservedCompilation,
    PrimarySourceSpan, SourcePosition,
};
use crate::presentation::{
    Presentation,
    value::{object, tagged},
};
use serde_json::Value;

/// A supplied exact compilation observation with its diagnostic anchor.
pub fn compilation_observation(record: &ObservedCompilation) -> Presentation {
    Presentation::projected(
        "compilation-observation",
        "harness/oracle/compiled",
        "recorded",
        observation(record),
    )
}

/// An existing exact compilation verdict and its retained disagreement.
pub fn compilation_comparison(record: &CompilationVerdict) -> Presentation {
    Presentation::projected(
        "compilation-comparison",
        "harness/oracle/compiled",
        "recorded",
        verdict(record),
    )
}

pub(in crate::presentation) fn observation(record: &ObservedCompilation) -> Value {
    match record.refusal() {
        None => tagged("compiled", Value::Null),
        Some(refusal) => tagged("refused-by-compiler", anchor(refusal)),
    }
}

pub(in crate::presentation) fn verdict(record: &CompilationVerdict) -> Value {
    match record {
        CompilationVerdict::Conforms => tagged("conforms", Value::Null),
        CompilationVerdict::Deviates(cause) => tagged("deviates", disagreement(cause)),
    }
}

pub(super) fn disagreement(record: &CompilationDisagreement) -> Value {
    match record {
        CompilationDisagreement::AcceptedWhereRefusalDeclared => {
            tagged("accepted-where-refusal-declared", Value::Null)
        }
        CompilationDisagreement::RefusedWhereAcceptanceDeclared { observed } => tagged(
            "refused-where-acceptance-declared",
            object([("observed", anchor(observed))]),
        ),
        CompilationDisagreement::ErrorCode { expected, observed } => tagged(
            "error-code",
            object([
                ("expected", expected.spelling().into()),
                ("observed", observed.spelling().into()),
            ]),
        ),
        CompilationDisagreement::PrimarySpan { expected, observed } => tagged(
            "primary-span",
            object([("expected", span(expected)), ("observed", span(observed))]),
        ),
    }
}

fn anchor(record: &DiagnosticAnchor) -> Value {
    object([
        ("code", record.code().spelling().into()),
        ("primary", span(record.primary())),
    ])
}

fn span(record: &PrimarySourceSpan) -> Value {
    object([
        ("source", record.source().spelling().into()),
        ("start", position(record.start())),
        ("end", position(record.end())),
    ])
}

fn position(record: SourcePosition) -> Value {
    object([
        ("line", record.line().into()),
        ("column", record.column().into()),
    ])
}

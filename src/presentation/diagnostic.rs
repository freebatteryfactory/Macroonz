//! Compiler-owned causes and cited repairs projected without parsing summaries.

use super::{
    Presentation,
    value::{array, hex, object, tagged},
};
use crate::compiler::bounded::Capping;
use crate::compiler::diagnostic::{
    Diagnostic, LineBody, Observed, RelatedIdentity, Site, SiteCoordinate,
};
use crate::compiler::token::{CoordinateRole, SourceCoordinate};
use serde_json::Value;

/// Project the compiler's typed diagnostic, including its declared cause and repairs.
pub fn compiler_diagnostic(value: &Diagnostic) -> Presentation {
    Presentation::projected(
        "diagnostic",
        "macroonz-compiler/diagnostic",
        "recorded",
        object([
            ("family", value.family().name().into()),
            ("phase", value.phase().name().into()),
            ("class", value.class().name().into()),
            ("class_description", value.class().described().into()),
            ("summary", value.summary().into()),
            ("body", body(value.body())),
            ("site", site(value.site())),
            ("expected", hex(value.expected().as_bytes())),
            ("observed", observed(value.observed())),
            (
                "related",
                array(
                    value
                        .related()
                        .carried()
                        .iter()
                        .map(|identity| match identity {
                            RelatedIdentity::Body(address) => {
                                tagged("body", hex(address.as_bytes()))
                            }
                            RelatedIdentity::Issue(address) => {
                                tagged("issue", hex(address.as_bytes()))
                            }
                        }),
                ),
            ),
            ("related_capping", capping(value.related().capping())),
            (
                "repairs",
                array(value.repairs().iter().map(|repair| {
                    object([
                        ("home", repair.declared_by.home.into()),
                        ("name", repair.declared_by.name.into()),
                        ("description", repair.description.shown().into()),
                    ])
                })),
            ),
            ("route", hex(value.route().entry().as_bytes())),
        ]),
    )
}

fn observed(value: Observed) -> Value {
    let description = match value {
        Observed::Declared { described, name: _ } => described.into(),
        Observed::SeatAbsent
        | Observed::ContractDisagreement
        | Observed::IdentityDisagreement
        | Observed::ProfileDisagreement
        | Observed::BoundExceeded
        | Observed::OriginAbsent => Value::Null,
    };
    object([("name", value.name().into()), ("description", description)])
}

fn capping(value: Capping) -> Value {
    match value {
        Capping::Complete => tagged("complete", Value::Null),
        Capping::Truncated { omitted } => tagged("truncated", omitted.into()),
    }
}

fn body(value: LineBody) -> Value {
    match value {
        LineBody::SingleCause => tagged("single-cause", Value::Null),
        LineBody::Body {
            further,
            capping: posture,
        } => tagged(
            "body",
            object([("further", further.into()), ("capping", capping(posture))]),
        ),
    }
}

fn coordinate(value: SourceCoordinate) -> Value {
    object([
        (
            "role",
            match value.role {
                CoordinateRole::Byte => "byte".into(),
                CoordinateRole::SemanticOrigin => "semantic-origin".into(),
            },
        ),
        ("position", value.position.into()),
    ])
}

fn resolution(value: SiteCoordinate) -> Value {
    match value {
        SiteCoordinate::Resolved(value) => tagged("resolved", coordinate(value)),
        SiteCoordinate::NotReached(refusal) => tagged(
            "not-reached",
            object([
                ("handle", refusal.handle.index().into()),
                ("reaches", refusal.reaches.into()),
            ]),
        ),
    }
}

fn site(value: Site) -> Value {
    match value {
        Site::WholeDeclaration => tagged("whole-declaration", Value::Null),
        Site::BeforeCapture {
            coordinate: position,
        } => tagged("before-capture", coordinate(position)),
        Site::AtToken {
            token,
            coordinate: position,
        } => tagged(
            "at-token",
            object([
                ("token", token.index().into()),
                ("coordinate", resolution(position)),
            ]),
        ),
    }
}

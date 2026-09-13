//! Caller-reported compiled values and member disagreements.

use crate::harness::oracle::{
    CompiledDisagreement, CompiledObservation, CompiledVerdict, ObservedValue,
};
use crate::presentation::{
    Presentation,
    value::{array, object, tagged},
};
use serde_json::Value;

/// A supplied compiled observation retaining every member in read order.
pub fn compiled_observation(record: &CompiledObservation) -> Presentation {
    let observed = match record {
        CompiledObservation::RefusedByCompiler => tagged("refused-by-compiler", Value::Null),
        CompiledObservation::ReadBack(members) => tagged(
            "read-back",
            array(members.iter().map(|member| {
                object([
                    ("name", member.name.as_str().into()),
                    ("value", value(&member.value)),
                ])
            })),
        ),
    };
    Presentation::projected(
        "compiled-observation",
        "harness/oracle/compiled",
        "recorded",
        observed,
    )
}

/// An existing compiled-value verdict retaining its member disagreement.
pub fn compiled_comparison(record: &CompiledVerdict) -> Presentation {
    let verdict = match record {
        CompiledVerdict::Conforms => tagged("conforms", Value::Null),
        CompiledVerdict::Deviates(cause) => tagged("deviates", disagreement(cause)),
    };
    Presentation::projected(
        "compiled-comparison",
        "harness/oracle/compiled",
        "recorded",
        verdict,
    )
}

pub(super) fn disagreement(record: &CompiledDisagreement) -> Value {
    match record {
        CompiledDisagreement::AcceptedWhereRefusalDeclared => {
            tagged("accepted-where-refusal-declared", Value::Null)
        }
        CompiledDisagreement::RefusedWhereAcceptanceDeclared => {
            tagged("refused-where-acceptance-declared", Value::Null)
        }
        CompiledDisagreement::UnexpectedMember { member } => {
            tagged("unexpected-member", member.as_str().into())
        }
        CompiledDisagreement::DuplicateMember { member } => {
            tagged("duplicate-member", member.as_str().into())
        }
        CompiledDisagreement::MissingMember { member } => {
            tagged("missing-member", member.as_str().into())
        }
        CompiledDisagreement::MemberValue { member } => {
            tagged("member-value", member.as_str().into())
        }
    }
}

fn value(record: &ObservedValue) -> Value {
    match record {
        ObservedValue::Word(word) => tagged("word", word.as_str().into()),
        ObservedValue::Text(text) => tagged("text", text.as_str().into()),
        ObservedValue::Count(count) => tagged("count", (*count).into()),
        ObservedValue::Truth(truth) => tagged("truth", (*truth).into()),
        ObservedValue::Series(values) => tagged("series", array(values.iter().map(value))),
    }
}

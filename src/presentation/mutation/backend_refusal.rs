//! Typed backend reading and artifact admission refusals.

use super::{axes, backend};
use crate::harness::muterprater::{
    ArtifactManifestRefusal, BaselinePrecondition, KillRefusal, WrapRefusal,
};
use crate::presentation::{
    Presentation,
    value::{object, tagged},
};
use serde_json::Value;

/// A backend console refusal retaining the responsible precondition or record coordinate.
pub fn backend_reading_refusal(record: &WrapRefusal) -> Presentation {
    Presentation::projected(
        "backend-reading-refusal",
        "harness/muterprater/backend",
        "recorded",
        object([
            ("phase", "console-reading".into()),
            ("cause", reading(record)),
        ]),
    )
}

/// An artifact admission refusal retaining the reading failure or source relationship.
pub fn backend_manifest_refusal(record: &ArtifactManifestRefusal) -> Presentation {
    Presentation::projected(
        "backend-manifest-refusal",
        "harness/muterprater/backend",
        "recorded",
        object([
            ("phase", "artifact-join".into()),
            ("cause", manifest(record)),
        ]),
    )
}

pub(in crate::presentation) fn manifest(record: &ArtifactManifestRefusal) -> Value {
    match record {
        ArtifactManifestRefusal::Reading(refusal) => tagged("reading", reading(refusal)),
        ArtifactManifestRefusal::MutationSiteNotReported => {
            tagged("mutation-site-not-reported", Value::Null)
        }
        ArtifactManifestRefusal::DuplicateSource(file) => {
            tagged("duplicate-source", file.as_str().into())
        }
        ArtifactManifestRefusal::ReportedSourceMissing(file) => {
            tagged("reported-source-missing", file.as_str().into())
        }
        ArtifactManifestRefusal::SourceNotReported(file) => {
            tagged("source-not-reported", file.as_str().into())
        }
    }
}

fn reading(record: &WrapRefusal) -> Value {
    match record {
        WrapRefusal::BaselineNotStated => tagged("baseline-not-stated", Value::Null),
        WrapRefusal::BaselineNotQualified(precondition) => {
            let cause = match precondition {
                BaselinePrecondition::BaselineFailed => "baseline-failed",
                BaselinePrecondition::BaselineNotRun => "baseline-not-run",
            };
            tagged("baseline-not-qualified", cause.into())
        }
        WrapRefusal::KillNotLawful { ordinal, cause } => tagged(
            "kill-not-lawful",
            object([("ordinal", (*ordinal).into()), ("cause", kill(*cause))]),
        ),
        WrapRefusal::VerdictPastCeiling {
            at,
            verdict,
            ceiling,
        } => tagged(
            "verdict-past-ceiling",
            object([
                ("at", (*at).into()),
                ("verdict", axes::verdict(*verdict)),
                ("ceiling", backend::ceiling(*ceiling)),
            ]),
        ),
    }
}

fn kill(record: KillRefusal) -> Value {
    match record {
        KillRefusal::BaselineNotQualified(axis) => {
            tagged("baseline-not-qualified", axes::baseline(axis))
        }
        KillRefusal::NotMaterialized(axis) => {
            tagged("not-materialized", axes::materialization(axis))
        }
        KillRefusal::ActivationNotObserved => tagged("activation-not-observed", Value::Null),
        KillRefusal::WitnessDidNotComplete(axis) => {
            tagged("witness-did-not-complete", axes::execution(axis))
        }
    }
}

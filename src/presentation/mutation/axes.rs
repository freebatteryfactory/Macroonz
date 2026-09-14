//! Display names for independently owned mutation axes.

use crate::harness::muterprater::{
    BaselineAxis, EquivalenceAxis, ExecutionAxis, InconclusiveCause, MaterializationAxis,
    MutationCensus, MutationVerdict,
};
use crate::presentation::value::object;
use serde_json::Value;

pub(super) fn baseline(record: BaselineAxis) -> Value {
    match record {
        BaselineAxis::Qualified => "qualified",
        BaselineAxis::Failed => "failed",
        BaselineAxis::NotRun => "not-run",
    }
    .into()
}

pub(super) fn materialization(record: MaterializationAxis) -> Value {
    match record {
        MaterializationAxis::Built => "built",
        MaterializationAxis::Unviable => "unviable",
        MaterializationAxis::ToolFailed => "tool-failed",
    }
    .into()
}

pub(super) fn execution(record: ExecutionAxis) -> Value {
    match record {
        ExecutionAxis::Completed => "completed",
        ExecutionAxis::NotExecuted => "not-executed",
        ExecutionAxis::TimedOut => "timed-out",
        ExecutionAxis::Crashed => "crashed",
        ExecutionAxis::InfrastructureFailed => "infrastructure-failed",
    }
    .into()
}

pub(super) fn equivalence(record: EquivalenceAxis) -> Value {
    match record {
        EquivalenceAxis::NotAssessed => "not-assessed",
        EquivalenceAxis::ProvenInScope => "proven-in-scope",
        EquivalenceAxis::Refuted => "refuted",
        EquivalenceAxis::Inconclusive => "inconclusive",
    }
    .into()
}

pub(super) fn cause(record: InconclusiveCause) -> Value {
    match record {
        InconclusiveCause::BaselineNotQualified => "baseline-not-qualified",
        InconclusiveCause::NotMaterialized => "not-materialized",
        InconclusiveCause::NotActivated => "not-activated",
        InconclusiveCause::WitnessIncomplete => "witness-incomplete",
        InconclusiveCause::UnobservableAndUnrejected => "unobservable-and-unrejected",
        InconclusiveCause::ProvenEquivalentInScope => "proven-equivalent-in-scope",
    }
    .into()
}

pub(super) fn verdict(record: MutationVerdict) -> Value {
    match record {
        MutationVerdict::Killed => "killed",
        MutationVerdict::Survived => "survived",
        MutationVerdict::Inconclusive => "inconclusive",
    }
    .into()
}

pub(super) fn census(record: MutationCensus) -> Value {
    object([
        ("pressed", record.pressed().into()),
        ("killed", record.killed().into()),
        ("survived", record.survived().into()),
        ("inconclusive", record.inconclusive().into()),
    ])
}

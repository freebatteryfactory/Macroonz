//! Complete historical executable subsets without a missing discovery denominator.

use crate::harness::muterprater::discovery_archive::{
    ArchivedAlternative, ArchivedEvaluationSurface, ArchivedMutationPoint, ArchivedSelection,
};
use crate::presentation::{
    context::historical_name as name,
    value::{array, hex, object},
};
use serde_json::Value;

pub(super) fn surface(record: &ArchivedEvaluationSurface) -> Value {
    object([
        ("archive_address", hex(record.address().as_bytes())),
        ("identity", hex(record.identity().as_bytes())),
        ("canonical", hex(record.canonical_bytes())),
        ("family", name(record.family())),
        ("policy", hex(record.policy().as_bytes())),
        ("points", array(record.points().iter().map(point))),
    ])
}

fn point(record: &ArchivedMutationPoint) -> Value {
    object([
        ("name", name(record.name())),
        ("owner_claim", name(record.owner_claim())),
        ("original_operation", hex(record.original_operation())),
        ("activation_site", name(record.activation_site())),
        (
            "alternatives",
            array(record.alternatives().iter().map(alternative)),
        ),
    ])
}

fn alternative(record: &ArchivedAlternative) -> Value {
    object([
        ("identity", hex(record.identity().as_bytes())),
        ("family", record.family().into()),
        ("operation", hex(record.operation())),
    ])
}

pub(super) fn selection(record: &ArchivedSelection) -> Value {
    object([
        ("surface", hex(record.surface().as_bytes())),
        ("point", name(record.point())),
        ("alternative", hex(record.alternative().as_bytes())),
    ])
}

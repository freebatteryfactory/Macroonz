//! Every discovered site and its owner-policy disposition in producer order.

use crate::harness::muterprater::{
    DiscoveredMutationSite, DiscoveryDisposition, DiscoveryEntry, MappedUnpermittedCause,
    MutationDiscoveryReading, OwnerClaimMapping,
};
use crate::presentation::{
    Presentation,
    context::declared_name as name,
    value::{array, hex, object, tagged},
};
use serde_json::Value;

/// A complete discovery reading including sites withheld from the executable surface.
pub fn mutation_discovery(record: &MutationDiscoveryReading) -> Presentation {
    Presentation::projected(
        "mutation-discovery",
        "harness/muterprater/discovery",
        "recorded",
        object([
            ("identity", hex(record.identity().address().as_bytes())),
            ("family", name(record.family().name())),
            ("policy", hex(record.policy().address().as_bytes())),
            ("denominator", record.entries().len().into()),
            ("entries", array(record.entries().iter().map(entry))),
        ]),
    )
}

fn entry(record: &DiscoveryEntry) -> Value {
    object([
        ("site", site(record.site())),
        ("disposition", disposition(record.disposition())),
    ])
}

fn site(record: &DiscoveredMutationSite) -> Value {
    let mapping = match record.mapping() {
        OwnerClaimMapping::Mapped(claim) => tagged("mapped", name(claim.name())),
        OwnerClaimMapping::OwnerUnmapped => tagged("owner-unmapped", Value::Null),
    };
    object([
        ("name", name(record.identity().name())),
        ("mapping", mapping),
        ("original_operation", hex(record.original_operation())),
        (
            "alternatives",
            array(record.alternatives().iter().map(|alternative| {
                object([
                    ("family", alternative.family().slug().into()),
                    ("operation", hex(alternative.operation())),
                ])
            })),
        ),
        ("activation_site", name(record.activation_site().name())),
    ])
}

fn disposition(record: DiscoveryDisposition) -> Value {
    match record {
        DiscoveryDisposition::Mapped { point } => tagged("mapped", name(point.name())),
        DiscoveryDisposition::OwnerUnmapped => tagged("owner-unmapped", Value::Null),
        DiscoveryDisposition::MappedUnpermitted { cause } => tagged(
            "mapped-unpermitted",
            match cause {
                MappedUnpermittedCause::Claim(claim) => tagged("claim", name(claim.name())),
                MappedUnpermittedCause::Family { at, family } => tagged(
                    "family",
                    object([("at", at.into()), ("family", family.slug().into())]),
                ),
            },
        ),
    }
}

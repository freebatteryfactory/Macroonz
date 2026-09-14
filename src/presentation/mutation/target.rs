//! Target and activation coordinates without lookup or interpretation.

use crate::harness::muterprater::{
    ActivationDisposition, ActiveSelection, FamilyAttribution, MappingPosture, MutationIdentity,
    MutationSite, MutationTarget, SourceCoordinate,
};
use crate::presentation::{
    context,
    value::{hex, object, tagged},
};
use serde_json::Value;

pub(super) fn target(record: &MutationTarget) -> Value {
    let family = match record.family() {
        FamilyAttribution::Declared(family) => tagged("declared", family.slug().into()),
        FamilyAttribution::OutsideTheBank => tagged("outside-the-bank", Value::Null),
    };
    let owner = match record.owner() {
        MappingPosture::Mapped(claim) => tagged("mapped", context::declared_name(claim.name())),
        MappingPosture::OwnerUnmapped => tagged("owner-unmapped", Value::Null),
    };
    let site = match record.site() {
        MutationSite::Reported(at) => tagged("reported", coordinate(at)),
        MutationSite::Declared(site) => tagged("declared", context::declared_name(site.name())),
    };
    object([
        ("identity", identity(record.identity())),
        ("family", family),
        ("owner", owner),
        ("site", site),
    ])
}

fn identity(record: MutationIdentity) -> Value {
    match record {
        MutationIdentity::External(identity) => {
            tagged("external", hex(identity.address().as_bytes()))
        }
        MutationIdentity::Interpreted { point, alternative } => tagged(
            "interpreted",
            object([
                ("point", context::declared_name(point.name())),
                ("alternative", hex(alternative.address().as_bytes())),
            ]),
        ),
        MutationIdentity::CompiledProjection { point, alternative } => tagged(
            "compiled-projection",
            object([
                ("point", context::declared_name(point.name())),
                ("alternative", hex(alternative.address().as_bytes())),
            ]),
        ),
    }
}

pub(super) fn coordinate(record: &SourceCoordinate) -> Value {
    object([
        ("file", record.file().into()),
        ("line", record.line().into()),
        ("column", record.column().into()),
    ])
}

pub(super) fn selection(record: ActiveSelection) -> Value {
    object([
        ("surface", hex(record.surface().address().as_bytes())),
        ("point", context::declared_name(record.point().name())),
        (
            "alternative",
            hex(record.alternative().address().as_bytes()),
        ),
    ])
}

pub(super) fn activation(record: ActivationDisposition) -> Value {
    match record {
        ActivationDisposition::Observed(reading) => tagged(
            "observed",
            object([
                ("selection", selection(reading.selection())),
                ("witness", hex(reading.witness().address().as_bytes())),
                ("firings", reading.firings().into()),
            ]),
        ),
        ActivationDisposition::NotObserved => tagged("not-observed", Value::Null),
        ActivationDisposition::UnobservableUnderBackend => {
            tagged("unobservable-under-backend", Value::Null)
        }
    }
}

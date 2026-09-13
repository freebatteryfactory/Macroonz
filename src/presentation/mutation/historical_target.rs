//! Historical target names and callback claims without current bindings.

use super::target::coordinate;
use crate::harness::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutationIdentity, ArchivedMutationSite, ArchivedMutationTarget,
};
use crate::presentation::{
    context::historical_name as name,
    value::{hex, object, tagged},
};
use serde_json::Value;

pub(in crate::presentation) fn target(record: &ArchivedMutationTarget) -> Value {
    let family = match record.family() {
        Some(family) => tagged("declared", family.into()),
        None => tagged("outside-the-bank", Value::Null),
    };
    let owner = match record.owner() {
        Some(claim) => tagged("mapped", name(claim)),
        None => tagged("owner-unmapped", Value::Null),
    };
    let site = match record.site() {
        ArchivedMutationSite::Reported(at) => tagged("reported", coordinate(at)),
        ArchivedMutationSite::Declared(site) => tagged("declared", name(site)),
    };
    object([
        ("identity", identity(record.identity())),
        ("family", family),
        ("owner", owner),
        ("site", site),
    ])
}

fn identity(record: &ArchivedMutationIdentity) -> Value {
    match record {
        ArchivedMutationIdentity::External(identity) => {
            tagged("external", hex(identity.as_bytes()))
        }
        ArchivedMutationIdentity::Interpreted { point, alternative } => tagged(
            "interpreted",
            object([
                ("point", name(point)),
                ("alternative", hex(alternative.as_bytes())),
            ]),
        ),
        ArchivedMutationIdentity::CompiledProjection { point, alternative } => tagged(
            "compiled-projection",
            object([
                ("point", name(point)),
                ("alternative", hex(alternative.as_bytes())),
            ]),
        ),
    }
}

pub(in crate::presentation) fn activation(record: &ArchivedActivation) -> Value {
    match record {
        ArchivedActivation::Observed(reading) => tagged(
            "observed",
            object([
                (
                    "selection",
                    object([
                        ("surface", hex(reading.surface().as_bytes())),
                        ("point", name(reading.point())),
                        ("alternative", hex(reading.alternative().as_bytes())),
                    ]),
                ),
                ("witness", hex(reading.witness().as_bytes())),
                ("firings", reading.firings().into()),
            ]),
        ),
        ArchivedActivation::NotObserved => tagged("not-observed", Value::Null),
        ArchivedActivation::UnobservableUnderBackend => {
            tagged("unobservable-under-backend", Value::Null)
        }
    }
}

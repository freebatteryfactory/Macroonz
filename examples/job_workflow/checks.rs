//! Independent job expectations and the invocation facts explicitly supplied by this caller.

use crate::job::{Event, Record, Stage, baked};
use macroonz::harness::descriptor::{DerivedRevision, RevisionBinding};
use macroonz::harness::properties::{Holding, concluded};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, InvocationProfile, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion,
};
use macroonz::harness::runner::Invocation;

pub(super) const BUDGETS: InvocationProfile = InvocationProfile::declared(
    CaseBudget::declared(1),
    ByteBudget::declared(64),
    TimeBudget::declared(1_000_000),
);

pub(super) fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("x86_64-pc-windows-msvc"),
        ToolchainIdentity::declared("rustc 1.98.1 48a229ceaefd4985c50990b14116b6d856af0985"),
    )
}

pub(super) fn subject_revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(include_bytes!(
        "declaration.rs"
    )))
}

pub(super) fn check_revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(
        &[
            include_bytes!("checks.rs").as_slice(),
            include_bytes!("main.rs").as_slice(),
            include_bytes!("ordinary.rs").as_slice(),
        ]
        .concat(),
    ))
}

pub(super) fn job_completes(_: &Invocation) -> TrialConclusion {
    let actual = baked::apply(Stage::Draft, Event::Queue)
        .and_then(|stage| baked::apply(stage, Event::Complete));
    concluded(
        if actual == Ok(Stage::Done) {
            Holding::Holds
        } else {
            Holding::Fails
        },
        FailureClass::PropertyDisagreement,
        FindingCause::named("neutral-job", "queued-job-completes"),
    )
}

pub(super) fn record_preserves_count(_: &Invocation) -> TrialConclusion {
    let record = Record {
        count: 513,
        attempts: 7,
    };
    let mut bytes = Vec::new();
    record.encode_canonical(&mut bytes);
    concluded(
        if Record::decode_canonical(&bytes) == Ok(record) {
            Holding::Holds
        } else {
            Holding::Fails
        },
        FailureClass::PropertyDisagreement,
        FindingCause::named("neutral-job", "record-preserves-count"),
    )
}

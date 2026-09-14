//! An independently stated work bound and explicit input and executable coordinates.

use macroonz::harness::descriptor::{DerivedRevision, NamespacedName, RevisionBinding};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::input::{BoundInput, InputBinding, InputProfile, InputRefusal};
use macroonz::harness::properties::{Holding, concluded};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, InvocationProfile, RunAttempt,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion,
};
use macroonz::harness::runner::Invocation;
use macroonz::workflow::InputRun;
use std::cell::Cell;

std::thread_local! {
    static DECODES: Cell<u32> = const { Cell::new(0) };
    static CHECKS: Cell<u32> = const { Cell::new(0) };
    static PROBES: Cell<u32> = const { Cell::new(0) };
}

pub(super) const CAUSE: FindingCause = FindingCause::named("neutral-job", "bounded-work");
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

pub(super) fn revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(material))
}

pub(super) fn subject_revision() -> RevisionBinding {
    revision(
        &[
            include_bytes!("declaration.rs").as_slice(),
            include_bytes!("../job_workflow/declaration.rs").as_slice(),
            include_bytes!("../../Cargo.lock").as_slice(),
        ]
        .concat(),
    )
}

pub(super) fn decoder() -> Result<InputBinding<Vec<u8>>, InputRefusal> {
    let name = NamespacedName::named("neutral-job", "work-increments")
        .map_err(|_| InputRefusal::ProfileMismatch)?;
    let schema = ContentAddress::derived(
        DomainTag::declared(
            "neutral-work-count-input",
            IdentityProfileVersion::declared(1),
        ),
        b"ordered u8 work increments",
    );
    Ok(InputBinding::declared(
        InputProfile::declared(name, 1, schema),
        revision(include_bytes!("checks.rs")),
        |source| {
            DECODES.set(DECODES.get().saturating_add(1));
            source.bytes(source.len()).map(<[u8]>::to_vec)
        },
    ))
}

pub(super) fn within_bound(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    let record = crate::declaration::work::accumulate(invocation.input().value());
    CHECKS.set(CHECKS.get().saturating_add(1));
    concluded(
        if record.count <= 3 {
            Holding::Holds
        } else {
            Holding::Fails
        },
        FailureClass::PropertyDisagreement,
        CAUSE,
    )
}

pub(super) fn passed(run: &InputRun) -> Result<(), String> {
    assert_eq!(
        crate::selected(run)?.attempt(),
        &RunAttempt::Executed(TrialConclusion::Passed)
    );
    Ok(())
}

pub(super) fn probe() {
    PROBES.set(PROBES.get().saturating_add(1));
}

pub(super) fn observations() -> (u32, u32, u32) {
    (DECODES.get(), CHECKS.get(), PROBES.get())
}

//! An independent slice-length oracle and explicit input and execution bindings.

use macroonz::harness::descriptor::{DerivedRevision, NamespacedName, RevisionBinding};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::input::{BoundInput, InputBinding, InputProfile, InputRefusal};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, InvocationProfile,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding,
};
use macroonz::harness::runner::Invocation;
use std::cell::Cell;

std::thread_local! {
    static DECODES: Cell<u32> = const { Cell::new(0) };
    static CHECKS: Cell<u32> = const { Cell::new(0) };
    static PROBES: Cell<u32> = const { Cell::new(0) };
}

pub(super) const BUDGETS: InvocationProfile = InvocationProfile::declared(
    CaseBudget::declared(1),
    ByteBudget::declared(64),
    TimeBudget::declared(1000),
);

pub(super) fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("caller-declared-counting-target"),
        ToolchainIdentity::declared("caller-declared-counting-toolchain"),
    )
}

pub(super) fn revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(material))
}

pub(super) fn decoder() -> Result<InputBinding<Vec<u8>>, InputRefusal> {
    let name =
        NamespacedName::named("counting", "bytes").map_err(|_| InputRefusal::ProfileMismatch)?;
    let schema = ContentAddress::derived(
        DomainTag::declared("counting-input", IdentityProfileVersion::declared(1)),
        b"each byte is an element",
    );
    Ok(InputBinding::declared(
        InputProfile::declared(name, 1, schema),
        revision(b"all-bytes-v1"),
        |source| {
            DECODES.set(DECODES.get().saturating_add(1));
            source.bytes(source.len()).map(<[u8]>::to_vec)
        },
    ))
}

pub(super) fn lossy(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    let bytes = invocation.input().value();
    judge(crate::declaration::counted::lossy(bytes), bytes.len())
}

pub(super) fn corrected(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    let bytes = invocation.input().value();
    judge(crate::declaration::counted::corrected(bytes), bytes.len())
}

fn judge(actual: usize, expected: usize) -> TrialConclusion {
    CHECKS.set(CHECKS.get().saturating_add(1));
    if actual == expected {
        TrialConclusion::Passed
    } else {
        TrialConclusion::Refused(TrialFinding::established(
            FailureClass::PropertyDisagreement,
            FindingCause::named("counting", "length-disagreement"),
            FindingLocation::at(file!(), line!()),
            None,
        ))
    }
}

pub(super) fn probe() {
    PROBES.set(PROBES.get().saturating_add(1));
}

pub(super) fn observations() -> (u32, u32, u32) {
    (DECODES.get(), CHECKS.get(), PROBES.get())
}

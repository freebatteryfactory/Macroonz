//! Independent arithmetic expectations and explicitly supplied invocation facts.

use macroonz::harness::descriptor::{DerivedRevision, NamespacedName, RevisionBinding};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::input::{BoundInput, InputBinding, InputProfile, InputRefusal};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, InvocationProfile,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding,
};
use macroonz::harness::runner::Invocation;

pub(super) const BUDGETS: InvocationProfile = InvocationProfile::declared(
    CaseBudget::declared(1),
    ByteBudget::declared(64),
    TimeBudget::declared(1000),
);
pub(super) fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("caller-declared-example-target"),
        ToolchainIdentity::declared("caller-declared-example-toolchain"),
    )
}

pub(super) fn revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(material))
}

pub(super) fn decoder() -> Result<InputBinding<Vec<u8>>, InputRefusal> {
    let name = NamespacedName::named("arithmetic", "byte-operands")
        .map_err(|_| InputRefusal::ProfileMismatch)?;
    let schema = ContentAddress::derived(
        DomainTag::declared("arithmetic-input", IdentityProfileVersion::declared(1)),
        b"each byte is one unsigned operand",
    );
    Ok(InputBinding::declared(
        InputProfile::declared(name, 1, schema),
        revision(b"consume-all-bytes-v1"),
        |source| source.bytes(source.len()).map(<[u8]>::to_vec),
    ))
}

pub(super) fn total(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    judge(crate::arithmetic::total(invocation.input().value()), 6)
}

pub(super) fn length(invocation: &Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion {
    if invocation.input().value().len() == 3 {
        TrialConclusion::Passed
    } else {
        disagreement()
    }
}

fn judge(actual: u64, expected: u64) -> TrialConclusion {
    if actual == expected {
        TrialConclusion::Passed
    } else {
        disagreement()
    }
}

fn disagreement() -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        FindingCause::named("arithmetic", "independent-answer"),
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

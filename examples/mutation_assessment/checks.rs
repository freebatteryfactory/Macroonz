//! Independently authored weak and strong expectations over observed predicate results.

use macroonz::harness::descriptor::{
    CheckRef, Classification, DerivedRevision, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, Provenance, RevisionBinding, Row, SubjectRoute,
};
use macroonz::harness::muterprater::{MeaningCheck, MutationWitness};
use macroonz::harness::report::{
    FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding,
};
use macroonz::harness::runner::{Invocation, TrialBinding};

pub(super) fn revision(bytes: &[u8]) -> RevisionBinding {
    RevisionBinding::declared(DerivedRevision::from_material(bytes).revision())
}

pub(super) fn weak() -> Result<MutationWitness<u32>, String> {
    witness(
        "well-formed",
        |value| well_formed(*value),
        |_invocation| well_formed(super::declaration::predicate(1)),
    )
}

pub(super) fn strong() -> Result<MutationWitness<u32>, String> {
    witness(
        "positive-input",
        |value| positive(*value),
        |_invocation| positive(super::declaration::predicate(1)),
    )
}

pub(super) fn well_formed(observed: u32) -> TrialConclusion {
    match observed {
        0 | 1 => TrialConclusion::Passed,
        _ => refused(),
    }
}

pub(super) fn positive(observed: u32) -> TrialConclusion {
    match observed {
        1 => TrialConclusion::Passed,
        _ => refused(),
    }
}

fn refused() -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::PropertyDisagreement,
        FindingCause::named("nonzero", "unexpected-result"),
        FindingLocation::at(file!(), line!()),
        None,
    ))
}

fn witness(
    name: &'static str,
    check: MeaningCheck<u32>,
    ordinary: fn(&Invocation) -> TrialConclusion,
) -> Result<MutationWitness<u32>, String> {
    let row = row(name, Origin::HandWritten)?;
    let check_ref = row.check();
    let attachment = ExecutableAttachment::attached(
        row.subject(),
        check_ref,
        revision(b"nonzero-v1"),
        revision(name.as_bytes()),
        ordinary,
    );
    let binding =
        TrialBinding::bound(row, attachment, Provenance::Unproduced).map_err(super::debug)?;
    MutationWitness::bound(binding, check_ref, check).map_err(super::debug)
}

pub(super) fn row(name: &'static str, origin: Origin) -> Result<Row, String> {
    let subject = SubjectRoute::named("nonzero", "predicate").map_err(super::debug)?;
    let check_ref = CheckRef::named("nonzero", name).map_err(super::debug)?;
    Row::declared(
        super::declaration::claim()?,
        ExecutionSuite::named("nonzero", "independent").map_err(super::debug)?,
        Classification::authored(Vec::new(), Vec::new()).map_err(super::debug)?,
        subject,
        check_ref,
        PopulationRef::named("nonzero", "positive-integers").map_err(super::debug)?,
        origin,
    )
    .map_err(super::debug)
}

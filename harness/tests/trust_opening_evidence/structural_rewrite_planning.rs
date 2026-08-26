//! Outside observations of structural rewrite declaration, planning, admission, and seed vocabulary.

use macroonz_harness::depot::operator_families::OPERATOR_FAMILIES;
use macroonz_harness::muterprater::rewrite::{admission, planned, unrealized_families};
use macroonz_harness::muterprater::{
    ARTIFACT_MUTATIONS, ArtifactMutation, InterpreterAvailability, MissingTrustEvidence,
    OperatorFamilyRef, RewriteAdmission, RewriteCandidate, RewriteDescriptor, RewriteRefusal,
    RewriteRoster, RewriteTrust, RewriteWithheld, RosterRefusal, ScopeShape,
};
use std::collections::BTreeSet;

fn family(slug: &str) -> Result<OperatorFamilyRef, ()> {
    OperatorFamilyRef::of_slug(slug).ok_or(())
}

fn descriptor(
    family: OperatorFamilyRef,
    pattern: &'static str,
    rewrite: &'static str,
) -> Result<RewriteDescriptor, ()> {
    RewriteDescriptor::declared(family, pattern, rewrite).map_err(|_| ())
}

/// Claim: A rewrite descriptor refuses malformed authoring inputs in the documented dependent order and retains every lawful field.
///
/// Subject: The public rewrite-descriptor constructor and its three readings.
/// Population: Every refusal stage and one lawful descriptor.
/// Hostile control: Inputs violate both early and late stages so an implementation that reorders or skips a check disagrees.
/// Denominator: All three refusal arms and all three retained fields.
/// Evidence ceiling: This outside test establishes constructor behavior only and does not interpret either text.
/// Retained regression: Refusal-order or field-custody drift remains a permanent regression.
#[test]
fn descriptor_admission_is_ordered_and_lossless() -> Result<(), ()> {
    let family = family("comparison-boundaries")?;
    assert_eq!(
        RewriteDescriptor::declared(family, "", ""),
        Err(RewriteRefusal::EmptyPattern)
    );
    assert_eq!(
        RewriteDescriptor::declared(family, "input < bound", ""),
        Err(RewriteRefusal::EmptyRewrite)
    );
    assert_eq!(
        RewriteDescriptor::declared(family, "input < bound", "input < bound"),
        Err(RewriteRefusal::RewriteIsPattern)
    );

    let declared = descriptor(family, "input < bound", "input <= bound")?;
    assert_eq!(declared.family(), family);
    assert_eq!(declared.pattern(), "input < bound");
    assert_eq!(declared.rewrite(), "input <= bound");
    Ok(())
}

/// Claim: A rewrite roster is nonempty, rejects repeated pattern-and-rewrite pairs, and preserves declared order.
///
/// Subject: The public rewrite-roster constructor and descriptor reading.
/// Population: The empty roster, one duplicate under another family, and two lawful orders.
/// Hostile control: Reversing the lawful input reverses the reading while a repeated pair refuses at its second position.
/// Denominator: Both roster refusal arms and both positions of the lawful roster.
/// Evidence ceiling: This establishes roster structure and not the semantic correctness of a rewrite pair.
/// Retained regression: Silent duplicate folding or order normalization remains a permanent regression.
#[test]
fn roster_admission_preserves_authored_structure() -> Result<(), ()> {
    assert_eq!(
        RewriteRoster::declared(Vec::new()),
        Err(RosterRefusal::EmptyRoster)
    );
    let comparison = family("comparison-boundaries")?;
    let boolean = family("boolean-operators")?;
    let first = descriptor(comparison, "input < bound", "input <= bound")?;
    let second = descriptor(boolean, "left && right", "left || right")?;
    let repeated_under_another_family = descriptor(boolean, "input < bound", "input <= bound")?;
    assert_eq!(
        RewriteRoster::declared(vec![first, second, repeated_under_another_family]),
        Err(RosterRefusal::DuplicateDescriptor { at: 2 })
    );

    let declared = RewriteRoster::declared(vec![first, second]).map_err(|_| ())?;
    let reversed = RewriteRoster::declared(vec![second, first]).map_err(|_| ())?;
    assert_eq!(declared.descriptors(), &[first, second]);
    assert_eq!(reversed.descriptors(), &[second, first]);
    assert_ne!(declared.descriptors(), reversed.descriptors());
    Ok(())
}

/// Claim: Planning maps every declared descriptor to one audit-pending candidate without changing order or scope.
///
/// Subject: The public roster planner and candidate constructor.
/// Population: Two descriptors under one repo-wide scope and the reversed roster.
/// Hostile control: Reversing the roster must reverse candidates rather than reveal hidden sorting.
/// Denominator: Every candidate field for both roster positions under both input orders.
/// Evidence ceiling: This observes planning only and does not claim that any rewrite executed or earned evidence.
/// Retained regression: Candidate loss, sorting, scope drift, or trust inflation remains a permanent regression.
#[test]
fn planning_preserves_order_scope_and_audit_pending_trust() -> Result<(), ()> {
    let first = descriptor(
        family("comparison-boundaries")?,
        "input < bound",
        "input <= bound",
    )?;
    let second = descriptor(
        family("boolean-operators")?,
        "left && right",
        "left || right",
    )?;
    let scope = ScopeShape::RepoWide;
    let roster = RewriteRoster::declared(vec![first, second]).map_err(|_| ())?;
    let candidates = planned(&roster, &scope);
    let [first_candidate, second_candidate] = candidates.as_slice() else {
        return Err(());
    };
    assert_eq!(
        first_candidate,
        &RewriteCandidate::planned(first, scope.clone())
    );
    assert_eq!(first_candidate.descriptor(), first);
    assert_eq!(first_candidate.scope(), &scope);
    assert_eq!(first_candidate.trust(), RewriteTrust::AuditPending);
    assert_eq!(second_candidate.descriptor(), second);
    assert_eq!(second_candidate.scope(), &scope);
    assert_eq!(second_candidate.trust(), RewriteTrust::AuditPending);

    let reversed = RewriteRoster::declared(vec![second, first]).map_err(|_| ())?;
    let reversed_candidates = planned(&reversed, &scope);
    let [reversed_first, reversed_second] = reversed_candidates.as_slice() else {
        return Err(());
    };
    assert_eq!(reversed_first.descriptor(), second);
    assert_eq!(reversed_second.descriptor(), first);
    Ok(())
}

/// Claim: The unrealized-family reading is computed from the public bank and depends on membership rather than descriptor order.
///
/// Subject: The public rewrite family-gap operation.
/// Population: Two realized families and every other row of the declared operator-family bank.
/// Hostile control: Reversing the descriptor roster leaves the bank-ordered gap unchanged.
/// Denominator: The complete public operator-family bank.
/// Evidence ceiling: This observes an absence reading only and says nothing about whether any gap matters.
/// Retained regression: Hand-counting, bank truncation, or roster-order dependence remains a permanent regression.
#[test]
fn unrealized_families_follow_the_complete_bank() -> Result<(), ()> {
    let first = descriptor(
        family("comparison-boundaries")?,
        "input < bound",
        "input <= bound",
    )?;
    let second = descriptor(
        family("boolean-operators")?,
        "left && right",
        "left || right",
    )?;
    let declared = RewriteRoster::declared(vec![first, second]).map_err(|_| ())?;
    let reversed = RewriteRoster::declared(vec![second, first]).map_err(|_| ())?;
    let expected = OPERATOR_FAMILIES
        .iter()
        .copied()
        .filter(|family| !["comparison-boundaries", "boolean-operators"].contains(&family.slug()))
        .collect::<Vec<_>>();
    assert_eq!(unrealized_families(&declared), expected);
    assert_eq!(unrealized_families(&reversed), expected);
    Ok(())
}

/// Claim: The artifact-mutation seed surface exposes the complete declared order and one distinct nonempty description per seed.
///
/// Subject: The public artifact-mutation roster and description projection.
/// Population: All fifteen declared seed variants.
/// Hostile control: Reversing the expected roster is observably different, and duplicate or empty descriptions fail the observation.
/// Denominator: Every public seed arm, its roster position, and its description.
/// Evidence ceiling: This establishes seed vocabulary only and does not claim any surgery was realized or caught.
/// Retained regression: Seed omission, order drift, or description collapse remains a permanent regression.
#[test]
fn artifact_mutation_seeds_are_complete_ordered_and_distinct() {
    let expected = [
        ArtifactMutation::OrderPermuted,
        ArtifactMutation::IdentityRecycled,
        ArtifactMutation::PlannedOutputOmitted,
        ArtifactMutation::UnplannedOutputAdded,
        ArtifactMutation::ImplTargetAltered,
        ArtifactMutation::ShapeAltered,
        ArtifactMutation::OutputDuplicated,
        ArtifactMutation::TraitPathWrong,
        ArtifactMutation::DecoyInComment,
        ArtifactMutation::ImplMemberDuplicated,
        ArtifactMutation::ImplMemberUnexpected,
        ArtifactMutation::ConstructorPathAltered,
        ArtifactMutation::ImplPostureAltered,
        ArtifactMutation::MeaningBearingAttributeAdded,
        ArtifactMutation::MalformedRust,
    ];
    assert_eq!(ARTIFACT_MUTATIONS, expected.as_slice());
    let reversed = expected.iter().copied().rev().collect::<Vec<_>>();
    assert_ne!(ARTIFACT_MUTATIONS, reversed.as_slice());
    let descriptions = expected
        .iter()
        .copied()
        .map(ArtifactMutation::described)
        .collect::<BTreeSet<_>>();
    assert_eq!(descriptions.len(), expected.len());
    assert!(
        descriptions
            .iter()
            .all(|description| !description.is_empty())
    );
}

/// Claim: Rewrite admission preserves every unavailable interpretation posture without upgrading it to evidence.
///
/// Subject: The public rewrite-admission operation over interpretation availability.
/// Population: No conforming surface and all three missing-trust reasons.
/// Hostile control: Every missing reason must survive exactly rather than collapse into a generic refusal or admission.
/// Denominator: Every nonavailable arm of `InterpreterAvailability`.
/// Evidence ceiling: The available arm is observed at the existing cross-owner composition seat that can lawfully mint trust.
/// Retained regression: Withheld-reason loss or accidental admission remains a permanent regression.
#[test]
fn admission_preserves_every_withheld_posture() {
    type EmptyAvailability =
        InterpreterAvailability<'static, 'static, 'static, 'static, 'static, 'static, (), ()>;

    let absent: EmptyAvailability = InterpreterAvailability::NoConformingSurface;
    assert_eq!(
        admission(&absent),
        RewriteAdmission::Withheld(RewriteWithheld::InterpreterUnavailable)
    );
    for missing in [
        MissingTrustEvidence::CompiledSuitePressure,
        MissingTrustEvidence::CompiledProjectionPressure,
        MissingTrustEvidence::ProjectionPressureForAnotherSurface,
    ] {
        let withheld: EmptyAvailability = InterpreterAvailability::TrustNotOpened { missing };
        assert_eq!(
            admission(&withheld),
            RewriteAdmission::Withheld(RewriteWithheld::TrustNotOpened(missing))
        );
    }
}

//! Outside observations of mutation planning and cross-owner evidence composition.

use super::support::{
    CompiledRosterMeaning, EVALUATION, MutationRoadFailure, SELECTED_OPERATION, active_selection,
    compiled_suite_pressure, family, interpreted_kill, lock_specimen_tests, pair, policy,
    qualification_of, qualified_no_mutation, standard_projection, surface_with, witness,
};
use macroonz_harness::depot::operator_families::OPERATOR_FAMILIES;
use macroonz_harness::muterprater::discover::lower_discoveries;
use macroonz_harness::muterprater::interpret::availability;
use macroonz_harness::muterprater::rewrite::{admission, planned, unrealized_families};
use macroonz_harness::muterprater::{
    ARTIFACT_MUTATIONS, ArtifactMutation, InterpreterAvailability, MissingTrustEvidence,
    MutationVerdict, OperatorFamilyRef, RewriteAdmission, RewriteCandidate, RewriteDescriptor,
    RewriteRefusal, RewriteRoster, RewriteTrust, RewriteWithheld, RosterRefusal, ScopeShape,
};
use std::collections::BTreeSet;

fn rewrite_family(slug: &str) -> Result<OperatorFamilyRef, ()> {
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
    let family = rewrite_family("comparison-boundaries")?;
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
    let comparison = rewrite_family("comparison-boundaries")?;
    let boolean = rewrite_family("boolean-operators")?;
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
        rewrite_family("comparison-boundaries")?,
        "input < bound",
        "input <= bound",
    )?;
    let second = descriptor(
        rewrite_family("boolean-operators")?,
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
        rewrite_family("comparison-boundaries")?,
        "input < bound",
        "input <= bound",
    )?;
    let second = descriptor(
        rewrite_family("boolean-operators")?,
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

/// Claim: point-free parity cannot cross the specimen or rewrite joins into active mutation authority.
///
/// Subject: the interpretation parity, specimen pressure, and rewrite admission composition.
/// Population: one lawfully qualified point-free surface.
/// Hostile control: generic suite pressure is present while selection-scoped projection pressure is absent.
/// Denominator: the point-free qualification, interpretation availability, and rewrite admission readings.
/// Evidence ceiling: this establishes one typed cross-owner composition and does not annex any participating claim.
/// Retained regression: the composition claim remains in the original integration target.
#[test]
fn point_free_trust_does_not_admit_mutation_execution() -> Result<(), MutationRoadFailure> {
    let family = family("point-free-family")?;
    let policy = policy(family)?;
    let surface = lower_discoveries(&policy, Vec::new())?.into_parts().1;
    let pair = pair(family, &surface, EVALUATION)?;
    let input = [1u32, 0, 0];
    let standing = qualified_no_mutation(&pair, witness()?, &input)?;
    let qualification = qualification_of(&standing)?;
    let suite = compiled_suite_pressure()?;
    let availability =
        availability::<[u32; 3], CompiledRosterMeaning>(Some(&surface), Some(&suite), None);
    assert!(matches!(
        &availability,
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledProjectionPressure,
        }
    ));
    assert_eq!(
        admission(&availability),
        RewriteAdmission::Withheld(RewriteWithheld::TrustNotOpened(
            MissingTrustEvidence::CompiledProjectionPressure,
        ))
    );
    assert_eq!(qualification.reading().pair().standing(), pair.standing());
    Ok(())
}

/// Claim: generic suite pressure and exact projection pressure join without flattening their evidence into interpreted execution.
///
/// Subject: the compiled-suite, specimen-projection, and interpretation execution composition.
/// Population: one two-alternative surface and one selected compiled projection.
/// Hostile control: a foreign invocation is refused before evaluation or clock effects.
/// Denominator: every evidence book and custody join traversed by the selected execution.
/// Evidence ceiling: this establishes one complete outside composition and preserves each owner's narrower evidence ceiling.
/// Retained regression: the cross-owner claim remains in the original integration target.
#[test]
fn compiled_and_interpreted_evidence_join_without_flattening() -> Result<(), MutationRoadFailure> {
    let mutation = interpreted_kill()?;
    assert_eq!(mutation.verdict(), MutationVerdict::Killed);
    Ok(())
}

/// Claim: exact projection pressure cannot open interpreted trust for another surface.
///
/// Subject: the public specimen demonstration and interpretation availability roads.
/// Population: two surfaces under one family with distinct admitted selections.
/// Hostile control: a lawful projection from the second surface is offered to the first.
/// Denominator: one complete projection and the one crossed surface join.
/// Evidence ceiling: this establishes the typed cross-owner join for one outside fixture and does not widen either evidence book.
/// Retained regression: this composition claim remains in the original integration target.
#[test]
fn compiled_pressure_cannot_open_trust_for_another_surface() -> Result<(), MutationRoadFailure> {
    let _specimen_guard = lock_specimen_tests()?;
    let family = family("same-family-pair-scope")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let evaluation_pair = pair(family, &surface, EVALUATION)?;
    let input = [1u32, 0, 0];
    let another_surface = surface_with(family, vec![SELECTED_OPERATION])?;
    let another_pair = pair(family, &another_surface, EVALUATION)?;
    assert_ne!(another_pair.standing(), evaluation_pair.standing());
    let another_standing = qualified_no_mutation(&another_pair, witness()?, &input)?;
    let another_qualification = qualification_of(&another_standing)?;
    let another_selection = active_selection(&another_surface)?;
    let another_projection = standard_projection(
        &another_surface,
        another_qualification,
        &another_pair,
        another_selection,
    )?;
    let suite = compiled_suite_pressure()?;
    assert!(matches!(
        availability(Some(&surface), Some(&suite), Some(&another_projection)),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::ProjectionPressureForAnotherSurface,
        }
    ));
    Ok(())
}

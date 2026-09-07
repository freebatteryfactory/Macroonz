//! Independent current bindings expose every changed coordinate without erasing execution.

use super::{LIMITS, fixture, replay as evidence};
use macroonz_harness::descriptor::{NamespacedName, RevisionBinding};
use macroonz_harness::input::{InputBinding, InputProfile};
use macroonz_harness::report::archive::{ArchivedCapsule, read_capsule, retain_capsule};
use macroonz_harness::report::replay::{
    HistoricalReplayRefusal, HistoricalReplayStanding, ReplayCoordinate, ReplayMovement,
    ReplayNonReproduction, ReplayOutcome, ReplayReading, WitnessLineage,
};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, ReplayPosture, TargetBinding, TargetTriple,
    TimeBudget, ToolchainIdentity,
};
use macroonz_harness::runner::{Invocation, replay};

fn current(
    capsule: &ArchivedCapsule,
    decoder: &InputBinding<Vec<u8>>,
    invocation: Invocation,
) -> Result<ReplayReading, ()> {
    evidence::reading(
        &replay(
            capsule,
            &evidence::binding(evidence::moved_revision(), evidence::fixed)?,
            decoder,
            invocation,
            evidence::INPUT_LIMITS,
        )
        .map_err(|_| ())?,
    )
}

fn not_repair(reading: ReplayReading) {
    assert_eq!(
        reading.outcome(),
        ReplayOutcome::NotReproduced(ReplayNonReproduction::PassedWithoutRepairStanding)
    );
}

#[test]
fn profile_namespace_name_version_and_schema_movement_prevent_a_repair_claim() -> Result<(), ()> {
    let capsule = evidence::historical()?;
    let original = fixture::decoder()?;
    let base = original.profile();
    for profile in [
        InputProfile::declared(
            NamespacedName::named("changed", "bytes").map_err(|_| ())?,
            base.version(),
            base.schema(),
        ),
        InputProfile::declared(
            NamespacedName::named("archive", "changed").map_err(|_| ())?,
            base.version(),
            base.schema(),
        ),
        InputProfile::declared(base.name(), 2, base.schema()),
    ] {
        let decoder = InputBinding::declared(profile, fixture::revision(), fixture::bytes);
        let reading = current(&capsule, &decoder, fixture::invocation())?;
        assert_eq!(reading.movement().profile(), ReplayCoordinate::Moved);
        assert_eq!(reading.movement().schema(), ReplayCoordinate::Same);
        assert_eq!(reading.movement().decoder(), ReplayCoordinate::Same);
        assert_eq!(reading.lineage(), WitnessLineage::ConventionMoved);
        not_repair(reading);
        let failed = replay(
            &capsule,
            &evidence::binding(fixture::revision(), fixture::subject)?,
            &decoder,
            fixture::invocation(),
            evidence::INPUT_LIMITS,
        )
        .map_err(|_| ())?;
        assert_eq!(
            evidence::reading(&failed)?.outcome(),
            ReplayOutcome::DefectReproduced
        );
        assert_eq!(
            evidence::reading(&failed)?.movement().profile(),
            ReplayCoordinate::Moved
        );
    }
    let schema_profile = InputProfile::declared(
        base.name(),
        base.version(),
        evidence::moved_revision().revision(),
    );
    let schema_decoder =
        InputBinding::declared(schema_profile, fixture::revision(), fixture::bytes);
    let schema_reading = current(&capsule, &schema_decoder, fixture::invocation())?;
    assert_eq!(schema_reading.movement().profile(), ReplayCoordinate::Same);
    assert_eq!(schema_reading.movement().schema(), ReplayCoordinate::Moved);
    assert_eq!(schema_reading.lineage(), WitnessLineage::ConventionMoved);
    not_repair(schema_reading);
    Ok(())
}

#[test]
fn decoder_identity_and_posture_are_independent_of_equal_input_bytes() -> Result<(), ()> {
    let capsule = evidence::historical()?;
    let original = fixture::decoder()?;
    for revision in [
        evidence::moved_revision(),
        RevisionBinding::declared(original.revision().revision()),
        RevisionBinding::untracked(original.revision().revision()),
    ] {
        let decoder = InputBinding::declared(original.profile(), revision, fixture::bytes);
        let reading = current(&capsule, &decoder, fixture::invocation())?;
        assert_eq!(reading.movement().decoder(), ReplayCoordinate::Moved);
        assert_eq!(reading.movement().profile(), ReplayCoordinate::Same);
        assert_eq!(reading.movement().schema(), ReplayCoordinate::Same);
        assert_eq!(reading.lineage(), WitnessLineage::ReachedWitness);
        not_repair(reading);
    }
    Ok(())
}

#[test]
fn changed_check_cannot_authorize_repair_even_when_the_witness_passes() -> Result<(), ()> {
    let capsule = evidence::historical()?;
    let binding = fixture::binding_under(
        super::super::row()?,
        evidence::moved_revision(),
        evidence::moved_revision(),
        evidence::fixed,
    )?;
    let replayed = evidence::execute(&capsule, &binding)?;
    let reading = evidence::reading(&replayed)?;
    assert_eq!(reading.movement().trial(), ReplayCoordinate::Same);
    assert_eq!(reading.movement().subject(), ReplayCoordinate::Moved);
    assert_eq!(reading.movement().check(), ReplayCoordinate::Moved);
    not_repair(reading);
    let defective = fixture::binding_under(
        super::super::row()?,
        evidence::moved_revision(),
        evidence::moved_revision(),
        fixture::subject,
    )?;
    let still_refused = evidence::reading(&evidence::execute(&capsule, &defective)?)?;
    assert_eq!(still_refused.outcome(), ReplayOutcome::DefectReproduced);
    assert_eq!(still_refused.movement().subject(), ReplayCoordinate::Moved);
    assert_eq!(still_refused.movement().check(), ReplayCoordinate::Moved);
    Ok(())
}

#[test]
fn each_budget_target_and_toolchain_moves_without_erasing_the_current_pass() -> Result<(), ()> {
    let capsule = evidence::historical()?;
    let original = fixture::invocation();
    let budget_axis: fn(ReplayMovement) -> ReplayCoordinate = ReplayMovement::invocation;
    let mut invocations = Vec::new();
    for (cases, bytes, nanos) in [(2, 64, 1000), (1, 65, 1000), (1, 64, 1001)] {
        invocations.push((
            Invocation::declared(
                InvocationProfile::declared(
                    CaseBudget::declared(cases),
                    ByteBudget::declared(bytes),
                    TimeBudget::declared(nanos),
                ),
                original.target().clone(),
                original.site(),
                original.clock(),
            ),
            budget_axis,
        ));
    }
    let target_axis: fn(ReplayMovement) -> ReplayCoordinate = ReplayMovement::target;
    for (target, axis) in [
        (
            TargetBinding::bound(
                TargetTriple::declared("other-target"),
                original.target().toolchain().clone(),
            ),
            target_axis,
        ),
        (
            TargetBinding::bound(
                original.target().target().clone(),
                ToolchainIdentity::declared("other-toolchain"),
            ),
            ReplayMovement::toolchain,
        ),
    ] {
        invocations.push((
            Invocation::declared(
                original.profile(),
                target,
                original.site(),
                original.clock(),
            ),
            axis,
        ));
    }
    for (invocation, axis) in invocations {
        let reading = current(&capsule, &fixture::decoder()?, invocation)?;
        assert_eq!(axis(reading.movement()), ReplayCoordinate::Moved);
        assert_eq!(reading.movement().profile(), ReplayCoordinate::Same);
        not_repair(reading);
    }
    Ok(())
}

fn with_historical_posture(capsule: &ArchivedCapsule, slot: u8) -> Result<ArchivedCapsule, ()> {
    let mut encoded = capsule.encoded().to_vec();
    *encoded.last_mut().ok_or(())? = slot;
    let digest = super::vector::hash("historical-replay-capsule/v1", encoded.get(32..).ok_or(())?);
    encoded.get_mut(..32).ok_or(())?.copy_from_slice(&digest);
    read_capsule(&encoded, LIMITS).map_err(|_| ())
}

#[test]
fn historical_claims_and_current_revisions_never_strengthen_each_others_ceiling() -> Result<(), ()>
{
    let capsule = evidence::historical()?;
    let derived = evidence::moved_revision();
    let declared = RevisionBinding::declared(derived.revision());
    let untracked = RevisionBinding::untracked(derived.revision());
    let exact = HistoricalReplayStanding::Comparable(ReplayPosture::ExactDerived);
    let author = HistoricalReplayStanding::Comparable(ReplayPosture::DeclaredByAuthor);
    let unavailable = HistoricalReplayStanding::Unverifiable(HistoricalReplayRefusal::Untracked);
    for (slot, revision, expected) in [
        (0, derived, exact),
        (0, declared, author),
        (0, untracked, unavailable),
        (1, derived, author),
        (1, declared, author),
        (1, untracked, unavailable),
        (2, derived, unavailable),
        (2, declared, unavailable),
        (2, untracked, unavailable),
    ] {
        let historical = with_historical_posture(&capsule, slot)?;
        let reading = evidence::reading(&evidence::execute(
            &historical,
            &evidence::binding(revision, evidence::fixed)?,
        )?)?;
        assert_eq!(reading.standing(), expected);
        match expected {
            HistoricalReplayStanding::Comparable(_) => {
                assert_eq!(reading.outcome(), ReplayOutcome::FixedOnWitness);
            }
            HistoricalReplayStanding::Unverifiable(_) => not_repair(reading),
        }
    }
    Ok(())
}

#[test]
fn unit_history_keeps_missing_coordinates_while_current_execution_remains_real() -> Result<(), ()> {
    let historical = retain_capsule(&fixture::unit_capsule()?, LIMITS).map_err(|_| ())?;
    let current = evidence::execute(
        &historical,
        &evidence::binding(evidence::moved_revision(), evidence::fixed)?,
    )?;
    let reading = evidence::reading(&current)?;
    assert_eq!(
        reading.standing(),
        HistoricalReplayStanding::Unverifiable(HistoricalReplayRefusal::InputUnrecorded)
    );
    assert_eq!(reading.lineage(), WitnessLineage::HistoricalInputUnrecorded);
    assert_eq!(reading.movement().profile(), ReplayCoordinate::Unrecorded);
    assert_eq!(reading.movement().schema(), ReplayCoordinate::Unrecorded);
    assert_eq!(reading.movement().decoder(), ReplayCoordinate::Unrecorded);
    assert!(current.report().standing().key().input().is_some());
    not_repair(reading);
    Ok(())
}

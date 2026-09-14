//! Independent movements of complete-surface, active-report and mutation claims.

use super::interpreted::LIMITS;
use super::interpreted_vector::{InterpretedVector, pair};
use macroonz_harness::muterprater::interpretation_archive::{
    InterpretedArchiveRefusal, read_interpreted,
};
use macroonz_harness::muterprater::verdict_archive::MutationArchiveRefusal;

fn bind_changed_surface(vector: &mut InterpretedVector) -> Result<(), ()> {
    let address = super::vector::hash("evaluation-surface/v1", &vector.surface.canonical());
    vector.projection.parity.pair = pair(vector.surface.family, &address);
    vector
        .projection
        .selection
        .get_mut(8..40)
        .ok_or(())?
        .copy_from_slice(&address);
    Ok(())
}

#[test]
fn surface_family_policy_membership_and_operation_preimage_are_separate_joins() -> Result<(), ()> {
    for moved in 0u8..6 {
        let mut vector = InterpretedVector::declared()?;
        match moved {
            0 => vector.surface.family = (b"different", b"family"),
            1 => *vector.surface.policy.first_mut().ok_or(())? ^= 1,
            2 => {
                vector.surface.points.clear();
                bind_changed_surface(&mut vector)?;
            }
            3 => {
                vector.surface.points.first_mut().ok_or(())?.name = (b"absent", b"point");
                bind_changed_surface(&mut vector)?;
            }
            4 => {
                vector
                    .surface
                    .points
                    .first_mut()
                    .ok_or(())?
                    .alternatives
                    .first_mut()
                    .ok_or(())?
                    .operation = vec![2];
                bind_changed_surface(&mut vector)?;
            }
            _ => {
                vector.surface.family = (b"different", b"family");
                let address =
                    super::vector::hash("evaluation-surface/v1", &vector.surface.canonical());
                vector.projection.parity.pair = pair((b"evaluation", b"family"), &address);
                vector
                    .projection
                    .selection
                    .get_mut(8..40)
                    .ok_or(())?
                    .copy_from_slice(&address);
            }
        }
        assert_eq!(
            read_interpreted(&vector.encoded(), &LIMITS),
            Err(InterpretedArchiveRefusal::SurfaceJoinMismatch),
            "move={moved}"
        );
    }
    Ok(())
}

#[test]
fn compiled_target_family_and_site_require_actual_retained_surface_membership() -> Result<(), ()> {
    for family in [true, false] {
        let mut vector = InterpretedVector::declared()?;
        let mut target = vector.target(2)?;
        let field = if family {
            &mut target.family
        } else {
            &mut target.site
        };
        *field.last_mut().ok_or(())? ^= 1;
        vector.projection.mutation.target = target.body();
        assert_eq!(
            read_interpreted(&vector.encoded(), &LIMITS),
            Err(InterpretedArchiveRefusal::TargetJoinMismatch)
        );
    }
    let mut vector = InterpretedVector::declared()?;
    vector.surface.points.first_mut().ok_or(())?.owner = (b"another", b"owner");
    bind_changed_surface(&mut vector)?;
    assert_eq!(
        read_interpreted(&vector.encoded(), &LIMITS),
        Err(InterpretedArchiveRefusal::TargetJoinMismatch)
    );
    Ok(())
}

#[test]
fn active_road_point_alternative_family_site_and_owner_refuse_independently() -> Result<(), ()> {
    for moved in 0u8..6 {
        let mut vector = InterpretedVector::declared()?;
        let mut target = vector.target(1)?;
        match moved {
            0 => *target.identity.first_mut().ok_or(())? = 2,
            1 => *target.identity.get_mut(9).ok_or(())? ^= 1,
            2 => *target.identity.last_mut().ok_or(())? ^= 1,
            3 => *target.family.last_mut().ok_or(())? ^= 1,
            4 => *target.site.last_mut().ok_or(())? ^= 1,
            _ => *target.owner.last_mut().ok_or(())? ^= 1,
        }
        vector.mutation.target = target.body();
        assert_eq!(
            read_interpreted(&vector.encoded(), &LIMITS),
            Err(InterpretedArchiveRefusal::TargetJoinMismatch),
            "move={moved}"
        );
    }
    Ok(())
}

#[test]
fn active_execution_coordinates_and_posture_cannot_substitute_a_different_trial() -> Result<(), ()>
{
    for offset in [8usize, 48, 88, 123, 131, 139, 148] {
        let mut vector = InterpretedVector::declared()?;
        *vector.report.key.get_mut(offset).ok_or(())? ^= 1;
        assert!(
            read_interpreted(&vector.encoded(), &LIMITS).is_err(),
            "offset={offset}"
        );
    }
    let mut vector = InterpretedVector::declared()?;
    vector.report.posture = 2;
    assert_eq!(
        read_interpreted(&vector.encoded(), &LIMITS),
        Err(InterpretedArchiveRefusal::ReportJoinMismatch)
    );
    Ok(())
}

#[test]
fn positive_activation_binds_surface_point_alternative_and_witness_separately() -> Result<(), ()> {
    for moved in 0u8..4 {
        let mut vector = InterpretedVector::declared()?;
        let length = vector.mutation.activation.len();
        let offset = match moved {
            0 => 9,
            1 => 49,
            2 => length.saturating_sub(45),
            _ => length.saturating_sub(5),
        };
        *vector.mutation.activation.get_mut(offset).ok_or(())? ^= 1;
        assert_eq!(
            read_interpreted(&vector.encoded(), &LIMITS),
            Err(InterpretedArchiveRefusal::ActivationJoinMismatch),
            "move={moved}"
        );
    }
    let mut vector = InterpretedVector::declared()?;
    vector.mutation.activation = vector.activation(0)?;
    assert!(read_interpreted(&vector.encoded(), &LIMITS).is_err());
    vector.mutation.activation = vec![2];
    assert_eq!(
        read_interpreted(&vector.encoded(), &LIMITS),
        Err(InterpretedArchiveRefusal::ActivationJoinMismatch)
    );
    Ok(())
}

#[test]
fn surviving_and_every_incomplete_trial_keep_their_actual_mutation_axes() -> Result<(), ()> {
    for (attempt, execution, outcome) in [
        (vec![0], 0, vec![1]),
        (vec![2, 0], 1, vec![2, 3]),
        (vec![3], 2, vec![2, 3]),
        (vec![4, 0, 0], 4, vec![2, 3]),
    ] {
        let mut vector = InterpretedVector::declared()?;
        vector.report.attempt = attempt;
        vector.mutation.execution = execution;
        vector.mutation.outcome = outcome;
        assert!(
            read_interpreted(&vector.encoded(), &LIMITS).is_ok(),
            "execution={execution}"
        );
        vector.report.attempt = vec![0];
        if execution == 0 {
            vector.report.attempt = vec![3];
        }
        assert_eq!(
            read_interpreted(&vector.encoded(), &LIMITS),
            Err(InterpretedArchiveRefusal::Mutation(
                MutationArchiveRefusal::InterpretedTrialMismatch
            ))
        );
    }
    Ok(())
}

#[test]
fn full_active_finding_and_each_inconclusive_axis_cannot_hide_behind_valid_inner_records()
-> Result<(), ()> {
    for (field, distance) in [
        ("foreign", 4usize),
        ("line", 14),
        ("file", 18),
        ("cause", 55),
        ("class", 34),
        ("trial", 101),
    ] {
        let mut vector = InterpretedVector::declared()?;
        let offset = vector.mutation.outcome.len().saturating_sub(distance);
        *vector.mutation.outcome.get_mut(offset).ok_or(())? ^= 1;
        assert!(
            macroonz_harness::muterprater::verdict_archive::read_mutation(
                &vector.mutation.encoded(),
                LIMITS.mutation()
            )
            .is_ok(),
            "inner finding={field}"
        );
        assert_eq!(
            read_interpreted(&vector.encoded(), &LIMITS),
            Err(InterpretedArchiveRefusal::Mutation(
                MutationArchiveRefusal::InterpretedTrialMismatch
            )),
            "finding={field}"
        );
    }
    for moved in 0u8..5 {
        let mut vector = InterpretedVector::declared()?;
        vector.report.attempt = vec![3];
        vector.mutation.execution = 2;
        vector.mutation.outcome = vec![2, 3];
        match moved {
            0 => vector.mutation.baseline = 1,
            1 => vector.mutation.materialization = 1,
            2 => vector.mutation.execution = 3,
            3 => vector.mutation.equivalence = 2,
            _ => vector.mutation.outcome = vec![2, 4],
        }
        assert_eq!(
            read_interpreted(&vector.encoded(), &LIMITS),
            Err(InterpretedArchiveRefusal::Mutation(
                MutationArchiveRefusal::InterpretedTrialMismatch
            )),
            "axis={moved}"
        );
    }
    Ok(())
}

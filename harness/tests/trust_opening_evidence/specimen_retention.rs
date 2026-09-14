//! Exact source retention observed at the independent real compiler-host boundary.

use super::support::{
    COMPILED_SPECIMEN_HOST, CompiledRosterMeaning, EVALUATION, MutationRoadFailure,
    SELECTED_OPERATION, SPECIMEN_HOST_CALLS, SPECIMEN_MATERIALIZER, SPECIMEN_MATERIALIZER_CALLS,
    active_selection, family, invocation, lock_specimen_tests, pair, qualification_of,
    qualified_no_mutation, surface_with, witness,
};
use macroonz_harness::muterprater::specimen::demonstrate_compiled_projection;
use macroonz_harness::muterprater::{
    ArtifactContent, CompiledSpecimenHostRefusal, CompiledSpecimenObservation,
    CompiledSpecimenRequest, CompiledSpecimenRole, SpecimenMaterializerBinding,
};
use macroonz_harness::report::ForeignText;
use std::sync::Mutex;
use std::sync::atomic::Ordering;

static REQUEST_CONTENT: Mutex<Vec<(CompiledSpecimenRole, ArtifactContent)>> =
    Mutex::new(Vec::new());

fn observed_host(
    request: CompiledSpecimenRequest<'_, '_, [u32; 3]>,
) -> Result<CompiledSpecimenObservation<CompiledRosterMeaning>, CompiledSpecimenHostRefusal> {
    REQUEST_CONTENT
        .lock()
        .map_err(|error| {
            CompiledSpecimenHostRefusal::Execution(ForeignText::admitted(
                error.to_string().as_bytes(),
            ))
        })?
        .push((request.role(), request.content().clone()));
    COMPILED_SPECIMEN_HOST(request)
}

/// Pressure retains the exact bytes delivered to each real compiler-host request after both calls return.
///
/// The expected content is captured from requests, without rerunning the source materializer.
/// Distinct roles and identities prevent substituting one source for the other, and counters exclude new materializer or host effects during inspection.
/// The existing pinned compiler host and independent witness remain unchanged.
#[test]
fn completed_projection_retains_both_exact_host_request_buffers() -> Result<(), MutationRoadFailure>
{
    let _specimen_guard = lock_specimen_tests()?;
    REQUEST_CONTENT
        .lock()
        .map_err(|_| MutationRoadFailure::NativeToolchain)?
        .clear();
    let family = family("retained-specimen-content")?;
    let surface = surface_with(family, vec![SELECTED_OPERATION])?;
    let pair = pair(family, &surface, EVALUATION)?;
    let input = [1u32, 0, 0];
    let standing = qualified_no_mutation(&pair, witness()?, &input)?;
    let qualification = qualification_of(&standing)?;
    let selection = active_selection(&surface)?;
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    let pressure = {
        let materializer = SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER);
        let invocation = invocation()?;
        demonstrate_compiled_projection(
            &surface,
            qualification,
            &materializer,
            selection,
            &invocation,
            observed_host,
        )?
    };
    let captured = core::mem::take(
        &mut *REQUEST_CONTENT
            .lock()
            .map_err(|_| MutationRoadFailure::NativeToolchain)?,
    );
    let [(baseline_role, baseline), (selected_role, selected)] = captured.as_slice() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(*baseline_role, CompiledSpecimenRole::Baseline);
    assert_eq!(*selected_role, CompiledSpecimenRole::Selected(selection));
    assert_ne!(baseline.bytes(), selected.bytes());
    for _inspection in 0u8..2 {
        assert_eq!(pressure.baseline_content(), baseline);
        assert_eq!(pressure.selected_content(), selected);
        assert_eq!(pressure.baseline_artifact(), baseline.identity());
        assert_eq!(pressure.standing().artifact(), selected.identity());
    }
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    Ok(())
}

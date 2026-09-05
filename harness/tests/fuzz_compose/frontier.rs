//! The frontier claims: the neighboring frontier is bounded, unique, and repeatable, its budget is an exact priority prefix, and stable rustc profiles cross generation, novelty, and corpus.

use super::support::{
    FuzzRoadFailure, admit_byte_sequences, external, rustc_profile_request, wait_for_exit,
};
use macroonz_harness::corpus::{SeedInput, pack, warm_start};
use macroonz_harness::descriptor::PopulationRef;
use macroonz_harness::fuzz::{
    CoverageAdmission, CoverageCorpus, FuzzExecution, MutationKind, MutationPlan,
    neighboring_inputs, observe_rustc_profile,
};
use macroonz_harness::generate::{
    ByteSource, CaseWidth, GenerationPlan, InputOrigin, RejectionAllowance, SizeProgression, drive,
};
use macroonz_harness::report::{ByteBudget, CaseBudget, GenerationProfile};
use std::collections::BTreeSet;

#[test]
fn neighboring_frontier_is_bounded_unique_and_repeatable() -> Result<(), FuzzRoadFailure> {
    let plan = MutationPlan::declared(128, 16, vec![b"token".to_vec()]).map_err(external)?;
    let first = neighboring_inputs(&[11, 200], Some(b"peer"), &plan).map_err(external)?;
    let second = neighboring_inputs(&[11, 200], Some(b"peer"), &plan).map_err(external)?;
    assert_eq!(first, second);
    assert!(!first.is_empty());
    assert!(first.len() <= 128);
    assert!(
        first
            .iter()
            .all(|candidate| { !candidate.bytes().is_empty() && candidate.bytes().len() <= 16 })
    );
    let unique = first
        .iter()
        .map(|candidate| candidate.bytes().to_vec())
        .collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), first.len());
    for kind in [
        MutationKind::BitFlip,
        MutationKind::BoundarySubstitution,
        MutationKind::Increment,
        MutationKind::Decrement,
        MutationKind::Delete,
        MutationKind::InsertBoundary,
        MutationKind::Duplicate,
        MutationKind::Splice,
        MutationKind::DictionaryInsert,
    ] {
        assert!(first.iter().any(|candidate| candidate.kind() == kind));
    }
    Ok(())
}

#[test]
fn neighboring_frontier_budget_is_an_exact_priority_prefix() -> Result<(), FuzzRoadFailure> {
    let exhaustive = MutationPlan::declared(512, 16, vec![b"token".to_vec()]).map_err(external)?;
    let full = neighboring_inputs(&[11, 200], Some(b"peer"), &exhaustive).map_err(external)?;
    for limit in 1..=full.len() {
        let budget = u32::try_from(limit).map_err(external)?;
        let bounded =
            MutationPlan::declared(budget, 16, vec![b"token".to_vec()]).map_err(external)?;
        let observed = neighboring_inputs(&[11, 200], Some(b"peer"), &bounded).map_err(external)?;
        let expected = full.iter().take(limit).cloned().collect::<Vec<_>>();
        assert_eq!(observed, expected);
    }
    let over_budget = u32::try_from(full.len().saturating_add(1)).map_err(external)?;
    let exhausted =
        MutationPlan::declared(over_budget, 16, vec![b"token".to_vec()]).map_err(external)?;
    assert_eq!(
        neighboring_inputs(&[11, 200], Some(b"peer"), &exhausted).map_err(external)?,
        full
    );

    let eight = MutationPlan::declared(8, 4, Vec::new()).map_err(external)?;
    let bit_prefix = neighboring_inputs(&[0], None, &eight).map_err(external)?;
    assert_eq!(bit_prefix.len(), 8);
    assert!(
        bit_prefix
            .iter()
            .all(|candidate| candidate.kind() == MutationKind::BitFlip)
    );
    Ok(())
}

#[test]
fn stable_rustc_profiles_cross_generation_novelty_and_corpus() -> Result<(), FuzzRoadFailure> {
    let (ready, run) = rustc_profile_request("feedback")?;
    let Some(population) = PopulationRef::named("harness", "rustc-profile-seeds").ok() else {
        return Err(FuzzRoadFailure::Fixture);
    };
    let seeds = vec![SeedInput::declared(vec![0]).map_err(external)?];
    let supplied = pack(population, seeds).map_err(external)?;
    let mut coverage = CoverageCorpus::opening(&ready);

    for origin in warm_start(&supplied) {
        let InputOrigin::Supplied(material) = origin else {
            return Err(FuzzRoadFailure::Fixture);
        };
        let width = CaseWidth::declared(material.len()).map_err(external)?;
        let bytes = u64::try_from(material.len()).map_err(external)?;
        let plan = GenerationPlan::declared(
            population,
            GenerationProfile::declared("rustc-profile-candidate", 1),
            InputOrigin::Supplied(material.clone()),
            CaseBudget::declared(1),
            ByteBudget::declared(bytes),
            RejectionAllowance::NoRejections,
            SizeProgression::Constant { width },
        )
        .map_err(external)?;
        let source = ByteSource::of_plan(&plan);
        let generated = drive::<u8>(
            &plan,
            &source,
            macroonz_harness::generate::decode_arbitrary::<u8>,
            admit_byte_sequences,
        );
        let [candidate] = generated.sequences() else {
            return Err(FuzzRoadFailure::Fixture);
        };
        assert_eq!(candidate.input(), material.as_slice());
        let result =
            observe_rustc_profile(&ready, &mut coverage, candidate.input(), wait_for_exit)?;
        assert_eq!(result.execution(), FuzzExecution::Success);
        match coverage.admit(result)? {
            CoverageAdmission::Interesting(_) => {}
            CoverageAdmission::Known => return Err(FuzzRoadFailure::Fixture),
        }
    }

    let mutation = MutationPlan::declared(8, 4, Vec::new()).map_err(external)?;
    let neighbors = neighboring_inputs(&[0], None, &mutation).map_err(external)?;
    let mut known = 0usize;
    for candidate in &neighbors {
        let result =
            observe_rustc_profile(&ready, &mut coverage, candidate.bytes(), wait_for_exit)?;
        assert_eq!(result.execution(), FuzzExecution::Success);
        match coverage.admit(result)? {
            CoverageAdmission::Interesting(_) => {}
            CoverageAdmission::Known => known = known.saturating_add(1),
        }
    }

    assert_eq!(neighbors.len(), 8);
    assert!(known > 0);
    assert_eq!(coverage.interesting().len(), 4);
    let evolved = coverage
        .interesting()
        .iter()
        .map(|interesting| SeedInput::declared(interesting.as_bytes().to_vec()).map_err(external))
        .collect::<Result<Vec<_>, _>>()?;
    let retained = pack(population, evolved).map_err(external)?;
    assert_eq!(retained.seeds().len(), 4);
    assert_eq!(
        retained
            .seeds()
            .iter()
            .map(SeedInput::bytes)
            .collect::<Vec<_>>(),
        vec![&[0][..], &[1][..], &[2][..], &[0x80][..]]
    );
    run.removed()?;
    Ok(())
}

//! Multi-row generation, exhaustive versus sampled standing, and deterministic replay through the proc entry.

use mh::interleave::{ExplorationMode, InterleavingSpace};
use mh::report::TrialConclusion;

/// Both authored rows compile as ordinary functions and retain their distinct evidence ceilings.
#[test]
fn authored_rows_cross_into_exhaustive_and_sampled_harness_readings() -> Result<(), ()> {
    let strands = super::support::strands().ok_or(())?;
    let contract = super::support::contract().ok_or(())?;

    let (sampled, sampled_conclusion) =
        crate::generated::sampled(&strands, &contract).map_err(|_refusal| ())?;
    assert!(matches!(sampled.mode(), ExplorationMode::Sampled { .. }));
    assert_eq!(sampled_conclusion, TrialConclusion::Passed);

    let (exhaustive, exhaustive_conclusion) =
        crate::generated::exhaustive(&strands, &contract).map_err(|_refusal| ())?;
    assert_eq!(exhaustive.space(), InterleavingSpace::Counted(2u128));
    assert_eq!(exhaustive.mode(), ExplorationMode::Exhaustive);
    assert_eq!(exhaustive_conclusion, TrialConclusion::Passed);

    let (again, again_conclusion) =
        crate::generated::sampled(&strands, &contract).map_err(|_refusal| ())?;
    assert_eq!(again, sampled);
    assert_eq!(again_conclusion, sampled_conclusion);
    Ok(())
}

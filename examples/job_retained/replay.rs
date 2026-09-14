//! Fresh typed execution of the reached Job witness and a separate lawful control.

use macroonz::configuration::v1;
use macroonz::harness::report::replay::ReplayOutcome;
use macroonz::workflow::StoredRun;

pub(super) fn run(saved: &StoredRun) -> Result<(), String> {
    assert_eq!(crate::checks::observations(), (0, 0, 0));
    let binding = crate::trials::row::bounded_work().map_err(crate::debug)?;
    let replayed = saved
        .replay(
            0,
            &binding,
            &crate::checks::decoder().map_err(crate::debug)?,
            crate::invocation(),
            v1::input_limits(),
        )
        .map_err(crate::debug)?;
    let comparison = replayed.comparison().as_ref().map_err(crate::debug)?;
    assert_eq!(comparison.outcome(), ReplayOutcome::DefectReproduced);
    assert_eq!(replayed.witness().payload(), &[4]);
    assert_eq!(crate::checks::observations(), (1, 1, 0));
    crate::checks::passed(&crate::run(&[1])?)?;
    assert_eq!(crate::checks::observations(), (2, 2, 0));
    crate::write(&macroonz::presentation::replay_comparison(comparison).json())?;
    crate::write("DefectReproduced; witness: decodes=1 checks=1 probes=0; lawful [1]: passed")
}

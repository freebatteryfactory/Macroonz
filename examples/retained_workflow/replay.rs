//! Fresh execution chooses current code independently of the stored historical record.

use macroonz::configuration::v1;
use macroonz::harness::descriptor::{Binding, ExecutableAttachment};
use macroonz::harness::input::BoundInput;
use macroonz::harness::report::replay::ReplayOutcome;
use macroonz::harness::runner::TrialBinding;
use macroonz::workflow::StoredRun;

pub(super) fn original(saved: &StoredRun) -> Result<(), String> {
    run(saved, &declared()?, ReplayOutcome::DefectReproduced)
}

pub(super) fn corrected(saved: &StoredRun) -> Result<(), String> {
    let declared = declared()?;
    let row = declared.row().clone();
    let attachment = ExecutableAttachment::attached(
        row.subject(),
        row.check(),
        crate::checks::revision(b"count-every-element-v2"),
        crate::checks::revision(b"slice-length-v1"),
        crate::checks::corrected,
    );
    let binding = Binding::bound(row, attachment, declared.provenance()).map_err(crate::debug)?;
    run(saved, &binding, ReplayOutcome::FixedOnWitness)
}

fn declared() -> Result<TrialBinding<BoundInput<Vec<u8>>>, String> {
    let table = crate::trials::table().map_err(crate::debug)?;
    table
        .bindings()
        .first()
        .cloned()
        .ok_or_else(|| "declared count trial missing".to_owned())
}

fn run(
    saved: &StoredRun,
    binding: &TrialBinding<BoundInput<Vec<u8>>>,
    expected: ReplayOutcome,
) -> Result<(), String> {
    assert_eq!(crate::checks::observations(), (0, 0, 0));
    let replayed = saved
        .replay(
            0,
            binding,
            &crate::checks::decoder().map_err(crate::debug)?,
            crate::invocation(),
            v1::input_limits(),
        )
        .map_err(crate::debug)?;
    let comparison = replayed.comparison().as_ref().map_err(crate::debug)?;
    assert_eq!(comparison.outcome(), expected);
    assert_eq!(replayed.witness().payload(), &[1]);
    assert_eq!(crate::checks::observations(), (1, 1, 0));
    crate::write(&macroonz::presentation::replay_comparison(comparison).json())?;
    crate::write(&format!("{expected:?}; decodes=1 checks=1 probes=0"))
}

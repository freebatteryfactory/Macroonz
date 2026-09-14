//! The caller's selected historical capsule must name its independently chosen current trial.

use macroonz::harness::report::TrialId;
use macroonz::harness::report::archive::ArchivedCapsule;
use macroonz::workflow::StoredRun;

pub(super) fn capsule(saved: &StoredRun, expected: TrialId) -> Result<&ArchivedCapsule, String> {
    let capsule = saved
        .capsule(0)
        .ok_or("selected row has no saved witness")?;
    if capsule.key().trial().as_bytes() != expected.address().as_bytes() {
        return Err("saved witness belongs to another trial or population".to_owned());
    }
    Ok(capsule)
}

//! Repeated controls execute the observation again rather than replaying its recorded vector.

use super::{Control, WorkFamily};
use macroonz_harness::bench::{WorkRecorder, WorkRecordingRefusal, run_all};
use macroonz_harness::identity::{DomainTag, IdentityProfileVersion};
use macroonz_harness::report::FindingCause;
use std::sync::atomic::{AtomicU64, Ordering};

static OBSERVATIONS: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
struct Counted;

impl WorkFamily for Counted {
    const CHECK: &'static str = "executed-observations";
    const EXECUTION_SUITE: &'static str = "repetition";
    const POPULATION: &'static str = "two-size-axis";
    const PREFLIGHT_REFUSED: FindingCause =
        FindingCause::named("harness.recipe-economics", "counted-preflight");
    const REVISION_TAG: DomainTag = DomainTag::declared(
        "counted-observation-revision",
        IdentityProfileVersion::declared(1),
    );
    const TAG: &'static str = "executed";

    fn stem(self) -> &'static str {
        "counted"
    }

    fn axes(self) -> &'static [u64] {
        &[1, 2]
    }

    fn preflight_stem(self) -> &'static str {
        "counted-preflight"
    }

    fn repeated_stem(self) -> &'static str {
        "counted-repeat"
    }

    fn complexity_stem(self) -> &'static str {
        "counted-work"
    }

    fn claim_stem(self) -> &'static str {
        "executed-not-replayed"
    }

    fn counts(self, axis: u64) -> Vec<u64> {
        OBSERVATIONS.fetch_add(1, Ordering::SeqCst);
        vec![axis, 0]
    }

    fn observation_names() -> &'static [&'static str] {
        &["executions", "refusals"]
    }

    fn preflight() -> Result<(), String> {
        Ok(())
    }
}

fn measured(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    super::record(Counted, axis, 1, recorder)
}

fn repeated(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    super::record(Counted, axis, 2, recorder)
}

#[test]
fn the_control_reexecutes_the_observer_instead_of_replaying_counts() -> Result<(), String> {
    OBSERVATIONS.store(0, Ordering::SeqCst);
    let table = super::table(
        Control::Repeated,
        "counted-repetition",
        "counted-identical",
        (Counted, measured, repeated),
        &[],
    )?;
    let report = run_all(&table, &super::super::invocation()).map_err(super::super::debug)?;
    super::assert_repeated(&report, 1, "counted execution did not qualify")?;
    // Each size has one primary, two control, and one timed call; no warmups.
    assert_eq!(OBSERVATIONS.load(Ordering::SeqCst), 8);
    Ok(())
}

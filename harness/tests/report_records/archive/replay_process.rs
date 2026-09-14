//! A new process independently binds and executes a saved witness without running reduction.

use super::{LIMITS, fixture, process, replay as evidence};
use macroonz_harness::input::BoundInput;
use macroonz_harness::report::TrialConclusion;
use macroonz_harness::report::archive::{
    ArchivedAttempt, ArchivedConclusion, read_capsule, read_trial, retain_trial,
};
use macroonz_harness::report::replay::ReplayOutcome;
use macroonz_harness::runner::{Invocation, replay};
use std::error::Error;
use std::io::Read as _;

fn child(
    call: fn(&Invocation<BoundInput<Vec<u8>>>) -> TrialConclusion,
    expected: ReplayOutcome,
    marker: u8,
) -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(4097)
        .read_to_end(&mut encoded)?;
    let historical = read_capsule(&encoded, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(encoded);
    // This binding is independently chosen by the child, never decoded from the archive.
    let decoder = fixture::decoder().map_err(|()| std::io::Error::other("profile refused"))?;
    let binding = evidence::binding(evidence::moved_revision(), call)
        .map_err(|()| std::io::Error::other("binding refused"))?;
    evidence::reset();
    let current = replay(
        &historical,
        &binding,
        &decoder,
        fixture::invocation(),
        evidence::INPUT_LIMITS,
    )
    .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    assert_eq!(evidence::observations(), (1, vec![vec![1]], 0));
    assert_eq!(
        evidence::reading(&current)
            .map_err(|()| std::io::Error::other("comparison refused"))?
            .outcome(),
        expected
    );
    let retained = retain_trial(current.report(), super::trials::LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    let mut response = vec![1, 1, marker];
    response.extend_from_slice(retained.encoded());
    process::publish(&response)
}

#[test]
#[ignore = "driven by the saved-witness execution process claim"]
fn child_reproduces_defect() -> Result<(), Box<dyn Error>> {
    child(evidence::defective, ReplayOutcome::DefectReproduced, 0)
}

#[test]
#[ignore = "driven by the saved-witness execution process claim"]
fn child_observes_repair() -> Result<(), Box<dyn Error>> {
    child(evidence::fixed, ReplayOutcome::FixedOnWitness, 1)
}

#[test]
fn fresh_process_executes_saved_witness_against_defective_and_fixed_bindings()
-> Result<(), Box<dyn Error>> {
    let historical =
        evidence::historical().map_err(|()| std::io::Error::other("capsule refused"))?;
    let decoder = fixture::decoder().map_err(|()| std::io::Error::other("profile refused"))?;
    let witness = macroonz_harness::input::pack(decoder.profile(), &[1], evidence::INPUT_LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    for (name, marker) in [
        ("archive::replay_process::child_reproduces_defect", 0),
        ("archive::replay_process::child_observes_repair", 1),
    ] {
        let response = process::round_trip(historical.encoded(), name)?;
        assert_eq!(response.get(..3), Some([1, 1, marker].as_slice()));
        let archived = read_trial(
            response.get(3..).ok_or("missing current report")?,
            super::trials::LIMITS,
        )
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
        assert_eq!(archived.key().trial(), historical.key().trial());
        assert_ne!(archived.key().subject(), historical.key().subject());
        assert_eq!(archived.key().check(), historical.key().check());
        assert_eq!(
            archived
                .key()
                .input()
                .ok_or("missing current case")?
                .case()
                .as_bytes(),
            witness.case().address().as_bytes()
        );
        assert_ne!(archived.key().address(), historical.key().address());
        match (marker, archived.attempt()) {
            (0, ArchivedAttempt::Executed(ArchivedConclusion::Refused(finding))) => assert_eq!(
                finding.fingerprint().address(),
                historical.fingerprint().address()
            ),
            (1, ArchivedAttempt::Executed(ArchivedConclusion::Passed)) => {}
            _ => return Err("unexpected fresh-process conclusion".into()),
        }
    }
    Ok(())
}

//! Current checks judge saved selected output without compiling or executing its subject.

use super::{capture, types::CapturedResult};
use macroonz::harness::input::BoundInput;
use macroonz::harness::muterprater::proposal_archive::{ArchivedProposal, ArchivedProposalGround};
use macroonz::harness::report::replay::{ReplayNonReproduction, ReplayOutcome};
use macroonz::harness::runner::{Invocation, TrialBinding, replay};
use std::io::Write;

pub(super) fn original(record: &ArchivedProposal, invocation: Invocation) -> Result<(), String> {
    run(
        record,
        &capture::candidate()?,
        invocation,
        ReplayOutcome::DefectReproduced,
    )
}

pub(super) fn weakened(record: &ArchivedProposal, invocation: Invocation) -> Result<(), String> {
    run(
        record,
        &capture::weakened()?,
        invocation,
        ReplayOutcome::NotReproduced(ReplayNonReproduction::PassedWithoutRepairStanding),
    )
}

fn run(
    record: &ArchivedProposal,
    binding: &TrialBinding<BoundInput<CapturedResult>>,
    invocation: Invocation,
    expected: ReplayOutcome,
) -> Result<(), String> {
    let ArchivedProposalGround::MutantKilled(ground) = record.ground() else {
        return Err("expected the retained mutant-killed proposal".to_owned());
    };
    assert_eq!(capture::observations(), (0, 0));
    let replayed = replay(
        ground.capsule(),
        binding,
        &capture::decoder()?,
        invocation,
        capture::LIMITS,
    )
    .map_err(super::debug)?;
    let comparison = replayed.comparison().as_ref().map_err(super::debug)?;
    assert_eq!(comparison.outcome(), expected);
    assert_eq!(capture::observations(), (1, 1));
    let mut output = std::io::stdout();
    writeln!(
        output,
        "{}",
        macroonz::presentation::trial(replayed.report()).json()
    )
    .map_err(super::debug)?;
    writeln!(
        output,
        "{}",
        macroonz::presentation::replay_comparison(comparison).json()
    )
    .map_err(super::debug)?;
    writeln!(
        output,
        "{expected:?}; decodes=1 checks=1 compiled-executions=0 admissions=0"
    )
    .map_err(super::debug)
}

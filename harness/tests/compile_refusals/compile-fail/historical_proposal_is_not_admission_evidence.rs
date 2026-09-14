//! Historical proposals and their grounds cannot become live admission or mutation evidence.

use macroonz_harness::muterprater::proposal_archive::{ArchivedKillGround, ArchivedProposal};
use macroonz_harness::muterprater::verdict_archive::{ArchivedActivationReading, ArchivedMutationTarget};
use macroonz_harness::muterprater::{ActivationEvidence, MutationTarget, ProposalDocument, ReplayBearingProposal};
use macroonz_harness::report::TrialReport;

fn needs_proposal(_proposal: &impl ProposalDocument) {}
fn needs_replay(_proposal: &impl ReplayBearingProposal) {}

fn promote_proposal(historical: &ArchivedProposal) {
    needs_proposal(historical);
    needs_replay(historical);
}

fn promote_target(historical: ArchivedMutationTarget) -> MutationTarget { historical }
fn promote_activation(historical: ArchivedActivationReading) -> ActivationEvidence { historical }
fn promote_trial(historical: &ArchivedKillGround) -> TrialReport { historical.trial_report().clone() }

fn main() {}

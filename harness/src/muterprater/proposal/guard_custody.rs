//! The custody roads: where a sink stored a proposal, and what a completed human admission retains.

use crate::depot::capsules::{ReplayCapsuleEntry, StoredReplayEntryRef};
use crate::descriptor::{ProposalId, Row};
use crate::muterprater::proposal::types::{
    DischargeAdmissionReceipt, ReplayAdmissionReceipt, SinkRefusal, StoredProposalRef,
};

// ---------------------------------------------------------------------------
// Custody, and the admission receipts.
// ---------------------------------------------------------------------------

impl StoredProposalRef {
    /// Bind a sink's storage location to the proposal it stored.
    ///
    /// # Errors
    ///
    /// Refuses an empty token, which names nowhere.
    pub fn at(proposal: ProposalId, token: &str) -> Result<Self, SinkRefusal> {
        if token.is_empty() {
            return Err(SinkRefusal::EmptyLocation);
        }
        Ok(Self {
            proposal,
            token: token.to_owned(),
        })
    }

    /// The content identity of the proposal stored at this location.
    #[must_use]
    pub const fn proposal(&self) -> ProposalId {
        self.proposal
    }

    /// The token, for a sink to read its own location back.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}

impl ReplayAdmissionReceipt {
    /// Retain the exact outputs of one completed replay-bearing human admission.
    #[must_use]
    pub(in crate::muterprater) fn completed(
        row: Row,
        entry: ReplayCapsuleEntry,
        proposal_custody: StoredProposalRef,
        replay_custody: StoredReplayEntryRef,
    ) -> Self {
        Self {
            row,
            entry,
            proposal_custody,
            replay_custody,
        }
    }

    /// The row whose candidate origin became human-admitted provenance.
    #[must_use]
    pub const fn row(&self) -> &Row {
        &self.row
    }

    /// The exact capsule entry the human admission stored.
    #[must_use]
    pub const fn entry(&self) -> &ReplayCapsuleEntry {
        &self.entry
    }

    /// The caller's review-durable custody of the proposal.
    #[must_use]
    pub const fn proposal_custody(&self) -> &StoredProposalRef {
        &self.proposal_custody
    }

    /// The caller's storage location for the replay entry.
    #[must_use]
    pub const fn replay_custody(&self) -> &StoredReplayEntryRef {
        &self.replay_custody
    }
}

impl DischargeAdmissionReceipt {
    /// Retain the outputs of one completed obligation-discharge human admission.
    #[must_use]
    pub(in crate::muterprater) fn completed(row: Row, proposal_custody: StoredProposalRef) -> Self {
        Self {
            row,
            proposal_custody,
        }
    }

    /// The row whose candidate origin became human-admitted provenance.
    #[must_use]
    pub const fn row(&self) -> &Row {
        &self.row
    }

    /// The caller's review-durable custody of the proposal.
    #[must_use]
    pub const fn proposal_custody(&self) -> &StoredProposalRef {
        &self.proposal_custody
    }
}

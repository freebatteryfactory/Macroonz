#![doc = include_str!("README.md")]

mod encode;
mod size;
mod type_contract;
mod types;

pub use encode::{retain_claim_pin, retain_mutant_kill, retain_obligation_discharge};
pub use types::{
    ArchivedDischargeGround, ArchivedKillGround, ArchivedPinGround, ArchivedProposal,
    ArchivedProposalGround, PROPOSAL_ARCHIVE_TAG, ProposalArchiveLimits, ProposalArchiveRefusal,
    read_proposal,
};

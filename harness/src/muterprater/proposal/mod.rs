#![doc = include_str!("README.md")]

pub mod propose;
mod type_contract;
mod types;

pub use types::{
    CandidateSketch, CheckGap, ClaimPinnedGround, ClaimPinnedProposal, Demonstration,
    DischargeAdmissionReceipt, DischargeEvidence, DischargeProposalRefusal, DuplicateRefusal,
    ExplanationRefusal, FailureComparison, HumanAdmissionRefusal, InferredObligation,
    KillProposalRefusal, MutantKilledGround, MutantKilledProposal, NoComparison,
    NoComparisonReason, ObligationComparison, ObligationDischargedGround,
    ObligationDischargedProposal, ObligationLane, OracleClass, OwedClaim, OwedClaimRefusal,
    OwedDeclaration, PROPOSAL_TAG, ProofDelta, ProofDeltaRefusal, ProofRefusal, ProofShape,
    ProposalDestination, ProposalDocument, ProposalRefusal, ProposalSink, ReplayAdmissionReceipt,
    ReplayBearingProposal, SinkRefusal, StoredProposalRef, SurvivorExplanation, SynthesisRefusal,
};

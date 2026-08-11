//! Band 23 — evidence: the receipt matrix, the verification tuple, routes
//! and independence, the diagnostic posture, calibration, the lifecycles.

pub mod types;

pub use types::{
    AdoptionDecisionReceipt, Basis, CalibrationEvidence, CalibrationModel, CauseDisposition,
    CommitmentLayers, Coverage, DiagnosticCause, DiagnosticCauseCandidates, EVIDENCE_NON_COLLAPSE,
    EXPLANATION_LADDER, Enforcement, EvidenceCarriage, GeneratedPublicationReceipt, Lane,
    LaneDomain, Method, QualificationTerminal, RECEIPT_FAMILIES, ReleaseEvidence, Route,
    SubstrateDisclosure, VerificationDenominator, VerificationResult, VerificationTerminal,
    VerifiedClaim,
};

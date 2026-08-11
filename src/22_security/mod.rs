//! Band 22 — security: the lease, revocation distribution, shred, secrets,
//! mechanism standing, trust-boundary disclosure, supply-chain law.

pub mod types;

pub use types::{
    CRYPTO_ROLES, CapabilityLease, FIREWALL_ACT_TABLE, ForeignExecution, HOSTILE_FAMILIES,
    LabelArrow, LeaseRenewalAuthority, MechanismAdmissionFact, MechanismQualificationFact,
    MechanismRetirementFact, MechanismStandingView, MechanismSupportFact, REVOCATION_DEFAULTS,
    RevocationAcknowledgement, RevocationEvidence, RevocationObservation, SECRET_CAPABILITIES,
    SecretAuthorityBackend, SecretUseHandle, ShredDenominatorRow, ShredEvidence, ShredProgress,
    ShredRowStatus, TRUST_BOUNDARY_MEMBERS, WitnessRole,
};

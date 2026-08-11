//! Band 06 — authority: the capability and `KeyScope` value algebra, protected
//! resolution, release-contract shapes, and the postcondition matrix. The
//! machine's first collection-shaped refusal family lives here.

pub mod types;

pub use types::{
    Attenuation, AttenuationAxis, AttenuationLimit, AuthMethod, Authentication, CapabilityClaim,
    CapabilityClaimConstruction, CapabilityClaimConstructionIssue, CapabilityGrant,
    CapabilityGrantId, CapabilityGrantRole, ClaimIssueLimit, ClaimMemberLimit,
    ConstraintSourcePair, Credential, CredentialLimit, DelegationLimit, DelegationLink,
    ForeignSurface, GrantUseHandle, InformationReleaseAuthority, InformationReleaseBounds,
    InformationReleaseClassification, InformationReleaseContract, InformationReleaseEvidence,
    InformationReleaseProjection, InformationReleaseRetention, InformationReleaseSource, KeyScope,
    KeyScopeLimit, OperationAdmission, POSTCONDITION_NON_SUBSTITUTIONS, ProofOfPossession,
    ProtectedResolution, ReleaseClaim, ScopeComponent, ThreatProfile, ThreatProfileLimit,
    ThreatProfileRow, ThreatSubjectLimit, TrustPosture,
};

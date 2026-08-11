//! Band 12 — port: the port contract algebra and host-obligation shapes.

pub mod types;

pub use types::{
    AdmittedForeign, DeadlineExpiryPosture, ForeignClaim, HOST_OBLIGATION_AXES, OutboundBounds,
    OutboundExternalOperation, PortBoundsDeclaration, PortEffectPosture, PortFamilyId,
    PortFamilyVersion, PortOperation, PortPostcondition, PortPostconditionLimit, PortRole,
    RESULT_PROJECTION_AXES, ResponseBinding, SELF_DESCRIBING_REFUSAL_STATEMENTS,
    SecretAuthorityVerb,
};

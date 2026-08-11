//! Band 21 — application: the identity model, composition, invocation
//! profiles, the global interaction contract, and the ingress paved road.

pub mod types;

pub use types::{
    APPLICATION_VALIDATION_LADDER, AUTH_ROLES, AckProfile, ActivationGeneration,
    ActivationImageBinding, AppImageDigest, AppImageRef, AppSemanticCommitment,
    CONTRACT_COMPONENTS, CarrierObservation, CarrierRequestId, DeliveryDirection,
    DeliveryGuarantee, DeliveryIndex, DirectionState, EARLY_DATA_NEVER, EntrypointId, FAMILY_FACTS,
    FLOW_CONTROL_FACTS, INGRESS_PIPELINE, IdempotencyIdentity, IngressAck, InstanceId,
    InstanceLifecycle, InterfaceCommitment, InvocationProfile, LOCAL_NOUNS, LagOverrunObservation,
    MESSAGE_FAMILIES, NON_IDENTITIES, NON_SUBSTITUTABLE_PREIMAGES, RAW_RETENTION_GUARDRAILS,
    REMOTE_VERBS, RESOURCE_NEVER_BECOMES, RESTRICTED_QUERY_OPERATIONS, RejectedContentReason,
    RemovalHole, ResourceRef, SessionId, SessionState, SessionTerminal, StreamClosure, StreamState,
};

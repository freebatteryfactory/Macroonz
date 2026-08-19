#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::{remote_surface_plan, surface_availability};
pub use render::{
    CLOSED_BINDING, ENTRY_PARAMETER, OPENED_BINDING, SERVED_BINDING, SURFACE_SENTENCE, answered,
    associated, attribute, bound, checked_call, doc_attribute, group, language_path, pairing_call,
    result_type, statement, surface_entry, surface_road, type_path, unbounded,
};
pub use type_contract::{
    PAIRING_CONTRACT, PairingContract, REMOTE_SURFACE_CONTRACT_MINT, SurfaceFacing, facing,
};
pub use types::{
    CodecPairing, IntegrationTargetLanding, PairedCodecRoad, RemoteSurface,
    RemoteSurfaceDeclarationRefusal, RemoteSurfaceIssue, RemoteSurfacePlan, RemoteSurfaceShape,
    SurfaceAvailability, SurfaceContractMint, SurfacePathRooting, SurfacePathSegmentLimit,
    SurfaceSignature, SurfaceTypePath, is_surface_identifier,
};

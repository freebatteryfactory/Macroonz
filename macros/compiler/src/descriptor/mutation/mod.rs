#![doc = include_str!("README.md")]

mod capture;
mod complete;
mod render;
mod type_contract;
mod types;

pub use capture::captured;
pub use complete::{completed, completed_from_order};
pub use render::generated_module;
pub use types::{
    ALTERNATIVE_LIMIT, Address, Alternative, DECLARED_ORDER_FAMILY, Declaration, FactMapping,
    FamilySlug, MAPPING_LIMIT, MUTATION_HELPER_POSITION, MutationCaptureError, MutationSurface,
    OPERATOR_FAMILY_LIMIT, PERMISSION_LIMIT, Permission, Policy, Site, Surface, SurfaceRole,
};

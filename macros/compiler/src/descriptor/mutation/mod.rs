#![doc = include_str!("README.md")]

mod capture;
mod codec_order;
mod complete;
mod recipe_changes;
mod render;
mod type_contract;
mod types;

pub use capture::captured;
pub use codec_order::completed_from_codec_order;
pub use complete::{completed, completed_from_order};
pub use recipe_changes::{completed_from_effect_bindings, completed_from_transition_targets};
pub use render::generated_module;
pub use types::{
    ALTERNATIVE_LIMIT, Address, Alternative, CodecMutationError, DECLARED_ORDER_FAMILY,
    Declaration, EFFECT_BINDING_FAMILY, FactMapping, FamilySlug, MAPPING_LIMIT,
    MUTATION_HELPER_POSITION, MutationCaptureError, MutationSurface, OPERATOR_FAMILY_LIMIT,
    PERMISSION_LIMIT, Permission, Policy, RecipeMutationError, Site, Surface, SurfaceRole,
    TRANSITION_TARGET_FAMILY,
};

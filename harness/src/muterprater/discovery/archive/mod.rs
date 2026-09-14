#![doc = include_str!("README.md")]

mod encode;
mod encode_selection;
mod size;
mod types;
mod type_contract;

pub use encode::retain_surface;
pub(crate) use encode_selection::{selection_size, write_selection};
pub(crate) use size::encoded_size as surface_size;
pub use types::ArchivedSelection;
pub(crate) use types::read_selection;
pub use types::{
    ArchivedAlternative, ArchivedEvaluationSurface, ArchivedMutationPoint, SURFACE_ARCHIVE_TAG,
    SurfaceArchiveLimits, SurfaceArchiveRefusal, read_surface,
};

#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::{SurfacePlan, surface_plan};
pub use render::{active_point_enum, evaluation_copy, occurrences, selection, variant_spelling};
pub use types::{
    EvaluationBinding, ImplementationSurface, ImplementationSurfaceComposition,
    ImplementationSurfaceIssue, ImplementationSurfaces, MutationAlternativeLimit, MutationClaimRef,
    MutationEvaluationSurface, MutationOperation, MutationPoint, MutationPointLimit,
    MutationPointName, MutationPointTable, NO_MUTATION_NAMESPACE, NO_MUTATION_STEM,
    NO_MUTATION_VARIANT, NoMutationControl, ProductionSurface, SurfaceDeclarationRefusal,
    SurfaceIssueLimit, SurfaceParity,
};

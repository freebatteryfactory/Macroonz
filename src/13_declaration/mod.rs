//! Band 13 — declaration: the shared authoring algebra — phase roots, name
//! roles, the linker's families, the six facets, staged meta, frontend roles.

pub mod types;

pub use types::{
    AuthoredName, AuthoredNameConstruction, AuthoredNameConstructionIssue, AuthoringRole,
    CANONICAL_FACET_SEQUENCE, CONVERGENCE_ROUTES, ClaimKind, ClosureNamespace,
    ClosureNamespaceIssue, CoordinateRole, DeclarationFragment, DeclarationGraph, ExportAlias,
    ExportAliasDerivation, Facet, FacetForm, FrontendRole, HOW_FACET_CONTENT, HygieneClass,
    LINKER_CONTRACT, LinkResolution, LinkResolutionIssue, META_EVALUATION_LOCKS, MetaStageLaw,
    OriginGraph, ProjectionClaim, ProjectionContract, ProjectionContractConstruction,
    ProjectionContractConstructionIssue, ProjectionProfileId, ProjectionProfileVersion,
    SourceCoordinate, SourceForm, Stage, SymbolIdentity, TopLevelForm, WHAT_FACET_CONTENT,
    WHEN_FACET_CONTENT, WHERE_FACET_CONTENT, WHO_FACET_CONTENT, WHY_FACET_CONTENT,
};

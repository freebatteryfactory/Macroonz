//! Band 08 — schema: the nine-meaning non-collapse, contracts, the schema
//! model, the four value-shape axes, refinements, validation, migration,
//! compatibility, codec profiles, and the seven construction-refusal families.

pub mod types;

pub use types::{
    CheckableObject, CodecConstruction, CodecConstructionIssue, CodecIssueLimit, CodecProfile,
    CompatibilityClaim, CompatibilityEdge, CompatibilityEdgeConstruction,
    CompatibilityEdgeConstructionIssue, CompatibilityIssueLimit, Contract, ContractAxis,
    ContractAxisLimit, ContractConstruction, ContractConstructionIssue, ContractIssueLimit,
    DefaultPolicy, DynamicValue, EdgeDirection, EdgeLimit, FieldCardinality, FieldId, FieldPath,
    FieldPathLimit, FieldRole, ImportCollisionAxis, IssueTextLimit, LayoutConstruction,
    LayoutConstructionIssue, LayoutIssueLimit, MigrationBoundary, MigrationConstruction,
    MigrationConstructionIssue, MigrationIssueLimit, Nullability, PathSegment, PreservationObject,
    ProtectedDataTransformation, REFINEMENT_PROPERTIES, RefinementConstruction,
    RefinementConstructionIssue, RefinementIssueLimit, RefinementKind, SchemaConstruction,
    SchemaConstructionIssue, SchemaDescriptorDigest, SchemaDescriptorRole, SchemaFamilyId,
    SchemaFamilyRole, SchemaIssueLimit, SchemaMeaningDomain, SchemaSemanticCommitment,
    SchemaVersion, TransformDomain, TransformRef, UnknownMemberPolicy, VALIDATION_PIPELINE,
    ValidatedOwned, ValidatedView, ValidationIssue, ValidationStage, ValueShapeAxis, VariantId,
    VariantRole, WeakeningKind,
};

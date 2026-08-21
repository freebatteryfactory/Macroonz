#![doc = include_str!("README.md")]

pub mod encode;
pub mod gate;
pub mod stamp;
mod type_contract;
pub mod types;

pub use encode::encode_generated_support_schema;
pub use gate::PUBLISHED_GENERATED_SUPPORT_SCHEMA_ID;
pub use types::{
    AdmissionFacts, AdmissionGround, AuthoredTable, AuthoredTableName, AuthoredTableRefusal,
    BENCH_FIELDS, BenchSchema, Binding, BindingRefusal, CanonicalRowBytes, CapsulePosture,
    CheckRef, ClaimRef, Classification, ClassificationRefusal, DESCRIPTOR_FIELDS, DescriptorSchema,
    DischargeAdmission, DoorRef, EncodeRefusal, ExecutableAttachment, ExecutionSuite,
    FieldCardinality, FieldShape, GeneratedSupportSchema, GeneratedSupportSchemaId,
    MUTATION_POINT_FIELDS, MutationPointRef, MutationPointSchema, NameRefusal, Namespace,
    NamespacedName, Origin, PopulationRef, ProducerFacts, ProducerName, ProjectionRef, ProposalId,
    Provenance, ReplayAdmission, ReplayBearingGround, ReplayRef, RevisionBinding, RevisionPosture,
    Role, Row, RowRefusal, SchemaField, SchemaRefusal, StagedTableRefusal, StagedTableView, Stem,
    SubjectRoute, SynthesisFacts, TablePosture, TableView, Tag, TrialCoordinates, TrialKey,
    TrialTableRefusal,
};

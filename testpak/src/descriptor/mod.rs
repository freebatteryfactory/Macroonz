#![doc = include_str!("README.md")]

pub mod encode;
mod type_contract;
pub mod types;

pub use encode::encode_generated_support_schema;
pub use types::{
    AdmissionFacts, AdmissionGround, AuthoredTable, AuthoredTableName, AuthoredTableRefusal,
    BenchSchema, Binding, BindingRefusal, CapsulePosture, CheckRef, ClaimRef, Classification,
    ClassificationRefusal, DESCRIPTOR_FIELDS, DescriptorSchema, DoorRef, EncodeRefusal,
    ExecutableAttachment, ExecutionSuite, FieldCardinality, FieldShape, GeneratedSupportSchema,
    GeneratedSupportSchemaId, MutationPointRef, MutationPointSchema, NameRefusal, NamespacedName,
    Origin, PopulationRef, ProducerFacts, ProducerName, ProjectionRef, ProposalId, Provenance,
    ReplayRef, RevisionBinding, RevisionPosture, Role, Row, RowRefusal, SchemaField, SchemaRefusal,
    StagedTableRefusal, StagedTableView, SubjectRoute, SynthesisFacts, TablePosture, TableView,
    Tag, TrialKey,
};

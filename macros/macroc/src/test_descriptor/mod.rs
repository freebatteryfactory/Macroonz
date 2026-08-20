#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::descriptor_plan;
pub use render::{
    ATTACHMENT, ATTACHMENT_ROAD, BINDING, BINDING_ROAD, CHECK_REF, CLAIM_REF, CLASSIFICATION,
    CLASSIFICATION_ROAD, DEFERRED_CLAUSE, DESCRIPTOR_MODULE, DOOR_REF, EXECUTION_SUITE,
    EXPECTED_CLAUSE, GATE_MACRO, HARNESS_CLAUSE, INVOCATION_CLAUSE, NAME_ROAD, NAMED_CLAUSE,
    ORIGIN, ORIGIN_GENERATED, POPULATION_REF, PRODUCER_FACTS, PRODUCER_FACTS_ROAD, PRODUCER_NAME,
    PROJECTION_REF, PROVENANCE, PROVENANCE_CLAUSE, PROVENANCE_PRODUCED, PROVENANCE_PRODUCER_SEAT,
    PROVENANCE_SCHEMA_SEAT, REVISION_BINDING, ROLE_REF, ROW, ROW_ROAD, SCHEMA_IDENTITY,
    SCHEMA_NOT_DECLARED, SCHEMA_NOT_ENCODED, SCHEMA_PUBLISHED, SCHEMA_TYPE, SUBJECT_ROUTE,
    TABLE_REFUSAL, TAG_REF, TRIALS_CLAUSE, attachment, attribute, bound_path, classification,
    declared_row, deferred_module, descriptor_path, documentation, expectation_literal,
    expectation_literal_of, exported_shell, gate_invocation, group, matcher, metavariable,
    name_arguments, named_clause, origin, parsed_name, provenance, revision_binding, roster,
    row_expression, row_schema_identity, stamped_module, suite_group, table_schema_identity,
    trial_cargo, twin_path, unbounded,
};
pub use type_contract::{ROW_CONVERSIONS, RowConversion};
pub use types::{
    ActivePointSelector, BoundPath, CrateFacing, DeferredCargo, DeferredDelivery, DescriptorPlan,
    DescriptorPlanIssue, DescriptorRow, GeneratedSupportShell, PathSegmentLimit, ProducerOrigin,
    RevisionReference, RevisionStanding, RoleLimit, RowAttachment, RowLimit, RowReferences,
    SelectorLimit, ShellDeclarationRefusal, ShellIssueLimit, ShellName, ShellRenderIssue,
    ShellRendering, SuiteGroup, SuiteGroupLimit, TagLimit, TrialDelivery, TrialTablePayload,
    WallName, is_rendered_identifier,
};

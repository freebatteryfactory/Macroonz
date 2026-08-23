#![doc = include_str!("README.md")]

mod capture;
mod plan;
mod render;
mod type_contract;
mod types;

pub use capture::{TRIAL_ATTRIBUTE, captured_trials};
pub use plan::descriptor_plan;
pub use render::{
    ATTACHMENT, ATTACHMENT_ROAD, ATTACHMENTS_CLAUSE, BINDING, BINDING_ROAD, CALL_SEAT, CHECK_REF,
    CHECK_REVISION_SEAT, CLAIM_REF, CLASSIFICATION, CLASSIFICATION_ROAD, CLOCK_CLAUSE,
    DEFERRED_CLAUSE, DESCRIPTOR_MODULE, DOOR_REF, EXECUTION_SUITE, EXPECTED_CLAUSE, GATE_MACRO,
    HARNESS_CLAUSE, HOST_CLAUSES, INVOCATION_CLAUSE, NAME_ROAD, NAMED_CLAUSE, ORIGIN,
    ORIGIN_GENERATED, POPULATION_REF, PRODUCER_FACTS, PRODUCER_FACTS_ROAD, PRODUCER_NAME,
    PROJECTION_REF, PROVENANCE, PROVENANCE_CLAUSE, PROVENANCE_PRODUCED, PROVENANCE_PRODUCER_SEAT,
    PROVENANCE_SCHEMA_SEAT, ROLE_REF, ROW, ROW_ROAD, SCHEMA_IDENTITY, SCHEMA_NOT_DECLARED,
    SCHEMA_NOT_ENCODED, SCHEMA_PUBLISHED, SCHEMA_TYPE, SUBJECT_REVISION_SEAT, SUBJECT_ROUTE,
    TABLE_REFUSAL, TAG_REF, TARGET_CLAUSE, TRIALS_CLAUSE, attachment, attachment_metavariable,
    attribute, bound_local, bound_path, classification, declared_row, descriptor_path,
    documentation, expectation_roster, expectation_roster_of, exported_shell, gate_invocation,
    group, host_clause, matched_attachment, matched_clause, metavariable, name_arguments,
    named_clause, origin, parsed_name, parsed_spelling, provenance, roster, row_expression,
    row_schema_identity, spelled_arguments, stamped_module, stamped_visibility, suite_group,
    table_schema_identity, twin_path, unbounded,
};
pub use type_contract::{ROW_CONVERSIONS, RowConversion};
// The two delivery views the shell rendering reads its seats through. Borrowed
// and crate-internal: the road that consumes them is this crate's own carrier
// rendering, and a caller outside it holds an assembly rather than a view of
// one.
pub use types::{
    BoundPath, CrateFacing, DeclarationDoor, DeferredCargo, DescriptorPlan, DescriptorPlanIssue,
    DescriptorRow, GENERATED_ROW_PROJECTION, GENERATED_TABLE_PRODUCER, GeneratedSupportShell,
    PRODUCER_NAMESPACE, PathSegmentLimit, RoleLimit, RowLimit, RowReferences,
    ShellDeclarationRefusal, ShellIssueLimit, ShellName, ShellRenderIssue, ShellRendering,
    SuiteGroup, SuiteGroupLimit, SupportMacroName, TagLimit, TrialDeclarationCause,
    TrialDeclarationRefusal, TrialLensName, TrialModuleName, TrialSeatName, TrialTablePayload,
    WallName, is_rendered_identifier,
};
pub(crate) use types::{DeferredDelivery, SupportDelivery, TrialDelivery};

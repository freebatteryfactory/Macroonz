#![doc = include_str!("README.md")]

mod capture;
mod render;
mod type_contract;
mod types;

pub use capture::captured;
pub use render::{
    attachment, attachment_metavariable, classification, declared_row, named_clause, origin,
    provenance, row_expression, row_schema_identity, stamped_module, suite_group,
    table_schema_identity,
};
pub use types::{
    ROLE_LIMIT, ROW_LIMIT, References, Row, SUITE_GROUP_LIMIT, SuiteGroup, TAG_LIMIT,
    TRIAL_HELPER_POSITION, TrialAnswer, TrialCaptureError, TrialQuestion, TrialRole, TrialTable,
    Trials,
};

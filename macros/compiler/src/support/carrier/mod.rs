#![doc = include_str!("README.md")]
mod deliver;
mod encode;
mod render;
mod type_contract;
mod types;
pub use deliver::delivered;
pub(crate) use render::rooted_path;
pub use render::{
    EXPECTED_CLAUSE, GATE_MACRO, expectation_roster, exported_shell, gate_invocation,
    matched_clause, matcher, public_alias, rendered_path,
};
pub use types::{ShellError, ShellName, SupportCarrier, SupportShell};

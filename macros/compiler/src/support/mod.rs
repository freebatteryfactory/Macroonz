#![doc = include_str!("README.md")]

mod deliver;
mod encode;
mod establish;
mod render;
mod type_contract;
mod types;

pub use deliver::delivered;
pub use render::{
    EXPECTED_CLAUSE, GATE_MACRO, expectation_roster, exported_shell, gate_invocation,
    matched_clause, matcher, public_alias, rendered_path,
};
pub use types::{
    ASSEMBLY_FACT, ASSEMBLY_ISSUE_LIMIT, AssemblyError, AssemblyIssue, AxisCargo, BoundPath,
    CargoAxis, CrateFacing, DeclarationError, DeclaredCargo, DeferredCargo, DeliveryForm,
    EXPECTED_SCHEMA_ID, PATH_SEGMENT_LIMIT, ProvedCargo, SchemaId, ShellError, ShellName,
    SupportAssembly, SupportAxes, SupportCarrier, SupportName, SupportShell, WallName,
    is_rendered_identifier,
};

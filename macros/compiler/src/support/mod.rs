#![doc = include_str!("README.md")]
mod assembly;
mod cargo;
mod carrier;
mod encode;
mod type_contract;
mod types;
pub use assembly::{
    ASSEMBLY_FACT, ASSEMBLY_ISSUE_LIMIT, AssemblyError, AssemblyIssue, SupportAssembly,
};
pub use cargo::{AxisCargo, CargoAxis, DeclaredCargo, DeferredCargo, ProvedCargo, SupportAxes};
pub(crate) use carrier::rooted_path;
pub use carrier::{
    EXPECTED_CLAUSE, GATE_MACRO, ShellError, ShellName, SupportCarrier, SupportShell, delivered,
    expectation_roster, exported_shell, gate_invocation, matched_clause, matcher, public_alias,
    rendered_path,
};
pub(crate) use types::DeclaringBinding;
pub use types::{
    BoundPath, CrateFacing, DeclarationError, DeliveryForm, EXPECTED_SCHEMA_ID, PATH_SEGMENT_LIMIT,
    SchemaId, SupportName, WallName, rendered_identifier, rendered_name,
};

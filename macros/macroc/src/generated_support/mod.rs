#![doc = include_str!("README.md")]

mod establish;
mod render;
mod type_contract;
mod types;

pub use render::assembled_shell;
pub use types::{
    AccountedExpansion, AssemblyIssue, AssemblyIssueLimit, AxisCargo, CargoAxis, CarrierAssembly,
    DeclaredTrialCargo, JoinedExpansion, ProvedCargo, ShellComposition, SupportAssembly,
};

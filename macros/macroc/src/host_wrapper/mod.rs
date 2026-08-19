#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::{host_wrapper_plan, wrapper_availability};
pub use render::{
    ENTRY_PARAMETER, WRAPPER_SENTENCE, answered, associated, attribute, bound, checked_call,
    composition_order, doc_attribute, group, language_path, result_type, statement, type_path,
    unbounded, wrapper_entry, wrapper_shell,
};
pub use type_contract::{HOST_WRAPPER_CONTRACT_MINT, StageContract, stage, stage_contract};
pub use types::{
    HostTargetLanding, HostWrapperPlan, WrapperAvailability, WrapperComposition,
    WrapperCompositionIssueLimit, WrapperContractMint, WrapperDeclarationRefusal,
    WrapperPathRooting, WrapperPathSegmentLimit, WrapperShape, WrapperStage, WrapperStageLimit,
    WrapperSurface, WrapperSurfaceIssue, WrapperTypePath, is_wrapper_identifier,
};

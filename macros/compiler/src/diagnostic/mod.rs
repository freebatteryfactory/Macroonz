#![doc = include_str!("README.md")]

mod project;
mod type_contract;
mod types;

pub use project::composed;
pub use types::{
    ASSEMBLY_FAMILY, BENCH_HELPER_FAMILY, BINDING_FAMILY, CAPTURE_FAMILY, CLOSURE_FAMILY,
    CODEC_DECLARATION_FAMILY, DECLARATION_FAMILY, DESCRIPTOR_PLAN_FAMILY, Diagnostic, Door,
    EXPLANATION_FAMILY, ExplanationSeat, FIRST_HELPER_FAMILY, Family, Line, LineBody, LineSite,
    Observed, PLANNING_FAMILY, Phase, Placement, RELATED_ISSUE_LIMIT, RENDERING_FAMILY,
    REPAIR_LIMIT, RefusalClass, Refused, RelatedIdentity, RelatedSet, RenderedMagnitude, Repair,
    Route, SECOND_HELPER_FAMILY, SHADOW_HELPER_FAMILY, SHELL_FAMILY, SUPPORT_DECLARATION_FAMILY,
    Site, SiteCoordinate,
};

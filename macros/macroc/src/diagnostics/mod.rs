#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    DiagnosticSite, MachineAnchoring, MachineAnchors, MacrocDiagnostic, MacrocPhase,
    ObservedClassification, RelatedSetCompletion, ReleasePosture, RepairAction, ReproductionRoute,
    SiteCoordinate,
};

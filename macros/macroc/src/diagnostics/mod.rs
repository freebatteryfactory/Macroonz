#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    DiagnosticSite, MachineAnchoring, MachineAnchors, MacrocDiagnostic, MacrocPhase,
    ObservedClassification, RelatedIdentity, RelatedSet, RelatedSetCompletion,
    RelatedSetTruncation, ReleasePosture, RepairAction, ReproductionRoute, SiteCoordinate,
};

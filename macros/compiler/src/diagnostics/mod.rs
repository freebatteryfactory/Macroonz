#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    DiagnosticSite, MacrocDiagnostic, MacrocPhase, ObservedClassification, RelatedIdentity,
    RelatedIssueLimit, RelatedSet, RelatedSetCompletion, RelatedSetTruncation, RepairAction,
    ReproductionRoute, SiteCoordinate,
};

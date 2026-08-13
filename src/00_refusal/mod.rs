//! Band 00 — refusal: the machine's typed "no". See `types` for the shapes and
//! the selector law; see this home's README for the narrative, obligations, and
//! the seed trigger roster.

pub mod types;

pub use types::{
    AdmittedPrefix, AdmittedRefusalFamily, CauseId, CauseOrderDeclaration, CauseOrdinal,
    CompletionPosture, DeclaredCause, DeclaredCauseOrder, FamilyAdmission, FamilyAdmissionCoverage,
    FamilyShape, HandlingClass, LocalCauseKey, OrderAdmission, OrderProjected, ReasonId, Refusal,
    RefusalFamily, RefusalFamilyId, ReportTruncation, ShapeAdmission, ShapeCoherent, StopBound,
    admit_order, admit_shape,
};

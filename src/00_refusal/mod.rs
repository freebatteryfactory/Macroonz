//! Band 00 — refusal: the machine's typed "no". See `types` for the shapes and
//! the selector law; see this home's README for the narrative, obligations, and
//! the seed trigger roster.

pub mod types;

pub use types::{
    CauseId, CauseKey, CauseOrderDeclaration, CauseOrdinal, CompletionPosture, DeclaredCause,
    DeclaredCauseOrder, FamilyShape, HandlingClass, LocalCauseKey, ReasonId, Refusal,
    RefusalFamily, RefusalFamilyId, StopBound,
};

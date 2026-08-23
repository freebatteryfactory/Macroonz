//! The origin-graph home's declarative surface: the closed table a recorded
//! decision is read through.
//!
//! A constant per variant and nothing else.
//! The discriminant is written ahead of a decision's citation, which is what
//! keeps a selection over one fact from ever encoding as an omission over the
//! same fact — so this table is part of the trace's meaning rather than a
//! convenience for the encoder that consumes it.

use super::TraceDecision;

impl TraceDecision {
    /// The decision's discriminant byte, written ahead of its citation so a
    /// selection can never encode as an omission over the same fact.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::SelectedBecause(_) => 0,
            Self::OmittedBecause(_) => 1,
            Self::NotRun => 2,
        }
    }
}

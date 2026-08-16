//! The diagnostics home's declarative surface: the one closed table a site
//! posture is read through.
//!
//! Both roads are total const mappings rather than computations.
//! Nothing here decides anything — the deciding happened where the table was
//! asked.

use super::SiteCoordinate;
use crate::token::SpanResolutionRefusal;
use threadpak::declaration::SourceCoordinate;

impl SiteCoordinate {
    /// The posture one span table's answer takes.
    #[must_use]
    pub const fn answered(answer: Result<SourceCoordinate, SpanResolutionRefusal>) -> Self {
        match answer {
            Ok(coordinate) => Self::Resolved(coordinate),
            Err(refusal) => Self::NotReached(refusal),
        }
    }

    /// The resolved coordinate, where the table reached the handle.
    #[must_use]
    pub const fn resolved(self) -> Option<SourceCoordinate> {
        match self {
            Self::Resolved(coordinate) => Some(coordinate),
            Self::NotReached(_) => None,
        }
    }
}

//! The stated tables of this home's two rosters, and how a refused trail reads.
//!
//! A relation's slot and a decision's discriminant are bytes a canonical encoding carries, so both tables are part of what an origin MEANS rather than a convenience for the encoder that reads them.
//! Each is total: a row admitted later stops the compiler here until somebody says what its name and its byte are.

use super::{OriginRelation, TraceDecision, TrailError};
use crate::bounded::NonEmptyError;

impl OriginRelation {
    /// The complete roster, in slot order.
    pub const ALL: &'static [Self] = &[
        Self::AuthoredDeclaration,
        Self::PatternInstantiation,
        Self::SemanticDerivation,
        Self::ExplicitLink,
        Self::Normalization,
        Self::ProfileSelection,
        Self::ProjectionSelection,
        Self::Rendering,
        Self::TestDerivation,
        Self::BenchmarkDerivation,
        Self::DiagnosticDerivation,
    ];

    /// The relation's declared stable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::AuthoredDeclaration => "authored-declaration",
            Self::PatternInstantiation => "pattern-instantiation",
            Self::SemanticDerivation => "semantic-derivation",
            Self::ExplicitLink => "explicit-link",
            Self::Normalization => "normalization",
            Self::ProfileSelection => "profile-selection",
            Self::ProjectionSelection => "projection-selection",
            Self::Rendering => "rendering",
            Self::TestDerivation => "test-derivation",
            Self::BenchmarkDerivation => "benchmark-derivation",
            Self::DiagnosticDerivation => "diagnostic-derivation",
        }
    }

    /// The byte an edge's canonical bytes carry for this relation.
    ///
    /// A row is APPENDED and never renumbered: renumbering an occupied slot re-encodes trails that were already encoded.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::AuthoredDeclaration => 0,
            Self::PatternInstantiation => 1,
            Self::SemanticDerivation => 2,
            Self::ExplicitLink => 3,
            Self::Normalization => 4,
            Self::ProfileSelection => 5,
            Self::ProjectionSelection => 6,
            Self::Rendering => 7,
            Self::TestDerivation => 8,
            Self::BenchmarkDerivation => 9,
            Self::DiagnosticDerivation => 10,
        }
    }
}

const _: () = assert!(
    slots_are_ordered(OriginRelation::ALL, 0),
    "a relation whose published slot disagrees with its position in the roster",
);

/// Whether every row's published slot is its own position in the roster.
///
/// The slot is what an edge carries and the roster order is what a reader walks; two rows at one slot would encode two relations alike.
const fn slots_are_ordered(relations: &[OriginRelation], at: u8) -> bool {
    match relations.split_first() {
        None => true,
        Some((first, rest)) => first.slot() == at && slots_are_ordered(rest, at.saturating_add(1)),
    }
}

impl TraceDecision {
    /// The discriminant byte, written ahead of a citation so a selection can never encode as an omission over the same fact.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::SelectedBecause(_) => 0,
            Self::OmittedBecause(_) => 1,
            Self::NotRun => 2,
        }
    }
}

impl core::fmt::Display for TrailError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Discontinuous { at } => write!(
                into,
                "the edge at position {at} does not start where the edge before it ended"
            ),
            Self::Empty(empty) => write!(into, "{empty}"),
            Self::Overflow(overflow) => write!(into, "{overflow}"),
        }
    }
}

impl core::error::Error for TrailError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Discontinuous { .. } => None,
            Self::Empty(empty) => Some(empty),
            Self::Overflow(overflow) => Some(overflow),
        }
    }
}

impl From<NonEmptyError> for TrailError {
    fn from(refusal: NonEmptyError) -> Self {
        match refusal {
            NonEmptyError::Empty(empty) => Self::Empty(empty),
            NonEmptyError::Overflow(overflow) => Self::Overflow(overflow),
        }
    }
}

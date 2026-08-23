//! The explanation-protocol home's declarative surface: the shape its refusal
//! family declares, and the closed table an answer's own question is read
//! through.
//!
//! Both are declarations rather than computations.
//! The table is what makes the pairing derived rather than supplied, so a
//! mismatched question-and-answer pair is a value nobody can build.

use super::{ExplanationAnswer, ExplanationCoverage};
use crate::question::ExplanationQuestion;
use macroonz::{FamilyShape, RefusalFamily};

impl RefusalFamily for ExplanationCoverage {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
}

impl ExplanationAnswer {
    /// This answer's position in the declared roster, written ahead of the
    /// answer's own material so two answers never encode alike.
    ///
    /// It is written into an explanation's preimage beside the question's own
    /// slot, and the two are not one fact stated twice: the question is what was
    /// ASKED and the discriminant is which answer SHAPE was given. They agree
    /// today because [`ExplanationAnswer::question`] is one-to-one, and a roster
    /// that ever admitted two answer shapes for one question would separate them
    /// here rather than deriving one preimage for both.
    ///
    /// A position is APPENDED and never renumbered: renumbering an occupied
    /// position re-encodes answers that were already encoded, which renames
    /// every explanation derived over them.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::Kind { .. } => 0,
            Self::Owner { .. } => 1,
            Self::CausingDeclarations { .. } => 2,
            Self::PatternInstance { .. } => 3,
            Self::Profile { .. } => 4,
            Self::AssumptionsAndSpecializations { .. } => 6,
            Self::OutputAndDigest { .. } => 7,
            Self::ChallengingTests { .. } => 8,
            Self::MeasuringBenchmarks { .. } => 9,
            Self::Invalidators { .. } => 11,
            Self::RelatedProjectionDisposition { .. } => 12,
            Self::Repairs { .. } => 13,
        }
    }

    /// The question this answer answers.
    ///
    /// Total, and the only road there is: a pairing between a question and an
    /// answer that does not fit it cannot be built, because the pairing is
    /// derived rather than supplied.
    #[must_use]
    pub const fn question(&self) -> ExplanationQuestion {
        match self {
            Self::Kind { .. } => ExplanationQuestion::WhatAreYou,
            Self::Owner { .. } => ExplanationQuestion::WhichOwnerRequired,
            Self::CausingDeclarations { .. } => ExplanationQuestion::WhichDeclarationCaused,
            Self::PatternInstance { .. } => ExplanationQuestion::WhichPatternInstance,
            Self::Profile { .. } => ExplanationQuestion::WhichProfile,
            Self::AssumptionsAndSpecializations { .. } => {
                ExplanationQuestion::WhichAssumptionsAndSpecializations
            }
            Self::OutputAndDigest { .. } => ExplanationQuestion::WhichOutputIdentityAndDigest,
            Self::ChallengingTests { .. } => ExplanationQuestion::WhichTestsChallenge,
            Self::MeasuringBenchmarks { .. } => ExplanationQuestion::WhichBenchmarksMeasure,
            Self::Invalidators { .. } => ExplanationQuestion::WhatInvalidates,
            Self::RelatedProjectionDisposition { .. } => {
                ExplanationQuestion::WhyWasRelatedProjectionNotGenerated
            }
            Self::Repairs { .. } => ExplanationQuestion::WhatRepairsARefusal,
        }
    }
}

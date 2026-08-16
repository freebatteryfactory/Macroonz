//! The explanation-protocol home's declarative surface: the shape its refusal
//! family declares, and the closed table an answer's own question is read
//! through.
//!
//! Both are declarations rather than computations.
//! The table is what makes the pairing derived rather than supplied, so a
//! mismatched question-and-answer pair is a value nobody can build.

use super::{ExplanationAnswer, ExplanationCoverage};
use crate::question::ExplanationQuestion;
use threadpak::refusal::{FamilyShape, RefusalFamily};

impl RefusalFamily for ExplanationCoverage {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl ExplanationAnswer {
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
            Self::PatternInstance { .. } => ExplanationQuestion::WhichTemplateOrPatternInstance,
            Self::GraphAndProfile { .. } => ExplanationQuestion::WhichGraphAndProfile,
            Self::SelectedWrappers { .. } => ExplanationQuestion::WhichCapabilitiesSelectedWrappers,
            Self::AssumptionsAndSpecializations { .. } => {
                ExplanationQuestion::WhichAssumptionsAndSpecializations
            }
            Self::OutputAndDigest { .. } => ExplanationQuestion::WhichOutputIdentityAndDigest,
            Self::ChallengingTests { .. } => ExplanationQuestion::WhichTestsChallenge,
            Self::MeasuringBenchmarks { .. } => ExplanationQuestion::WhichBenchmarksMeasure,
            Self::CorrespondingRuntimeTraces { .. } => {
                ExplanationQuestion::WhichRuntimeTracesCorrespond
            }
            Self::Invalidators { .. } => ExplanationQuestion::WhatInvalidates,
            Self::RelatedProjectionDisposition { .. } => {
                ExplanationQuestion::WhyWasRelatedProjectionNotGenerated
            }
            Self::Repairs { .. } => ExplanationQuestion::WhatRepairsARefusal,
        }
    }
}

//! The explanation protocol's closed question roster.
//!
//! The fourteen questions every generated thing must be able to answer, and the
//! typed answer to "does this kind admit that question at all". Nothing else
//! lives here: the roster is a vocabulary, not machinery.
//!
//! # Why the roster is its own module
//!
//! Both ends of the protocol need the questions. A projection kind declares its
//! roster while it is being PLANNED, before any explanation exists; the
//! explanation machinery reads that roster while it is being CHECKED, after the
//! plan exists. Left in the machinery module, that pair of needs is a cycle —
//! planning importing explanation, explanation importing planning — and a cycle
//! is a dependency order nobody can state. Seated here, the roster is a leaf
//! both sides import, and the order is a straight line again.
//!
//! The module imports nothing at all, from this crate or from the machine. That
//! is the point: a closed roster of names is the one thing in the plane with no
//! dependencies to have.

/// The version of the explanation protocol this roster states.
///
/// A plain number rather than a typed carrier, because this module imports
/// nothing and that absence is the reason it exists: a leaf that reached for a
/// version type would stop being a leaf.
///
/// Bump it when the protocol changes — a question added, a question removed, or
/// a question that keeps its spelling and asks something else. It is
/// load-bearing where a closure identity is derived: a closure claims that a
/// rendering answers this protocol, and a claim made under a different protocol
/// is a different claim.
pub const EXPLANATION_PROTOCOL_VERSION: u32 = 1;

/// The fourteen questions. A generated thing that cannot answer one of these is
/// a generated thing nobody can hold to account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplanationQuestion {
    /// What are you?
    WhatAreYou,
    /// Which owner required you?
    WhichOwnerRequired,
    /// Which declaration caused you?
    WhichDeclarationCaused,
    /// Which template or pattern instance produced you?
    WhichTemplateOrPatternInstance,
    /// Which graph and profile were you decided under?
    WhichGraphAndProfile,
    /// Which capabilities selected your wrappers?
    WhichCapabilitiesSelectedWrappers,
    /// Which assumptions and specializations do you rest on?
    WhichAssumptionsAndSpecializations,
    /// Which output identity and digest are you?
    WhichOutputIdentityAndDigest,
    /// Which tests challenge you?
    WhichTestsChallenge,
    /// Which benchmarks measure you?
    WhichBenchmarksMeasure,
    /// Which runtime traces correspond to you?
    WhichRuntimeTracesCorrespond,
    /// What invalidates you?
    WhatInvalidates,
    /// Why was a related projection not generated?
    WhyWasRelatedProjectionNotGenerated,
    /// What repairs a refusal?
    WhatRepairsARefusal,
}

/// The declared question roster, in the order the protocol states it.
pub const EXPLANATION_QUESTIONS: [ExplanationQuestion; 14] = [
    ExplanationQuestion::WhatAreYou,
    ExplanationQuestion::WhichOwnerRequired,
    ExplanationQuestion::WhichDeclarationCaused,
    ExplanationQuestion::WhichTemplateOrPatternInstance,
    ExplanationQuestion::WhichGraphAndProfile,
    ExplanationQuestion::WhichCapabilitiesSelectedWrappers,
    ExplanationQuestion::WhichAssumptionsAndSpecializations,
    ExplanationQuestion::WhichOutputIdentityAndDigest,
    ExplanationQuestion::WhichTestsChallenge,
    ExplanationQuestion::WhichBenchmarksMeasure,
    ExplanationQuestion::WhichRuntimeTracesCorrespond,
    ExplanationQuestion::WhatInvalidates,
    ExplanationQuestion::WhyWasRelatedProjectionNotGenerated,
    ExplanationQuestion::WhatRepairsARefusal,
];

impl ExplanationQuestion {
    /// The question's position in the declared roster — what a canonical
    /// encoding of a coverage issue carries for it.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::WhatAreYou => 0,
            Self::WhichOwnerRequired => 1,
            Self::WhichDeclarationCaused => 2,
            Self::WhichTemplateOrPatternInstance => 3,
            Self::WhichGraphAndProfile => 4,
            Self::WhichCapabilitiesSelectedWrappers => 5,
            Self::WhichAssumptionsAndSpecializations => 6,
            Self::WhichOutputIdentityAndDigest => 7,
            Self::WhichTestsChallenge => 8,
            Self::WhichBenchmarksMeasure => 9,
            Self::WhichRuntimeTracesCorrespond => 10,
            Self::WhatInvalidates => 11,
            Self::WhyWasRelatedProjectionNotGenerated => 12,
            Self::WhatRepairsARefusal => 13,
        }
    }

    /// The question as it is asked, for a person to read. A projection of the
    /// typed value: nothing reads it back.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::WhatAreYou => "what are you",
            Self::WhichOwnerRequired => "which owner required you",
            Self::WhichDeclarationCaused => "which declaration caused you",
            Self::WhichTemplateOrPatternInstance => {
                "which template or pattern instance produced you"
            }
            Self::WhichGraphAndProfile => "which graph and profile were you decided under",
            Self::WhichCapabilitiesSelectedWrappers => "which capabilities selected your wrappers",
            Self::WhichAssumptionsAndSpecializations => {
                "which assumptions and specializations do you rest on"
            }
            Self::WhichOutputIdentityAndDigest => "which output identity and digest are you",
            Self::WhichTestsChallenge => "which tests challenge you",
            Self::WhichBenchmarksMeasure => "which benchmarks measure you",
            Self::WhichRuntimeTracesCorrespond => "which runtime traces correspond to you",
            Self::WhatInvalidates => "what invalidates you",
            Self::WhyWasRelatedProjectionNotGenerated => {
                "why was a related projection not generated"
            }
            Self::WhatRepairsARefusal => "what repairs a refusal",
        }
    }
}

/// Whether one kind's plans admit one question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestionApplicability {
    /// The kind's plans answer this question.
    Applicable,
    /// The kind's plans do not admit this question at all.
    NotApplicableToKind,
}

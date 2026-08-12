//! The question home's declarations: the protocol version, the closed roster of
//! fourteen questions, and the typed answer to whether a kind admits one.
//!
//! Declarations only, and the home has no other seat. Nothing here has a private
//! field, so there is no invariant nucleus to guard; nothing here computes, so
//! there is no role file to compute it.

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

threadpak::closed_register! {
    /// The fourteen questions. A generated thing that cannot answer one of
    /// these is a generated thing nobody can hold to account.
    ///
    /// `ALL` is the declared roster in protocol order, `slot` is what a
    /// canonical encoding of a coverage issue carries for a question, and
    /// `described` is the question as it is asked.
    pub enum ExplanationQuestion {
        /// What are you?
        WhatAreYou = "what-are-you", "what are you";
        /// Which owner required you?
        WhichOwnerRequired = "which-owner-required", "which owner required you";
        /// Which declaration caused you?
        WhichDeclarationCaused = "which-declaration-caused",
            "which declaration caused you";
        /// Which template or pattern instance produced you?
        WhichTemplateOrPatternInstance = "which-template-or-pattern-instance",
            "which template or pattern instance produced you";
        /// Which graph and profile were you decided under?
        WhichGraphAndProfile = "which-graph-and-profile",
            "which graph and profile were you decided under";
        /// Which capabilities selected your wrappers?
        WhichCapabilitiesSelectedWrappers = "which-capabilities-selected-wrappers",
            "which capabilities selected your wrappers";
        /// Which assumptions and specializations do you rest on?
        WhichAssumptionsAndSpecializations = "which-assumptions-and-specializations",
            "which assumptions and specializations do you rest on";
        /// Which output identity and digest are you?
        WhichOutputIdentityAndDigest = "which-output-identity-and-digest",
            "which output identity and digest are you";
        /// Which tests challenge you?
        WhichTestsChallenge = "which-tests-challenge", "which tests challenge you";
        /// Which benchmarks measure you?
        WhichBenchmarksMeasure = "which-benchmarks-measure",
            "which benchmarks measure you";
        /// Which runtime traces correspond to you?
        WhichRuntimeTracesCorrespond = "which-runtime-traces-correspond",
            "which runtime traces correspond to you";
        /// What invalidates you?
        WhatInvalidates = "what-invalidates", "what invalidates you";
        /// Why was a related projection not generated?
        WhyWasRelatedProjectionNotGenerated = "why-was-related-projection-not-generated",
            "why was a related projection not generated";
        /// What repairs a refusal?
        WhatRepairsARefusal = "what-repairs-a-refusal", "what repairs a refusal";
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

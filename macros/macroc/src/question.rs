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
//! The module names nothing from this crate, and that is the point: a closed
//! roster of names is the one thing in the plane with no machinery to depend
//! on. The one thing it takes from the machine is the authoring stamp that
//! writes a closed roster down. A stamp decides no meaning, carries no
//! semantic noun, and reaches no band's material, so taking it costs the leaf
//! nothing it was protecting — what the leaf protects is the absence of an edge
//! to another module of THIS crate, and that absence is exactly as complete as
//! it was.

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

//! The constant answers this home's rosters settle, and the contracts a coverage refusal stands under.
//!
//! Each table is total, so a row admitted later stops the compiler in every one of them until somebody says what that row's name, position, sentence, and classification are.
//! The answer-to-question table is what makes the pairing DERIVED rather than supplied: a true answer filed under the wrong question is a value nobody can build.

use super::encode::answer_material;
use super::project::human_line;
use super::{
    ExplanationError, ExplanationIssue, UNIVERSAL_QUESTION_COUNT, UniversalAnswer,
    UniversalQuestion,
};
use crate::bounded::{Bounded, Capping};
use crate::diagnostic::{
    EXPLANATION_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused,
    Repair,
};
use crate::identity::encode_bytes;
use crate::kind::{Answer, Question};
use core::fmt;

const _: () = assert!(
    UNIVERSAL_QUESTION_COUNT == UniversalQuestion::ALL.len(),
    "the universal seat width and the universal roster are one number, stated twice",
);

impl Question for UniversalQuestion {
    const ALL: &'static [Self] = &[
        Self::WhatAreYou,
        Self::WhichOwnerRequired,
        Self::WhichDeclarationCaused,
        Self::WhichProfile,
        Self::WhichOutputAndDigest,
        Self::WhichAssumptions,
        Self::WhatInvalidates,
        Self::WhyRelatedNotGenerated,
        Self::WhatRepairsARefusal,
    ];

    type Answer = UniversalAnswer;

    fn name(self) -> &'static str {
        match self {
            Self::WhatAreYou => "what-are-you",
            Self::WhichOwnerRequired => "which-owner-required",
            Self::WhichDeclarationCaused => "which-declaration-caused",
            Self::WhichProfile => "which-profile",
            Self::WhichOutputAndDigest => "which-output-and-digest",
            Self::WhichAssumptions => "which-assumptions",
            Self::WhatInvalidates => "what-invalidates",
            Self::WhyRelatedNotGenerated => "why-related-not-generated",
            Self::WhatRepairsARefusal => "what-repairs-a-refusal",
        }
    }
}

impl UniversalQuestion {
    /// This question in the words a person asks it.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::WhatAreYou => "what are you",
            Self::WhichOwnerRequired => "which owner required you",
            Self::WhichDeclarationCaused => "which declaration caused you",
            Self::WhichProfile => "which profile were you decided under",
            Self::WhichOutputAndDigest => "which output identity and digest are you",
            Self::WhichAssumptions => "which assumptions do you rest on",
            Self::WhatInvalidates => "what invalidates you",
            Self::WhyRelatedNotGenerated => "why was a related projection not generated",
            Self::WhatRepairsARefusal => "what repairs a refusal",
        }
    }
}

impl Answer for UniversalAnswer {
    type Question = UniversalQuestion;

    fn question(&self) -> UniversalQuestion {
        match self {
            Self::Kind { .. } => UniversalQuestion::WhatAreYou,
            Self::Owner { .. } => UniversalQuestion::WhichOwnerRequired,
            Self::CausingDeclarations { .. } => UniversalQuestion::WhichDeclarationCaused,
            Self::Profile { .. } => UniversalQuestion::WhichProfile,
            Self::OutputAndDigest { .. } => UniversalQuestion::WhichOutputAndDigest,
            Self::Assumptions { .. } => UniversalQuestion::WhichAssumptions,
            Self::Invalidators { .. } => UniversalQuestion::WhatInvalidates,
            Self::RelatedDispositions { .. } => UniversalQuestion::WhyRelatedNotGenerated,
            Self::Repairs { .. } => UniversalQuestion::WhatRepairsARefusal,
        }
    }

    fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        let mut material = Vec::new();
        answer_material(self, &mut material);
        encode_bytes(&material, into);
    }

    fn human(&self) -> String {
        human_line(self)
    }
}

impl UniversalAnswer {
    /// This answer's position in the declared roster, written ahead of its own material.
    ///
    /// Not the question's position stated twice: the question is what was ASKED and this is which answer SHAPE was given.
    /// They agree today because the table above is one-to-one, and a roster that ever admitted two shapes for one question would separate them here rather than deriving one preimage for both.
    /// A position is appended and never renumbered.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::Kind { .. } => 0,
            Self::Owner { .. } => 1,
            Self::CausingDeclarations { .. } => 2,
            Self::Profile { .. } => 3,
            Self::OutputAndDigest { .. } => 4,
            Self::Assumptions { .. } => 5,
            Self::Invalidators { .. } => 6,
            Self::RelatedDispositions { .. } => 7,
            Self::Repairs { .. } => 8,
        }
    }
}

impl ExplanationIssue {
    /// This row's position in the declared roster, written ahead of the issue's own material.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::UniversalUnanswered { .. } => 0,
            Self::UniversalAnsweredTwice { .. } => 1,
            Self::DeclaredUnanswered { .. } => 2,
            Self::DeclaredAnsweredTwice { .. } => 3,
            Self::QuestionOutsideRoster { .. } => 4,
            Self::SeatBoundExceeded { .. } => 5,
            Self::OutputsBesideTheProof { .. } => 6,
        }
    }

    /// How what this issue observed differs from the contract that was expected.
    #[must_use]
    pub const fn observed(&self) -> Observed {
        match self {
            Self::UniversalUnanswered { .. } | Self::DeclaredUnanswered { .. } => {
                Observed::SeatAbsent
            }
            Self::UniversalAnsweredTwice { .. }
            | Self::DeclaredAnsweredTwice { .. }
            | Self::QuestionOutsideRoster { .. }
            | Self::OutputsBesideTheProof { .. } => Observed::ContractDisagreement,
            Self::SeatBoundExceeded { .. } => Observed::BoundExceeded,
        }
    }
}

impl fmt::Display for ExplanationIssue {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UniversalUnanswered { question } => write!(
                into,
                "the universal question \"{}\" has no answer",
                question.described()
            ),
            Self::UniversalAnsweredTwice { question } => write!(
                into,
                "the universal question \"{}\" was answered more than once",
                question.described()
            ),
            Self::DeclaredUnanswered { question, slot } => write!(
                into,
                "the kind's question \"{question}\" at position {slot} has no answer"
            ),
            Self::DeclaredAnsweredTwice { question, slot } => write!(
                into,
                "the kind's question \"{question}\" at position {slot} was answered twice or more"
            ),
            Self::QuestionOutsideRoster { question } => write!(
                into,
                "an answer names the question \"{question}\", which its own roster does not carry"
            ),
            Self::SeatBoundExceeded { bound, observed } => {
                write!(into, "{observed} seats offered where {bound} are declared")
            }
            Self::OutputsBesideTheProof {
                expected,
                observed,
                diverges,
            } => write!(
                into,
                "the output answer carries {observed} rows beside the proof's {expected}, diverging at roster position {diverges}"
            ),
        }
    }
}

impl fmt::Display for ExplanationError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(into, "{}", self.first_issue())?;
        let further = self.issues().count().saturating_sub(1);
        if further > 0 {
            write!(into, ", and {further} further issues")?;
        }
        if let Capping::Truncated { omitted } = self.capping() {
            write!(into, ", {omitted} of them not carried")?;
        }
        Ok(())
    }
}

impl core::error::Error for ExplanationError {}

impl Refused for ExplanationError {
    const PHASE: Phase = Phase::Explanation;
    const FAMILY: Family = EXPLANATION_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::ExplanationNotCovered
    }

    fn first(&self) -> String {
        self.first_issue().to_string()
    }

    fn observed(&self) -> Observed {
        self.first_issue().observed()
    }

    fn body(&self) -> LineBody {
        let further = self.issues().count().saturating_sub(1);
        let capping = self.capping();
        if further == 0 && capping == Capping::Complete {
            LineBody::SingleCause
        } else {
            LineBody::Body { further, capping }
        }
    }

    /// The issues established beyond the primary cause; the primary is the summary's own subject, never a member of its related set.
    fn related(&self) -> Vec<Vec<u8>> {
        self.issues()
            .iter()
            .skip(1)
            .map(ExplanationIssue::canonical_bytes)
            .collect()
    }

    /// This home declares no repair of its own.
    ///
    /// Every issue above is about which questions the caller answered, so the repair is that answer sheet; a sentence composed here would be this compiler citing a fact nobody declared.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}

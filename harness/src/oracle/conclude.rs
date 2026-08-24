//! The causes each lane owns, and the one road from each verdict into the record vocabulary.
//!
//! Each lane concludes in its own vocabulary and the harness records failures in one, so the translation is stated once per finding rather than composed at whichever call site noticed it.
//! A caller spelling a cause inline would mint a second name for a failure this home already named, and two names for one failure are two rows in every count that groups by cause.
//!
//! Every arm is a constant answer per variant, declared rather than derived.
//! Which class a failure normalizes to is stated exactly once, in this file's own `refused`, because [`FailureClass::OracleDisagreement`] is true of every finding here.

use super::types::{
    CompiledDisagreement, CompiledVerdict, ORACLE_CAUSE_FAMILY, StructuralDisagreement,
    StructuralVerdict, TranscriptVerdict, VectorVerdict,
};
use crate::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding};

/// The cause a golden-vector disagreement is cited under.
const GOLDEN_VECTOR_DISAGREEMENT: FindingCause =
    FindingCause::named(ORACLE_CAUSE_FAMILY, "golden-vector-disagreement");

/// The cause a transcript re-derivation disagreement is cited under.
const TRANSCRIPT_DISAGREEMENT: FindingCause =
    FindingCause::named(ORACLE_CAUSE_FAMILY, "transcript-derivation-disagreement");

/// The cause an artifact that is not parseable Rust is cited under.
///
/// Its own cause and never a structural deviation's: a reading that never happened is a different fact from a reading that disagreed.
const STRUCTURAL_UNPARSABLE: FindingCause =
    FindingCause::named(ORACLE_CAUSE_FAMILY, "structural-unparsable");

/// One typed refusal, as the record vocabulary states one.
fn refused(cause: FindingCause, located: FindingLocation) -> TrialConclusion {
    TrialConclusion::Refused(TrialFinding::established(
        FailureClass::OracleDisagreement,
        cause,
        located,
        None,
    ))
}

/// The cause one structural finding is cited under.
const fn structural_cause(found: &StructuralDisagreement) -> FindingCause {
    let local = match found {
        StructuralDisagreement::UnexpectedItem => "structural-unexpected-item",
        StructuralDisagreement::OutputCardinality { .. } => "structural-output-cardinality",
        StructuralDisagreement::DuplicateImplementation { .. } => {
            "structural-duplicate-implementation"
        }
        StructuralDisagreement::ImplementationTarget { .. } => "structural-implementation-target",
        StructuralDisagreement::TraitPath { .. } => "structural-trait-path",
        StructuralDisagreement::ImplPosture { .. } => "structural-impl-posture",
        StructuralDisagreement::MeaningBearingAttribute { .. } => {
            "structural-meaning-bearing-attribute"
        }
        StructuralDisagreement::UnexpectedImplMember { .. } => "structural-unexpected-member",
        StructuralDisagreement::DuplicateMember { .. } => "structural-duplicate-member",
        StructuralDisagreement::MissingImplMember { .. } => "structural-missing-member",
        StructuralDisagreement::MemberValueUnread { .. } => "structural-member-value-unread",
        StructuralDisagreement::MemberValue { .. } => "structural-member-value",
    };
    FindingCause::named(ORACLE_CAUSE_FAMILY, local)
}

/// The cause one compiled read-back finding is cited under.
const fn compiled_cause(found: &CompiledDisagreement) -> FindingCause {
    let local = match found {
        CompiledDisagreement::AcceptedWhereRefusalDeclared => {
            "compiled-accepted-where-refusal-declared"
        }
        CompiledDisagreement::RefusedWhereAcceptanceDeclared => {
            "compiled-refused-where-acceptance-declared"
        }
        CompiledDisagreement::UnexpectedMember { .. } => "compiled-unexpected-member",
        CompiledDisagreement::DuplicateMember { .. } => "compiled-duplicate-member",
        CompiledDisagreement::MissingMember { .. } => "compiled-missing-member",
        CompiledDisagreement::MemberValue { .. } => "compiled-member-value",
    };
    FindingCause::named(ORACLE_CAUSE_FAMILY, local)
}

impl VectorVerdict {
    /// What this verdict concludes, as the record vocabulary states a conclusion.
    ///
    /// The conclusion is the normalized record: it carries the cause and the class, and deliberately not the two renderings, which are the disagreement's own.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        match self {
            Self::Agrees => TrialConclusion::Passed,
            Self::Disagrees(_) => refused(GOLDEN_VECTOR_DISAGREEMENT, located),
        }
    }
}

impl TranscriptVerdict {
    /// What this verdict concludes, as the record vocabulary states a conclusion.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        match self {
            Self::Agrees => TrialConclusion::Passed,
            Self::Disagrees(_) => refused(TRANSCRIPT_DISAGREEMENT, located),
        }
    }
}

impl StructuralVerdict {
    /// What this verdict concludes, as the record vocabulary states a conclusion.
    ///
    /// [`StructuralVerdict::Unparsable`] concludes a refusal under its own cause, because folding it into a pass would report a verdict about a tree that was never built.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        match self {
            Self::Conforms => TrialConclusion::Passed,
            Self::Deviates(found) => refused(structural_cause(found), located),
            Self::Unparsable => refused(STRUCTURAL_UNPARSABLE, located),
        }
    }
}

impl CompiledVerdict {
    /// What this verdict concludes, as the record vocabulary states a conclusion.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        match self {
            Self::Conforms => TrialConclusion::Passed,
            Self::Deviates(found) => refused(compiled_cause(found), located),
        }
    }
}

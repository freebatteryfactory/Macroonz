//! The report conclusions for compiler-resolved verdicts.

use super::{CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledVerdict};
use crate::oracle::ORACLE_CAUSE_FAMILY;
use crate::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding};

/// The cause one compiled read-back finding is cited under.
const fn cause(found: &CompiledDisagreement) -> FindingCause {
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

impl CompiledVerdict {
    /// What this verdict concludes, as the record vocabulary states a conclusion.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        match self {
            Self::Conforms => TrialConclusion::Passed,
            Self::Deviates(found) => TrialConclusion::Refused(TrialFinding::established(
                FailureClass::OracleDisagreement,
                cause(found),
                located,
                None,
            )),
        }
    }
}

/// The cause one exact compilation disagreement is cited under.
const fn compilation_cause(found: &CompilationDisagreement) -> FindingCause {
    let local = match found {
        CompilationDisagreement::AcceptedWhereRefusalDeclared => {
            "compiled-diagnostic-accepted-where-refusal-declared"
        }
        CompilationDisagreement::RefusedWhereAcceptanceDeclared { .. } => {
            "compiled-diagnostic-refused-where-acceptance-declared"
        }
        CompilationDisagreement::ErrorCode { .. } => "compiled-diagnostic-error-code",
        CompilationDisagreement::PrimarySpan { .. } => "compiled-diagnostic-primary-span",
    };
    FindingCause::named(ORACLE_CAUSE_FAMILY, local)
}

impl CompilationVerdict {
    /// What this exact compilation verdict concludes, as the record vocabulary states a conclusion.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        match self {
            Self::Conforms => TrialConclusion::Passed,
            Self::Deviates(found) => TrialConclusion::Refused(TrialFinding::established(
                FailureClass::OracleDisagreement,
                compilation_cause(found),
                located,
                None,
            )),
        }
    }
}

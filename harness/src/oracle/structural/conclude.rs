//! The structural verdict's report conclusion.

use super::{StructuralDisagreement, StructuralVerdict};
use crate::oracle::ORACLE_CAUSE_FAMILY;
use crate::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding};

/// The cause an artifact that is not parseable Rust is cited under.
const STRUCTURAL_UNPARSABLE: FindingCause =
    FindingCause::named(ORACLE_CAUSE_FAMILY, "structural-unparsable");

/// The cause one structural finding is cited under.
const fn cause(found: &StructuralDisagreement) -> FindingCause {
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

impl StructuralVerdict {
    /// What this verdict concludes, as the record vocabulary states a conclusion.
    ///
    /// [`StructuralVerdict::Unparsable`] concludes a refusal under its own cause, because folding it into a pass would report a verdict about a tree that was never built.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        let cause = match self {
            Self::Conforms => return TrialConclusion::Passed,
            Self::Deviates(found) => cause(found),
            Self::Unparsable => STRUCTURAL_UNPARSABLE,
        };
        TrialConclusion::Refused(TrialFinding::established(
            FailureClass::OracleDisagreement,
            cause,
            located,
            None,
        ))
    }
}

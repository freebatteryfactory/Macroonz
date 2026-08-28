//! The golden-vector verdict's report conclusion.

use super::VectorVerdict;
use crate::oracle::ORACLE_CAUSE_FAMILY;
use crate::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding};

/// The cause a golden-vector disagreement is cited under.
const GOLDEN_VECTOR_DISAGREEMENT: FindingCause =
    FindingCause::named(ORACLE_CAUSE_FAMILY, "golden-vector-disagreement");

impl VectorVerdict {
    /// What this verdict concludes, as the record vocabulary states a conclusion.
    ///
    /// The conclusion is the normalized record: it carries the cause and the class, and deliberately not the two renderings, which are the disagreement's own.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        match self {
            Self::Agrees => TrialConclusion::Passed,
            Self::Disagrees(_) => TrialConclusion::Refused(TrialFinding::established(
                FailureClass::OracleDisagreement,
                GOLDEN_VECTOR_DISAGREEMENT,
                located,
                None,
            )),
        }
    }
}

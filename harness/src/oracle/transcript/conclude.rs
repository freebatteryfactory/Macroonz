//! The independent-transcript verdict's report conclusion.

use super::TranscriptVerdict;
use crate::oracle::ORACLE_CAUSE_FAMILY;
use crate::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding};

/// The cause a transcript re-derivation disagreement is cited under.
const TRANSCRIPT_DISAGREEMENT: FindingCause =
    FindingCause::named(ORACLE_CAUSE_FAMILY, "transcript-derivation-disagreement");

impl TranscriptVerdict {
    /// What this verdict concludes, as the record vocabulary states a conclusion.
    #[must_use]
    pub fn concluded(&self, located: FindingLocation) -> TrialConclusion {
        match self {
            Self::Agrees => TrialConclusion::Passed,
            Self::Disagrees(_) => TrialConclusion::Refused(TrialFinding::established(
                FailureClass::OracleDisagreement,
                TRANSCRIPT_DISAGREEMENT,
                located,
                None,
            )),
        }
    }
}

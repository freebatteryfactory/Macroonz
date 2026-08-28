use super::types::{ActivationAxis, ActivationDisposition, MutationOutcome, MutationVerdict};

impl From<ActivationDisposition> for ActivationAxis {
    /// The axis one activation disposition reads as.
    ///
    /// The observed arm carries its evidence and the axis does not, so a caller that needs the evidence takes [`ActivationDisposition::evidence`](super::ActivationDisposition::evidence).
    fn from(disposition: ActivationDisposition) -> Self {
        match disposition {
            ActivationDisposition::Observed(_) => Self::Observed,
            ActivationDisposition::NotObserved => Self::NotObserved,
            ActivationDisposition::UnobservableUnderBackend => Self::UnobservableUnderBackend,
        }
    }
}

impl From<&MutationOutcome> for MutationVerdict {
    /// The verdict word one outcome reads as, which is what a census counts.
    fn from(outcome: &MutationOutcome) -> Self {
        match outcome {
            MutationOutcome::Killed(_) => Self::Killed,
            MutationOutcome::Survived => Self::Survived,
            MutationOutcome::Inconclusive(_) => Self::Inconclusive,
        }
    }
}

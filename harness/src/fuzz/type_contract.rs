use super::CoverageHostFailure;

impl<ExecutorError, Refusal> From<Refusal> for CoverageHostFailure<ExecutorError, Refusal> {
    fn from(refusal: Refusal) -> Self {
        Self::Refused(refusal)
    }
}

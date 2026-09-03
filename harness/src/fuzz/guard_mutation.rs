//! The mutation roads: the bounded neighboring-input plan and the candidate one operation produces.

use crate::fuzz::types::{MutationCandidate, MutationKind, MutationPlan, MutationPlanRefusal};

impl MutationPlan {
    /// Declare one bounded deterministic neighboring-input plan.
    ///
    /// # Errors
    ///
    /// Refuses a zero budget, zero byte ceiling, or empty dictionary token.
    pub fn declared(
        budget: u32,
        byte_limit: usize,
        dictionary: Vec<Vec<u8>>,
    ) -> Result<Self, MutationPlanRefusal> {
        if budget == 0 {
            return Err(MutationPlanRefusal::ZeroBudget);
        }
        if byte_limit == 0 {
            return Err(MutationPlanRefusal::ZeroByteLimit);
        }
        if let Some(at) = dictionary.iter().position(Vec::is_empty) {
            return Err(MutationPlanRefusal::EmptyDictionaryToken { at });
        }
        Ok(Self {
            budget,
            byte_limit,
            dictionary,
        })
    }

    pub(crate) const fn budget(&self) -> u32 {
        self.budget
    }

    pub(crate) const fn byte_limit(&self) -> usize {
        self.byte_limit
    }

    pub(crate) fn dictionary(&self) -> &[Vec<u8>] {
        &self.dictionary
    }
}

impl MutationCandidate {
    pub(crate) const fn established(kind: MutationKind, bytes: Vec<u8>) -> Self {
        Self { kind, bytes }
    }

    /// The operation that produced this neighbor.
    #[must_use]
    pub const fn kind(&self) -> MutationKind {
        self.kind
    }

    /// The exact neighboring bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

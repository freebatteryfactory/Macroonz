//! The canonical bytes one binding disagreement is named by.

use super::BindError;
use crate::identity::encode_bytes;

impl BindError {
    /// This disagreement's complete canonical material: which pair disagreed, then the identity that was BOUND and the identity the value handed in turned out to name, each at full width.
    ///
    /// The pair's position leads, so two pairs holding identities that happened to coincide still derive two related identities.
    /// The bound identity rides ahead of the carried one, so a reader of the two knows which is which without the diagnostic saying so twice.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let (bound, carried): (&[u8; 32], &[u8; 32]) = match self {
            Self::ClosureProvedAgainstAnotherPlan { planned, proved } => {
                (planned.as_bytes(), proved.as_bytes())
            }
            Self::ExplanationAnsweredOverAnotherPlan { planned, answered } => {
                (planned.as_bytes(), answered.as_bytes())
            }
            Self::ExplanationAnsweredOverAnotherClosure { proved, answered } => {
                (proved.as_bytes(), answered.as_bytes())
            }
        };
        let mut material = vec![self.slot()];
        encode_bytes(bound, &mut material);
        encode_bytes(carried, &mut material);
        material
    }
}

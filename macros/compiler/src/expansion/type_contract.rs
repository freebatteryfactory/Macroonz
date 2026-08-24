//! The constant answers this home's one roster settles, and the contracts a binding refusal stands under.
//!
//! The table is total, so a pair admitted later stops the compiler here until somebody says what its position and its sentence are.

use super::{BINDING_FACT, BindError};
use crate::bounded::Bounded;
use crate::diagnostic::{
    BINDING_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
};
use crate::identity::human_projection;
use core::fmt;

impl BindError {
    /// This pair's position in the declared roster, written ahead of the two identities it disagreed over.
    ///
    /// Appended and never renumbered: the byte stands inside every related identity derived over a binding refusal.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::ClosureProvedAgainstAnotherPlan { .. } => 0,
            Self::ExplanationAnsweredOverAnotherPlan { .. } => 1,
            Self::ExplanationAnsweredOverAnotherClosure { .. } => 2,
        }
    }
}

impl fmt::Display for BindError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        into.write_str(match self {
            Self::ClosureProvedAgainstAnotherPlan { .. } => {
                "the closure proves a rendering against a plan other than the one bound beside it"
            }
            Self::ExplanationAnsweredOverAnotherPlan { .. } => {
                "the explanation was answered over a plan other than the one bound beside it"
            }
            Self::ExplanationAnsweredOverAnotherClosure { .. } => {
                "the explanation was answered over a proof other than the one bound beside it"
            }
        })
    }
}

impl core::error::Error for BindError {}

impl Refused for BindError {
    const PHASE: Phase = Phase::Binding;
    const FAMILY: Family = BINDING_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::ExpansionNotBound
    }

    fn first(&self) -> String {
        self.to_string()
    }

    /// Every pair is two identities that had to match and did not.
    fn observed(&self) -> Observed {
        Observed::IdentityDisagreement
    }

    /// One disagreement, always.
    ///
    /// The binding compares each pair in turn and refuses at the first that disagrees, so there is no body behind the cause and nothing for a line to count.
    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    fn related(&self) -> Vec<Vec<u8>> {
        vec![self.canonical_bytes()]
    }

    /// The one repair, citing this home's own declared fact.
    ///
    /// Unlike a refusal about what a caller declared, the law here is this compiler's, so the fact is this home's to cite.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::from_array([Repair {
            declared_by: BINDING_FACT,
            description: human_projection!(
                "an expansion binds the plan its proof was taken against and the explanation answered over the two, so a proof or an explanation belonging to another expansion is refused rather than bound under one identity"
            ),
        }])
    }
}

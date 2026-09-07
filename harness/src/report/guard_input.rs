//! Input execution standing derived from an admitted specimen and its decoder binding.

use crate::descriptor::RevisionPosture;
use crate::identity::ContentAddress;
use crate::input::{BoundInput, InputCaseId, InputProfile};
use crate::report::ExecutionInput;

impl ExecutionInput {
    /// The input coordinates retained from the decoder's admitted value.
    #[must_use]
    pub fn of<Input>(input: &BoundInput<Input>) -> Self {
        Self {
            profile: input.envelope().profile(),
            case: input.envelope().case(),
            decoder: input.revision().revision(),
            posture: input.revision().posture(),
        }
    }

    /// The input convention under which the specimen was admitted.
    #[must_use]
    pub const fn profile(self) -> InputProfile {
        self.profile
    }

    /// The identity of the complete specimen envelope.
    #[must_use]
    pub const fn case(self) -> InputCaseId {
        self.case
    }

    /// The exact decoder revision recorded at input admission.
    #[must_use]
    pub const fn decoder(self) -> ContentAddress {
        self.decoder
    }

    /// The authority posture of the decoder revision.
    #[must_use]
    pub const fn posture(self) -> RevisionPosture {
        self.posture
    }
}

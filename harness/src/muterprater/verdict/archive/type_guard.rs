//! Read-only historical target and activation projections.

use super::ArchivedActivation;
use super::{
    ArchivedActivationReading, ArchivedMutation, ArchivedMutationIdentity, ArchivedMutationOutcome,
    ArchivedMutationSite, ArchivedMutationTarget,
};
use crate::descriptor::archive::ArchivedName;
use crate::identity::ContentAddress;
use crate::muterprater::{BaselineAxis, EquivalenceAxis, ExecutionAxis, MaterializationAxis};
use crate::report::archive::AddressClaim;

#[path = "read.rs"]
mod read;
pub(crate) use read::{read_activation, read_target};

#[path = "read_record.rs"]
mod record;
pub use record::read_mutation;

#[path = "read_run.rs"]
mod run;
#[path = "guard_run.rs"]
mod run_readings;
pub use run::read_mutation_run;

impl ArchivedMutation {
    /// The exact envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The integrity address of the historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The complete historical target.
    #[must_use]
    pub const fn target(&self) -> &ArchivedMutationTarget {
        &self.target
    }

    /// The recorded baseline axis.
    #[must_use]
    pub const fn baseline(&self) -> BaselineAxis {
        self.baseline
    }

    /// The recorded materialization axis.
    #[must_use]
    pub const fn materialization(&self) -> MaterializationAxis {
        self.materialization
    }

    /// The historical activation and its retained claims.
    #[must_use]
    pub const fn activation(&self) -> &ArchivedActivation {
        &self.activation
    }

    /// The recorded execution axis.
    #[must_use]
    pub const fn execution(&self) -> ExecutionAxis {
        self.execution
    }

    /// The historical outcome and retained rejection, when present.
    #[must_use]
    pub const fn outcome(&self) -> &ArchivedMutationOutcome {
        &self.outcome
    }

    /// The recorded scoped-equivalence axis.
    #[must_use]
    pub const fn equivalence(&self) -> EquivalenceAxis {
        self.equivalence
    }
}

impl ArchivedMutationIdentity {
    /// The historical point, absent for external coordinate identities.
    #[must_use]
    pub const fn point(&self) -> Option<&ArchivedName> {
        match self {
            Self::External(_) => None,
            Self::Interpreted {
                point,
                alternative: _,
            }
            | Self::CompiledProjection {
                point,
                alternative: _,
            } => Some(point),
        }
    }
}

impl ArchivedMutationTarget {
    /// The exact historical identity arm.
    #[must_use]
    pub const fn identity(&self) -> &ArchivedMutationIdentity {
        &self.identity
    }

    /// The historical family slug, absent for an outside-bank attribution.
    #[must_use]
    pub fn family(&self) -> Option<&str> {
        self.family.as_deref()
    }

    /// The historical source coordinate or declared activation site.
    #[must_use]
    pub const fn site(&self) -> &ArchivedMutationSite {
        &self.site
    }

    /// The historical owning claim, absent when unmapped.
    #[must_use]
    pub const fn owner(&self) -> Option<&ArchivedName> {
        self.owner.as_ref()
    }
}

impl ArchivedActivationReading {
    /// The historical surface address claim.
    #[must_use]
    pub const fn surface(&self) -> AddressClaim {
        self.surface
    }

    /// The historical point selected on that surface.
    #[must_use]
    pub const fn point(&self) -> &ArchivedName {
        &self.point
    }

    /// The selected alternative's address claim.
    #[must_use]
    pub const fn alternative(&self) -> AddressClaim {
        self.alternative
    }

    /// The callback's historical witness claim.
    #[must_use]
    pub const fn witness(&self) -> AddressClaim {
        self.witness
    }

    /// The positive count reported by that callback.
    #[must_use]
    pub const fn firings(&self) -> u32 {
        self.firings
    }
}

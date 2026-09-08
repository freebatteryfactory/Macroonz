//! Historical-surface bounds and read-only projections.

use super::{
    ArchivedAlternative, ArchivedEvaluationSurface, ArchivedMutationPoint, SurfaceArchiveLimits,
};
use crate::descriptor::archive::ArchivedName;
use crate::identity::ContentAddress;
use crate::report::archive::{AddressClaim, ArchiveLimits};

#[path = "read.rs"]
mod read;
#[path = "read_members.rs"]
mod members;
pub use read::read_surface;
#[path = "guard_selection.rs"]
mod selections;
pub(crate) use selections::read_selection;

impl SurfaceArchiveLimits {
    /// Declare byte, point and per-point alternative bounds.
    #[must_use]
    pub const fn declared(bytes: ArchiveLimits, points: usize, alternatives: usize) -> Self {
        Self {
            bytes,
            points,
            alternatives,
        }
    }

    /// The complete-envelope and framed-field ceilings.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }

    /// The maximum point population.
    #[must_use]
    pub const fn points(self) -> usize {
        self.points
    }

    /// The maximum alternative population at each point.
    #[must_use]
    pub const fn alternatives(self) -> usize {
        self.alternatives
    }
}

impl ArchivedAlternative {
    /// The address rederived from point, family and operation.
    #[must_use]
    pub const fn identity(&self) -> ContentAddress {
        self.identity
    }

    /// The exact historical family slug, without current catalogue lookup.
    #[must_use]
    pub fn family(&self) -> &str {
        &self.family
    }

    /// The exact nonempty alternative operation.
    #[must_use]
    pub fn operation(&self) -> &[u8] {
        &self.operation
    }
}

impl ArchivedMutationPoint {
    /// The historical point name.
    #[must_use]
    pub const fn name(&self) -> &ArchivedName {
        &self.name
    }

    /// The owner claim recorded under the enclosing surface's policy.
    #[must_use]
    pub const fn owner_claim(&self) -> &ArchivedName {
        &self.owner_claim
    }

    /// The exact nonempty unchanged operation.
    #[must_use]
    pub fn original_operation(&self) -> &[u8] {
        &self.original
    }

    /// The historical activation-site name.
    #[must_use]
    pub const fn activation_site(&self) -> &ArchivedName {
        &self.activation_site
    }

    /// The complete nonempty alternative roster in identity order.
    #[must_use]
    pub fn alternatives(&self) -> &[ArchivedAlternative] {
        &self.alternatives
    }
}

impl ArchivedEvaluationSurface {
    /// The complete historical envelope bytes.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The integrity address of the historical envelope body.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The original surface address rederived from its complete preimage.
    #[must_use]
    pub const fn identity(&self) -> ContentAddress {
        self.identity
    }

    /// The exact preimage in the discovery owner's canonical grammar.
    #[must_use]
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }

    /// The historical evaluation-family name.
    #[must_use]
    pub const fn family(&self) -> &ArchivedName {
        &self.family
    }

    /// The policy address claim without its missing permission roster.
    #[must_use]
    pub const fn policy(&self) -> AddressClaim {
        self.policy
    }

    /// The complete executable-subset roster in point-name order.
    #[must_use]
    pub fn points(&self) -> &[ArchivedMutationPoint] {
        &self.points
    }
}

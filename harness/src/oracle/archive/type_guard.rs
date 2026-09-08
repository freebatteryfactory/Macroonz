//! Historical record access and admission through the owning method grammars.

use super::{
    ArchivedMethod, ArchivedOracle, ArchivedTranscriptDisagreement, ArchivedVectorDisagreement,
    ArchivedVerdict,
};
use crate::identity::ContentAddress;
use crate::oracle::ByteDifference;
use crate::report::archive::AddressClaim;

#[path = "read.rs"]
mod read;
#[path = "read_compiled.rs"]
mod compiled;
#[path = "read_structural.rs"]
mod structural;

pub use read::read_verdict;

impl ArchivedMethod {
    pub(crate) const fn slot(self) -> u32 {
        match self {
            Self::Vector => 1,
            Self::Transcript => 2,
            Self::Structural => 3,
            Self::Compiled => 4,
            Self::Compilation => 5,
        }
    }
}

impl ArchivedOracle {
    /// The exact historical envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The address derived from this historical envelope body.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The complete method-specific historical disposition.
    #[must_use]
    pub const fn verdict(&self) -> &ArchivedVerdict {
        &self.verdict
    }
}

impl ArchivedVectorDisagreement {
    /// The complete expected historical buffer.
    #[must_use]
    pub fn expected(&self) -> &[u8] {
        &self.expected
    }

    /// The complete produced historical buffer.
    #[must_use]
    pub fn produced(&self) -> &[u8] {
        &self.produced
    }

    /// The first difference derived from the retained buffers.
    #[must_use]
    pub const fn difference(&self) -> ByteDifference {
        self.difference
    }
}

impl ArchivedTranscriptDisagreement {
    /// The identity the source claimed to have independently rederived.
    #[must_use]
    pub const fn rederived(self) -> AddressClaim {
        self.rederived
    }

    /// The identity the source claimed the producer published.
    #[must_use]
    pub const fn published(self) -> AddressClaim {
        self.published
    }
}

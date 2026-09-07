//! Owned historical projections without current execution constructors.

use super::super::{
    ArchivedEvaluationPair, ArchivedParity, ArchivedParityDisposition, ArchivedSubstrate,
    ArchivedSubstrateRoster, ArchivedValue, ArchivedValueConvention,
};
use crate::descriptor::archive::{ArchivedBinding, ArchivedName, ArchivedRevisionBinding};
use crate::identity::ContentAddress;
use crate::report::archive::{AddressClaim, ArchivedConclusion, ArchivedTrial};

impl ArchivedValueConvention {
    /// The original convention name.
    #[must_use]
    pub const fn name(&self) -> &ArchivedName {
        &self.name
    }
    /// The original convention version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }
    /// The schema address claim.
    #[must_use]
    pub const fn schema(&self) -> AddressClaim {
        self.schema
    }
    /// The encoder revision claim and posture.
    #[must_use]
    pub const fn revision(&self) -> ArchivedRevisionBinding {
        self.revision
    }
}

impl ArchivedEvaluationPair {
    /// The original family name.
    #[must_use]
    pub const fn family(&self) -> &ArchivedName {
        &self.family
    }
    /// The production revision claim.
    #[must_use]
    pub const fn production_revision(&self) -> ArchivedRevisionBinding {
        self.production
    }
    /// The evaluation revision claim.
    #[must_use]
    pub const fn evaluation_revision(&self) -> ArchivedRevisionBinding {
        self.evaluation
    }
    /// The surface address claim without a discovery roster.
    #[must_use]
    pub const fn surface(&self) -> AddressClaim {
        self.surface
    }
}

impl ArchivedValue {
    /// The convention claimed for these exact bytes.
    #[must_use]
    pub const fn convention(&self) -> &ArchivedValueConvention {
        &self.convention
    }
}

impl ArchivedParity {
    /// The integrity address of the complete historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }
    /// The historical pair standing.
    #[must_use]
    pub const fn pair(&self) -> &ArchivedEvaluationPair {
        &self.pair
    }
    /// The complete historical witness attachment and provenance.
    #[must_use]
    pub const fn witness(&self) -> &ArchivedBinding {
        &self.witness
    }
    /// The input encoded at retention time.
    #[must_use]
    pub const fn input(&self) -> &ArchivedValue {
        &self.input
    }
    /// The retained production meaning encoding.
    #[must_use]
    pub const fn production(&self) -> &ArchivedValue {
        &self.production
    }
    /// The retained evaluation meaning encoding.
    #[must_use]
    pub const fn evaluation(&self) -> &ArchivedValue {
        &self.evaluation
    }
    /// The original evaluation firing count.
    #[must_use]
    pub const fn evaluation_firings(&self) -> u32 {
        self.firings
    }
    /// The original shared-foundation declaration.
    #[must_use]
    pub const fn substrate(&self) -> &ArchivedSubstrate {
        &self.substrate
    }
    /// The complete historical equivalence conclusion.
    #[must_use]
    pub const fn conclusion(&self) -> &ArchivedConclusion {
        &self.conclusion
    }
    /// The complete historical production report.
    #[must_use]
    pub const fn production_report(&self) -> &ArchivedTrial {
        &self.production_report
    }
    /// The complete historical evaluation report.
    #[must_use]
    pub const fn evaluation_report(&self) -> &ArchivedTrial {
        &self.evaluation_report
    }
    /// The recorded disposition without current qualification authority.
    #[must_use]
    pub const fn disposition(&self) -> ArchivedParityDisposition {
        self.disposition
    }
}

impl ArchivedParity {
    /// The exact complete envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
}

impl ArchivedValue {
    /// The exact caller bytes with no decoder or temporal snapshot claim.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl ArchivedSubstrateRoster {
    /// The nonempty substrate roster in strict namespace-and-stem order.
    #[must_use]
    pub fn names(&self) -> &[ArchivedName] {
        &self.names
    }
}

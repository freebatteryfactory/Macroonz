//! The row roads: how an origin's payload is admitted, how a row is born over its canonical bytes, and how its trial key is derived.

use crate::descriptor::encode::{encode_row_content, encode_trial_coordinates};
use crate::descriptor::types::{
    AdmissionFacts, AdmissionGround, CanonicalRowBytes, CapsulePosture, CheckRef, ClaimRef,
    Classification, ClassificationRefusal, DischargeAdmission, DoorRef, ExecutionSuite, Origin,
    PopulationRef, ProducerFacts, ProjectionRef, ProposalId, ReplayAdmission, ReplayBearingGround,
    ReplayRef, Role, Row, RowRefusal, SubjectRoute, Tag, TrialCoordinates, TrialKey,
};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use std::collections::BTreeSet;

/// The domain a trial key is derived under.
///
/// Its own tag rather than the row-revision one beside it: a trial's four coordinates and a row's seven fields answer different questions.
const TRIAL_KEY_DOMAIN: DomainTag =
    DomainTag::declared("trial-key", IdentityProfileVersion::declared(1));

impl AdmissionGround {
    /// Whether admitting on this ground authors a depot capsule entry.
    ///
    /// The two replay-bearing grounds carry a reproduction account; a discharge stands on the admitted row itself.
    #[must_use]
    pub const fn capsule_posture(self) -> CapsulePosture {
        match self {
            Self::MutantKilled | Self::ClaimPinned => CapsulePosture::ReplayBearing,
            Self::ObligationDischarged => CapsulePosture::NoCapsule,
        }
    }
}

impl ProposalId {
    /// The proposal identity, over the content address the proposal road minted.
    #[must_use]
    pub const fn over(address: ContentAddress) -> Self {
        Self(address)
    }
}

crate::identity::content_address_reference! {
    /// The content address this identity carries.
    value ProposalId;
}

impl ReplayRef {
    /// The replay reference, over the content address of the capsule entry an admission act authored.
    #[must_use]
    pub(crate) const fn over(address: ContentAddress) -> Self {
        Self(address)
    }
}

crate::identity::content_address_reference! {
    /// The content address this reference carries.
    value ReplayRef;
}

impl Classification {
    /// The classification, parsed from the rosters exactly as authored.
    ///
    /// # Errors
    ///
    /// Refuses a repeated role, then a repeated tag.
    /// A duplicate is an authoring defect, and folding it away silently would normalize that defect out of sight.
    pub fn authored(roles: Vec<Role>, tags: Vec<Tag>) -> Result<Self, ClassificationRefusal> {
        let mut parsed_roles = BTreeSet::new();
        for role in roles {
            if !parsed_roles.insert(role) {
                return Err(ClassificationRefusal::DuplicateRole(role));
            }
        }
        let mut parsed_tags = BTreeSet::new();
        for tag in tags {
            if !parsed_tags.insert(tag) {
                return Err(ClassificationRefusal::DuplicateTag(tag));
            }
        }
        Ok(Self {
            roles: parsed_roles,
            tags: parsed_tags,
        })
    }

    /// The roles carried, in storage order.
    #[must_use]
    pub const fn roles(&self) -> &BTreeSet<Role> {
        &self.roles
    }

    /// The tags carried, in storage order.
    #[must_use]
    pub const fn tags(&self) -> &BTreeSet<Tag> {
        &self.tags
    }
}

impl ProducerFacts {
    /// The producer-side facts one generated row earns.
    #[must_use]
    pub const fn emitted(door: DoorRef, projection: ProjectionRef) -> Self {
        Self { door, projection }
    }

    /// The declaration door.
    #[must_use]
    pub const fn door(self) -> DoorRef {
        self.door
    }

    /// The projection that emitted the row.
    #[must_use]
    pub const fn projection(self) -> ProjectionRef {
        self.projection
    }
}

impl AdmissionFacts {
    /// What one admission act stated: the ground it stood on, and the suite the admitted row lands in.
    #[must_use]
    pub const fn stated(ground: AdmissionGround, destination: ExecutionSuite) -> Self {
        Self {
            ground,
            destination,
        }
    }

    /// The ground the admission stood on.
    #[must_use]
    pub const fn ground(self) -> AdmissionGround {
        self.ground
    }

    /// The suite the admitted row lands in.
    #[must_use]
    pub const fn destination(self) -> ExecutionSuite {
        self.destination
    }
}

impl ReplayAdmission {
    /// What one admission on a replay-bearing ground earned a row.
    ///
    /// The ground is the narrowed one, so this constructor cannot be handed a ground that authors no capsule.
    #[must_use]
    pub(crate) const fn admitted(
        proposal: ProposalId,
        ground: ReplayBearingGround,
        destination: ExecutionSuite,
        replay: ReplayRef,
    ) -> Self {
        Self {
            proposal,
            ground,
            destination,
            replay,
        }
    }

    /// The admitted proposal's content identity.
    #[must_use]
    pub const fn proposal(self) -> ProposalId {
        self.proposal
    }

    /// The replay-bearing ground the admission stood on.
    #[must_use]
    pub const fn ground(self) -> ReplayBearingGround {
        self.ground
    }

    /// The suite the admitted row lands in.
    #[must_use]
    pub const fn destination(self) -> ExecutionSuite {
        self.destination
    }

    /// The capsule entry the admission act authored.
    #[must_use]
    pub const fn replay(self) -> ReplayRef {
        self.replay
    }

    /// What the admission stated, at summary width.
    #[must_use]
    pub fn admission(self) -> AdmissionFacts {
        AdmissionFacts::stated(AdmissionGround::from(self.ground), self.destination)
    }
}

impl DischargeAdmission {
    /// What one admission on a discharge ground earned a row.
    ///
    /// No ground is taken: a discharge stands on exactly one, so a caller is not asked to supply what it cannot choose.
    #[must_use]
    pub(crate) const fn admitted(proposal: ProposalId, destination: ExecutionSuite) -> Self {
        Self {
            proposal,
            destination,
        }
    }

    /// The admitted proposal's content identity.
    #[must_use]
    pub const fn proposal(self) -> ProposalId {
        self.proposal
    }

    /// The suite the admitted row lands in.
    #[must_use]
    pub const fn destination(self) -> ExecutionSuite {
        self.destination
    }

    /// What the admission stated, at summary width — the forced ground, and this destination.
    #[must_use]
    pub const fn admission(self) -> AdmissionFacts {
        AdmissionFacts::stated(AdmissionGround::ObligationDischarged, self.destination)
    }
}

impl CanonicalRowBytes {
    /// The bytes, for the identity road that derives over them.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Row {
    /// One test's row, over the values it states.
    ///
    /// The canonical bytes and the trial key are written here, once, from exactly the values this call was given.
    /// Encoding at birth is what makes a revision identity a reading later on: no two encodings of one row can disagree because only one is ever performed.
    ///
    /// # Errors
    ///
    /// Refuses when those bytes could not be written — a length past the width the row encoding declares, which is unreachable on every target this crate is built for.
    /// Every other structural refusal was spent upstream.
    pub fn declared(
        claim: ClaimRef,
        execution_suite: ExecutionSuite,
        classification: Classification,
        subject: SubjectRoute,
        check: CheckRef,
        population: PopulationRef,
        origin: Origin,
    ) -> Result<Self, RowRefusal> {
        let canonical = encode_row_content(
            claim,
            execution_suite,
            &classification,
            subject,
            check,
            population,
            origin,
        )
        .map_err(RowRefusal::NotEncoded)?;
        let coordinates = TrialCoordinates::over(claim, subject, check, population);
        let trial_key = TrialKey::over(coordinates);
        Ok(Self {
            coordinates,
            trial_key,
            execution_suite,
            classification,
            origin,
            canonical: CanonicalRowBytes(canonical),
        })
    }

    /// Where this row's trial sits.
    #[must_use]
    pub const fn coordinates(&self) -> TrialCoordinates {
        self.coordinates
    }

    /// The claim this row serves.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.coordinates.claim()
    }

    /// The one aggregate seat this row runs under by default.
    #[must_use]
    pub const fn execution_suite(&self) -> ExecutionSuite {
        self.execution_suite
    }

    /// How this row is classified.
    #[must_use]
    pub const fn classification(&self) -> &Classification {
        &self.classification
    }

    /// The roles this row carries.
    #[must_use]
    pub const fn roles(&self) -> &BTreeSet<Role> {
        self.classification.roles()
    }

    /// The tags this row carries.
    #[must_use]
    pub const fn tags(&self) -> &BTreeSet<Tag> {
        self.classification.tags()
    }

    /// What this row exercises.
    #[must_use]
    pub const fn subject(&self) -> SubjectRoute {
        self.coordinates.subject()
    }

    /// The check that judges this row's subject.
    #[must_use]
    pub const fn check(&self) -> CheckRef {
        self.coordinates.check()
    }

    /// The population that supplies this row's inputs.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.coordinates.population()
    }

    /// Where this row came from.
    #[must_use]
    pub const fn origin(&self) -> Origin {
        self.origin
    }

    /// The canonical bytes this row committed to when it was built.
    ///
    /// A read and never a computation.
    #[must_use]
    pub const fn canonical_bytes(&self) -> &CanonicalRowBytes {
        &self.canonical
    }

    /// The compact identity that decides whether two rows are one trial.
    ///
    /// A read and never a computation, on the same terms as [`Row::canonical_bytes`].
    #[must_use]
    pub const fn trial_key(&self) -> TrialKey {
        self.trial_key
    }
}

impl TrialCoordinates {
    /// The four coordinates one trial sits at.
    #[must_use]
    pub const fn over(
        claim: ClaimRef,
        subject: SubjectRoute,
        check: CheckRef,
        population: PopulationRef,
    ) -> Self {
        Self {
            claim,
            subject,
            check,
            population,
        }
    }

    /// The claim the trial serves.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// What the trial exercises.
    #[must_use]
    pub const fn subject(self) -> SubjectRoute {
        self.subject
    }

    /// The check that judges it.
    #[must_use]
    pub const fn check(self) -> CheckRef {
        self.check
    }

    /// The population that supplies its inputs.
    #[must_use]
    pub const fn population(self) -> PopulationRef {
        self.population
    }
}

impl TrialKey {
    /// Derive one trial's compact identity from its coordinates.
    ///
    /// The one place the four coordinates are encoded, so there is no second preimage to drift from this one.
    /// Total: the four names were admitted at their own construction, and writing their preimage cannot fail.
    #[must_use]
    pub fn over(coordinates: TrialCoordinates) -> Self {
        let preimage = encode_trial_coordinates(coordinates);
        Self(ContentAddress::derived(TRIAL_KEY_DOMAIN, &preimage))
    }
}

crate::identity::content_address_reference! {
    /// The content address this key carries.
    value TrialKey;
}

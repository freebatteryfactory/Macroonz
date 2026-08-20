//! The descriptor home's invariant nucleus: every road that builds one of this
//! vocabulary's values, and every reader that hands its seats back.
//!
//! Declared inside `types.rs` as its own child, which is what makes the home's
//! claims structural rather than remembered. A name is parsed HERE, so a
//! reference that names nothing is not a value anybody can hold. An admitted
//! origin's payload is admitted HERE, each arm taking only the grounds that arm
//! earns, so an arm and its ground cannot disagree anywhere. A row is born
//! HERE, and it commits to its canonical bytes as it is born, so a row that
//! exists has exactly one encoding and nothing derives a second. A binding is
//! married HERE, so a row's references and its callable's references are one
//! pair. A table is closed HERE, so a duplicated trial and an authored
//! candidate are not values that exist. The readers travel with the mints
//! because they are the same private seats read back.
//!
//! Two hand-written `Clone` realizations sit here rather than in
//! `type_contract.rs` for one mechanical reason: they read private seats, and
//! only this file and `types.rs` can see them.

use super::{
    AdmissionFacts, AdmissionGround, AuthoredTable, AuthoredTableName, AuthoredTableRefusal,
    BENCH_FIELDS, BenchSchema, Binding, BindingRefusal, CanonicalRowBytes, CheckRef, ClaimRef,
    Classification, ClassificationRefusal, DESCRIPTOR_FIELDS, DescriptorSchema, DischargeAdmission,
    DoorRef, EncodeRefusal, ExecutableAttachment, ExecutionSuite, FieldCardinality, FieldShape,
    GeneratedSupportSchema, GeneratedSupportSchemaId, MUTATION_POINT_FIELDS, MutationPointRef,
    MutationPointSchema, NameRefusal, NamespacedName, Origin, PopulationRef, ProducerFacts,
    ProducerName, ProjectionRef, ProposalId, Provenance, ReplayAdmission, ReplayBearingGround,
    ReplayRef, RevisionBinding, RevisionPosture, Role, Row, RowRefusal, SchemaField, SchemaRefusal,
    StagedTableRefusal, StagedTableView, SubjectRoute, TablePosture, TableView, Tag, TrialKey,
};
use crate::descriptor::encode::{encode_generated_support_schema, encode_row_content};
use crate::identity::{ContentAddress, DomainTag};
use std::collections::BTreeSet;

/// The domain this home declares for the generated-support schema identity.
///
/// The tag is this kind's segment of the derivation context; the profile stem
/// and its version are the identity substrate's. Two kinds derived over
/// identical preimages under different tags are unrelated values.
const GENERATED_SUPPORT_SCHEMA_DOMAIN: DomainTag = DomainTag::declared("generated-support-schema");

impl NamespacedName {
    /// This name, parsed from the owner that declares it and the spelling it
    /// carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem.
    pub fn named(namespace: &'static str, stem: &'static str) -> Result<Self, NameRefusal> {
        if namespace.is_empty() {
            return Err(NameRefusal::EmptyNamespace);
        }
        if stem.is_empty() {
            return Err(NameRefusal::EmptyStem);
        }
        Ok(Self { namespace, stem })
    }

    /// The owner that declares the spelling.
    #[must_use]
    pub const fn namespace(self) -> &'static str {
        self.namespace
    }

    /// The spelling itself.
    #[must_use]
    pub const fn stem(self) -> &'static str {
        self.stem
    }
}

/// The two roads and the one reader every namespaced reference in this
/// vocabulary carries, written once and stamped over the roster.
///
/// Each reference is its own type so the compiler keeps a claim out of a
/// subject's seat; what they SHARE is how a name is parsed, and a hand-copied
/// parser per newtype would be that one law standing in a dozen places.
macro_rules! namespaced_reference {
    ($($reference:ident),+ $(,)?) => {
        $(
            impl $reference {
                /// This reference, parsed from the owner that declares it and
                /// the spelling it carries.
                ///
                /// # Errors
                ///
                /// Refuses an empty namespace, then an empty stem.
                pub fn named(
                    namespace: &'static str,
                    stem: &'static str,
                ) -> Result<Self, NameRefusal> {
                    NamespacedName::named(namespace, stem).map(Self)
                }

                /// This reference, over a name already parsed.
                #[must_use]
                pub const fn over(name: NamespacedName) -> Self {
                    Self(name)
                }

                /// The namespaced name this reference carries.
                #[must_use]
                pub const fn name(self) -> NamespacedName {
                    self.0
                }
            }
        )+
    };
}

namespaced_reference!(
    AuthoredTableName,
    CheckRef,
    ClaimRef,
    DoorRef,
    ExecutionSuite,
    MutationPointRef,
    PopulationRef,
    ProducerName,
    ProjectionRef,
    Role,
    SubjectRoute,
    Tag,
);

impl ProposalId {
    /// The proposal identity, over the content address the proposal road
    /// minted.
    #[must_use]
    pub const fn over(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The content address this identity carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl ReplayRef {
    /// The replay reference, over the content address of the depot capsule
    /// entry an admission act authored.
    #[must_use]
    pub const fn over(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The content address this reference carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl GeneratedSupportSchemaId {
    /// This identity, over a content address already derived — the road a
    /// published literal is read back through.
    #[must_use]
    pub const fn over(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The content address this identity carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl Classification {
    /// The classification, parsed from the rosters exactly as authored.
    ///
    /// # Errors
    ///
    /// Refuses a repeated role, then a repeated tag. A duplicate is an
    /// authoring defect, and folding it away silently would be the harness
    /// normalizing that defect out of sight.
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
    /// The producer-side facts one generated row earns: the door its
    /// declaration was authored through, and the projection that emitted it.
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
    /// What one admission act stated: the ground it stood on, and the suite the
    /// admitted row lands in.
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

    /// The suite the admitted row lands in. Its namespace is the semantic owner
    /// the destination names.
    #[must_use]
    pub const fn destination(self) -> ExecutionSuite {
        self.destination
    }
}

impl ReplayAdmission {
    /// What one admission on a replay-bearing ground earned a row.
    ///
    /// The ground is the narrowed one, so this constructor cannot be handed a
    /// ground that authors no capsule — the seat has no spelling for one.
    #[must_use]
    pub const fn admitted(
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

    /// The depot capsule entry the admission act authored.
    #[must_use]
    pub const fn replay(self) -> ReplayRef {
        self.replay
    }

    /// What the admission stated, at summary width.
    ///
    /// The ground widens to the vocabulary every admission is summarised in, so
    /// a reader that wants the word rather than the arm reads it here.
    #[must_use]
    pub fn admission(self) -> AdmissionFacts {
        AdmissionFacts::stated(AdmissionGround::from(self.ground), self.destination)
    }
}

impl DischargeAdmission {
    /// What one admission on a discharge ground earned a row.
    ///
    /// No ground is taken: a discharge stands on exactly one, so the value is
    /// forced and a caller is not asked to supply what it cannot choose.
    #[must_use]
    pub const fn admitted(proposal: ProposalId, destination: ExecutionSuite) -> Self {
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

    /// What the admission stated, at summary width — the forced ground, and the
    /// destination this admission named.
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
    /// The row's canonical bytes are written HERE, once, from exactly the values
    /// this call was given, and the row carries them afterwards. Encoding at
    /// birth is what makes a revision identity a reading later on: nothing
    /// re-encodes a row per run, and no two encodings of one row can disagree
    /// because only one is ever performed.
    ///
    /// # Errors
    ///
    /// Refuses when those bytes could not be written — a length past the width
    /// the row encoding declares, which is unreachable on every target this
    /// crate is built for. Every other structural refusal this row could earn
    /// was already spent upstream: the name parsers refuse an empty reference,
    /// [`Classification`] refuses a repeated label, and an origin whose arm and
    /// ground disagree is not a value that reaches this call.
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
        Ok(Self {
            claim,
            execution_suite,
            classification,
            subject,
            check,
            population,
            origin,
            canonical: CanonicalRowBytes(canonical),
        })
    }

    /// The claim this row serves.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
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
        self.subject
    }

    /// The check that judges this row's subject.
    #[must_use]
    pub const fn check(&self) -> CheckRef {
        self.check
    }

    /// The population that supplies this row's inputs.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.population
    }

    /// Where this row came from.
    #[must_use]
    pub const fn origin(&self) -> Origin {
        self.origin
    }

    /// The canonical bytes this row committed to when it was built.
    ///
    /// A read and never a computation: the encoding happened once, at
    /// [`Row::declared`], and this hands back what was written there.
    #[must_use]
    pub const fn canonical_bytes(&self) -> &CanonicalRowBytes {
        &self.canonical
    }

    /// The semantic content that decides whether two rows are one trial.
    #[must_use]
    pub const fn trial_key(&self) -> TrialKey {
        TrialKey {
            claim: self.claim,
            subject: self.subject,
            check: self.check,
            population: self.population,
        }
    }
}

impl TrialKey {
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

impl RevisionPosture {
    /// The weaker of two postures.
    ///
    /// Derived outranks Declared, and Declared outranks Untracked, so the meet
    /// is what BOTH halves of a pair can honestly claim. Every combination is
    /// stated rather than folded, because the order is a declaration.
    ///
    /// What the meet governs — cache eligibility and the replay posture — is
    /// the report instrument's one statement ([`crate::report`]); the operation
    /// lives here, with the postures it is over.
    #[must_use]
    pub const fn meet(self, other: Self) -> Self {
        match (self, other) {
            (Self::Derived, Self::Derived) => Self::Derived,
            (Self::Derived | Self::Declared, Self::Declared) | (Self::Declared, Self::Derived) => {
                Self::Declared
            }
            (Self::Untracked, Self::Derived | Self::Declared | Self::Untracked)
            | (Self::Derived | Self::Declared, Self::Untracked) => Self::Untracked,
        }
    }
}

impl RevisionBinding {
    /// A revision generated from an owned declaration.
    #[must_use]
    pub const fn derived(revision: ContentAddress) -> Self {
        Self {
            revision,
            posture: RevisionPosture::Derived,
        }
    }

    /// A revision a hand author committed to explicitly.
    #[must_use]
    pub const fn declared(revision: ContentAddress) -> Self {
        Self {
            revision,
            posture: RevisionPosture::Declared,
        }
    }

    /// A revision under no stable commitment.
    #[must_use]
    pub const fn untracked(revision: ContentAddress) -> Self {
        Self {
            revision,
            posture: RevisionPosture::Untracked,
        }
    }

    /// The revision identity.
    #[must_use]
    pub const fn revision(self) -> ContentAddress {
        self.revision
    }

    /// The posture the identity is held under.
    #[must_use]
    pub const fn posture(self) -> RevisionPosture {
        self.posture
    }
}

impl<Invocation, Conclusion> ExecutableAttachment<Invocation, Conclusion> {
    /// What makes one row executable: the references it is over, a
    /// posture-bearing revision binding for each, and the callable itself.
    #[must_use]
    pub const fn attached(
        subject: SubjectRoute,
        check: CheckRef,
        subject_revision: RevisionBinding,
        check_revision: RevisionBinding,
        call: fn(&Invocation) -> Conclusion,
    ) -> Self {
        Self {
            subject,
            check,
            subject_revision,
            check_revision,
            call,
        }
    }

    /// The subject route this attachment executes.
    #[must_use]
    pub const fn subject(&self) -> SubjectRoute {
        self.subject
    }

    /// The check reference this attachment judges under.
    #[must_use]
    pub const fn check(&self) -> CheckRef {
        self.check
    }

    /// The subject's revision binding.
    #[must_use]
    pub const fn subject_revision(&self) -> RevisionBinding {
        self.subject_revision
    }

    /// The check's revision binding.
    #[must_use]
    pub const fn check_revision(&self) -> RevisionBinding {
        self.check_revision
    }

    /// The weaker of the two revision postures — the one every per-posture
    /// reading of this attachment is stated over.
    #[must_use]
    pub const fn posture(&self) -> RevisionPosture {
        self.subject_revision
            .posture()
            .meet(self.check_revision.posture())
    }

    /// The callable, as the pure map it is.
    #[must_use]
    pub const fn call(&self) -> fn(&Invocation) -> Conclusion {
        self.call
    }

    /// The conclusion this attachment reaches over one set of invocation facts.
    #[must_use]
    pub fn conclude(&self, invocation: &Invocation) -> Conclusion {
        (self.call)(invocation)
    }
}

/// Cloning copies the five seats. The derive is not used because it would
/// demand `Invocation: Clone` and `Conclusion: Clone`, which the parameters do
/// not owe: they appear only behind a function pointer, and a function pointer
/// is `Copy` whatever its ends are.
impl<Invocation, Conclusion> Clone for ExecutableAttachment<Invocation, Conclusion> {
    fn clone(&self) -> Self {
        Self {
            subject: self.subject,
            check: self.check,
            subject_revision: self.subject_revision,
            check_revision: self.check_revision,
            call: self.call,
        }
    }
}

impl<Invocation, Conclusion> Binding<Invocation, Conclusion> {
    /// One row married to the attachment that executes it.
    ///
    /// # Errors
    ///
    /// Refuses a row and an attachment that name different subjects, then ones
    /// that name different checks — the marriage is what closes the seam a
    /// hidden row-to-function registry would open. Refuses, last, a row
    /// carrying producer facts inside a binding that names no schema the
    /// producer emitted against: a produced row whose pin went missing is a
    /// crossing with nothing pinning it.
    pub fn bound(
        row: Row,
        attachment: ExecutableAttachment<Invocation, Conclusion>,
        provenance: Provenance,
    ) -> Result<Self, BindingRefusal> {
        if row.subject() != attachment.subject() {
            return Err(BindingRefusal::SubjectMismatch {
                row: row.subject(),
                attachment: attachment.subject(),
            });
        }
        if row.check() != attachment.check() {
            return Err(BindingRefusal::CheckMismatch {
                row: row.check(),
                attachment: attachment.check(),
            });
        }
        if let (Origin::Generated(_), Provenance::Unproduced) = (row.origin(), provenance) {
            return Err(BindingRefusal::GeneratedWithoutSchemaPin);
        }
        Ok(Self {
            row,
            attachment,
            provenance,
        })
    }

    /// The row this binding carries.
    #[must_use]
    pub const fn row(&self) -> &Row {
        &self.row
    }

    /// The attachment that executes it.
    #[must_use]
    pub const fn attachment(&self) -> &ExecutableAttachment<Invocation, Conclusion> {
        &self.attachment
    }

    /// Whether a producer stands behind this binding, and which schema it
    /// emitted against.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// The semantic content that decides whether two bindings are one trial.
    #[must_use]
    pub const fn trial_key(&self) -> TrialKey {
        self.row.trial_key()
    }
}

/// Cloning copies the row, the attachment, and the provenance. The derive is
/// not used for the reason [`ExecutableAttachment`]'s realization states.
impl<Invocation, Conclusion> Clone for Binding<Invocation, Conclusion> {
    fn clone(&self) -> Self {
        Self {
            row: self.row.clone(),
            attachment: self.attachment.clone(),
            provenance: self.provenance,
        }
    }
}

impl<Invocation, Conclusion> AuthoredTable<Invocation, Conclusion> {
    /// The complete authored world, over the bindings authored into it.
    ///
    /// # Errors
    ///
    /// Refuses a binding carrying the candidate origin arm, so a candidate
    /// joins the authored world only through a human's admission. Refuses two
    /// bindings stating one trial, so a denominator can never read two where
    /// one thing is measured.
    pub fn authored(
        name: AuthoredTableName,
        provenance: Provenance,
        bindings: Vec<Binding<Invocation, Conclusion>>,
    ) -> Result<Self, AuthoredTableRefusal> {
        let mut trials = BTreeSet::new();
        for binding in &bindings {
            let key = binding.trial_key();
            if let Origin::Candidate(_) = binding.row().origin() {
                return Err(AuthoredTableRefusal::CandidateOrigin(key));
            }
            if !trials.insert(key) {
                return Err(AuthoredTableRefusal::DuplicateTrial(key));
            }
        }
        Ok(Self {
            name,
            provenance,
            bindings,
        })
    }

    /// The name this world is known by.
    #[must_use]
    pub const fn name(&self) -> AuthoredTableName {
        self.name
    }

    /// Whether a producer stands behind this table, and which schema it emitted
    /// against.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// Every binding this world holds, in authored order.
    #[must_use]
    pub fn bindings(&self) -> &[Binding<Invocation, Conclusion>] {
        &self.bindings
    }

    /// This world as the one sealed read surface.
    #[must_use]
    pub const fn view(&self) -> TableView<'_, Invocation, Conclusion> {
        TableView::Authored(self)
    }
}

impl<'parent, Invocation, Conclusion> StagedTableView<'parent, Invocation, Conclusion> {
    /// A complete authored world with candidates overlaid on it.
    ///
    /// # Errors
    ///
    /// Refuses an overlaid binding that does not carry the candidate origin
    /// arm, so the staging door cannot be an authoring door. Refuses a
    /// candidate stating a trial the parent or another candidate already
    /// states, so uniqueness holds across both worlds at once.
    pub fn staged(
        parent: &'parent AuthoredTable<Invocation, Conclusion>,
        candidates: Vec<Binding<Invocation, Conclusion>>,
    ) -> Result<Self, StagedTableRefusal> {
        let mut trials: BTreeSet<TrialKey> =
            parent.bindings().iter().map(Binding::trial_key).collect();
        for candidate in &candidates {
            let key = candidate.trial_key();
            match candidate.row().origin() {
                Origin::Candidate(_) => {}
                Origin::HandWritten
                | Origin::Generated(_)
                | Origin::AdmittedReplay(_)
                | Origin::AdmittedDischarge(_) => {
                    return Err(StagedTableRefusal::NotACandidate(key));
                }
            }
            if !trials.insert(key) {
                return Err(StagedTableRefusal::DuplicateTrial(key));
            }
        }
        Ok(Self { parent, candidates })
    }

    /// The authored world this view is overlaid on.
    #[must_use]
    pub const fn parent(&self) -> &'parent AuthoredTable<Invocation, Conclusion> {
        self.parent
    }

    /// The candidates overlaid, in staged order.
    #[must_use]
    pub fn candidates(&self) -> &[Binding<Invocation, Conclusion>] {
        &self.candidates
    }

    /// This staged world as the one sealed read surface.
    #[must_use]
    pub const fn view(&self) -> TableView<'_, Invocation, Conclusion> {
        TableView::Staged(self)
    }
}

impl<Invocation, Conclusion> TableView<'_, Invocation, Conclusion> {
    /// Every binding this view presents: the authored world in authored order,
    /// then the overlay in staged order.
    pub fn bindings(&self) -> impl Iterator<Item = &Binding<Invocation, Conclusion>> {
        let (authored, overlay): (
            &[Binding<Invocation, Conclusion>],
            &[Binding<Invocation, Conclusion>],
        ) = match self {
            Self::Authored(table) => (table.bindings(), &[]),
            Self::Staged(staged) => (staged.parent().bindings(), staged.candidates()),
        };
        authored.iter().chain(overlay.iter())
    }

    /// Which world this view presents, and — when it is staged — the authored
    /// parent it was overlaid on.
    #[must_use]
    pub fn posture(&self) -> TablePosture {
        match self {
            Self::Authored(_) => TablePosture::Authored,
            Self::Staged(staged) => TablePosture::Staged {
                parent: staged.parent().name(),
            },
        }
    }
}

impl SchemaField {
    /// One field of one producer-facing vocabulary, as the schema declares it.
    #[must_use]
    pub const fn declared(
        name: &'static str,
        shape: FieldShape,
        cardinality: FieldCardinality,
    ) -> Self {
        Self {
            name,
            shape,
            cardinality,
        }
    }

    /// The field's name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The shape its values take.
    #[must_use]
    pub const fn shape(self) -> FieldShape {
        self.shape
    }

    /// How many values it carries.
    #[must_use]
    pub const fn cardinality(self) -> FieldCardinality {
        self.cardinality
    }
}

/// The one roster law all three schema members are parsed under: a member
/// declares at least one field, every field is named, and no name is stated
/// twice.
fn parse_roster(fields: &'static [SchemaField]) -> Result<(), SchemaRefusal> {
    if fields.is_empty() {
        return Err(SchemaRefusal::EmptyRoster);
    }
    let mut names = BTreeSet::new();
    for field in fields {
        if field.name().is_empty() {
            return Err(SchemaRefusal::EmptyFieldName);
        }
        if !names.insert(field.name()) {
            return Err(SchemaRefusal::DuplicateFieldName(field.name()));
        }
    }
    Ok(())
}

impl DescriptorSchema {
    /// The descriptor vocabulary's roster, parsed under the roster law.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then an unnamed field, then a repeated name.
    pub fn declared(fields: &'static [SchemaField]) -> Result<Self, SchemaRefusal> {
        parse_roster(fields)?;
        Ok(Self { fields })
    }

    /// The roster this member declares, in declared order.
    #[must_use]
    pub const fn fields(self) -> &'static [SchemaField] {
        self.fields
    }
}

impl MutationPointSchema {
    /// The mutation-point vocabulary's roster, parsed under the roster law.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then an unnamed field, then a repeated name.
    pub fn declared(fields: &'static [SchemaField]) -> Result<Self, SchemaRefusal> {
        parse_roster(fields)?;
        Ok(Self { fields })
    }

    /// The roster this member declares, in declared order.
    #[must_use]
    pub const fn fields(self) -> &'static [SchemaField] {
        self.fields
    }
}

impl BenchSchema {
    /// The bench-row vocabulary's roster, parsed under the roster law.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then an unnamed field, then a repeated name.
    pub fn declared(fields: &'static [SchemaField]) -> Result<Self, SchemaRefusal> {
        parse_roster(fields)?;
        Ok(Self { fields })
    }

    /// The roster this member declares, in declared order.
    #[must_use]
    pub const fn fields(self) -> &'static [SchemaField] {
        self.fields
    }
}

impl GeneratedSupportSchema {
    /// The root declaration, over three members already parsed.
    #[must_use]
    pub const fn declared(
        descriptor: DescriptorSchema,
        mutation_point: MutationPointSchema,
        bench: BenchSchema,
    ) -> Self {
        Self {
            descriptor,
            mutation_point,
            bench,
        }
    }

    /// The descriptor member.
    #[must_use]
    pub const fn descriptor(self) -> DescriptorSchema {
        self.descriptor
    }

    /// The mutation-point member.
    #[must_use]
    pub const fn mutation_point(self) -> MutationPointSchema {
        self.mutation_point
    }

    /// The bench member.
    #[must_use]
    pub const fn bench(self) -> BenchSchema {
        self.bench
    }

    /// THE root declaration this harness publishes: the three rosters this home
    /// states, each parsed under the one roster law.
    ///
    /// # Authority
    ///
    /// One assembled value, in one place. Everything downstream reads it rather
    /// than rebuilding it from the rosters, so there is no second assembly that
    /// could pin a different set of members than this one.
    ///
    /// Its canonical bytes are
    /// [`encode_generated_support_schema`](crate::descriptor::encode_generated_support_schema)
    /// over this value, and its identity is [`identity`](Self::identity) over
    /// those bytes. The two steps are separate calls rather than one, because
    /// they refuse for separate reasons and folding them would put one cause
    /// where two are true.
    ///
    /// # Errors
    ///
    /// Refuses when any member's roster refuses — an empty roster, an unnamed
    /// field, or a repeated field name — in the declared member order:
    /// descriptor, then mutation-point, then bench.
    ///
    /// # Nonclaims
    ///
    /// The cause names the offending FIELD and not the member it was found in.
    /// A reader that needs the member reads which roster carries the name.
    pub fn published() -> Result<Self, SchemaRefusal> {
        Ok(Self::declared(
            DescriptorSchema::declared(DESCRIPTOR_FIELDS)?,
            MutationPointSchema::declared(MUTATION_POINT_FIELDS)?,
            BenchSchema::declared(BENCH_FIELDS)?,
        ))
    }

    /// The identity derived from this declaration's canonical bytes.
    ///
    /// This is the one derivation this home performs. The bytes
    /// ([`crate::descriptor::encode`]) are the preimage and this identity is
    /// derived from them under the schema family's own domain tag, so a change
    /// to ANY member moves it and one pin governs all three crossings.
    ///
    /// # Errors
    ///
    /// Refuses when the encoding refuses — a length past the width the encoding
    /// declares, which is unreachable on every target this crate is built for.
    pub fn identity(&self) -> Result<GeneratedSupportSchemaId, EncodeRefusal> {
        let preimage = encode_generated_support_schema(self)?;
        Ok(GeneratedSupportSchemaId(ContentAddress::derived(
            GENERATED_SUPPORT_SCHEMA_DOMAIN,
            &preimage,
        )))
    }
}

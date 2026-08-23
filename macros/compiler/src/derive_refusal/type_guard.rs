//! The derive home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Two of the roads here are the home's whole structural claim.
//! [`RefusalDeriveSurface::assembled`] is crate-internal, so the only way to
//! hold a captured surface is to have captured one;
//! [`RefusalFamilyExpansion::bound`] is crate-internal and binds through the
//! generic terminal, which takes a plan, a closure proved against that plan, and
//! a complete explanation answered over the two — so the only way to hold this
//! family's view, and therefore the only way to reach an emission, is to have
//! walked the whole road.
//! Neither can be written anywhere else, which is why deleting any step on that
//! road deletes the emission rather than shortening it.
//!
//! The two projections of a capture refusal — the compiler-facing line and the
//! structured diagnostic — read the refusal's own two private seats.
//!
//! One captured documentation row's canonical bytes are written here for the
//! same reason: the row's text and the declaration it was written on are private
//! seats, and the documentation commitment is derived over exactly them.

use super::{
    CapturedCause, CapturedCommitments, CapturedDocumentation, CapturedFamilyFacts,
    CauseOrderStanding, CrateBinding, DEFAULT_CRATE_BINDING, DeclaredMutations, DeclaredTrials,
    DeriveCauseLimit, DerivedMembership, DocumentedDeclaration, MutationDeclarationPosture,
    RefusalCompileContext, RefusalDerivationDraft, RefusalDeriveFact, RefusalDeriveSurface,
    RefusalFamilyExpansion, RefusalOwnerFacts, RefusalSite, TrialDeclarationPosture,
};
use crate::closure::{
    ClosedExpansion, ExpansionBindingRefusal, PartitionCargo, ProjectionClosure, RenderedProjection,
};
use crate::derive_refusal::diagnose::{composed, shown, witnessed};
use crate::derive_refusal::types::{LineBody, LineSite, RefusalClass, RefusalLine};
use crate::diagnostics::{
    DiagnosticSite, MacrocDiagnostic, MacrocPhase, RelatedSet, RepairAction, ReproductionRoute,
    SiteCoordinate,
};
use crate::explanation_protocol::ProjectionExplanationView;
use crate::mutation_descriptor::MutationDeclaration;
use crate::plane::{
    CapturedDeclarationSubject, CapturedTokenLimit, ClosedExpansionId, ContractSubject,
    ProjectionIdentity, ProjectionProvenance, ProjectionRole, ProjectionTranscript,
    ServiceEntrySubject, encode_bytes,
};
use crate::planning::{
    ProjectionDisposition, ProjectionPlan, RefusalFamilyImplementationProjection,
    RenderedImplementation,
};
use crate::test_descriptor::TrialTablePayload;
use crate::token::{GeneratedTree, SpanHandle, SpanTable};
use macroonz::{Bounded, FamilyShape};

impl CrateBinding {
    /// The default binding: the refusal contracts under their package name.
    #[must_use]
    pub fn default_binding() -> Self {
        Self {
            spelling: DEFAULT_CRATE_BINDING.to_owned(),
        }
    }

    /// The binding a caller declared with `crate = <name>`.
    #[must_use]
    pub fn declared(spelling: &str) -> Self {
        Self {
            spelling: spelling.to_owned(),
        }
    }

    /// The name the consumer calls the machine.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }
}

impl CapturedCause {
    /// Read one cause from its declared parts.
    #[must_use]
    pub fn read(spelling: &str, local_key: &str) -> Self {
        Self {
            spelling: spelling.to_owned(),
            local_key: local_key.to_owned(),
        }
    }

    /// The Rust variant that spells this cause.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }

    /// The local key the author declared for it.
    #[must_use]
    pub fn local_key(&self) -> &str {
        &self.local_key
    }
}

impl CapturedDocumentation {
    /// Read one documentation row off the attribute that carries it.
    ///
    /// Crate-internal, on the terms every other captured seat stands under: a
    /// row states that this text was written on this declaration at this token,
    /// and the only party that can say so is the walk that read it there.
    pub(crate) fn read(declared_on: DocumentedDeclaration, text: &str, token: SpanHandle) -> Self {
        Self {
            declared_on,
            text: text.to_owned(),
            token,
        }
    }

    /// Which declaration this row was written on.
    pub const fn declared_on(&self) -> &DocumentedDeclaration {
        &self.declared_on
    }

    /// The text the attribute carries, exactly as the capture read it.
    ///
    /// The escaping is the token producer's and was undone before the text
    /// reached this seat, so what stands here is what an author wrote rather
    /// than the spelling a literal wore.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The token this row sits at, as a handle into the producer's own span
    /// table.
    ///
    /// The ATTRIBUTE's own token and never the text inside it: a reader sent to
    /// a documentation row is sent to the line an author would edit.
    #[must_use]
    pub const fn token(&self) -> SpanHandle {
        self.token
    }

    /// Append this row's canonical bytes: which declaration it was written on,
    /// then the text it carries.
    ///
    /// The SPAN is not written and is not missing. A handle is the producer's
    /// own table index — two producers reading one declaration issue different
    /// ones — so a commitment that carried it would move for a reason no
    /// author's prose changed by. The handle belongs to the diagnostic rail,
    /// which is where it stays.
    ///
    /// Crate-internal, with one caller: the documentation commitment's
    /// derivation. A second road to these bytes would be a second spelling of
    /// what a row is.
    pub(crate) fn encode_into(&self, into: &mut Vec<u8>) {
        self.declared_on.encode_into(into);
        encode_bytes(self.text.as_bytes(), into);
    }
}

impl DocumentedDeclaration {
    /// The seat's discriminant byte, written ahead of whatever it names so a row
    /// on the family never encodes as a row on a variant.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::Family => 0,
            Self::Variant(_) => 1,
        }
    }

    /// Append this seat's canonical bytes: the discriminant, then the variant's
    /// spelling where the seat names one.
    ///
    /// The family arm writes an empty material behind its discriminant rather
    /// than nothing at all, so the two arms are cut at the same boundary and a
    /// family row can never be read as a variant row with an empty name.
    pub(crate) fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        match self {
            Self::Family => encode_bytes(&[], into),
            Self::Variant(spelling) => encode_bytes(spelling.as_bytes(), into),
        }
    }
}

impl DeclaredTrials {
    /// The rows one declaration states, under the commitment the capture named
    /// them by.
    ///
    /// Crate-internal, on the terms every other captured seat stands under: the
    /// commitment is derived over the exact token material the payload was read
    /// from, and the only party that can say so is the walk that read it.
    pub(crate) const fn read(
        commitment: ProjectionIdentity<CapturedDeclarationSubject>,
        payload: TrialTablePayload,
    ) -> Self {
        Self {
            commitment,
            payload,
        }
    }

    /// The commitment these rows are named under.
    #[must_use]
    pub const fn commitment(&self) -> ProjectionIdentity<CapturedDeclarationSubject> {
        self.commitment
    }

    /// The rows themselves, as the carrier's own payload.
    #[must_use]
    pub const fn payload(&self) -> &TrialTablePayload {
        &self.payload
    }
}

impl DeclaredMutations {
    /// Bind the parsed helper to the commitment derived over its exact body.
    pub(crate) const fn read(
        commitment: ProjectionIdentity<CapturedDeclarationSubject>,
        declaration: MutationDeclaration,
    ) -> Self {
        Self {
            commitment,
            declaration,
        }
    }

    /// The independent mutation-helper commitment.
    #[must_use]
    pub const fn commitment(&self) -> ProjectionIdentity<CapturedDeclarationSubject> {
        self.commitment
    }

    /// The helper's complete typed reading.
    #[must_use]
    pub const fn declaration(&self) -> &MutationDeclaration {
        &self.declaration
    }
}

impl RefusalDeriveSurface {
    /// Assemble one captured surface.
    ///
    /// Crate-internal: the only road to one is the capture itself.
    pub(crate) fn assembled(
        facts: CapturedFamilyFacts,
        causes: Bounded<CapturedCause, DeriveCauseLimit>,
        documentation: Bounded<CapturedDocumentation, CapturedTokenLimit>,
        trials: TrialDeclarationPosture,
        mutations: MutationDeclarationPosture,
        membership: DerivedMembership,
        commitments: CapturedCommitments,
    ) -> Self {
        let CapturedFamilyFacts {
            family_name,
            family_id,
            binding,
            shape,
        } = facts;
        Self {
            family_name,
            family_id,
            binding,
            shape,
            causes,
            documentation,
            trials,
            mutations,
            membership,
            commitments,
        }
    }

    /// The declared family's Rust name.
    #[must_use]
    pub fn family_name(&self) -> &str {
        &self.family_name
    }

    /// The declared family's stable identity, as `<domain>.<family>`.
    #[must_use]
    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    /// How the consumer names the machine.
    #[must_use]
    pub const fn binding(&self) -> &CrateBinding {
        &self.binding
    }

    /// The declared body shape.
    #[must_use]
    pub const fn shape(&self) -> FamilyShape {
        self.shape
    }

    /// The declared causes, in the canonical selection order the author stated.
    pub fn causes(&self) -> impl Iterator<Item = &CapturedCause> {
        self.causes.iter()
    }

    /// The documentation rows the declaration carries, in the order the walk
    /// read them: the family's own ahead of the variants', and each variant's in
    /// the order its lines were written.
    ///
    /// # Ordering
    ///
    /// This order IS meaning for the rows of one declaration, because prose
    /// reordered is prose rewritten. It ranks nothing across declarations: which
    /// declaration a row belongs to is the row's own seat
    /// ([`CapturedDocumentation::declared_on`]) rather than its position here.
    pub fn documentation(&self) -> impl Iterator<Item = &CapturedDocumentation> {
        self.documentation.iter()
    }

    /// This captured declaration's SEMANTIC commitment — what the declaration
    /// IS, derived from its token material with every documentation attribute
    /// set aside, so the same declaration captures to the same identity whoever
    /// produced the tokens.
    ///
    /// # Content
    ///
    /// The derivation runs over the declaration's own canonical bytes at full
    /// length, minus the documentation attributes the walk drops. So a
    /// declaration whose SHAPE, family identity, binding, or causes changed
    /// captures to a different identity, plans differently, and closes under a
    /// different expansion — and a declaration whose PROSE alone changed keeps
    /// this name, which is what an implementation projection is entitled to.
    ///
    /// # Authority
    ///
    /// Current implementation, test, and codec planning reads this semantic commitment.
    /// The separate documentation identity remains available only as an inspectable reading of the same capture.
    #[must_use]
    pub const fn identity(&self) -> ProjectionIdentity<CapturedDeclarationSubject> {
        self.commitments.semantic()
    }

    /// This captured declaration's DOCUMENTATION commitment — what the
    /// declaration SAYS, derived over the semantic commitment and the ordered
    /// rows above it.
    ///
    /// # Content
    ///
    /// The semantic commitment stands at the anchor, followed by each row's declaration seat and text in capture order.
    /// A prose-only change moves this identity without moving the semantic identity.
    ///
    /// # Bounds
    ///
    /// It is a second READING of one surface and never a second account of it.
    /// It is derived FROM the semantic commitment, so it cannot name a
    /// declaration the semantic commitment does not; and the rows it stands over
    /// are cut from the material that commitment was taken over.
    #[must_use]
    pub const fn documentation_identity(&self) -> ProjectionIdentity<CapturedDeclarationSubject> {
        self.commitments.documentation()
    }

    /// Whether this declaration states trial rows, and the rows where it does.
    pub const fn trials(&self) -> &TrialDeclarationPosture {
        &self.trials
    }

    /// Whether this declaration states generated mutation policy and mapping.
    pub const fn mutations(&self) -> &MutationDeclarationPosture {
        &self.mutations
    }

    /// Fix the complete declared output set.
    ///
    /// This is the one road to a [`RefusalDerivationDraft`].
    #[must_use]
    pub fn planned(self) -> RefusalDerivationDraft {
        let membership = self.membership;
        RefusalDerivationDraft {
            surface: self,
            membership,
        }
    }
}

pub use seat::RefusalDeriveRefusal;

mod seat {
    use super::super::{RefusalDeriveCapture, RefusalSite};
    use crate::token::SpanHandle;

    /// One capture refusal: the established cause, and where it was established.
    ///
    /// Both seats are required.
    /// A refusal that could omit its site would send the caller looking, and a
    /// refusal that could omit its cause would be a complaint rather than an
    /// answer.
    #[must_use = "a capture refusal carries the established cause and where it was established"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RefusalDeriveRefusal {
        cause: RefusalDeriveCapture,
        site: RefusalSite,
    }

    impl RefusalDeriveRefusal {
        /// The established refusal at one site of the declared input.
        ///
        /// Reachable only from inside this home, which is where the capture pass
        /// lives.
        /// Both seats are private, so no caller can write the literal, and this
        /// road is what a caller would reach for instead.
        /// A cause word plus a site are both values anybody can spell, so a
        /// public road here would hand any holder of those two a refusal the
        /// capture pass never established, at a place it never read.
        pub(in crate::derive_refusal) const fn established(
            cause: RefusalDeriveCapture,
            site: RefusalSite,
        ) -> Self {
            Self { cause, site }
        }

        /// The established cause.
        pub const fn cause(self) -> RefusalDeriveCapture {
            self.cause
        }

        /// Where the observation was established: one token of a captured
        /// declaration, or one byte of a text that refused before any capture
        /// existed.
        pub const fn site(self) -> RefusalSite {
            self.site
        }

        /// The token the observation sits at, where a capture issued one.
        ///
        /// The producer resolves it to the exact compiler span; the services
        /// never do.
        ///
        /// # Nonclaims
        ///
        /// It answers with nothing for a refusal established BEFORE a capture,
        /// because no table was built and no handle was issued. That is a stated
        /// posture rather than a missing value: a handle invented here would
        /// index a table that never existed, and would read exactly like an
        /// honest handle naming the declaration's first token.
        /// The byte such a refusal was born at is on
        /// [`RefusalDeriveRefusal::site`].
        #[must_use]
        pub const fn token(self) -> Option<SpanHandle> {
            match self.site {
                RefusalSite::AtToken(token) => Some(token),
                RefusalSite::BeforeCapture(_) => None,
            }
        }
    }
}

impl RefusalDeriveRefusal {
    /// Where this refusal sits, as the diagnostics home's own site.
    ///
    /// [`RefusalSite`]'s two arms land on [`DiagnosticSite`]'s two arms, one
    /// for one: a CAPTURED refusal names its handle and carries whatever the
    /// producer's table answered about it — including the typed statement that
    /// the table does not reach the handle, which is a fact about the TABLE and
    /// leaves the observation standing — and a PRE-CAPTURE refusal carries the
    /// byte it was born at, under an arm that has no handle seat at all.
    ///
    /// # Nonclaims
    ///
    /// No handle is invented here, and none can be: the pre-capture arm
    /// declares no [`SpanHandle`](crate::token::SpanHandle) seat, so there is
    /// nothing for handle zero to be forced into and no branch in which this
    /// home would write one.
    /// Lifting that byte into the answered coordinate posture is
    /// [`DiagnosticSite::coordinate`]'s statement to make, and it is made once,
    /// there — this road repeats it nowhere.
    fn diagnostic_site(self, spans: &SpanTable) -> DiagnosticSite {
        match self.site() {
            RefusalSite::AtToken(token) => DiagnosticSite::at_token(
                token,
                SiteCoordinate::answered(spans.coordinate_of(token)),
            ),
            RefusalSite::BeforeCapture(coordinate) => DiagnosticSite::before_capture(coordinate),
        }
    }

    /// The one line this refusal reaches a compiler under.
    ///
    /// The ONE owner of the capture family's compiler prose: both the public
    /// projection ([`RefusalDeriveRefusal::compiler_message`]) and the
    /// structured diagnostic's summary are this string, so the two cannot say
    /// different things about one refusal.
    ///
    /// Every part of it is a projection of a typed value — the home's declared
    /// prefix, the refusal class, the cause's own description, and the site
    /// clause the coordinate composes — and no phrase here restates any of them
    /// in other words.
    fn compiler_line(self, coordinate: SiteCoordinate) -> String {
        composed(
            &RefusalLine {
                class: RefusalClass::DeclarationNotRead,
                first: self.cause().described(),
                // The capture family is single-cause: it establishes one cause
                // and enumerates nothing, so there is no remainder to report and
                // no examination bound an enumeration could have stopped at.
                body: LineBody::SingleCause,
            },
            LineSite::At(coordinate),
        )
    }

    /// The compiler-facing rendering: one line naming the cause and where it
    /// was established, in whatever coordinate role the producer speaks.
    ///
    /// A projection of the typed value, produced here so that the expansion
    /// shell composes no sentence of its own.
    ///
    /// Where the supplied table does not reach the handle, the line says THAT
    /// rather than a position.
    /// The cause is established either way — it is a fact about the declaration,
    /// not about the table — so the sentence still names it, and the reader is
    /// told the locating half is missing instead of being handed a number that
    /// means nothing.
    #[must_use]
    pub fn compiler_message(self, spans: &SpanTable) -> String {
        self.compiler_line(self.diagnostic_site(spans).coordinate())
    }

    /// Project this refusal into the services' structured diagnostic.
    ///
    /// The machine anchoring is the CALLER's: where a caller holds the machine's
    /// identities it supplies them, and where none exists at this seam the
    /// diagnostic says so.
    /// This module mints none of them, because none of them is its to mint — the
    /// services classify what they OBSERVED
    /// ([`RefusalDeriveCapture::observed`](super::RefusalDeriveCapture::observed))
    /// and never mint the machine's cause commitment.
    ///
    /// The repair cites the fact THIS cause is a violation of
    /// ([`RefusalDeriveCapture::declared_by`](super::RefusalDeriveCapture::declared_by))
    /// and shows that fact's own repair.
    /// One citation for the whole family would point a caller repairing a
    /// malformed local key at the rule about body shapes.
    pub fn diagnosed(self, spans: &SpanTable) -> MacrocDiagnostic {
        // Built once and read twice: the prose and the diagnostic's own site
        // are projections of the SAME value, so a line saying one position
        // beside a seat holding another is unrepresentable here.
        let site = self.diagnostic_site(spans);
        let coordinate = site.coordinate();
        // The capture road establishes one cause and enumerates nothing, so
        // there is no per-issue set to stop short of: zero identities are
        // carried and zero are omitted.
        let related = RelatedSet::nothing_enumerated();
        let fact = self.cause().declared_by();
        MacrocDiagnostic {
            summary: shown(&witnessed(
                &self.compiler_line(coordinate),
                related.completion(),
            )),
            phase: MacrocPhase::Capture,
            site,
            expected: expected_contract(),
            observed: self.cause().observed(),
            related,
            repairs: Bounded::from_array([RepairAction {
                declared_by: fact.citation(),
                description: fact.repair(),
            }]),
            reproduction: ReproductionRoute::CallableServices {
                entry: callable_entry(),
            },
        }
    }
}

/// The compiler-plane contract this derive expects a declaration to satisfy.
///
/// # A declared name, under the declared-name family
///
/// The preimage is one thing: a stable name this home wrote down. It is not a
/// closed expansion and it holds no member of that grammar, so it stands under
/// [`ProjectionRole::DeclaredName`] and its own family's version ladder — where
/// it used to ride the closed-expansion role and be renamed by every bump to
/// what a terminal commits to.
#[must_use]
pub fn expected_contract() -> ProjectionIdentity<ContractSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::DeclaredName,
        b"macroc.derive_refusal.declaration-grammar",
        0,
    ))
}

/// The callable entry point that reproduces one observation without a
/// proc-macro anywhere in the path.
///
/// A declared name on the same terms as [`expected_contract`], separated from it
/// by its own SUBJECT and by its own content, and standing at roster position
/// one because this home declares two such names.
#[must_use]
pub fn callable_entry() -> ProjectionIdentity<ServiceEntrySubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::DeclaredName,
        b"macroc.derive_refusal.compile_refusal",
        1,
    ))
}

impl RefusalDerivationDraft {
    /// The captured surface this draft was fixed from.
    #[must_use]
    pub const fn surface(&self) -> &RefusalDeriveSurface {
        &self.surface
    }

    /// The complete declared output set.
    #[must_use]
    pub const fn declared_membership(&self) -> DerivedMembership {
        self.membership
    }

    /// Whether the typed cause order stands for this family's shape.
    #[must_use]
    pub const fn cause_order_standing(&self) -> CauseOrderStanding {
        match self.membership {
            DerivedMembership::FamilyAndCauseOrder
            | DerivedMembership::FamilyCauseOrderAndMutationEvaluation => {
                CauseOrderStanding::Declared
            }
            DerivedMembership::FamilyOnly => CauseOrderStanding::NotApplicableToShape,
        }
    }
}

impl RefusalOwnerFacts {
    /// The three facts, cited by the declared names the refusal home wrote down.
    ///
    /// This is the posture an expansion runs under: the refusal home's fact
    /// identities are not published to the compiler plane, so the citation names
    /// them and mints nothing.
    ///
    /// The names are READ off [`RefusalDeriveFact`] rather than spelled here.
    /// A name spelled at this seat and again at a diagnostic's repair would be
    /// two spellings of one fact, and the two encode differently the moment
    /// either one is edited.
    #[must_use]
    pub const fn declared() -> Self {
        let shape_ruled = RefusalDeriveFact::CanonicalOrderStandsForSingleCauseAlone;
        Self {
            body_shapes: RefusalDeriveFact::BodyShapesAreThreeAndClosed.citation(),
            canonical_order_is_shape_ruled: shape_ruled.citation(),
            cause_key_grammar: RefusalDeriveFact::CauseIdentityIsFamilyAndKey.citation(),
        }
    }
}

impl RefusalCompileContext {
    /// The context an expansion shell supplies: it holds the compiler's spans,
    /// holds none of the machine's identities, and cites the refusal home's
    /// facts by their declared names.
    #[must_use]
    pub fn expanding() -> Self {
        Self {
            spans: SpanTable::ProducerHeld,
            owner_facts: RefusalOwnerFacts::declared(),
            nonclaims: Bounded::empty(),
        }
    }
}

impl RefusalFamilyExpansion {
    /// Bind one refusal-family expansion: this family's two facts, over the
    /// closed expansion the generic terminal binds.
    ///
    /// Crate-internal: the only road to one is
    /// [`compile_refusal`](crate::derive_refusal::compile_refusal).
    ///
    /// # One binding, and no transcript of its own
    ///
    /// The plan, the proof, and the explanation are handed straight to
    /// [`ClosedExpansion::bound`], which derives the identity and refuses a
    /// closure proved against another plan or an explanation answered over
    /// another plan or closure. This road derives nothing.
    /// A second transcript here would be a second name for one expansion — two
    /// identities over one plan, one proof, and one explanation, agreeing until
    /// either derivation was edited — and the terminal's is the one every
    /// projection kind's door already ends at.
    ///
    /// # Errors
    ///
    /// Returns [`ExpansionBindingRefusal`] exactly as the terminal returns it,
    /// naming the two identities the binding was asked to hold as one.
    /// It is handed through rather than folded into a diagnostic here, because
    /// the road that projects it is [`diagnose::expansion_refused`], and this
    /// seat composes no sentence of its own.
    ///
    /// [`diagnose::expansion_refused`]: crate::derive_refusal::diagnose::expansion_refused
    pub(crate) fn bound(
        surface: RefusalDeriveSurface,
        plan: ProjectionPlan<RefusalFamilyImplementationProjection>,
        closure: ProjectionClosure<RenderedImplementation>,
        explanation: ProjectionExplanationView<RefusalFamilyImplementationProjection>,
        cause_order: ProjectionDisposition,
    ) -> Result<Self, ExpansionBindingRefusal> {
        let expansion = ClosedExpansion::bound(plan, closure, explanation)?;
        Ok(Self {
            surface,
            expansion,
            cause_order,
        })
    }

    /// The closed expansion this view stands over — the terminal every
    /// projection kind's door ends at.
    ///
    /// Every road below that answers about the plan, the proof, the explanation,
    /// or an emission reads THIS value, so a caller that wants the terminal's own
    /// surface — its published artifacts, its delivery addressing — reads it here
    /// rather than through a copy of it seated beside one.
    pub const fn expansion(&self) -> &ClosedExpansion<RefusalFamilyImplementationProjection> {
        &self.expansion
    }

    /// This expansion's own identity: the terminal's, and never a second one
    /// derived beside it.
    #[must_use]
    pub const fn identity(&self) -> ClosedExpansionId {
        self.expansion.identity()
    }

    /// How that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &ProjectionProvenance {
        self.expansion.provenance()
    }

    /// The captured typed declaration this expansion was compiled from.
    ///
    /// This family's own fact and the one seat the terminal does not carry: the
    /// terminal is generic over every projection kind, and a captured
    /// refusal-family surface is a value only this door produces.
    #[must_use]
    pub const fn surface(&self) -> &RefusalDeriveSurface {
        &self.surface
    }

    /// The complete plan: context, content, membership, invalidation set,
    /// decision trace, origin trail, and nonclaims.
    pub const fn plan(&self) -> &ProjectionPlan<RefusalFamilyImplementationProjection> {
        self.expansion.plan()
    }

    /// The proof that what was rendered is what was planned.
    pub const fn closure(&self) -> &ProjectionClosure<RenderedImplementation> {
        self.expansion.closure()
    }

    /// The complete explanation over this kind's applicable questions, answered
    /// over the plan and the proof above.
    pub const fn explanation(
        &self,
    ) -> &ProjectionExplanationView<RefusalFamilyImplementationProjection> {
        self.expansion.explanation()
    }

    /// What happened to the typed cause-order projection.
    ///
    /// This family's other fact, and the second seat the terminal does not
    /// carry: which related projection a shape declares is a question about a
    /// refusal family's shape, and the explanation protocol asks it of this kind
    /// alone.
    pub const fn cause_order(&self) -> &ProjectionDisposition {
        &self.cause_order
    }

    /// What the declaration site expands into: the cargo the consumer's normal
    /// build compiles, and the only cargo it compiles.
    ///
    /// The generated mutation module is not here and cannot be.
    /// It is planned into the TEST CARRIER, the proof splits the rendering by the delivery each member declared, and this road reads the declaration-site seat of that split.
    ///
    /// It is the CLOSURE's own proved cargo, reached through the terminal: no
    /// second join happens anywhere, so what is emitted is what was proved.
    pub const fn emitted(&self) -> &PartitionCargo {
        self.expansion.declaration_site()
    }

    /// What the declaration site's cargo looks like as Rust source text — an
    /// inspection projection of the SAME tokens that are emitted, never a second
    /// rendering.
    ///
    /// # Nonclaims
    ///
    /// It answers with nothing where the plan declared no member into the
    /// declaration site. That is a stated posture rather than a missing value:
    /// an empty text is what a rendering of no tokens projects, and an
    /// unoccupied emission is one nothing was ever planned into. This road never
    /// turns the second into the first.
    #[must_use]
    pub fn inspected(&self) -> Option<String> {
        self.emitted().tokens().map(GeneratedTree::inspected)
    }

    /// The rendering this expansion closed over — every unit, under every role,
    /// whichever delivery it was planned into.
    #[must_use]
    pub const fn rendered(&self) -> &RenderedProjection<RenderedImplementation> {
        self.expansion.closure().rendered()
    }
}

impl CapturedCommitments {
    /// The pair one capture derived, in the order they depend on each other.
    ///
    /// The semantic one first, because the documentation one stands over it:
    /// the signature is the order, so a caller cannot seat the pair backwards.
    #[must_use]
    pub(crate) const fn derived(
        semantic: ProjectionIdentity<CapturedDeclarationSubject>,
        documentation: ProjectionIdentity<CapturedDeclarationSubject>,
    ) -> Self {
        Self {
            semantic,
            documentation,
        }
    }

    /// The declaration's own semantic commitment.
    #[must_use]
    pub const fn semantic(self) -> ProjectionIdentity<CapturedDeclarationSubject> {
        self.semantic
    }

    /// The commitment over the prose written on that declaration.
    #[must_use]
    pub const fn documentation(self) -> ProjectionIdentity<CapturedDeclarationSubject> {
        self.documentation
    }
}

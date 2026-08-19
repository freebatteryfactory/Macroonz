//! The derive home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Two of the roads here are the home's whole structural claim.
//! [`RefusalDeriveSurface::assembled`] is crate-internal, so the only way to
//! hold a captured surface is to have captured one; [`ClosedExpansion::bound`]
//! is crate-internal and binds through the generic terminal, which takes a plan,
//! a closure proved against that plan, and a complete explanation — so the only
//! way to hold this family's view, and therefore the only way to reach an
//! emission, is to have walked the whole road.
//! Neither can be written anywhere else, which is why deleting any step on that
//! road deletes the emission rather than shortening it.
//!
//! The two projections of a capture refusal — the compiler-facing line and the
//! structured diagnostic — read the refusal's own two private seats.

use super::{
    CapturedCause, CapturedDocumentation, CauseOrderStanding, ClosedExpansion, CrateBinding,
    DEFAULT_CRATE_BINDING, DerivedMembership, DocumentedDeclaration, RefusalCompileContext,
    RefusalDerivationDraft, RefusalDeriveFact, RefusalDeriveSurface, RefusalOwnerFacts, RefusalSite,
};
use crate::closure::{
    PartitionCargo, ProjectionClosure, ProjectionReceipt, ReceiptBindingRefusal, RenderedProjection,
};
use crate::derive_refusal::diagnose::{
    LineBody, LineSite, RefusalClass, RefusalLine, composed, shown, witnessed,
};
use crate::diagnostics::{
    DiagnosticSite, MachineAnchoring, MacrocDiagnostic, MacrocPhase, RelatedSet, ReleasePosture,
    RepairAction, ReproductionRoute, SiteCoordinate,
};
use crate::explanation_protocol::ProjectionExplanationView;
use crate::plane::{
    CapturedDeclarationSubject, CapturedTokenLimit, ClosedExpansionId, ContractSubject,
    DeriveCauseLimit, ProjectionIdentity, ProjectionProvenance, ProjectionRole,
    ProjectionTranscript, ServiceEntrySubject,
};
use crate::planning::{
    DeriveImplProjection, ProjectionDisposition, ProjectionPlan, RenderedImplementation,
};
use crate::token::{GeneratedTree, SpanHandle, SpanTable};
use threadpak::evidence::CauseDisposition;
use threadpak::refusal::FamilyShape;
use threadpak::types::Bounded;

impl CrateBinding {
    /// The default binding: the machine under its own package name.
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
    #[must_use]
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
}

impl RefusalDeriveSurface {
    /// Assemble one captured surface.
    ///
    /// Crate-internal: the only road to one is the capture itself.
    pub(crate) const fn assembled(
        family_name: String,
        family_id: String,
        binding: CrateBinding,
        shape: FamilyShape,
        causes: Bounded<CapturedCause, DeriveCauseLimit>,
        documentation: Bounded<CapturedDocumentation, CapturedTokenLimit>,
        identity: ProjectionIdentity<CapturedDeclarationSubject>,
    ) -> Self {
        Self {
            family_name,
            family_id,
            binding,
            shape,
            causes,
            documentation,
            identity,
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

    /// This captured declaration's own identity — derived from the token
    /// material, so the same declaration captures to the same identity whoever
    /// produced the tokens.
    ///
    /// # Content
    ///
    /// **The token material is the whole of what this stands over, and the
    /// documentation is part of it.** The derivation runs over the declared
    /// input's canonical bytes at full length, and a documentation attribute is
    /// declared input like any other token — so a declaration whose prose
    /// changed captures to a different identity, plans differently, and closes
    /// under a different receipt. The rows on this surface are a READING of that
    /// same material, never a second thing to commit to.
    #[must_use]
    pub const fn identity(&self) -> ProjectionIdentity<CapturedDeclarationSubject> {
        self.identity
    }

    /// Fix the complete declared output set.
    ///
    /// This is the one road to a [`RefusalDerivationDraft`].
    #[must_use]
    pub fn planned(self) -> RefusalDerivationDraft {
        let membership = match self.shape {
            FamilyShape::SingleCause => DerivedMembership::FamilyAndCauseOrder,
            FamilyShape::IssueCollection | FamilyShape::InseparablePair => {
                DerivedMembership::FamilyOnly
            }
        };
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
    pub fn diagnosed(self, spans: &SpanTable, machine: MachineAnchoring) -> MacrocDiagnostic {
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
            machine,
            summary: shown(&witnessed(
                &self.compiler_line(coordinate),
                related.completion(),
            )),
            phase: MacrocPhase::Capture,
            site,
            expected: expected_contract(),
            observed: self.cause().observed(),
            // The plane classifies what it observed and never elects the
            // machine's cause posture: narrowing is the machine's progress to
            // report, not the compiler plane's to assert.
            cause: CauseDisposition::UnresolvedCause,
            related,
            repairs: Bounded::from_array([RepairAction {
                declared_by: fact.citation(),
                description: fact.repair(),
            }]),
            reproduction: ReproductionRoute::CallableServices {
                entry: callable_entry(),
            },
            release: ReleasePosture::NoReleasePromise,
        }
    }
}

/// The compiler-plane contract this derive expects a declaration to satisfy.
#[must_use]
pub fn expected_contract() -> ProjectionIdentity<ContractSubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::ClosedExpansion,
        b"macroc.derive_refusal.declaration-grammar",
        0,
    ))
}

/// The callable entry point that reproduces one observation without a
/// proc-macro anywhere in the path.
#[must_use]
pub fn callable_entry() -> ProjectionIdentity<ServiceEntrySubject> {
    ProjectionIdentity::derived(ProjectionTranscript::rooted(
        ProjectionRole::ClosedExpansion,
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
            DerivedMembership::FamilyAndCauseOrder => CauseOrderStanding::Declared,
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
            machine: MachineAnchoring::UnmintedAtThisSeam,
            owner_facts: RefusalOwnerFacts::declared(),
            nonclaims: Bounded::empty(),
        }
    }
}

impl ClosedExpansion {
    /// Bind one closed expansion: this family's two facts, over the receipt the
    /// generic terminal binds.
    ///
    /// Crate-internal: the only road to one is
    /// [`compile_refusal`](crate::derive_refusal::compile_refusal).
    ///
    /// # One binding, and no transcript of its own
    ///
    /// The plan, the proof, and the explanation are handed straight to
    /// [`ProjectionReceipt::bound`], which derives the identity and refuses a
    /// closure proved against another plan. This road derives nothing.
    /// A second transcript here would be a second name for one expansion — two
    /// identities over one plan and one proof, agreeing until either derivation
    /// was edited — and the terminal's is the one every projection kind's door
    /// already ends at.
    ///
    /// # Errors
    ///
    /// Returns [`ReceiptBindingRefusal`] exactly as the terminal returns it,
    /// naming the plan handed in and the plan the closure was proved against.
    /// It is handed through rather than folded into a diagnostic here, because
    /// the road that projects it is [`diagnose::receipt_refused`], and this seat
    /// composes no sentence of its own.
    ///
    /// [`diagnose::receipt_refused`]: crate::derive_refusal::diagnose::receipt_refused
    pub(crate) fn bound(
        surface: RefusalDeriveSurface,
        plan: ProjectionPlan<DeriveImplProjection>,
        closure: ProjectionClosure<RenderedImplementation>,
        explanation: ProjectionExplanationView<DeriveImplProjection>,
        cause_order: ProjectionDisposition,
    ) -> Result<Self, ReceiptBindingRefusal> {
        let receipt = ProjectionReceipt::bound(plan, closure, explanation)?;
        Ok(Self {
            surface,
            receipt,
            cause_order,
        })
    }

    /// The receipt this view stands over — the terminal every projection kind's
    /// door ends at.
    ///
    /// Every road below that answers about the plan, the proof, the explanation,
    /// or an emission reads THIS value, so a caller that wants the terminal's own
    /// surface — its published artifacts, its delivery addressing — reads it here
    /// rather than through a copy of it seated beside one.
    #[must_use]
    pub const fn receipt(&self) -> &ProjectionReceipt<DeriveImplProjection> {
        &self.receipt
    }

    /// This closed expansion's own identity: the receipt's, and never a second
    /// one derived beside it.
    #[must_use]
    pub const fn identity(&self) -> ClosedExpansionId {
        self.receipt.identity()
    }

    /// How that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &ProjectionProvenance {
        self.receipt.provenance()
    }

    /// The captured typed declaration this expansion was compiled from.
    ///
    /// This family's own fact and the one seat the receipt does not carry: the
    /// terminal is generic over every projection kind, and a captured
    /// refusal-family surface is a value only this door produces.
    #[must_use]
    pub const fn surface(&self) -> &RefusalDeriveSurface {
        &self.surface
    }

    /// The complete plan: context, content, membership, invalidation set,
    /// decision trace, origin trail, and nonclaims.
    pub const fn plan(&self) -> &ProjectionPlan<DeriveImplProjection> {
        self.receipt.plan()
    }

    /// The proof that what was rendered is what was planned.
    pub const fn closure(&self) -> &ProjectionClosure<RenderedImplementation> {
        self.receipt.closure()
    }

    /// The complete explanation over this kind's applicable questions.
    pub const fn explanation(&self) -> &ProjectionExplanationView<DeriveImplProjection> {
        self.receipt.explanation()
    }

    /// What happened to the typed cause-order projection.
    ///
    /// This family's other fact, and the second seat the receipt does not carry:
    /// which related projection a shape declares is a question about a refusal
    /// family's shape, and the explanation protocol asks it of this kind alone.
    pub const fn cause_order(&self) -> &ProjectionDisposition {
        &self.cause_order
    }

    /// What the declaration site expands into: the cargo the consumer's normal
    /// build compiles, and the only cargo it compiles.
    ///
    /// The evaluation copies are not here and cannot be. They are planned into
    /// the TEST CARRIER, the proof splits the rendering by the delivery each
    /// member declared, and this road reads the declaration-site seat of that
    /// split — so a selector-bearing copy standing in what a normal build
    /// compiles is not a value this road can hand back.
    ///
    /// It is the CLOSURE's own proved cargo, reached through the receipt: no
    /// second join happens anywhere, so what is emitted is what was proved.
    #[must_use]
    pub const fn emitted(&self) -> &PartitionCargo {
        self.receipt.declaration_site()
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
        self.receipt.closure().rendered()
    }
}

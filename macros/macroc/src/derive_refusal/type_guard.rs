//! The derive home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child. Two of the roads here are the
//! home's whole structural claim. [`RefusalDeriveSurface::assembled`] is
//! crate-internal, so the only way to hold a captured surface is to have
//! captured one; [`ClosedExpansion::bound`] is crate-internal and takes a proved
//! closure, so the only way to hold a receipt — and therefore the only way to
//! reach an emitted tree — is to have walked the whole road. Neither can be
//! written anywhere else, which is why deleting any step on that road deletes
//! the emission rather than shortening it.
//!
//! The two projections of a capture refusal — the compiler-facing line and the
//! structured diagnostic — read the refusal's own two private seats, so they sit
//! here beside them rather than in `diagnose.rs`, which projects the refusals
//! raised by the later stages.

use super::{
    CapturedCause, CauseOrderStanding, ClosedExpansion, CrateBinding, DEFAULT_CRATE_BINDING,
    DerivedMembership, RefusalCompileContext, RefusalDerivationDraft, RefusalDeriveRefusal,
    RefusalDeriveSurface, RefusalOwnerFacts,
};
use crate::closure::{ProjectionClosure, RenderedProjection};
use crate::diagnostics::{
    DiagnosticSite, MachineAnchoring, MacrocDiagnostic, MacrocPhase, RelatedSetCompletion,
    ReleasePosture, RepairAction, ReproductionRoute, SiteCoordinate,
};
use crate::explanation_protocol::ProjectionExplanationView;
use crate::plane::{
    CapturedDeclarationSubject, ClosedExpansionId, ContractSubject, DeriveCauseLimit, OwnerFactRef,
    ProjectionIdentity, ProjectionProvenance, ProjectionRole, ProjectionTranscript,
    ServiceEntrySubject, encode_bytes,
};
use crate::planning::{
    DeriveImplProjection, ProjectionDisposition, ProjectionPlan, RenderedImplementation,
};
use crate::token::{GeneratedTree, SpanHandle, SpanTable};
use threadpak::evidence::CauseDisposition;
use threadpak::refusal::FamilyShape;
use threadpak::types::Bounded;

use super::RefusalDeriveCapture;

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

impl RefusalDeriveSurface {
    /// Assemble one captured surface. Crate-internal: the only road to one is
    /// the capture itself.
    pub(crate) const fn assembled(
        family_name: String,
        family_id: String,
        binding: CrateBinding,
        shape: FamilyShape,
        causes: Bounded<CapturedCause, DeriveCauseLimit>,
        identity: ProjectionIdentity<CapturedDeclarationSubject>,
    ) -> Self {
        Self {
            family_name,
            family_id,
            binding,
            shape,
            causes,
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

    /// This captured declaration's own identity — derived from the token
    /// material, so the same declaration captures to the same identity whoever
    /// produced the tokens.
    #[must_use]
    pub const fn identity(&self) -> ProjectionIdentity<CapturedDeclarationSubject> {
        self.identity
    }

    /// Fix the complete declared output set. This is the one road to a
    /// [`RefusalDerivationDraft`].
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

impl RefusalDeriveRefusal {
    /// The established refusal at one token of the declared input.
    #[must_use]
    pub const fn established(cause: RefusalDeriveCapture, token: SpanHandle) -> Self {
        Self { cause, token }
    }

    /// The established cause.
    #[must_use]
    pub const fn cause(self) -> RefusalDeriveCapture {
        self.cause
    }

    /// The token the observation sits at. The producer resolves it to the exact
    /// compiler span; the services never do.
    #[must_use]
    pub const fn token(self) -> SpanHandle {
        self.token
    }

    /// The compiler-facing rendering: one line naming the cause and where it
    /// was established, in whatever coordinate role the producer speaks.
    ///
    /// A projection of the typed value, produced here so that the expansion
    /// shell composes no sentence of its own.
    ///
    /// Where the supplied table does not reach the handle, the line says THAT
    /// rather than a position. The cause is established either way — it is a
    /// fact about the declaration, not about the table — so the sentence still
    /// names it, and the reader is told the locating half is missing instead of
    /// being handed a number that means nothing.
    #[must_use]
    pub fn compiler_message(self, spans: &SpanTable) -> String {
        let described = self.cause.described();
        match spans.coordinate_of(self.token) {
            Ok(coordinate) => {
                let position = coordinate.position;
                format!(
                    "threadpak refusal-family derive: {described} (at token position {position})"
                )
            }
            Err(refusal) => format!(
                "threadpak refusal-family derive: {described} ({})",
                refusal.described()
            ),
        }
    }

    /// Project this refusal into the services' structured diagnostic.
    ///
    /// The machine anchoring is the CALLER's: where a caller holds the machine's
    /// identities it supplies them, and where none exists at this seam the
    /// diagnostic says so. This module mints none of them, because none of them
    /// is its to mint — the services classify what they OBSERVED
    /// ([`RefusalDeriveCapture::observed`]) and never mint the machine's cause
    /// commitment.
    #[must_use]
    pub fn diagnosed(self, spans: &SpanTable, machine: MachineAnchoring) -> MacrocDiagnostic {
        let repairs = Bounded::from_array([RepairAction {
            declared_by: OwnerFactRef::named("refusal", "family-shapes-are-three-and-closed"),
            description: self.cause.description(),
        }]);
        MacrocDiagnostic {
            machine,
            summary: self.cause.description(),
            phase: MacrocPhase::Capture,
            site: DiagnosticSite {
                token: self.token,
                coordinate: SiteCoordinate::answered(spans.coordinate_of(self.token)),
            },
            expected: expected_contract(),
            observed: self.cause.observed(),
            // The plane classifies what it observed and never elects the
            // machine's cause posture: narrowing is the machine's progress to
            // report, not the compiler plane's to assert.
            cause: CauseDisposition::UnresolvedCause,
            // The capture road establishes one cause and enumerates nothing, so
            // there is no per-issue set to stop short of: zero identities are
            // carried and zero are omitted.
            related: Bounded::empty(),
            related_completion: RelatedSetCompletion::Complete,
            repairs,
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
    /// This is the posture an expansion runs under: the home's fact identities
    /// have not been published to the compiler plane, so the citation names them
    /// and mints nothing.
    #[must_use]
    pub const fn declared() -> Self {
        Self {
            body_shapes: OwnerFactRef::named("refusal", "family-shapes-are-three-and-closed"),
            canonical_order_is_shape_ruled: OwnerFactRef::named(
                "refusal",
                "canonical-order-stands-for-single-cause-alone",
            ),
            cause_key_grammar: OwnerFactRef::named("refusal", "cause-identity-is-family-and-key"),
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
    /// Bind one closed expansion. Crate-internal: the only road to one is
    /// [`compile_refusal`](crate::derive_refusal::compile_refusal).
    ///
    /// # The closed-expansion transcript
    ///
    /// The identity is derived under [`ProjectionRole::ClosedExpansion`],
    /// anchored on the CLOSURE's identity — because a receipt exists only where
    /// a closure does — over a content transcript committing to the captured
    /// declaration's identity, the plan's identity, and the emitted token tree's
    /// canonical bytes at full length. Those are exactly the three things a
    /// reader of one receipt asks about: what was read, what was decided, and
    /// what was handed to the compiler.
    ///
    /// The emitted tree is not a parameter. It is read off the closure, which
    /// owns it: a receipt that was handed a tree separately could be handed one
    /// the closure never proved.
    pub(crate) fn bound(
        surface: RefusalDeriveSurface,
        plan: ProjectionPlan<DeriveImplProjection>,
        closure: ProjectionClosure<RenderedImplementation>,
        explanation: ProjectionExplanationView<DeriveImplProjection>,
        cause_order: ProjectionDisposition,
    ) -> Self {
        let mut content = Vec::new();
        encode_bytes(surface.identity().as_bytes(), &mut content);
        encode_bytes(plan.identity().as_bytes(), &mut content);
        encode_bytes(&closure.emitted().canonical_bytes(), &mut content);
        let closure_identity = closure.identity();
        let (identity, provenance) =
            ClosedExpansionId::derived_with_provenance(ProjectionTranscript::under_projection(
                ProjectionRole::ClosedExpansion,
                &closure_identity,
                &content,
                0,
            ));
        Self {
            identity,
            provenance,
            surface,
            plan,
            closure,
            explanation,
            cause_order,
        }
    }

    /// This closed expansion's own identity: the name of the whole receipt.
    #[must_use]
    pub const fn identity(&self) -> ClosedExpansionId {
        self.identity
    }

    /// How that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &ProjectionProvenance {
        &self.provenance
    }

    /// The captured typed declaration this expansion was compiled from.
    #[must_use]
    pub const fn surface(&self) -> &RefusalDeriveSurface {
        &self.surface
    }

    /// The complete plan: context, content, membership, invalidation set,
    /// decision trace, origin trail, and nonclaims.
    #[must_use]
    pub const fn plan(&self) -> &ProjectionPlan<DeriveImplProjection> {
        &self.plan
    }

    /// The proof that what was rendered is what was planned.
    #[must_use]
    pub const fn closure(&self) -> &ProjectionClosure<RenderedImplementation> {
        &self.closure
    }

    /// The complete explanation over this kind's applicable questions.
    #[must_use]
    pub const fn explanation(&self) -> &ProjectionExplanationView<DeriveImplProjection> {
        &self.explanation
    }

    /// What happened to the typed cause-order projection.
    #[must_use]
    pub const fn cause_order(&self) -> &ProjectionDisposition {
        &self.cause_order
    }

    /// The token tree an expansion emits. The shell's only act is to hand this
    /// to the compiler.
    ///
    /// It is the CLOSURE's tree, borrowed rather than copied: the receipt keeps
    /// no second tree, so what is emitted is what was proved and there is no
    /// pair of values to drift apart.
    #[must_use]
    pub const fn emitted(&self) -> &GeneratedTree {
        self.closure.emitted()
    }

    /// What one rendered unit looks like as Rust source text — an inspection
    /// projection of the SAME tree that is emitted, never a second rendering.
    #[must_use]
    pub fn inspected(&self) -> String {
        self.emitted().inspected()
    }

    /// The rendering this expansion closed over.
    #[must_use]
    pub const fn rendered(&self) -> &RenderedProjection<RenderedImplementation> {
        self.closure.rendered()
    }
}

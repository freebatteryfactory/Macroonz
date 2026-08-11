//! The refusal-family derive's public types: what was declared, what was
//! refused, what was planned, and what one closed expansion binds.
//!
//! The home is a directory rather than one file because the responsibilities
//! genuinely separated: reading a declaration, planning over it, rendering it,
//! explaining it, and binding the whole receipt are five different jobs against
//! five different vocabularies. The types they all speak live here, once.

use crate::closure::{ProjectionClosure, RenderedProjection};
use crate::diagnostics::{
    DiagnosticSite, MachineAnchoring, MacrocDiagnostic, MacrocPhase, ObservedClassification,
    ReleasePosture, RepairAction, ReproductionRoute,
};
use crate::explanation_protocol::ProjectionExplanationView;
use crate::origin_graph::Nonclaim;
use crate::plane::{
    CapturedDeclarationSubject, ContractSubject, DeriveCauseLimit, HumanProjection, HumanTextLimit,
    NonclaimLimit, OwnerFactRef, ProjectionIdentity, ProjectionPreimage, ProjectionRole,
    ServiceEntrySubject, human_projection,
};
use crate::planning::{
    DeriveImplProjection, ProjectionDisposition, ProjectionPlan, RenderedImplementation,
};
use crate::token::{GeneratedTree, SpanHandle, SpanTable};
use threadpak::evidence::CauseDisposition;
use threadpak::refusal::{
    CauseId, CauseKey, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder, FamilyShape,
    LocalCauseKey, RefusalFamily, RefusalFamilyId,
};
use threadpak::types::Bounded;

// ---------------------------------------------------------------------------
// The authored grammar's vocabulary.
// ---------------------------------------------------------------------------

/// The authored shape word for a single-cause family.
pub const SHAPE_WORD_SINGLE_CAUSE: &str = "single_cause";

/// The authored shape word for an issue-collection family.
pub const SHAPE_WORD_ISSUE_COLLECTION: &str = "issue_collection";

/// The authored shape word for an inseparable-pair family.
pub const SHAPE_WORD_INSEPARABLE_PAIR: &str = "inseparable_pair";

/// The crate binding a consumer reaches the machine through, by default.
pub const DEFAULT_CRATE_BINDING: &str = "threadpak";

/// How the consumer names the machine on its own dependency list.
///
/// # Why a rendering may not hardcode `::threadpak`
///
/// A consumer is allowed to rename its dependencies. `tp = { package =
/// "threadpak" }` is an ordinary Cargo edge, and in that crate the machine is
/// not called `threadpak` at all — so a rendering that spelled `::threadpak`
/// would name a crate the consumer does not have, and the expansion would fail
/// to compile for a reason that has nothing to do with the declaration.
///
/// So the binding is part of what is CAPTURED. It travels into the plan, into
/// the explanation, into the rendering, and into the invalidation set, because a
/// consumer that renames its dependency has changed what the rendering must say.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateBinding {
    spelling: String,
}

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

/// One cause as the capture read it: the Rust variant that spells it, and the
/// LOCAL key the author declared for it.
///
/// The local key is not the cause identity. The identity is the family's
/// identity joined to this key under band 00's canonical key grammar, and the
/// derive composes it rather than asking the author to write it out — which is
/// what keeps a family's causes from drifting apart one hand-typed prefix at a
/// time.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedCause {
    spelling: String,
    local_key: String,
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

/// One refusal-family declaration, captured from a typed token tree.
///
/// The causes are non-empty exactly when the shape is
/// [`FamilyShape::SingleCause`]; the other two shapes declare no canonical
/// order, so there is nothing here to carry for them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalDeriveSurface {
    family_name: String,
    family_id: String,
    binding: CrateBinding,
    shape: FamilyShape,
    causes: Bounded<CapturedCause, DeriveCauseLimit>,
    identity: ProjectionIdentity<CapturedDeclarationSubject>,
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

// ---------------------------------------------------------------------------
// The capture refusal family.
// ---------------------------------------------------------------------------

/// Declares the capture family's causes once, and derives from that single
/// declaration the typed roster, the selection order, the stable identities, the
/// observed classification, the text, and the bounded human projection.
///
/// One literal per cause. A second copy of any of these would be a second thing
/// to keep true, and the human projection in particular is proven to FIT its
/// limit family at compile time — so the explanation road has no refusal to
/// swallow and no empty fallback to fall into.
macro_rules! capture_causes {
    ($(
        $(#[$note:meta])*
        $variant:ident = $key:literal, $observed:expr, $text:literal
    );+ $(;)?) => {
        /// The single-cause family for capturing a refusal-family declaration.
        ///
        /// Single cause because the checks are dependent: there is no shape word
        /// to admit until an attribute was found, no coverage to check until
        /// both the order clause and the body were read, and no distinctness to
        /// check until the keys were parsed. Claiming a result from a check that
        /// never ran is unrepresentable here, which is exactly what the shape is
        /// for.
        ///
        /// The canonical order below is the SELECTOR's order, not the execution
        /// schedule.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum RefusalDeriveCapture {
            $( $(#[$note])* $variant ),+
        }

        impl RefusalFamily for RefusalDeriveCapture {
            const SHAPE: FamilyShape = FamilyShape::SingleCause;
            const SELECTION_ORDER: &'static [&'static str] = &[
                $( stringify!($variant) ),+
            ];
        }

        /// Hand-declared, and deliberately so: the services never derive their
        /// own contracts. A generator that produced its own declared facts would
        /// be its own oracle, and the parity this module is qualified by would
        /// compare it to itself.
        impl CauseOrderDeclaration for RefusalDeriveCapture {
            const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
                $(
                    DeclaredCause::declared(
                        CauseId::declared(
                            concat!("macroc.refusal-derive-capture.", $key),
                        ),
                        stringify!($variant),
                    )
                ),+
            ]);
        }

        impl RefusalDeriveCapture {
            /// This family's own stable identity.
            pub const FAMILY: RefusalFamilyId =
                RefusalFamilyId::declared("macroc.refusal-derive-capture");

            /// The derived key pair for one cause: the family, and the cause's
            /// local key inside it.
            #[must_use]
            pub const fn key(self) -> CauseKey {
                CauseKey::declared(
                    Self::FAMILY,
                    match self {
                        $( Self::$variant => LocalCauseKey::declared($key) ),+
                    },
                )
            }

            /// How what was found differs from the contract that was expected.
            #[must_use]
            pub const fn observed(self) -> ObservedClassification {
                match self {
                    $( Self::$variant => $observed ),+
                }
            }

            /// The cause rendered for a person. A projection of the typed value:
            /// nothing reads it back, and no decision consults it.
            #[must_use]
            pub const fn described(self) -> &'static str {
                match self {
                    $( Self::$variant => $text ),+
                }
            }

            /// The same rendering as a bounded projection, proven to fit its
            /// limit family at compile time.
            #[must_use]
            pub fn description(self) -> HumanProjection<HumanTextLimit> {
                match self {
                    $( Self::$variant => human_projection!(HumanTextLimit, $text) ),+
                }
            }
        }
    };
}

capture_causes! {
    /// The declared input carries no item this grammar recognizes at all.
    NotAnEnum = "not-an-enum", ObservedClassification::ContractDisagreement,
        "the declared input carries no item declaration this grammar recognizes";

    /// A real Rust item arrived that is not an enum — a struct, a union, a
    /// trait, or a function. It is a real declaration and it is the wrong FORM,
    /// which is a different answer than "this is not an enum at all".
    UnsupportedDeclarationForm = "unsupported-declaration-form",
        ObservedClassification::ContractDisagreement,
        "a refusal family is declared as an enum, and this declaration is a different item form";

    /// The `enum` keyword is not followed by a name.
    NotNamed = "not-named", ObservedClassification::SeatAbsent,
        "the `enum` keyword is not followed by a name";

    /// A real enum arrived carrying a form this compiler profile does not read:
    /// generic parameters, or a `where` clause.
    UnavailableUnderCompilerProfile = "unavailable-under-compiler-profile",
        ObservedClassification::ProfileDisagreement,
        "this declaration carries generics or a `where` clause, which the derive's declared \
         compiler profile does not read";

    /// The enum declares no body at all.
    NotBodied = "not-bodied", ObservedClassification::SeatAbsent,
        "the enum declares no body";

    /// The enum body declares no variant.
    NotInhabited = "not-inhabited", ObservedClassification::SeatAbsent,
        "the enum body declares no variant";

    /// A real variant arrived carrying a payload. The grammar admits bare
    /// variants only, so what is captured renders back without a construction
    /// question ever arising.
    UnsupportedVariantPayload = "unsupported-variant-payload",
        ObservedClassification::ContractDisagreement,
        "a variant carries a payload, and this grammar admits bare variants only";

    /// No `#[refusal(family = ...)]` was declared.
    NotFamilyDeclared = "not-family-declared", ObservedClassification::SeatAbsent,
        "no `#[refusal(family = \"<domain>.<family>\")]` was declared";

    /// The declared family identity does not follow the canonical grammar.
    NotFamilyGrammatical = "not-family-grammatical",
        ObservedClassification::ContractDisagreement,
        "the declared family identity is not two lowercase kebab-case segments joined by a dot";

    /// No `#[refusal(shape = ...)]` was declared.
    NotShapeDeclared = "not-shape-declared", ObservedClassification::SeatAbsent,
        "no `#[refusal(shape = ...)]` was declared";

    /// The declared shape word is none of the three the machine's roster admits.
    NotAnAdmittedShape = "not-an-admitted-shape", ObservedClassification::ContractDisagreement,
        "the declared shape word is none of `single_cause`, `issue_collection`, \
         `inseparable_pair`";

    /// The shape is `single_cause` and no `order(...)` clause was declared.
    NotOrderDeclared = "not-order-declared", ObservedClassification::SeatAbsent,
        "a `single_cause` family declares no `order(...)` clause";

    /// The shape declares no canonical cause order and an `order(...)` clause
    /// was declared anyway.
    NotOrderAdmitted = "not-order-admitted", ObservedClassification::ContractDisagreement,
        "this shape declares no canonical cause order, and an `order(...)` clause was declared \
         anyway";

    /// The order clause and the enum body do not name the same causes.
    NotCovered = "not-covered", ObservedClassification::ContractDisagreement,
        "the `order(...)` clause and the enum body name different causes";

    /// Two declared causes carry the same local key.
    NotDistinct = "not-distinct", ObservedClassification::IdentityDisagreement,
        "two declared causes carry the same local key";

    /// A declared local key does not follow the canonical grammar.
    NotKeyed = "not-keyed", ObservedClassification::ContractDisagreement,
        "a declared local key is not one lowercase kebab-case segment";

    /// The declared input exceeds a declared magnitude.
    Unbounded = "unbounded", ObservedClassification::BoundExceeded,
        "the declared input exceeds a declared magnitude";
}

/// One capture refusal: the established cause, and the token it sits at.
///
/// Both seats are required. A refusal that could omit its token would send the
/// caller looking, and a refusal that could omit its cause would be a complaint
/// rather than an answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalDeriveRefusal {
    cause: RefusalDeriveCapture,
    token: SpanHandle,
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
    #[must_use]
    pub fn compiler_message(self, spans: &SpanTable) -> String {
        let described = self.cause.described();
        let coordinate = spans.coordinate_of(self.token);
        let position = coordinate.position;
        format!("threadpak refusal-family derive: {described} (at token position {position})")
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
                coordinate: spans.coordinate_of(self.token),
            },
            expected: expected_contract(),
            observed: self.cause.observed(),
            // The plane classifies what it observed and never elects the
            // machine's cause posture: narrowing is the machine's progress to
            // report, not the compiler plane's to assert.
            cause: CauseDisposition::UnresolvedCause,
            related: Bounded::empty(),
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
    ProjectionIdentity::derived(ProjectionPreimage::rooted(
        ProjectionRole::ClosedExpansion,
        b"macroc.derive_refusal.declaration-grammar",
        0,
    ))
}

/// The callable entry point that reproduces one observation without a
/// proc-macro anywhere in the path.
#[must_use]
pub fn callable_entry() -> ProjectionIdentity<ServiceEntrySubject> {
    ProjectionIdentity::derived(ProjectionPreimage::rooted(
        ProjectionRole::ClosedExpansion,
        b"macroc.derive_refusal.compile_refusal",
        1,
    ))
}

// ---------------------------------------------------------------------------
// The declared output set.
// ---------------------------------------------------------------------------

/// The complete declared output set of one derivation.
///
/// A closed sum rather than a bounded collection, because the set is decided by
/// the shape and there are exactly two answers. Neither answer is empty: a
/// derivation that would generate nothing is a disposition, not a derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivedMembership {
    /// The family implementation alone — the shape declares no cause order.
    FamilyOnly,
    /// The family implementation and the cause-order implementation.
    FamilyAndCauseOrder,
}

impl DerivedMembership {
    /// The rendered roles this membership declares, in roster order.
    #[must_use]
    pub const fn roles(self) -> &'static [RenderedImplementation] {
        match self {
            Self::FamilyOnly => &[RenderedImplementation::RenderedFamilyImpl],
            Self::FamilyAndCauseOrder => &[
                RenderedImplementation::RenderedFamilyImpl,
                RenderedImplementation::RenderedCauseOrderImpl,
            ],
        }
    }

    /// The number of declared roles; structurally at least one.
    #[must_use]
    pub const fn len(self) -> usize {
        self.roles().len()
    }

    /// Always `false`: an empty declared output set is unrepresentable here.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        false
    }
}

/// Whether one derivation carries the typed cause order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CauseOrderStanding {
    /// The shape declares a canonical cause order, and the derivation carries
    /// it.
    Declared,
    /// The shape declares none — band 00 rules the canonical order for
    /// single-cause families alone.
    NotApplicableToShape,
}

/// The membership-only view of one derivation: what was captured, and the
/// complete output set the shape fixes.
///
/// # It is a DRAFT, and it renders nothing
///
/// This type used to be the front door: it carried a `rendered()` method, and a
/// caller could take a rendering off it without a plan, without identities,
/// without an origin graph, without an explanation, and without a closure. That
/// road existed beside the receipt-rich one and was shorter, so it was the road
/// anything in a hurry took — which made every receipt on the other road
/// optional in practice.
///
/// It is gone. A draft states what the shape fixed and nothing else; the road to
/// emitted tokens runs through
/// [`compile_refusal`](crate::derive_refusal::compile_refusal), which builds the
/// plan, the origin graph, the trace, the rendering, the closure, and the
/// explanation, in that order, and refuses before any of them is skipped.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalDerivationDraft {
    surface: RefusalDeriveSurface,
    membership: DerivedMembership,
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

/// The owner facts one refusal-family derivation cites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalOwnerFacts {
    /// The refusal home's fact that a family's body is one of exactly three
    /// shapes.
    pub body_shapes: OwnerFactRef,
    /// The refusal home's fact that the canonical cause order stands for
    /// single-cause families and for no other shape.
    pub canonical_order_is_shape_ruled: OwnerFactRef,
    /// The refusal home's fact that a cause identity is its family's identity
    /// joined to its local key.
    pub cause_key_grammar: OwnerFactRef,
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

/// What one live compilation needs supplied to it, and nothing more.
///
/// Every seat is something the CALLER genuinely has. There is no seat here for
/// an identity the machine has not published, because the honest answer to
/// "which closed graph?" inside an expansion is that there is none yet — and the
/// plan says so in its own anchoring rather than being handed a fiction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalCompileContext {
    /// How the producer resolves span handles.
    pub spans: SpanTable,
    /// Whether the machine's own identities stand behind a diagnostic raised
    /// here.
    pub machine: MachineAnchoring,
    /// The owner facts the derivation cites.
    pub owner_facts: RefusalOwnerFacts,
    /// What this compilation explicitly does not claim.
    pub nonclaims: Bounded<Nonclaim, NonclaimLimit>,
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

// ---------------------------------------------------------------------------
// The receipt.
// ---------------------------------------------------------------------------

/// One closed expansion: everything one live compilation produced, bound
/// together, with the emitted token tree reachable only from here.
///
/// # The one road to emitted tokens
///
/// A caller cannot hold this without the plan, the origin graph, the trace, the
/// rendering, the closure, and the explanation all having been produced and
/// having agreed. There is no constructor that skips one, and there is no other
/// value in the services that carries a token tree an expansion may emit.
///
/// # Inspection and emission read one value
///
/// [`ClosedExpansion::plan`] and [`ClosedExpansion::closure`] are the SAME
/// values [`ClosedExpansion::emitted`] is projected from. There is no parallel
/// plan built for inspection and no synthetic sibling built for emission, so
/// "what does it say it did" and "what did it do" cannot drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedExpansion {
    surface: RefusalDeriveSurface,
    plan: ProjectionPlan<DeriveImplProjection>,
    closure: ProjectionClosure<RenderedImplementation>,
    explanation: ProjectionExplanationView<DeriveImplProjection>,
    cause_order: ProjectionDisposition,
    emitted: GeneratedTree,
}

impl ClosedExpansion {
    /// Bind one closed expansion. Crate-internal: the only road to one is
    /// [`compile_refusal`](crate::derive_refusal::compile_refusal).
    pub(crate) const fn bound(
        surface: RefusalDeriveSurface,
        plan: ProjectionPlan<DeriveImplProjection>,
        closure: ProjectionClosure<RenderedImplementation>,
        explanation: ProjectionExplanationView<DeriveImplProjection>,
        cause_order: ProjectionDisposition,
        emitted: GeneratedTree,
    ) -> Self {
        Self {
            surface,
            plan,
            closure,
            explanation,
            cause_order,
            emitted,
        }
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
    #[must_use]
    pub const fn emitted(&self) -> &GeneratedTree {
        &self.emitted
    }

    /// What one rendered unit looks like as Rust source text — an inspection
    /// projection of the SAME tree that is emitted, never a second rendering.
    #[must_use]
    pub fn inspected(&self) -> String {
        self.emitted.inspected()
    }

    /// The rendering this expansion closed over.
    #[must_use]
    pub const fn rendered(&self) -> &RenderedProjection<RenderedImplementation> {
        self.closure.rendered()
    }
}

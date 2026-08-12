//! The refusal-family derive's public types: what was declared, what was
//! refused, what was planned, and what one closed expansion binds.
//!
//! The home is a directory rather than one file because the responsibilities
//! genuinely separated: reading a declaration, planning over it, rendering it,
//! explaining it, and binding the whole receipt are five different jobs against
//! five different vocabularies. The types they all speak live here, once.
//!
//! Declarations only, with one deliberate exception. `capture_causes!` below
//! declares the capture family AND writes its two contracts and its four
//! per-cause tables in the same expansion, because the whole point of that
//! declaration is that a cause is stated once and everything about it follows.
//! Splitting the contracts out to `type_contract.rs` would make a cause two
//! declarations in two files, which is the defect the macro exists to prevent.
//! Everything that reaches a private field is in `type_guard.rs`, this file's
//! own child.

use crate::closure::ProjectionClosure;
use crate::diagnostics::{MachineAnchoring, ObservedClassification};
use crate::explanation_protocol::ProjectionExplanationView;
use crate::origin_graph::Nonclaim;
use crate::plane::{
    CapturedDeclarationSubject, ClosedExpansionId, DeriveCauseLimit, HumanProjection,
    HumanTextLimit, NonclaimLimit, OwnerFactRef, ProjectionIdentity, ProjectionProvenance,
    human_projection,
};
use crate::planning::{
    DeriveImplProjection, ProjectionDisposition, ProjectionPlan, RenderedImplementation,
};
use crate::token::{SpanHandle, SpanTable};
use threadpak::refusal::{
    CauseId, CauseKey, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder, FamilyShape,
    LocalCauseKey, RefusalFamily, RefusalFamilyId,
};
use threadpak::types::Bounded;

#[path = "type_guard.rs"]
mod guard;

pub use guard::{callable_entry, expected_contract};

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
        #[must_use = "a capture refusal carries the established cause the declaration \
                      was not read"]
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
#[must_use = "a capture refusal carries the established cause and the offending token"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalDeriveRefusal {
    cause: RefusalDeriveCapture,
    token: SpanHandle,
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
///
/// The receipt holds no tree of its own. The emitted tree belongs to the
/// CLOSURE, which built it as part of proving and committed to its digest inside
/// its own identity; this value borrows it. A receipt that had been handed a
/// tree alongside a closure could have been handed one the closure never joined.
#[must_use = "a closed expansion is the whole receipt one live compilation produced"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedExpansion {
    identity: ClosedExpansionId,
    provenance: ProjectionProvenance,
    surface: RefusalDeriveSurface,
    plan: ProjectionPlan<DeriveImplProjection>,
    closure: ProjectionClosure<RenderedImplementation>,
    explanation: ProjectionExplanationView<DeriveImplProjection>,
    cause_order: ProjectionDisposition,
}

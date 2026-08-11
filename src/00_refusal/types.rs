//! How anything in the machine says no.
//!
//! Four observables never collapse: **success ≠ refusal ≠ uncertainty ≠ failure**.
//! A refusal is a typed, lawful "no" from a check that ran; it is never silent
//! normalization, never a panic, never an untyped default, and never a claim about
//! checks that did not run.
//!
//! # The three body shapes and the selector
//!
//! Every refusal family is shaped one of exactly three ways, selected by how its
//! checks relate — never by taste:
//!
//! 1. **Single cause** — dependent checks, where each check is meaningful only after
//!    the previous passed: a closed single-cause enum with a declared canonical
//!    selection order. One cause is all that can truthfully exist; the shape makes
//!    claiming unexecuted checks unrepresentable.
//! 2. **Issue collection** — independent, co-establishable facts: a bounded,
//!    non-empty collection ([`crate::types::NonEmptyBounded`]) over a closed issue
//!    set. The shape makes both a zero-issue refusal and a dropped co-true defect
//!    unrepresentable.
//! 3. **Inseparable pair** — exactly two questions neither of which means anything
//!    alone: a composite record with exactly two seats. If the questions can be
//!    separated, they must be.
//!
//! # Variant spelling conventions
//!
//! Family variants spell themselves one of four ways: a negated adjective
//! (`NotCanonical`), a `Not`-prefix on the failed requirement, the prohibited act
//! itself, or one of exactly two bounds spellings. New spellings are a law change.
//!
//! # What this home does not own
//!
//! Failure (infrastructure breakage) and uncertainty (the knowledge axes) are other
//! homes' words. Human-readable meanings for [`ReasonId`]s are registered by the
//! evidence home, downstream. Location types are carried by owner families, never by
//! the universal envelope.

/// The stable identity of one registered refusal reason. A registered reason is a
/// semantic commitment: new meaning mints a new id, never recycled. Opaque; equality
/// and hashing only; no ordering beyond the declared raw-byte storage order; minted
/// by derivation from its family (macro-projected once the macros crate lands),
/// never by a public constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReasonId([u8; 32]);

impl ReasonId {
    /// The declared raw-byte storage order of this identity.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// The treatment class carried on every refusal: what the caller may lawfully do
/// next. `DoNotRetry` is law — a refusal may explicitly forbid retry, and this
/// variant is never dropped from the roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandlingClass {
    /// The same request may lawfully be retried.
    Retryable,
    /// Retry is explicitly forbidden; repeating the request is unlawful.
    DoNotRetry,
    /// The request is lawful only after its configuration or inputs change.
    Reconfigure,
    /// The refusal requires an authority outside the caller to act.
    Escalate,
}

/// Which declared bound stopped an enumeration early — exactly two: an
/// enumeration stops at a declared issue bound or a declared work bound, and
/// says which.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StopBound {
    /// The declared issue bound was reached.
    DeclaredIssueBound,
    /// The declared work bound was reached.
    DeclaredWorkBound,
}

/// Whether an issue enumeration ran to completion or stopped at a declared
/// bound. Carried as an **instance value inside collection-shaped refusals
/// only** — single-cause families carry no posture at all, and no family mints a
/// local copy of this type. Stopping reports incomplete enumeration rather than
/// pretending no further defects exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompletionPosture {
    /// Enumeration covered every declared site.
    Complete,
    /// Enumeration stopped at a declared bound, naming which.
    EarlyStopped {
        /// The declared bound that stopped enumeration.
        stopped_at: StopBound,
    },
}

/// Which of the three lawful body shapes a family takes. The selector is
/// structural: dependent checks take `SingleCause`, independent co-establishable
/// facts take `IssueCollection`, and exactly two inseparable questions take
/// `InseparablePair`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyShape {
    /// A closed single-cause enum with a declared canonical selection order.
    SingleCause,
    /// A bounded, non-empty collection over a closed issue set.
    IssueCollection,
    /// A composite record with exactly two seats.
    InseparablePair,
}

/// The contract every refusal family implements. A family is a concrete Rust type
/// (its body is one of the three shapes); this trait carries the family's declared
/// facts so tooling can verify them — the declarations here are machine-readable
/// law, joined by name against the family's variants and its home README.
pub trait RefusalFamily {
    /// The family's body shape.
    const SHAPE: FamilyShape;

    /// The canonical selection order over the family's causes — the declared
    /// selector, never an execution schedule. Non-empty exactly when `SHAPE` is
    /// [`FamilyShape::SingleCause`]; empty otherwise. Hand-declared until the
    /// macros crate derives it from the family body itself.
    ///
    /// Reason granularity differs by shape and is law: a single-cause family
    /// maps every inhabited cause value to its own stable `ReasonId`; a
    /// collection family maps the envelope reason at the **family** level —
    /// issue identities stay inside the family value and no owner elects a
    /// "primary issue". And no implementation may ever match on a cause
    /// *spelling* rather than a family *type* — a shared word is shared
    /// vocabulary, never shared ownership.
    const SELECTION_ORDER: &'static [&'static str];
}

impl RefusalFamily for crate::types::BoundedConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["OverLimit"];
}

impl RefusalFamily for crate::types::NonEmptyBoundedConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["OverLimit"];
}

/// The universal refusal envelope: the registered reason, the treatment class, and
/// the family body. Deliberately location-free — an owner family that needs a
/// location carries its own location type, so this envelope imports nothing from
/// any later home. (Authored structural law, not an old-book quote: band 00 must
/// import nothing.)
pub struct Refusal<F: RefusalFamily> {
    reason: ReasonId,
    handling: HandlingClass,
    family: F,
}

impl<F: RefusalFamily> Refusal<F> {
    /// The registered reason identity.
    #[must_use]
    pub fn reason(&self) -> ReasonId {
        self.reason
    }

    /// The treatment class: what the caller may lawfully do next.
    #[must_use]
    pub fn handling(&self) -> HandlingClass {
        self.handling
    }

    /// The family body carrying the established cause, issues, or pair.
    #[must_use]
    pub fn family(&self) -> &F {
        &self.family
    }
}

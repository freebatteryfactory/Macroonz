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

/// The stable identity of one declared cause inside one refusal family.
///
/// Separate from the Rust variant that spells it, from any display text, from
/// prose, and from position — and that separation is the whole point. Renaming a
/// Rust variant must not change what a cause *is* or where it sits in the
/// declared order; a change of a cause's *meaning* must mint a different
/// identity rather than reuse this one. A cause identity is a semantic
/// commitment on exactly the terms [`ReasonId`] is: never recycled.
///
/// Opaque, equality and hashing only, and no ordering: a cause identity carries
/// no rank of its own. Rank is [`CauseOrdinal`]'s question, and only inside one
/// family's declared order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauseId(&'static str);

impl CauseId {
    /// Declare one stable cause identity.
    ///
    /// The text is an identity and only an identity: nothing in the machine
    /// reads it for meaning, renders it to a person, or derives a message from
    /// it. It exists so that the identity survives being written down — in a
    /// README row, in a registered reason, in a generated implementation.
    #[must_use]
    pub const fn declared(identity: &'static str) -> Self {
        Self(identity)
    }

    /// The declared identity, for equality and for machine-readable joins.
    #[must_use]
    pub const fn as_declared(self) -> &'static str {
        self.0
    }
}

/// The stable identity of one refusal family.
///
/// Spelled `<domain>.<family>` — two lowercase kebab-case segments, the domain
/// naming who owns the family and the family naming which one it is. It is an
/// identity on exactly [`CauseId`]'s terms: new meaning mints a new one, and it
/// is never recycled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalFamilyId(&'static str);

impl RefusalFamilyId {
    /// Declare one stable family identity.
    #[must_use]
    pub const fn declared(identity: &'static str) -> Self {
        Self(identity)
    }

    /// The declared identity, for equality and for machine-readable joins.
    #[must_use]
    pub const fn as_declared(self) -> &'static str {
        self.0
    }
}

/// One cause's key WITHIN its family.
///
/// A local key is unique inside its family and says nothing outside it. Two
/// families may both declare `not-canonical`; that is a shared word, never
/// shared ownership, and the [`CauseKey`] pair is what keeps the two apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalCauseKey(&'static str);

impl LocalCauseKey {
    /// Declare one local cause key: one lowercase kebab-case segment.
    #[must_use]
    pub const fn declared(key: &'static str) -> Self {
        Self(key)
    }

    /// The declared key.
    #[must_use]
    pub const fn as_declared(self) -> &'static str {
        self.0
    }
}

/// The derived pair one cause identity is built from: which family, and which
/// cause inside it.
///
/// # The canonical key grammar
///
/// ```text
/// CauseId  ::=  <family> "." <local>
/// family   ::=  <domain> "." <family-name>
/// domain, family-name, local  ::=  lowercase kebab-case segment
/// ```
///
/// So `refusal.derive-capture` + `not-an-enum` joins to
/// `refusal.derive-capture.not-an-enum`. The join is the grammar and nothing
/// else: nothing in the machine parses a [`CauseId`] back into a pair, because
/// an identity is read for equality and never for meaning.
///
/// # What the pair makes provable, and what it does not
///
/// A generator holding the pair can prove **local uniqueness** — that no two
/// causes in one family declare the same local key — and it can prove the
/// family's cause count fits [`CauseOrdinal`]'s declared magnitude. Both are
/// facts about ONE declaration and are provable where that declaration is read.
///
/// **Family uniqueness is not provable there** and is not claimed here: whether
/// two separately declared families collide on one `<domain>.<family>` is a
/// question about the whole program, and it is answered where the whole program
/// is assembled. That join is owed to the composition root and is stated as owed
/// rather than quietly assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauseKey {
    family: RefusalFamilyId,
    local: LocalCauseKey,
}

impl CauseKey {
    /// Declare one cause key: the family first, because the key is a position
    /// inside that family.
    #[must_use]
    pub const fn declared(family: RefusalFamilyId, local: LocalCauseKey) -> Self {
        Self { family, local }
    }

    /// The family this key sits in.
    #[must_use]
    pub const fn family(self) -> RefusalFamilyId {
        self.family
    }

    /// The cause's key inside that family.
    #[must_use]
    pub const fn local(self) -> LocalCauseKey {
        self.local
    }
}

/// The typed position of one cause in its family's declared order.
///
/// There is no constructor taking a number. An ordinal is minted only by
/// [`DeclaredCauseOrder`] out of the roster's own layout, so a position can
/// never disagree with the order it claims to be a position in — the
/// disagreement is unrepresentable rather than checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauseOrdinal(u16);

impl CauseOrdinal {
    /// The position this ordinal stands for, counted from the first cause.
    #[must_use]
    pub const fn position(self) -> u16 {
        self.0
    }
}

/// One declared cause: its stable identity, and the Rust variant that spells it.
///
/// The two members answer different questions and never substitute for each
/// other. The identity answers "which cause is this?"; the spelling answers
/// "what is it called in Rust today?" — and the second may be renamed without
/// touching the first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredCause {
    id: CauseId,
    spelling: &'static str,
}

impl DeclaredCause {
    /// Declare one cause: the stable identity first, because the identity is
    /// what the declaration is *about*; the Rust spelling second, because it is
    /// the projection.
    #[must_use]
    pub const fn declared(id: CauseId, spelling: &'static str) -> Self {
        Self { id, spelling }
    }

    /// The cause's stable identity.
    #[must_use]
    pub const fn id(self) -> CauseId {
        self.id
    }

    /// The Rust variant that spells this cause — a projection of the identity,
    /// never the identity itself.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        self.spelling
    }
}

/// The typed canonical order over one family's causes.
///
/// This is the order — the selector over established conditions, stated once, in
/// terms of stable identities. [`RefusalFamily::SELECTION_ORDER`] is its
/// *textual projection* and is checked against it by
/// [`DeclaredCauseOrder::projects_to`]; the two are one fact in two forms, never
/// two facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredCauseOrder {
    causes: &'static [DeclaredCause],
}

impl DeclaredCauseOrder {
    /// Declare the canonical order, first cause first.
    #[must_use]
    pub const fn declared(causes: &'static [DeclaredCause]) -> Self {
        Self { causes }
    }

    /// The order a family with no causes to order declares.
    ///
    /// Not a default and not a hole: collection-shaped and pair-shaped families
    /// carry no canonical selection order by law, and this is how they say so
    /// rather than leaving the seat unfilled.
    #[must_use]
    pub const fn none() -> Self {
        Self { causes: &[] }
    }

    /// The number of causes ordered.
    #[must_use]
    pub const fn len(self) -> usize {
        self.causes.len()
    }

    /// Whether the family orders no cause at all.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.causes.is_empty()
    }

    /// Read the declared causes, in the declared order.
    pub fn iter(self) -> impl Iterator<Item = DeclaredCause> {
        self.causes.iter().copied()
    }

    /// The position one cause identity holds in this order, or `None` when this
    /// order declares no such cause.
    #[must_use]
    pub fn ordinal_of(self, id: CauseId) -> Option<CauseOrdinal> {
        self.causes
            .iter()
            .position(|cause| cause.id == id)
            .and_then(|index| u16::try_from(index).ok())
            .map(CauseOrdinal)
    }

    /// The cause identity at one position, or `None` when this order is shorter
    /// than that.
    #[must_use]
    pub fn identity_at(self, ordinal: CauseOrdinal) -> Option<CauseId> {
        self.causes
            .get(usize::from(ordinal.position()))
            .map(|cause| cause.id)
    }

    /// Whether one textual order is exactly this order's projection: the same
    /// spellings, in the same positions, and no more.
    ///
    /// This is the join that keeps [`RefusalFamily::SELECTION_ORDER`] honest. A
    /// textual order that permutes, drops, or adds a spelling is not a
    /// projection of this order and this method says so.
    #[must_use]
    pub fn projects_to(self, textual: &[&str]) -> bool {
        self.causes.len() == textual.len()
            && self
                .causes
                .iter()
                .zip(textual.iter())
                .all(|(cause, spelling)| cause.spelling == *spelling)
    }
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
    /// [`FamilyShape::SingleCause`]; empty otherwise.
    ///
    /// # It is a projection, not the order
    ///
    /// The order itself is typed: [`CauseOrderDeclaration::DECLARED_ORDER`]
    /// states it as [`DeclaredCause`] rows carrying stable [`CauseId`]s, and
    /// this constant is that order's **textual projection** — the same causes,
    /// the same positions, spelled as the Rust variants spell them today.
    /// [`DeclaredCauseOrder::projects_to`] is the join that proves the two
    /// agree. Renaming a Rust variant therefore moves this constant and moves
    /// neither identity nor position; changing a cause's meaning mints a new
    /// [`CauseId`] and does not hide behind an unchanged spelling.
    ///
    /// A family that has not yet declared its typed order simply does not
    /// implement [`CauseOrderDeclaration`] — an absent declaration is visible,
    /// where a defaulted one would be a quiet claim nobody made.
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

/// The typed cause order one refusal family declares.
///
/// Separate from [`RefusalFamily`] on purpose. The textual selection order is
/// the older, weaker statement and every family carries it; the typed order is
/// the stronger one and a family carries it once its causes have been given
/// stable identities. Splitting the two means a family that has not yet been
/// given identities cannot pretend it has — there is no default order to
/// inherit, only an implementation that is present or absent.
pub trait CauseOrderDeclaration: RefusalFamily {
    /// The family's canonical order over its causes, by stable identity.
    ///
    /// Non-empty exactly when [`RefusalFamily::SHAPE`] is
    /// [`FamilyShape::SingleCause`]; [`DeclaredCauseOrder::none`] otherwise.
    const DECLARED_ORDER: DeclaredCauseOrder;
}

impl RefusalFamily for crate::types::BoundedConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["OverLimit"];
}

impl CauseOrderDeclaration for crate::types::BoundedConstruction {
    const DECLARED_ORDER: DeclaredCauseOrder =
        DeclaredCauseOrder::declared(&[DeclaredCause::declared(
            CauseId::declared("root.bounded-construction.over-limit"),
            "OverLimit",
        )]);
}

impl RefusalFamily for crate::types::NonEmptyBoundedConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["OverLimit"];
}

/// The two root construction families share a cause *spelling* and share no
/// cause *identity*: a shared word is shared vocabulary, never shared ownership,
/// and the stable identities say so where the spellings cannot.
impl CauseOrderDeclaration for crate::types::NonEmptyBoundedConstruction {
    const DECLARED_ORDER: DeclaredCauseOrder =
        DeclaredCauseOrder::declared(&[DeclaredCause::declared(
            CauseId::declared("root.non-empty-bounded-construction.over-limit"),
            "OverLimit",
        )]);
}

/// The universal refusal envelope: the registered reason, the treatment class, and
/// the family body. Deliberately location-free — an owner family that needs a
/// location carries its own location type, so this envelope imports nothing from
/// any later home. (Authored structural law, not an old-book quote: band 00 must
/// import nothing.)
#[must_use = "a refusal carries the lawful reason the operation did not proceed"]
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

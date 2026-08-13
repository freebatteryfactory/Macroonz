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
//!    set, carried inside an [`AdmittedPrefix`] so the body and what it says about
//!    its own coverage are one value. The shape makes a zero-issue refusal, a
//!    dropped co-true defect, and a coverage claim about somebody else's body all
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
//! # The report package's roads live in this file's child
//!
//! [`AdmittedPrefix`] and [`ReportTruncation`] declare their seats here and are
//! reached nowhere else: every road that touches those seats — the mints, the
//! truncation the mint performs, and the two readers that hand the carry and the
//! posture back — lives in `type_guard.rs`, this file's own child. That is what
//! makes "a coverage claim and the body it is about are one value" structural
//! rather than a rule somebody follows: the marriage is performed in one file,
//! and there is no seam elsewhere in the crate that can build either half.
//!
//! # What this home does not own
//!
//! Failure (infrastructure breakage) and uncertainty (the knowledge axes) are other
//! homes' words. Human-readable meanings for [`ReasonId`]s are registered by the
//! evidence home, downstream. Location types are carried by owner families, never by
//! the universal envelope.

use crate::types::{Limit, NonEmptyBounded, NonEmptyBoundedConstruction};
use core::marker::PhantomData;
use core::num::NonZeroUsize;

#[path = "type_guard.rs"]
mod guard;

/// The stable identity of one registered refusal reason. A registered reason is a
/// semantic commitment: new meaning mints a new id, never recycled. Opaque; equality
/// and hashing only; no ordering beyond the declared raw-byte storage order; minted
/// by derivation from its family (macro-projected once the macros crate lands),
/// never by a public constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReasonId([u8; 32]);

impl ReasonId {
    /// In-crate mint for laws. Test-gated until the evidence home registers
    /// reasons — the gate comes off when a lawful minter exists, never before.
    #[cfg(test)]
    pub(crate) const fn for_laws(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

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

/// How much one report left outside its declared bound.
///
/// Opaque, with no public constructor: the only road to one is
/// [`AdmittedPrefix::examined_completely`], and that road takes no count at all.
/// It takes the material itself and performs the truncation, so the count it
/// writes down is the count it just dropped. A body that carried every issue it
/// established cannot write down a truncation posture, and a body that truncated
/// cannot write down a count it did not truncate by — the posture and the carry
/// leave that road married inside one [`AdmittedPrefix`] and have no road back
/// apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReportTruncation {
    stopped_at: StopBound,
    omitted: NonZeroUsize,
}

/// What one collection-shaped refusal body says about its own coverage. Carried
/// as an **instance value inside collection-shaped refusals only** —
/// single-cause families carry no posture at all, and no family mints a local
/// copy of this type.
///
/// # Three postures, because they are three different facts
///
/// Two of them are easy to conflate and must never be, because a reader ACTS
/// differently on each. `EarlyStopped` says the examination itself halted: there
/// may be defects nobody looked for, so a caller who repairs what is reported
/// must run the pass again to learn whether anything remains.
/// `ReportTruncated` says the opposite about the examination and the same thing
/// about the report: every declared site WAS examined, the count of what was
/// established is known exactly, and the body simply does not have room for all
/// of it. Writing the first where the second is true claims ignorance the pass
/// does not have; writing `Complete` where the second is true claims coverage
/// the body does not have.
///
/// The truncation posture carries the count because the posture alone does not
/// say enough: a reader holding a shortened body otherwise cannot tell a
/// two-issue refusal from a two-hundred-issue one, and "some were dropped" is
/// the shape of a claim nobody can act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompletionPosture {
    /// Every declared site was examined and the body carries every issue that
    /// was established.
    Complete,
    /// Enumeration stopped at a declared bound, naming which. The sites past
    /// that bound were never examined, so nothing is known about them.
    EarlyStopped {
        /// The declared bound that stopped enumeration.
        stopped_at: StopBound,
    },
    /// Every declared site was examined; the body carries what the declared
    /// bound holds and names how many established issues stand outside it.
    ReportTruncated(ReportTruncation),
}

/// One collection-shaped refusal body: the issues the declared bound held,
/// married to the posture the same construction amounts to.
///
/// # Why the two travel as one value
///
/// A posture is a claim ABOUT a body, and a claim about a body that can be
/// carried away from it is a claim that can be told about a different one. Two
/// passes truncating under the same limit family produce two carries and two
/// postures; hand a caller four loose values and nothing in the types stops the
/// carry of one from being reported under the posture of the other. Both halves
/// stay individually honest and the pair is a lie — the kind no runtime check
/// catches, because there is nothing wrong to detect at either end.
///
/// So the mint is the only road in, the seats are private, and there is no road
/// back out to a loose pair: no `into_parts`, no owned carry, no seat a caller
/// can write. What a consumer holds is one value in which the coverage claim and
/// the material it is about were produced by a single act. A body that needs the
/// issues and the posture as separate FIELDS extracts them inside its own
/// guarded constructor, off this value, and never off two values it was handed.
///
/// [`AdmittedPrefix::completion`] hands the posture out for RENDERING, which is
/// a read and not a seat: a rendered sentence is not a body, and no body has a
/// posture seat to re-house it in.
///
/// All three postures have a road here, and each road IS the act it names.
/// [`AdmittedPrefix::examined_completely`] performs the truncation and lets what
/// it dropped select between `Complete` and `ReportTruncated`;
/// [`AdmittedPrefix::stopped_early`] couples a halted examination's carry to
/// `EarlyStopped` in one construction. No pass in the machine halts today, so
/// the halted road has no caller. It exists so that the first family whose
/// examination honestly stops early meets the same coupled seat every other
/// family meets, instead of being pushed back onto a loose body beside a loose
/// posture for want of a road. That absent caller is this type's claim ceiling,
/// and it is stated rather than hidden.
#[must_use = "a report body carries the issues it established and what it says about its own coverage"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdmittedPrefix<T, L: Limit> {
    carried: NonEmptyBounded<T, L>,
    completion: CompletionPosture,
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
/// shared ownership, and [`CauseId`]'s family seat is what keeps the two apart.
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

/// The stable identity of one declared cause: which family declares it, and
/// which cause it is inside that family.
///
/// # The identity IS the pair
///
/// Two seats, both required, neither derivable from the other. A cause is not a
/// name that happens to look like a family's name with something after it; it
/// is a position inside exactly one family, and the value says which family
/// without anybody parsing anything. Two families may declare the same local
/// key — `not-canonical` is an ordinary word — and the two identities are
/// different values because their family seats differ. A shared word is shared
/// vocabulary, never shared ownership, and here the shape is what says so.
///
/// Separate from the Rust variant that spells it, from any display text, from
/// prose, and from position — and that separation is the whole point. Renaming
/// a Rust variant must not change what a cause *is* or where it sits in the
/// declared order; a change of a cause's *meaning* must mint a different
/// identity rather than reuse this one. A cause identity is a semantic
/// commitment on exactly the terms [`ReasonId`] is: never recycled.
///
/// Equality and hashing only, and no ordering: a cause identity carries no rank
/// of its own. Rank is [`CauseOrdinal`]'s question, and only inside one family's
/// declared order.
///
/// # The canonical text form is derived, never stored
///
/// ```text
/// text form  ::=  <family> "." <local>
/// family     ::=  <domain> "." <family-name>
/// domain, family-name, local  ::=  lowercase kebab-case segment
/// ```
///
/// So `refusal.derive-capture` and `not-an-enum` render as
/// `refusal.derive-capture.not-an-enum`. [`CauseId::canonical_text`] composes
/// that text out of the two seats every time it is asked; nothing stores it, and
/// nothing parses it back. A stored join would be a third value that could
/// disagree with the two it was joined from, and a parse back would make the
/// separator load-bearing over a segment that is allowed to contain one.
///
/// # What the shape makes provable, and what it does not
///
/// A reader holding a family's causes can prove **local uniqueness** — that no
/// two causes in one family declare the same local key — and it can prove the
/// family's cause count fits [`CauseOrdinal`]'s declared magnitude. Both are
/// facts about ONE declaration and are provable where that declaration is read.
///
/// **Family uniqueness is not provable there** and is not claimed here: whether
/// two separately declared families collide on one `<domain>.<family>` is a
/// question about the whole program, and it is answered where the whole program
/// is assembled. That join is owed to the composition root and is stated as owed
/// rather than quietly assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauseId {
    family: RefusalFamilyId,
    local: LocalCauseKey,
}

impl CauseId {
    /// Declare one stable cause identity: the family first, because the
    /// identity is a position inside that family.
    #[must_use]
    pub const fn declared(family: RefusalFamilyId, local: LocalCauseKey) -> Self {
        Self { family, local }
    }

    /// The family that declares this cause.
    #[must_use]
    pub const fn family(self) -> RefusalFamilyId {
        self.family
    }

    /// The cause's key inside that family.
    #[must_use]
    pub const fn local(self) -> LocalCauseKey {
        self.local
    }

    /// This identity's canonical text form, composed from the two seats.
    ///
    /// A projection for writing an identity down — in a README row, in a
    /// registered reason, in a rendered implementation. Nothing in the machine
    /// reads it back, matches on it, or decides by it: identity questions are
    /// answered by comparing values, which is why the text is composed on demand
    /// and never carried.
    #[must_use]
    pub fn canonical_text(self) -> String {
        let mut text = String::from(self.family.as_declared());
        text.push('.');
        text.push_str(self.local.as_declared());
        text
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
            CauseId::declared(
                RefusalFamilyId::declared("root.bounded-construction"),
                LocalCauseKey::declared("over-limit"),
            ),
            "OverLimit",
        )]);
}

impl RefusalFamily for NonEmptyBoundedConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["OverLimit"];
}

/// The two root construction families share a cause *spelling* AND a cause
/// *local key*, and share no cause *identity*: a shared word is shared
/// vocabulary, never shared ownership, and the family seat of a stable identity
/// says so where neither the spelling nor the local key can.
impl CauseOrderDeclaration for NonEmptyBoundedConstruction {
    const DECLARED_ORDER: DeclaredCauseOrder =
        DeclaredCauseOrder::declared(&[DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("root.non-empty-bounded-construction"),
                LocalCauseKey::declared("over-limit"),
            ),
            "OverLimit",
        )]);
}

/// How admitting one refusal family's declaration refuses.
///
/// Single cause, because the checks are dependent: there is no order to project
/// against until the shape and the textual order were found to agree, so a
/// projection verdict from a run that never got that far would be a claim about
/// a check that did not run.
#[must_use = "an admission refusal carries the established reason a family's declaration was \
              not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyAdmission {
    /// The declared shape and the declared selection order contradict each
    /// other. A canonical selection order stands for single-cause families and
    /// for no other shape, so this one cause covers both directions: a
    /// single-cause family that orders nothing, and a collection or pair family
    /// that orders something.
    NotShapeCoherent,
    /// The typed cause order is not projected by the textual selection order —
    /// the two are supposed to be one fact in two forms, and here they are two.
    NotProjected,
}

impl RefusalFamily for FamilyAdmission {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["NotShapeCoherent", "NotProjected"];
}

impl CauseOrderDeclaration for FamilyAdmission {
    const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
        DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("refusal.family-admission"),
                LocalCauseKey::declared("not-shape-coherent"),
            ),
            "NotShapeCoherent",
        ),
        DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("refusal.family-admission"),
                LocalCauseKey::declared("not-projected"),
            ),
            "NotProjected",
        ),
    ]);
}

/// Which joins one admission run performed, read as a value.
///
/// This is the **inspection projection** of a witness's coverage and nothing
/// else. The coverage itself is a type parameter on
/// [`AdmittedRefusalFamily`], and that is where enforcement lives: a consumer
/// states the strength it needs as a bound and the compiler settles it. This
/// enum is how that settled fact writes itself down — in a diagnostic, in a
/// receipt, on the published envelope — so a reader can see which joins stood
/// behind a declaration.
///
/// No road decides by it. A road branching on this value would be re-deciding
/// at runtime what the type system already decided, and one missed arm would
/// let the weaker coverage act as the stronger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyAdmissionCoverage {
    /// The shape and the textual selection order were found coherent. The
    /// family declares no typed cause order, so there was nothing to project
    /// against.
    ShapeCoherence,
    /// The shape and the textual order were found coherent, AND the typed order
    /// was found to project onto the textual one.
    ShapeCoherenceAndOrderProjection,
}

mod sealed {
    /// The seal: another coverage is admitted only when a real admission road
    /// establishes a distinct set of joins — by decision, in this crate, never
    /// by downstream impl. A coverage an outside crate could implement would be
    /// a proof strength anybody could declare for itself, and every consumer
    /// demanding one would be demanding nothing.
    #[expect(
        unnameable_types,
        reason = "the sealed-trait pattern makes the supertrait deliberately unnameable so downstream crates cannot implement a coverage"
    )]
    pub trait Sealed {}
}

/// The coverage of a witness minted on the coherence join alone: the shape and
/// the textual selection order were found to agree, and nothing was projected.
///
/// A type-level token, never a value the machine constructs. What it does is
/// stand in [`AdmittedRefusalFamily`]'s coverage seat, where it satisfies
/// [`ShapeAdmission`] and fails [`OrderAdmission`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShapeCoherent;

/// The coverage of a witness minted on the coherence join AND the projection
/// join: the strictly stronger of the two.
///
/// It satisfies both [`ShapeAdmission`] and [`OrderAdmission`], so a witness
/// carrying it reaches every consumer a [`ShapeCoherent`] one reaches and the
/// order-sensitive consumers besides. That containment IS the implication
/// hierarchy, and it runs one way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderProjected;

/// The floor every coverage clears: the shape and the textual selection order
/// were joined.
///
/// A consumer that only needs a family's declaration to be self-consistent
/// takes its coverage generically under this bound, so both coverages reach it.
/// Sealed — see the module's admission rule.
pub trait ShapeAdmission: sealed::Sealed {
    /// This coverage's inspection projection: the value form a diagnostic, a
    /// receipt, or a published envelope writes down. One fact in two forms —
    /// the type is what a consumer demands, and this constant is what a reader
    /// reads.
    const INSPECTION: FamilyAdmissionCoverage;
}

/// The stronger coverage: the projection join ran too, so the family's typed
/// cause order and its textual projection were found to be one fact in two
/// forms.
///
/// An order-sensitive consumer demands this bound, and the supertrait relation
/// is what makes the demand asymmetric: every [`OrderAdmission`] coverage is a
/// [`ShapeAdmission`] coverage, and no [`ShapeAdmission`] coverage is admitted
/// here by that fact alone. Sealed on the same terms.
pub trait OrderAdmission: ShapeAdmission {}

impl sealed::Sealed for ShapeCoherent {}

impl ShapeAdmission for ShapeCoherent {
    const INSPECTION: FamilyAdmissionCoverage = FamilyAdmissionCoverage::ShapeCoherence;
}

impl sealed::Sealed for OrderProjected {}

impl ShapeAdmission for OrderProjected {
    const INSPECTION: FamilyAdmissionCoverage =
        FamilyAdmissionCoverage::ShapeCoherenceAndOrderProjection;
}

impl OrderAdmission for OrderProjected {}

/// Evidence that one refusal family's declaration closed its own joins.
///
/// # Why a declaration is not yet a machine fact
///
/// [`RefusalFamily`] is an extension point: any home, and any consumer outside
/// this crate, declares a family and states its own `SHAPE` and
/// `SELECTION_ORDER`. Nothing in the type system makes those two agree. A road
/// that reads either constant and acts on it is trusting a pair of declarations
/// nobody joined. This witness is that join, and it is opaque and
/// constructor-free, so holding one *is* the evidence.
///
/// # The coverage is a type parameter, so strength cannot be lost in transit
///
/// The two admission roads do not establish the same thing, and the difference
/// is carried in the witness's TYPE: [`admit_shape`] returns a witness covered
/// by [`ShapeCoherent`], [`admit_order`] one covered by [`OrderProjected`]. A
/// consumer states the strength it needs as a bound — [`ShapeAdmission`] where
/// self-consistency is enough, [`OrderAdmission`] where the family's declared
/// order is about to be acted on, as [`cause_order`](Self::cause_order) does —
/// and the compiler settles whether the witness in hand clears it. The weaker
/// coverage reaching a stronger consumer is unrepresentable rather than checked,
/// and no runtime read stands between the two.
///
/// [`FamilyAdmissionCoverage`] survives as the inspection projection of that
/// type — [`ShapeAdmission::INSPECTION`] — so the settled fact can still be
/// written down on a receipt. It is never the axis enforcement rides.
///
/// # What the roads establish
///
/// [`admit_shape`] runs the coherence join: `SELECTION_ORDER` is non-empty
/// exactly when `SHAPE` is [`FamilyShape::SingleCause`]. [`admit_order`] runs
/// that join and then the projection join — [`DeclaredCauseOrder::projects_to`]
/// over the family's typed order — and is available only where the family
/// declares one. Both refuse with a typed cause; neither normalizes, repairs,
/// or narrows.
///
/// # The claim ceiling, exactly
///
/// It establishes nothing about whether the declared order is the RIGHT
/// selector for the family's checks; that is the owner's declaration and no road
/// can check it. It establishes nothing about the family's Rust body — that the
/// variants a `SingleCause` family declares are the causes its order names is a
/// join the derive performs over a captured declaration, not one this witness
/// can reach. And family uniqueness across a whole program remains the
/// composition root's join, exactly as [`CauseId`] states.
#[must_use = "an admitted family is the evidence a declaration closed its joins; dropping it \
              discards the only proof a road may act on that declaration"]
pub struct AdmittedRefusalFamily<F: RefusalFamily, Coverage: ShapeAdmission> {
    _family: PhantomData<F>,
    _coverage: PhantomData<Coverage>,
}

impl<F: RefusalFamily, Coverage: ShapeAdmission> AdmittedRefusalFamily<F, Coverage> {
    /// This witness's coverage, written as a value.
    ///
    /// The projection of the coverage type, not a second record of it: there is
    /// no stored field to disagree with the parameter, so what a reader reads is
    /// what the compiler enforced.
    #[must_use]
    pub const fn coverage(&self) -> FamilyAdmissionCoverage {
        Coverage::INSPECTION
    }
}

/// Admit one family's declaration on the coherence join.
///
/// # Errors
///
/// Returns [`FamilyAdmission::NotShapeCoherent`] when the declared shape and
/// the declared selection order contradict each other.
pub fn admit_shape<F: RefusalFamily>()
-> Result<AdmittedRefusalFamily<F, ShapeCoherent>, FamilyAdmission> {
    if shape_coheres::<F>() {
        Ok(AdmittedRefusalFamily {
            _family: PhantomData,
            _coverage: PhantomData,
        })
    } else {
        Err(FamilyAdmission::NotShapeCoherent)
    }
}

/// Admit one family's declaration on the coherence join AND the projection
/// join. Available only where the family declares its typed cause order,
/// because there is nothing to project against otherwise.
///
/// # Errors
///
/// Returns [`FamilyAdmission::NotShapeCoherent`] when the declared shape and
/// the declared selection order contradict each other, and
/// [`FamilyAdmission::NotProjected`] when the typed order and the textual
/// order are two facts rather than one fact in two forms.
pub fn admit_order<F: CauseOrderDeclaration>()
-> Result<AdmittedRefusalFamily<F, OrderProjected>, FamilyAdmission> {
    if !shape_coheres::<F>() {
        return Err(FamilyAdmission::NotShapeCoherent);
    }
    if F::DECLARED_ORDER.projects_to(F::SELECTION_ORDER) {
        Ok(AdmittedRefusalFamily {
            _family: PhantomData,
            _coverage: PhantomData,
        })
    } else {
        Err(FamilyAdmission::NotProjected)
    }
}

impl<F: CauseOrderDeclaration, Coverage: OrderAdmission> AdmittedRefusalFamily<F, Coverage> {
    /// This family's typed cause order.
    ///
    /// The order-sensitive consumer, and the reason [`OrderAdmission`] exists. A
    /// caller holding this value is about to rank causes by it, and a rank taken
    /// from an order nobody projected against the family's textual one would be
    /// a position in an order the family may not be declaring. So the road hangs
    /// off the stronger bound, and a [`ShapeCoherent`] witness does not reach it.
    ///
    /// The witness is the permission rather than the storage: the order is the
    /// family's own declared constant, read off the type, so there is no second
    /// copy kept here to drift from the declaration it came from.
    #[must_use]
    pub fn cause_order(&self) -> DeclaredCauseOrder {
        F::DECLARED_ORDER
    }
}

/// Whether one family's declared shape and declared selection order agree.
///
/// Stated once, here, because both mints ask it and a second copy would be a
/// second thing to keep true.
fn shape_coheres<F: RefusalFamily>() -> bool {
    matches!(F::SHAPE, FamilyShape::SingleCause) != F::SELECTION_ORDER.is_empty()
}

/// The universal refusal envelope: the registered reason, the treatment class,
/// what the family's declaration was admitted as covering, and the family body.
/// Deliberately location-free — an owner family that needs a location carries
/// its own location type, so this envelope imports nothing from any later home.
/// (Authored structural law, not an old-book quote: band 00 must import
/// nothing.)
#[must_use = "a refusal carries the lawful reason the operation did not proceed"]
pub struct Refusal<F: RefusalFamily> {
    reason: ReasonId,
    handling: HandlingClass,
    admission: FamilyAdmissionCoverage,
    family: F,
}

impl<F: RefusalFamily> Refusal<F> {
    /// Publish one refusal under a family whose declaration was admitted.
    ///
    /// This is the envelope's only mint, and it is the seat where a family's
    /// declared facts become trusted: publication is exactly the act that hands
    /// a refusal to a reader who will act on the family's shape and order
    /// without re-reading them, so an unadmitted declaration must not reach it.
    ///
    /// This is a **shape-only consumer**: publication acts on the family's
    /// declared shape and needs no rank taken out of its cause order, so the
    /// coverage rides generically under [`ShapeAdmission`] and both coverages
    /// reach it. What the witness carried in its type is projected onto the
    /// envelope through [`ShapeAdmission::INSPECTION`] rather than being checked
    /// and forgotten, so a reader can see which joins stood behind the
    /// declaration it is reading — a refusal published under coherence alone and
    /// one published under coherence and projection are not the same receipt.
    ///
    /// Its reach today is the crate's own: [`ReasonId`] has no public mint until
    /// the evidence home registers reasons, so nothing outside can hold the
    /// first argument. That is a stated ceiling, not a claim of use.
    pub fn published<Coverage: ShapeAdmission>(
        reason: ReasonId,
        handling: HandlingClass,
        family: F,
        admitted: &AdmittedRefusalFamily<F, Coverage>,
    ) -> Self {
        Self {
            reason,
            handling,
            admission: admitted.coverage(),
            family,
        }
    }

    /// Which joins the family's declaration was admitted on.
    #[must_use]
    pub const fn admission(&self) -> FamilyAdmissionCoverage {
        self.admission
    }

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

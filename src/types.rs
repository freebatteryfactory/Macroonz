//! The root shape calculus: the generic composition shapes every home instantiates,
//! plus the two axes admitted to root by explicit decision. Nothing here is a semantic
//! noun beyond those two — a semantic noun lives at root only by an explicit root
//! admission decision.
//!
//! # The structural spine
//!
//! One pattern governs the whole crate: *a compile-time shape makes the wrong move
//! unrepresentable, and a runtime-validated fact carried in the value's own canonical
//! bytes enforces the right move at the operation boundary.* The shape never decides —
//! it cannot see runtime facts; it makes bypassing the runtime check impossible.
//!
//! # The opaque-newtype obligations
//!
//! Every role-distinct public type in this crate satisfies eight obligations: it is
//! opaque; it is minted only by its owner; it is `Eq`/`Hash`; it has no `Ord` beyond a
//! declared raw-byte storage order; it serializes through an explicit codec only (no
//! ambient serde); it has no public constructor, no `Default`, and no cross-family
//! `From`; wrong-role construction does not compile; wrong-role decode refuses.
//!
//! # Crossings never gain
//!
//! At every boundary crossing, uncertainty only widens, budgets only shrink, authority
//! only attenuates, and information classification only restricts. Each reverse
//! direction exists solely as a named, authority-bearing morphism that consumes new
//! evidence and leaves a receipt. The falsifier for every home: attempt the gain
//! without the named morphism — it must be unrepresentable or refuse.
//!
//! # Result conventions
//!
//! `ASK` returns a pure result and explanation, publishing nothing. `DO` admits a
//! bounded effect batch after required evidence and decisions pass. `REQUEST` durably
//! admits an asynchronous effect intent. `PEND` admits the same durable intent and
//! additionally performs and observes one immediate bounded attempt. `bool` is never a
//! result axis: a two-variant result is lawful only when the question is
//! decidable-total from data in hand; any question whose answer can lag composes a
//! knowledge axis into its result. Only the knowledge axes (`Truth`,
//! `CommitKnowledge`, `OutcomeKnowledge` — owned by their homes) may say "not yet";
//! no other enum grows a `Pending` variant, and a merely owed-but-not-yet-performed
//! posture spells itself `Outstanding` or `Unresolved`, never `Pending`.
//!
//! By explicit decision, [`Freshness`] and [`ProofDisposition`] are *evidence facts*,
//! not knowledge axes: neither can express "not yet", so the three-axis closure
//! stands unbroken.
//!
//! # Standing prohibitions
//!
//! There is no universal uncertainty wrapper and no parallel belief store. One owner
//! per public type: every public type has exactly one owning home defining its body;
//! all others reference it. A projection may adapt syntax, transport, or presentation;
//! it may never change identity, schemas, authority, capabilities, bounds, effects,
//! results, refusals, or evidence meaning.

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// Limits: the typed hole carrying limit IDENTITY at compile time, with magnitude
// supplied either at compile time (`ConstLimit`) or by a schema-minted witness.
// ---------------------------------------------------------------------------

/// A limit family marker. The type names *which* limit governs a bounded value, so
/// two different limits never unify: `Bounded<T, DecodeMax>` and
/// `Bounded<T, ArenaMax>` are distinct types regardless of their magnitudes.
///
/// Owner homes declare their limit families; the schema home is the only authority
/// that mints runtime magnitudes (as [`LimitWitness`] values).
pub trait Limit {}

/// A limit family whose magnitude is known at compile time.
pub trait ConstLimit: Limit {
    /// The maximum item count this family admits.
    const MAX: usize;
}

/// The ceiling one PLANE admits its declared magnitudes under.
///
/// # Root owns the algebra; a profile owns the number
///
/// This crate owns the admission-witness algebra — which witnesses exist, what
/// each one establishes, and which road consumes which. It does not own any
/// plane's admissible magnitude. There is no single number that is right for
/// every plane: an authoring plane bounding token material, a qualification
/// plane rehearsing hostiles, and a host sizing its own buffers are answering
/// different questions, and one number spanning all of them would be a number
/// nobody decided — which is the exact defect a declared ceiling exists to end.
///
/// So the number is written down where its plane's seats are written down, and
/// the generic roads below are instantiated with the downstream profile type.
/// That is why this crate needs no edge to any plane that declares one.
///
/// # What this crate may seat, exactly
///
/// A profile-independent witness algebra; an absolute bound the REPRESENTATION
/// imposes; and narrowly named profiles its own laws stand under, behind
/// `cfg(test)`. Nothing else. In particular there is no production default
/// profile here, because a default seated for convenience becomes the ceiling
/// every downstream reaches for without deciding anything.
pub trait LimitAdmissionProfile {
    /// The widest declared magnitude this profile admits.
    ///
    /// What it rules out is a "bound" that bounds nothing: a magnitude no input
    /// under this plane could reach makes its checked constructor
    /// unfalsifiable, and a constructor that cannot refuse is not a checked
    /// constructor.
    const MAX_DECLARED_LIMIT: usize;
}

/// The profile the ROOT'S OWN LAWS stand under, and nothing else.
///
/// Narrowly named and `cfg(test)`-gated on purpose. The proof surface needs
/// families to admit in order to exercise the witness algebra at all, and the
/// families it declares are small demonstrations rather than any plane's
/// vocabulary. Seating a ceiling for them is not seating a production default:
/// this profile does not exist in a built artifact, and nothing outside the
/// crate's own proof surface can name it.
///
/// The number leaves room above the widest family the laws instantiate, which
/// is a home's issue bound in the low tens. A law that needed a wider one would
/// raise this number deliberately rather than inherit a wide one.
#[cfg(test)]
pub(crate) struct RootLawsProfile;

#[cfg(test)]
impl LimitAdmissionProfile for RootLawsProfile {
    const MAX_DECLARED_LIMIT: usize = 1_024;
}

/// A second laws-only profile, deliberately narrower than [`RootLawsProfile`].
///
/// It exists so the proof surface can show that a witness names WHICH profile
/// admitted it. One profile alone cannot demonstrate that, because there is
/// nothing for it to fail to unify with.
#[cfg(test)]
pub(crate) struct NarrowLawsProfile;

#[cfg(test)]
impl LimitAdmissionProfile for NarrowLawsProfile {
    const MAX_DECLARED_LIMIT: usize = 8;
}

/// Evidence that one limit family's declared magnitude stands under one
/// profile's ceiling.
///
/// # Why a declaration is not yet a machine fact
///
/// [`Limit`] and [`ConstLimit`] are extension points: any home — and any
/// frontend outside this crate — declares a family, and the compiler checks
/// nothing about the number it declares. A road that reads `L::MAX` and acts on
/// it is trusting a value nobody validated. This witness is what a family's
/// declaration must pass through before a road may treat it as a fact, and it is
/// opaque and constructor-free, so holding one *is* the evidence.
///
/// # What the mint establishes, and what it deliberately does not
///
/// [`under_profile`](Self::under_profile) establishes exactly one thing about
/// `L`, at COMPILE TIME, so no artifact carrying an inadmissible family is ever
/// produced: `L::MAX` stands under `P::MAX_DECLARED_LIMIT`.
///
/// It does NOT establish that the family admits an item, and that absence is
/// deliberate. A family declaring `MAX = 0` mints this witness lawfully,
/// because a zero maximum is an honest declaration for a seat that holds
/// nothing: [`Bounded::empty`] under such a family is a real empty collection,
/// and a base witness that refused the declaration would refuse that seat with
/// it. The positivity claim is seated one witness up, in [`PositiveLimit`],
/// where exactly the roads promising an inhabitant consume it.
///
/// # Both parameters are load-bearing
///
/// The family tag stops one family's admission from authorizing another. The
/// profile tag stops one PLANE's admission from authorizing another: a
/// magnitude admitted under a wide authoring ceiling is not admitted under a
/// narrow qualification one, and `AdmittedLimit<L, A>` does not typecheck where
/// `AdmittedLimit<L, B>` is required.
///
/// # The claim ceiling, exactly
///
/// It establishes nothing about whether the magnitude is the RIGHT one for its
/// domain. That is the owner's declaration, no road can check it, and this
/// witness does not pretend to. It establishes nothing about any runtime value.
/// And it says nothing whatever about a [`Limit`] family that declares no
/// compile-time magnitude: such a family has no `MAX` to admit, its runtime
/// magnitude is the schema home's [`LimitWitness`] to mint, and no road here
/// admits it.
#[must_use = "an admitted limit is the evidence a family's declared magnitude passed admission; \
              dropping it discards the only proof a road may act on that declaration"]
pub struct AdmittedLimit<L: Limit, P: LimitAdmissionProfile> {
    max: usize,
    _family: PhantomData<L>,
    _profile: PhantomData<P>,
}

impl<L: ConstLimit, P: LimitAdmissionProfile> AdmittedLimit<L, P> {
    /// Admit one compile-time magnitude against one profile's declared ceiling.
    ///
    /// The `const` block below settles the question before the program runs: a
    /// `const` item refuses under `cargo check`, a function-body call refuses at
    /// codegen, and there is no road that reaches a running program with a
    /// family past its profile's ceiling. This is why the road has no refusal to
    /// return — the failing case is not a value a caller has to invent a repair
    /// for, it is a program that does not exist.
    pub const fn under_profile() -> Self {
        const {
            assert!(
                L::MAX <= P::MAX_DECLARED_LIMIT,
                "a declared magnitude past the admitting profile's ceiling bounds nothing"
            );
        }
        Self {
            max: L::MAX,
            _family: PhantomData,
            _profile: PhantomData,
        }
    }
}

impl<L: Limit, P: LimitAdmissionProfile> AdmittedLimit<L, P> {
    /// The admitted maximum this witness carries.
    #[must_use]
    pub const fn max(&self) -> usize {
        self.max
    }
}

/// Evidence that one limit family is admitted under a profile AND admits at
/// least one item.
///
/// # Why positivity is a separate witness rather than a stronger base
///
/// The two facts govern different roads. "Stands under the plane's ceiling" is
/// what a checked constructor needs in order to compare a runtime count against
/// a number somebody decided. "Admits at least one item" is what a road
/// PROMISING AN INHABITANT needs: [`NonEmptyBounded`] carries a first item by
/// signature, so a family declaring `MAX = 0` is one that road can never
/// satisfy, whatever the ceiling says.
///
/// Folding positivity into [`AdmittedLimit`] would have made a zero maximum
/// inadmissible everywhere, and a zero maximum is lawful for [`Bounded::empty`]
/// — the empty-only seat is a real seat rather than a mistake. So the stronger
/// claim is seated in the stronger witness, and the roads that promise an
/// inhabitant are the ones that consume it.
///
/// # The stronger witness CONTAINS the weaker one
///
/// The admission fact is not restated here — it is carried. This type's one
/// field is an [`AdmittedLimit`] minted by [`AdmittedLimit::under_profile`], and
/// the magnitude this witness reports is that witness's own. So the ceiling
/// comparison, the diagnostic it fails with, and the number it admits have
/// exactly one owner, and "the positive witness is the stronger form of the base
/// one" is a fact about this value's shape rather than an agreement between two
/// assertions somebody has to keep in step. A change to the base admission
/// reaches here because it IS the base admission, not because two copies were
/// edited together.
///
/// # No widening road back
///
/// Containment is not a conversion. The contained witness is private, no
/// accessor hands it out, and there is no road from here to [`AdmittedLimit`].
/// Dropping a claim would be lawful, but no seat needs it, and an unearned
/// conversion is surface nobody asked for.
#[must_use = "a positive limit is the evidence a family's declared magnitude passed admission and \
              admits an item; dropping it discards the only proof a road promising an inhabitant \
              may act on"]
pub struct PositiveLimit<L: Limit, P: LimitAdmissionProfile> {
    admitted: AdmittedLimit<L, P>,
}

impl<L: ConstLimit, P: LimitAdmissionProfile> PositiveLimit<L, P> {
    /// Admit one compile-time magnitude against one profile's ceiling AND
    /// establish that the family admits an item.
    ///
    /// The ceiling question is not asked here. It is asked by
    /// [`AdmittedLimit::under_profile`], whose witness this road holds, so
    /// instantiating the stronger witness instantiates the weaker one and its
    /// `const` block settles the ceiling in the one place that owns it. The
    /// `const` block below adds the single fact this witness is stronger by and
    /// nothing else. Both are settled before the program runs, so this road has
    /// no refusal to return either.
    pub const fn inhabited_under_profile() -> Self {
        const {
            assert!(L::MAX >= 1, "a limit family admitting no item at all");
        }
        Self {
            admitted: AdmittedLimit::under_profile(),
        }
    }
}

impl<L: Limit, P: LimitAdmissionProfile> PositiveLimit<L, P> {
    /// The admitted maximum this witness carries; at least one by construction.
    ///
    /// Read off the contained base witness, so no second copy of the magnitude
    /// stands here to disagree with the one that was admitted.
    #[must_use]
    pub const fn max(&self) -> usize {
        self.admitted.max()
    }
}

/// A runtime magnitude for the limit family `L`, minted only by schema validation.
/// Carrying the family as a type parameter keeps runtime-limited and compile-limited
/// values in the same shape without confusing their authorities.
#[must_use = "a limit witness is the magnitude schema validation established; dropping it \
              discards the only admitted bound for its family"]
pub struct LimitWitness<L: Limit> {
    max: usize,
    _family: PhantomData<L>,
}

impl<L: Limit> LimitWitness<L> {
    /// In-crate mint for laws. Test-gated until the schema home carries the real
    /// declaration path — the gate comes off when a lawful minter exists.
    #[cfg(test)]
    pub(crate) const fn declared(max: usize) -> Self {
        Self {
            max,
            _family: PhantomData,
        }
    }

    /// The admitted maximum this witness carries.
    #[must_use]
    pub fn max(&self) -> usize {
        self.max
    }
}

/// The construction refusal for bounded collections. A plain root enum — the
/// refusal-family binding is implemented by the refusal home, pointing downward.
#[must_use = "a refusal carries the lawful reason the construction did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundedConstruction {
    /// The supplied items exceed the limit family's admitted maximum.
    OverLimit,
}

/// A collection that structurally carries which limit family bounds it. There is no
/// public unbounded collection anywhere in the machine; both constructors are the
/// enforcement seams of that law.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bounded<T, L: Limit> {
    items: Vec<T>,
    _family: PhantomData<L>,
}

impl<T, L: ConstLimit> Bounded<T, L> {
    /// The fixed-arity collection: a *total structural* constructor whose "fits
    /// the bound" proof is discharged at COMPILE TIME.
    ///
    /// The item count is `N`, a compile-time constant, so the `const` block
    /// below decides the whole question before the program runs and this road
    /// has no refusal to return. Its reason for existing is downstream honesty:
    /// where the material is known statically — a declared roster, a fixed pair
    /// of assumptions, one repair — a caller has no runtime failure to invent a
    /// value for, so the place a caller reaches for `unwrap_or_else(empty)` and
    /// silently deletes its own content simply is not on the road.
    ///
    /// The honest scope is [`NonEmptyBounded::singleton`]'s: the refusal fires
    /// when the instantiation is const-evaluated, so no artifact carrying an
    /// over-long fixed collection is ever produced.
    ///
    /// # The claim class: LOCAL ARITY, and it is not an admission
    ///
    /// This road reads `L::MAX` bare, and it stays that way by decision. What it
    /// proves is that `N` items — a count written at the call site — fit under a
    /// type-level maximum. It does NOT prove that maximum was a sensible
    /// declaration, and it must never be read as though it had: proving the
    /// family's magnitude was admitted requires a profile, and no profile is
    /// involved here. A road that claims the ADMITTED FAMILY MAGNITUDE consumes
    /// an admission witness — [`Bounded::admitted_const`] is that road. This one
    /// proves exactly the local fact it needs and claims nothing wider, which is
    /// why it can stay total.
    #[must_use]
    pub fn from_array<const N: usize>(items: [T; N]) -> Self {
        const {
            assert!(
                N <= L::MAX,
                "a fixed collection longer than its limit family admits"
            );
        }
        Self {
            items: Vec::from(items),
            _family: PhantomData,
        }
    }

    /// Checked construction against the family's ADMITTED compile-time maximum.
    ///
    /// # The claim class: ADMITTED FAMILY MAGNITUDE
    ///
    /// [`Limit`] and [`ConstLimit`] are extension points, so `L::MAX` is
    /// whatever its author wrote and the compiler checks nothing about it. This
    /// road reads its bound off the [`AdmittedLimit`] witness instead, which
    /// means the number it compares against is one that stood under a named
    /// plane's ceiling rather than one nobody validated. That is the whole
    /// difference, and it is why the witness is a parameter rather than a
    /// comment: a caller that has not admitted the family under a profile has no
    /// value to pass.
    ///
    /// The profile rides on the witness and not on the collection: `P` is a
    /// parameter of this method, so admitting a family under one plane never
    /// stamps that plane onto the value the road returns.
    ///
    /// The total structural roads beside this one — [`Bounded::from_array`],
    /// [`NonEmptyBounded::singleton`], [`NonEmptyBounded::from_array`] — read
    /// `L::MAX` bare by decision. Each proves a LOCAL fact and claims no
    /// admission; see each road's own claim class.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the items exceed the
    /// admitted maximum.
    pub fn admitted_const<P: LimitAdmissionProfile>(
        items: Vec<T>,
        admitted: &AdmittedLimit<L, P>,
    ) -> Result<Self, BoundedConstruction> {
        if items.len() <= admitted.max() {
            Ok(Self {
                items,
                _family: PhantomData,
            })
        } else {
            Err(BoundedConstruction::OverLimit)
        }
    }
}

impl<T, L: Limit> Bounded<T, L> {
    /// The empty collection: a *total structural* constructor.
    ///
    /// The two constructor classes are not interchangeable and are not spelled
    /// alike. A **checked** constructor ([`Bounded::admitted_const`],
    /// [`Bounded::admitted`]) reads a runtime count against a declared bound and
    /// MAY REFUSE; its name carries `admitted` because admission is exactly what
    /// it performs. A **total structural** constructor CANNOT FORM THE FAILING
    /// CASE: no limit family admits fewer than zero items, so this road has no
    /// refusal to return, and callers outside this crate get a bounded value
    /// without an impossible error branch to invent a value for.
    ///
    /// # The claim class: NO MAGNITUDE EVIDENCE AT ALL
    ///
    /// This road never reads `L::MAX`, so it neither claims nor needs one. That
    /// is why it is bounded by `L: Limit` alone and why a family declaring
    /// `MAX = 0` inhabits it lawfully: an empty-only seat is a real seat, and
    /// the empty collection under it is honest rather than degenerate.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            items: Vec::new(),
            _family: PhantomData,
        }
    }

    /// Checked construction against a schema-minted runtime witness of the same
    /// limit family — a witness for another family does not typecheck.
    ///
    /// # The claim class: SCHEMA-MINTED RUNTIME MAGNITUDE
    ///
    /// No profile is involved and none could be: the magnitude here was
    /// established by schema validation at runtime rather than declared at
    /// compile time, so there is no `L::MAX` to admit. A profile-scoped
    /// admission is not evidence for this road and this road's witness is not
    /// evidence for that one.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the items exceed the
    /// witnessed maximum.
    pub fn admitted(items: Vec<T>, witness: &LimitWitness<L>) -> Result<Self, BoundedConstruction> {
        if items.len() <= witness.max() {
            Ok(Self {
                items,
                _family: PhantomData,
            })
        } else {
            Err(BoundedConstruction::OverLimit)
        }
    }

    /// Number of items held.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Read the held values. Read-only by construction: the collection is
    /// borrowed, not consumed, and no mutable or positional road exists beside
    /// this one — there is no `iter_mut`, no `Index`, and no slice escape.
    ///
    /// # The order law
    ///
    /// Iteration exposes values for observation; iteration order may influence
    /// semantic meaning ONLY where the owner type explicitly declares ordering
    /// as semantic. Identity-bearing generation over order-insensitive
    /// collections must canonicalize by an owner-declared order or key first.
    /// testpak owes the permutation hostiles: identical plans and identical
    /// output identities under permuted order-insensitive inputs.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

/// The construction refusal for non-empty bounded collections. Emptiness is not
/// a cause: the constructor signature takes the first item separately, so a
/// zero-item value is unrepresentable rather than refused.
#[must_use = "a refusal carries the lawful reason the construction did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonEmptyBoundedConstruction {
    /// The supplied items exceed the limit family's admitted maximum.
    OverLimit,
}

/// A bounded collection that structurally holds at least one item — a refusal with
/// zero issues is not a refusal, and this shape makes that unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyBounded<T, L: Limit> {
    first: T,
    rest: Vec<T>,
    _family: PhantomData<L>,
}

impl<T, L: ConstLimit> NonEmptyBounded<T, L> {
    /// Checked construction against the family's ADMITTED and POSITIVE
    /// compile-time maximum. The first item is a separate parameter — emptiness
    /// is unrepresentable, not refused.
    ///
    /// # The claim class: ADMITTED FAMILY MAGNITUDE, and it must be inhabited
    ///
    /// This road takes the stronger witness, and the reason is its own
    /// signature. It promises an inhabitant: whatever the runtime count turns
    /// out to be, the value it returns holds a first item. A family declaring
    /// `MAX = 0` can never lawfully satisfy that promise, so an
    /// [`AdmittedLimit`] — which admits zero-maximum families on purpose — is
    /// not enough evidence here. [`PositiveLimit`] carries both facts, and the
    /// bound this road compares against is read off it rather than off the
    /// declaration.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyBoundedConstruction::OverLimit`] when the total item
    /// count exceeds the admitted maximum.
    pub fn admitted_const<P: LimitAdmissionProfile>(
        first: T,
        rest: Vec<T>,
        admitted: &PositiveLimit<L, P>,
    ) -> Result<Self, NonEmptyBoundedConstruction> {
        if rest.len().saturating_add(1) <= admitted.max() {
            Ok(Self {
                first,
                rest,
                _family: PhantomData,
            })
        } else {
            Err(NonEmptyBoundedConstruction::OverLimit)
        }
    }

    /// Carry as many supplied items as the family's ADMITTED maximum holds, and
    /// hand back how many it could not carry.
    ///
    /// # The claim class: ADMITTED FAMILY MAGNITUDE, reported rather than refused
    ///
    /// A third constructor class, and it exists because the other two answer the
    /// wrong question for a REPORT. A checked constructor refuses the whole
    /// value, which is right for material that is meaningless in part — a trail,
    /// a membership, a ceiling. A collection-shaped refusal body is not that: the
    /// issues an over-bound pass established are each true on their own, and
    /// refusing the body would leave a caller with no findings at all. So this
    /// road carries the prefix the admitted magnitude holds and reports the count
    /// it dropped beside it, which is what keeps the truncation from being
    /// silent.
    ///
    /// # Why this road is crate-internal
    ///
    /// The two values it produces are a carry and a count, and a road that hands
    /// both to a caller hands out two things the caller may pair with anything.
    /// A body truncated by one pass could then wear the count another pass
    /// dropped, and both values would still be honest on their own while the
    /// pair was a lie. So this road is the crate's own seam and has exactly one
    /// consumer: [`crate::refusal::AdmittedPrefix`], band 00's package, which
    /// takes the carry and the count in the one construction that produced them
    /// and never lets them apart again. The public road to a truncated
    /// collection is that package and nothing else.
    ///
    /// It is total for the same reason [`NonEmptyBounded::singleton`] is: the
    /// witness is [`PositiveLimit`], so the maximum is at least one, so the first
    /// item always fits and the prefix is never empty. There is no failing case
    /// to return, and therefore no impossible arm for a caller to fill with a
    /// value nobody computed.
    ///
    /// The bound is read off the witness, so the number this road truncates at is
    /// one that stood under a named plane's ceiling rather than one nobody
    /// validated. The profile rides on the witness and is not stamped onto the
    /// returned value.
    #[must_use = "the dropped count is what keeps a truncated report from claiming completeness"]
    pub(crate) fn admitted_prefix<P: LimitAdmissionProfile>(
        first: T,
        rest: Vec<T>,
        admitted: &PositiveLimit<L, P>,
    ) -> (Self, usize) {
        let carried = admitted.max().saturating_sub(1);
        let omitted = rest.len().saturating_sub(carried);
        let mut prefix = rest;
        prefix.truncate(carried);
        (
            Self {
                first,
                rest: prefix,
                _family: PhantomData,
            },
            omitted,
        )
    }

    /// The fixed-arity collection: a *total structural* constructor whose "at
    /// least one" and "fits the bound" proofs are BOTH discharged at COMPILE
    /// TIME.
    ///
    /// The first item is a separate parameter, so emptiness is unrepresentable
    /// rather than refused, and the remainder's arity is `N`, a compile-time
    /// constant, so the `const` block below settles the bound question before
    /// the program runs. This road has no refusal to return.
    ///
    /// Its reason for existing is [`Bounded::from_array`]'s: where a COMPLETE
    /// set is known statically — a two-member output roster fixed by a declared
    /// shape, a pair of citations written side by side — a caller has no runtime
    /// failure to invent a value for, so the place a caller reaches for a
    /// shortened collection is not on the road at all.
    ///
    /// # The claim class: LOCAL ARITY AND LOCAL POSITIVITY
    ///
    /// The `const` block proves that `N + 1` items fit under a type-level
    /// maximum, and the separate first parameter is the inhabitant. Both are
    /// facts about THIS call, and neither claims the family's magnitude was
    /// admitted — that claim needs a profile, and no profile is involved here.
    /// The road that makes it is [`NonEmptyBounded::admitted_const`].
    #[must_use]
    pub fn from_array<const N: usize>(first: T, rest: [T; N]) -> Self {
        const {
            assert!(
                N < L::MAX,
                "a fixed non-empty collection longer than its limit family admits"
            );
        }
        Self {
            first,
            rest: Vec::from(rest),
            _family: PhantomData,
        }
    }

    /// The one-item collection: a *total structural* constructor whose "at
    /// least one" proof is discharged at COMPILE TIME.
    ///
    /// The two constructor classes are not interchangeable. A **checked**
    /// constructor ([`NonEmptyBounded::admitted_const`],
    /// [`NonEmptyBounded::admitted`]) reads a runtime count against a declared
    /// bound and MAY REFUSE. This **total structural** road CANNOT FORM THE
    /// FAILING CASE: the only way a single item could exceed a family's
    /// maximum is a family declaring `MAX = 0`, and the `const` block below
    /// rejects that instantiation at const evaluation. The honest scope: the
    /// refusal fires when the instantiation is const-evaluated — a `const`
    /// item refuses under `cargo check`, while a function-body call refuses at
    /// codegen — so no artifact containing this road under a zero-maximum
    /// family is ever produced, and the failing case never reaches a running
    /// program. The qualification fixture exercises the `const`-item form.
    ///
    /// Its reason for existing is downstream honesty: a caller assembling a
    /// one-issue refusal body has no impossible error branch to fabricate a
    /// value for, so refusal construction never becomes the place a consumer
    /// reaches for a panic.
    ///
    /// # The claim class: LOCAL POSITIVITY
    ///
    /// The `const` block proves `L::MAX >= 1` — the one fact this call needs in
    /// order to hold its single item — and proves it structurally, off the
    /// declaration. It claims nothing about whether that declaration was
    /// admitted under any plane's ceiling, and it must never be read as though
    /// it did. [`PositiveLimit`] is where the same positivity fact is seated
    /// TOGETHER WITH admission, for the road that needs both.
    #[must_use]
    pub const fn singleton(value: T) -> Self {
        const { assert!(L::MAX >= 1, "a limit family admitting no item at all") }
        Self {
            first: value,
            rest: Vec::new(),
            _family: PhantomData,
        }
    }
}

impl<T, L: Limit> NonEmptyBounded<T, L> {
    /// Checked construction against a schema-minted runtime witness.
    ///
    /// # The claim class: SCHEMA-MINTED RUNTIME MAGNITUDE
    ///
    /// [`Bounded::admitted`]'s exactly, and the inhabitant is supplied by the
    /// signature rather than by evidence: the first item is a separate
    /// parameter, so emptiness is unrepresentable here whatever the witnessed
    /// magnitude turns out to be.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyBoundedConstruction::OverLimit`] when the total item
    /// count exceeds the witnessed maximum.
    pub fn admitted(
        first: T,
        rest: Vec<T>,
        witness: &LimitWitness<L>,
    ) -> Result<Self, NonEmptyBoundedConstruction> {
        if rest.len().saturating_add(1) <= witness.max() {
            Ok(Self {
                first,
                rest,
                _family: PhantomData,
            })
        } else {
            Err(NonEmptyBoundedConstruction::OverLimit)
        }
    }

    /// The guaranteed first item.
    #[must_use]
    pub fn first(&self) -> &T {
        &self.first
    }

    /// Number of items held; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rest.len().saturating_add(1)
    }

    /// Always `false`: the shape holds at least one item. Present because the
    /// `len`/`is_empty` pair is conventional; the constant answer *is* the law.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Read the held values, the guaranteed first item ahead of the rest.
    /// Read-only by construction: the collection is borrowed, not consumed, and
    /// no mutable or positional road exists beside this one — there is no
    /// `iter_mut`, no `Index`, and no slice escape.
    ///
    /// # The order law
    ///
    /// Iteration exposes values for observation; iteration order may influence
    /// semantic meaning ONLY where the owner type explicitly declares ordering
    /// as semantic. Identity-bearing generation over order-insensitive
    /// collections must canonicalize by an owner-declared order or key first.
    /// testpak owes the permutation hostiles: identical plans and identical
    /// output identities under permuted order-insensitive inputs.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        core::iter::once(&self.first).chain(self.rest.iter())
    }
}

// ---------------------------------------------------------------------------
// Freshness: root-admitted axis. Current and Stale are TYPES, not variants — an API
// that requires fresh input demands `Current<T>` and stale data does not typecheck.
// ---------------------------------------------------------------------------

/// The invalidation coordinate of one claim family. Every family implements this
/// with its *own* coordinate; there is no universal cut, and a flattened
/// anything-freshness substrate is refused by construction.
pub trait EvidenceCut {}

/// The uninhabited coordinate: a claim family with no admitted invalidation
/// coordinate parameterizes over [`Never`], which makes its `Stale` form
/// unrepresentable rather than runtime-checked.
pub enum Never {}

impl EvidenceCut for Never {}

/// A value proven applicable now. Minted only by the evidence-producing boundary;
/// there is no public constructor, so holding a `Current<T>` *is* the proof.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Current<T> {
    value: T,
}

impl<T> Current<T> {
    /// In-crate mint for laws. Test-gated until an evidence boundary exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(value: T) -> Self {
        Self { value }
    }

    /// The proven-fresh value.
    #[must_use]
    pub fn get(&self) -> &T {
        &self.value
    }
}

/// A value disclosed as stale, carrying exactly which coordinate it is stale
/// against. Staleness changes present admissibility, never the earlier claim's
/// truth. Returning to [`Current`] happens only through a named re-assessment
/// morphism that consumes a new observation — a crossing never gains.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stale<T, Cut: EvidenceCut> {
    value: T,
    against: Cut,
}

impl<T, Cut: EvidenceCut> Stale<T, Cut> {
    /// In-crate mint for laws. Test-gated until an evidence boundary exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(value: T, against: Cut) -> Self {
        Self { value, against }
    }

    /// The stale value, readable under disclosure.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// The coordinate this value is stale against.
    #[must_use]
    pub fn against(&self) -> &Cut {
        &self.against
    }
}

/// The classification join over the two freshness types, produced where an
/// assessment has not yet branched. The types are primary; this sum only carries
/// them to the branch point.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Freshness<T, Cut: EvidenceCut> {
    /// Proven applicable now.
    Current(Current<T>),
    /// Disclosed stale against a named coordinate.
    Stale(Stale<T, Cut>),
}

// ---------------------------------------------------------------------------
// Proof and completeness: root-admitted axis + the non-erasable-domain shape.
// ---------------------------------------------------------------------------

/// What one verification run established about one claim. Its own axis — never a
/// terminal variant, never a rank, and (by decision) not a knowledge axis: no variant
/// means "not yet".
#[must_use = "a disposition is what the verification run established; dropping it leaves the \
              run's conclusion unrecorded"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofDisposition {
    /// The claim held under the run's denominator.
    Established,
    /// The claim failed.
    Falsified,
    /// Independent support was added without full establishment.
    Corroborated,
    /// The claim survives only in a narrowed form.
    Narrowed,
    /// The run's denominator was not fully covered.
    Incomplete,
}

/// Completeness over an owner-specific domain. The domain parameter is
/// non-erasable, so a complete query can never masquerade as complete
/// verification. Owners instantiate it under their own names — source closure in
/// navigation, materialization coverage in derived data, the verification
/// denominator in evidence.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Completeness<D> {
    /// The full declared domain was covered.
    Complete {
        /// The domain that was covered.
        over: D,
    },
    /// Coverage fell short of the declared domain.
    Incomplete {
        /// The domain that was declared.
        expected: D,
        /// The portion not covered.
        missing: D,
    },
}

// ---------------------------------------------------------------------------
// Evidence references: the generic typed-reference shape (identity class E).
// ---------------------------------------------------------------------------

/// Whether a referent is currently reachable. A non-identifying runtime fact
/// carried on a reference (authored shape; the four-component roster is the old
/// book's: referent identity + version + availability + integrity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReferentAvailability {
    /// The referent is currently reachable.
    Available,
    /// The referent is not currently reachable.
    Unavailable,
}

/// Whether a referent's bytes verify against its identity. Non-identifying.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReferentIntegrity {
    /// The referent verifies.
    Intact,
    /// The referent fails verification.
    Damaged,
}

/// A typed reference to evidence about a claim, carrying all four Class-E
/// components: referent identity and version (the identifying pair — equality
/// and hashing use exactly these two), availability and integrity
/// (non-identifying runtime facts). The claim marker is defined by the owner
/// making the claim, never centrally. This is a reference, not a container — no
/// value comes out of it.
#[derive(Debug, Clone)]
pub struct EvidenceRef<Claim> {
    referent: [u8; 32],
    version: u64,
    availability: ReferentAvailability,
    integrity: ReferentIntegrity,
    _claim: PhantomData<Claim>,
}

impl<Claim> EvidenceRef<Claim> {
    /// In-crate mint for laws. Test-gated until an evidence boundary carries the
    /// real path — the gate comes off when a lawful minter exists.
    #[cfg(test)]
    pub(crate) const fn bound(
        referent: [u8; 32],
        version: u64,
        availability: ReferentAvailability,
        integrity: ReferentIntegrity,
    ) -> Self {
        Self {
            referent,
            version,
            availability,
            integrity,
            _claim: PhantomData,
        }
    }

    /// The referent's identity digest.
    #[must_use]
    pub fn referent(&self) -> [u8; 32] {
        self.referent
    }

    /// The referent's version.
    #[must_use]
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Current reachability — never part of identity.
    #[must_use]
    pub fn availability(&self) -> ReferentAvailability {
        self.availability
    }

    /// Current verification state — never part of identity.
    #[must_use]
    pub fn integrity(&self) -> ReferentIntegrity {
        self.integrity
    }
}

impl<Claim> PartialEq for EvidenceRef<Claim> {
    fn eq(&self, other: &Self) -> bool {
        self.referent == other.referent && self.version == other.version
    }
}

impl<Claim> Eq for EvidenceRef<Claim> {}

impl<Claim> core::hash::Hash for EvidenceRef<Claim> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.referent.hash(state);
        self.version.hash(state);
    }
}

// ---------------------------------------------------------------------------
// Transition grammar: the closure conformance bar every machine proves against.
// ---------------------------------------------------------------------------

/// One dispatch outcome. Generic over the owner's refusal family — the root grammar
/// names no concrete refusal type.
#[must_use = "a dispatch outcome carries the transition that fired or the typed refusal that \
              stood in its place; dropping it is the silent drop the grammar forbids"]
pub enum Dispatch<T, R> {
    /// Exactly one transition applies.
    One(T),
    /// More than one could apply, the ambiguity is declared, and the owner's
    /// transition table names the resolution that selected this one.
    DeclaredAmbiguous(T),
    /// No transition applies: the unmatched pair returns a typed refusal — no drop,
    /// no panic, no untyped default.
    Refused(R),
}

/// The transition-system closure bar. This is a conformance contract, not a
/// universal state type: each machine implements it and proves six obligations —
/// exact initial posture; every declared state reachable; every transition naming
/// exact source, destination, and firing input; terminals that no transition
/// leaves; deterministic dispatch or declared ambiguity with a named resolution;
/// and total typed refusal for every unmatched pair. Closure is evidenced, not
/// asserted: the machine and its judge never share the dispatch path being judged.
pub trait TransitionSystem {
    /// The machine's state space.
    type State;
    /// The inputs that fire transitions.
    type Input;
    /// What a fired transition emits.
    type Effect;
    /// The machine's typed refusal family for unmatched pairs.
    type Refusal;

    /// The one lawful start state; nothing enters mid-stream.
    fn initial() -> Self::State;

    /// Whether a state is terminal. `bool` is lawful here: the question is
    /// decidable-total from data in hand.
    fn is_terminal(state: &Self::State) -> bool;

    /// Dispatch one input against one state.
    fn dispatch(
        state: &Self::State,
        input: &Self::Input,
    ) -> Dispatch<(Self::State, Self::Effect), Self::Refusal>;
}

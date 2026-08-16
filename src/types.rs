//! The root shape calculus: the generic composition shapes every home
//! instantiates, plus the two axes admitted to root by explicit decision.
//! `src/README.md` owns the root narrative and the crate-wide laws;
//! each declaration below owns its own contract.

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// Limits: the typed hole carrying limit IDENTITY at compile time, with magnitude
// supplied either at compile time (`ConstLimit`) or by a schema-minted witness.
// ---------------------------------------------------------------------------

/// Which authority supplies one limit family's capacity.
///
/// A capacity arrives by exactly one road, and which road is a fact about the
/// family, never about a call site:
/// [`DeclaredMagnitude`] is a number written in the source,
/// [`EvidenceSelectedMagnitude`] is a number the owner's evidence selects while
/// the machine runs, and [`UnstatedMagnitude`] names neither.
/// Every marker implementing this is uninhabited:
/// its whole job is to be the type a family's [`Limit::Authority`] resolves to,
/// so two ladders demanding different authorities can never both be satisfied.
///
/// The set is open — any home, and any frontend outside this crate, declares a
/// family — but a foreign marker reaches nothing:
/// [`ConstLimit`] and [`EvidenceSelectedLimit`] name their authority exactly,
/// so a fourth authority satisfies neither ladder and reaches no mint.
pub trait CapacityAuthority {}

/// The authority of a magnitude written in the source.
///
/// [`ConstLimit`] is the declaration that supplies the number, and the two
/// compile-time roads — [`AdmittedLimit`] and [`PositiveLimit`] — stand it
/// under a plane's ceiling and prove it admits an item, both before the
/// program runs.
pub enum DeclaredMagnitude {}

impl CapacityAuthority for DeclaredMagnitude {}

/// The authority of a magnitude the owner's evidence selects while the machine
/// runs.
///
/// [`EvidenceSelectedLimit`] is the declaration that admits this road, and the
/// two runtime roads — [`LimitWitness`] and [`PositiveLimitWitness`] — carry
/// the selection and the promise that it admits an item, because no `const`
/// block can see a number that does not exist yet.
pub enum EvidenceSelectedMagnitude {}

impl CapacityAuthority for EvidenceSelectedMagnitude {}

/// The authority of a family that has named neither capacity road.
///
/// A family bounding only a [`Bounded`] seat needs no magnitude:
/// [`Bounded::empty`] reads none, and an empty collection under such a family
/// is honest rather than degenerate.
/// Declaring it states the one fact the type system carries — the family is on
/// no ladder and no mint takes it.
/// Choosing a capacity authority is a one-line change at the family's own
/// declaration.
pub enum UnstatedMagnitude {}

impl CapacityAuthority for UnstatedMagnitude {}

/// A limit family marker: the type names *which* limit governs a bounded
/// value, so two different limits never unify.
/// `Bounded<T, DecodeMax>` and `Bounded<T, ArenaMax>` are distinct types
/// regardless of their magnitudes.
///
/// Owner homes declare their limit families; the schema home is the only
/// authority that mints runtime magnitudes (as [`LimitWitness`] values).
pub trait Limit {
    /// Which authority supplies this family's capacity.
    ///
    /// A family declares this once, and the ladder traits name the authority
    /// they require exactly:
    /// [`ConstLimit`] requires [`DeclaredMagnitude`], and
    /// [`EvidenceSelectedLimit`] requires [`EvidenceSelectedMagnitude`].
    /// One associated type resolves to one type, so a family declaring both
    /// ladders does not compile — the exclusion is the arity of an associated
    /// type, not a bound, a law, or a sentence.
    ///
    /// Declaring an authority without implementing its ladder trait is inert
    /// rather than wrong:
    /// the family names a road and never walks it, and the ladder traits stay
    /// the one place a capacity is actually reachable from.
    type Authority: CapacityAuthority;
}

/// A limit family whose magnitude is written in the source.
///
/// The supertrait bound names the authority exactly, so implementing this for
/// a family whose [`Limit::Authority`] is anything else is a type mismatch at
/// the declaration.
pub trait ConstLimit: Limit<Authority = DeclaredMagnitude> {
    /// The maximum item count this family admits.
    const MAX: usize;
}

/// A limit family whose capacity is selected from admitted runtime evidence,
/// not written in the source.
///
/// Implementing it enables the runtime witness ladder:
/// [`LimitWitness`] carries the selected magnitude, and
/// [`PositiveLimitWitness`] establishes that the selection admits an item.
/// The validating owner remains responsible for selecting a magnitude suitable
/// for the family's domain.
/// The compile-time counterpart is [`ConstLimit`]; a family cannot declare
/// both — [`Limit::Authority`] carries that exclusion.
pub trait EvidenceSelectedLimit: Limit<Authority = EvidenceSelectedMagnitude> {}

/// The ceiling one plane admits its declared magnitudes under.
///
/// The root owns the admission-witness algebra; a plane owns its number.
/// There is no single magnitude right for every plane, so the ceiling is
/// declared where the plane's seats are declared, and the generic roads here
/// are instantiated with the downstream profile type.
/// The root seats no production profile:
/// a default seated for convenience becomes a ceiling nobody decided.
pub trait LimitAdmissionProfile {
    /// The widest declared magnitude this profile admits.
    ///
    /// What it rules out is a bound that bounds nothing:
    /// a magnitude no input under the plane could reach makes its checked
    /// constructor unfalsifiable.
    const MAX_DECLARED_LIMIT: usize;
}

/// The profile the root's own laws stand under, and nothing else.
///
/// `cfg(test)`-gated on purpose: it does not exist in a built artifact, and
/// nothing outside the crate's own proof surface can name it.
/// The number leaves room above the widest family the laws instantiate; a law
/// needing a wider one raises this number deliberately.
#[cfg(test)]
pub(crate) struct RootLawsProfile;

#[cfg(test)]
impl LimitAdmissionProfile for RootLawsProfile {
    const MAX_DECLARED_LIMIT: usize = 1_024;
}

/// A second laws-only profile, deliberately narrower than [`RootLawsProfile`],
/// so the proof surface can show that a witness names which profile admitted
/// it.
#[cfg(test)]
pub(crate) struct NarrowLawsProfile;

#[cfg(test)]
impl LimitAdmissionProfile for NarrowLawsProfile {
    const MAX_DECLARED_LIMIT: usize = 8;
}

/// Evidence that one limit family's declared magnitude stands under one
/// profile's ceiling.
///
/// [`Limit`] is an extension point, so `L::MAX` is whatever its author wrote
/// and the compiler checks nothing about the number.
/// This witness is what a declaration passes through before a road may treat
/// the number as a fact; it is opaque and constructor-free, so holding one is
/// the evidence.
/// Both tags are load-bearing:
/// the family tag stops one family's admission from authorizing another, and
/// the profile tag stops one plane's admission from authorizing another
/// plane's — `AdmittedLimit<L, A>` does not typecheck where
/// `AdmittedLimit<L, B>` is required.
///
/// # Nonclaims
///
/// It does not establish that the family admits an item:
/// a `MAX = 0` family mints this lawfully, because [`Bounded::empty`] under it
/// is a real empty collection, and positivity is seated in [`PositiveLimit`]
/// where the inhabitant-promising roads consume it.
/// It does not establish that the magnitude is right for its domain, and it
/// says nothing about runtime values or about families that declare no
/// compile-time magnitude.
#[must_use = "an admitted limit is the evidence a family's declared magnitude passed admission; \
              dropping it discards the only proof a road may act on that declaration"]
pub struct AdmittedLimit<L: Limit, P: LimitAdmissionProfile> {
    max: usize,
    _family: PhantomData<L>,
    _profile: PhantomData<P>,
}

impl<L: ConstLimit, P: LimitAdmissionProfile> AdmittedLimit<L, P> {
    /// Admit one compile-time magnitude against one profile's declared
    /// ceiling.
    ///
    /// The `const` block settles the question before the program runs — a
    /// `const` item refuses under `cargo check`, a function-body call refuses
    /// at codegen — so no artifact carrying an inadmissible family is ever
    /// produced, and the road has no refusal to return.
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

/// Evidence that one limit family is admitted under a profile and admits at
/// least one item.
///
/// Positivity is a separate witness because the two facts govern different
/// roads:
/// a checked constructor needs the ceiling fact, while a road promising an
/// inhabitant — [`NonEmptyBounded`] carries a first item by signature — needs
/// a family that can hold one at all.
/// Folding positivity into [`AdmittedLimit`] would make a zero maximum
/// inadmissible everywhere, and the empty-only seat is a real seat.
///
/// The stronger witness contains the weaker one:
/// this type's one field is an [`AdmittedLimit`] minted by
/// [`AdmittedLimit::under_profile`], so the ceiling comparison, its
/// diagnostic, and the admitted number have exactly one owner.
/// Containment is not a conversion — the contained witness is private, no
/// accessor hands it out, and there is no road back to [`AdmittedLimit`].
#[must_use = "a positive limit is the evidence a family's declared magnitude passed admission and \
              admits an item; dropping it discards the only proof a road promising an inhabitant \
              may act on"]
pub struct PositiveLimit<L: Limit, P: LimitAdmissionProfile> {
    admitted: AdmittedLimit<L, P>,
}

impl<L: ConstLimit, P: LimitAdmissionProfile> PositiveLimit<L, P> {
    /// Admit one compile-time magnitude against one profile's ceiling and
    /// establish that the family admits an item.
    ///
    /// The ceiling question is asked by [`AdmittedLimit::under_profile`],
    /// whose witness this road holds; the `const` block below adds the single
    /// fact this witness is stronger by.
    /// Both settle before the program runs, so the road has no refusal to
    /// return.
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
    /// The admitted maximum this witness carries; at least one by
    /// construction.
    ///
    /// Read off the contained base witness, so no second copy of the
    /// magnitude stands here to disagree with the one that was admitted.
    #[must_use]
    pub const fn max(&self) -> usize {
        self.admitted.max()
    }
}

/// A runtime magnitude for the limit family `L`, minted only by schema
/// validation.
///
/// The family tag is a type parameter, so one family's witnessed magnitude
/// never authorizes another's seat, whatever the two numbers are.
/// The [`EvidenceSelectedLimit`] bound sits on this base rung, so a
/// runtime-selected magnitude cannot even be named for a family whose owner
/// never declared the runtime ladder.
///
/// # Nonclaims
///
/// It does not establish that the family admits an item:
/// a witnessed magnitude of zero is an honest selection for a seat that holds
/// nothing, and positivity is seated in [`PositiveLimitWitness`].
/// No production mint exists yet; the opening condition is the schema home
/// carrying a validation path that selects a magnitude.
#[must_use = "a limit witness is the magnitude schema validation established; dropping it \
              discards the only admitted bound for its family"]
pub struct LimitWitness<L: EvidenceSelectedLimit> {
    max: usize,
    _family: PhantomData<L>,
}

impl<L: EvidenceSelectedLimit> LimitWitness<L> {
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

/// How admitting one evidence-selected magnitude as a capacity refuses.
///
/// Single cause, because there is exactly one question to ask:
/// a magnitude that admits an item has nothing left to fail.
/// A plain root enum — the refusal-family binding is implemented by the
/// refusal home, pointing downward.
#[must_use = "a refusal carries the lawful reason the admission did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapacityAdmission {
    /// The witnessed magnitude admits no item at all.
    NotInhabited,
}

/// Evidence that one limit family's evidence-selected magnitude admits at
/// least one item — the runtime rung of the ladder [`PositiveLimit`] holds at
/// compile time.
///
/// Holding one establishes three facts:
/// the family (carried in the contained witness's type parameter), a positive
/// capacity, and the admitted runtime maximum read back by
/// [`max`](Self::max).
/// Containment follows [`PositiveLimit`]'s doctrine:
/// the one field is the [`LimitWitness`] schema validation minted, it is
/// private, and there is no road back to the bare witness.
///
/// # Nonclaims
///
/// It does not establish that the magnitude is right for its domain — that is
/// the owner profile's and the evidence's to select, and reading a held
/// witness as "this capacity is appropriate" reads a claim nobody made.
/// It carries no ceiling fact either: a magnitude that does not exist until
/// runtime has nothing to compare against a declared ceiling.
#[must_use = "a positive limit witness is the evidence a family's evidence-selected magnitude \
              admits an item; dropping it discards the only proof a runtime road promising an \
              inhabitant may act on"]
pub struct PositiveLimitWitness<L: EvidenceSelectedLimit> {
    witness: LimitWitness<L>,
}

impl<L: EvidenceSelectedLimit> PositiveLimitWitness<L> {
    /// Admit one evidence-selected magnitude as a capacity that holds an item.
    ///
    /// The bound is [`EvidenceSelectedLimit`] rather than [`Limit`], and that
    /// is the gate:
    /// a family whose owner never declared its magnitude evidence-selected has
    /// no road here.
    /// It takes the witness by value and keeps it — a road that borrowed the
    /// selection and copied the number out would leave the caller holding a
    /// second value carrying the same magnitude under weaker evidence.
    /// It refuses rather than refusing to compile because the magnitude does
    /// not exist until the evidence selects it; where the same relation is
    /// visible in the source, [`PositiveLimit::inhabited_under_profile`] is
    /// the seat and settles it at compile time.
    ///
    /// # Errors
    ///
    /// Returns [`CapacityAdmission::NotInhabited`] when the witnessed
    /// magnitude admits no item at all.
    pub fn inhabited(witness: LimitWitness<L>) -> Result<Self, CapacityAdmission> {
        if witness.max() >= 1 {
            Ok(Self { witness })
        } else {
            Err(CapacityAdmission::NotInhabited)
        }
    }
}

impl<L: EvidenceSelectedLimit> PositiveLimitWitness<L> {
    /// The witnessed maximum this witness carries; at least one by
    /// construction.
    ///
    /// Read off the contained base witness, so no second copy of the
    /// magnitude stands here to disagree with the one schema validation
    /// selected.
    #[must_use]
    pub fn max(&self) -> usize {
        self.witness.max()
    }
}

/// The construction refusal for bounded collections.
/// A plain root enum — the refusal-family binding is implemented by the
/// refusal home, pointing downward.
#[must_use = "a refusal carries the lawful reason the construction did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundedConstruction {
    /// The supplied items exceed the limit family's admitted maximum.
    OverLimit,
}

/// A collection that structurally carries which limit family bounds it.
/// There is no public unbounded collection anywhere in the machine; the
/// constructors here are the enforcement seams of that law.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bounded<T, L: Limit> {
    items: Vec<T>,
    _family: PhantomData<L>,
}

impl<T, L: ConstLimit> Bounded<T, L> {
    /// The fixed-arity collection: a total structural constructor whose
    /// "fits the bound" proof is discharged at compile time.
    ///
    /// The item count is `N`, so the `const` block decides the whole question
    /// before the program runs and this road has no refusal to return —
    /// where the material is known statically, a caller has no runtime
    /// failure to invent a value for.
    ///
    /// # Nonclaims
    ///
    /// It proves that `N` items fit under a type-level maximum, and nothing
    /// wider: no profile is involved, so it never claims the family's
    /// magnitude was admitted — [`Bounded::admitted_const`] is that road.
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

    /// Checked construction against the family's admitted compile-time
    /// maximum.
    ///
    /// `L::MAX` is whatever its author wrote; this road reads its bound off
    /// the [`AdmittedLimit`] witness instead, so the number it compares
    /// against stood under a named plane's ceiling.
    /// The profile rides on the witness and not on the collection: admitting
    /// a family under one plane never stamps that plane onto the returned
    /// value.
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
    /// The empty collection: a total structural constructor.
    ///
    /// A checked constructor reads a runtime count against a declared bound
    /// and may refuse — its name carries `admitted` because admission is what
    /// it performs.
    /// A total structural constructor cannot form the failing case: no limit
    /// family admits fewer than zero items, so this road has no refusal to
    /// return.
    /// It never reads `L::MAX`, which is why it is bounded by `L: Limit`
    /// alone and why a `MAX = 0` family inhabits it lawfully — the empty-only
    /// seat is a real seat.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            items: Vec::new(),
            _family: PhantomData,
        }
    }

    /// Checked construction against a schema-minted runtime witness of the
    /// same limit family — a witness for another family does not typecheck.
    ///
    /// No profile is involved and none could be: the magnitude was
    /// established at runtime, so there is no `L::MAX` to admit, and a
    /// profile-scoped admission is not evidence for this road.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the items exceed the
    /// witnessed maximum.
    pub fn admitted(items: Vec<T>, witness: &LimitWitness<L>) -> Result<Self, BoundedConstruction>
    where
        L: EvidenceSelectedLimit,
    {
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

    /// Read the held values.
    /// Read-only by construction: the collection is borrowed, not consumed,
    /// and no mutable or positional road exists beside this one — no
    /// `iter_mut`, no `Index`, no slice escape.
    ///
    /// # Ordering
    ///
    /// Iteration order may influence semantic meaning only where the owner
    /// type explicitly declares ordering as semantic; identity-bearing
    /// generation over order-insensitive collections canonicalizes by an
    /// owner-declared order or key first.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

/// The construction refusal for non-empty bounded collections.
/// Emptiness is not a cause: the constructor signature takes the first item
/// separately, so a zero-item value is unrepresentable rather than refused.
#[must_use = "a refusal carries the lawful reason the construction did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonEmptyBoundedConstruction {
    /// The supplied items exceed the limit family's admitted maximum.
    OverLimit,
}

/// A bounded collection that structurally holds at least one item — a refusal
/// with zero issues is not a refusal, and this shape makes that
/// unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyBounded<T, L: Limit> {
    first: T,
    rest: Vec<T>,
    _family: PhantomData<L>,
}

impl<T, L: ConstLimit> NonEmptyBounded<T, L> {
    /// Checked construction against the family's admitted and positive
    /// compile-time maximum.
    /// The first item is a separate parameter — emptiness is unrepresentable,
    /// not refused.
    ///
    /// This road takes the stronger witness because its signature promises an
    /// inhabitant: a `MAX = 0` family can never satisfy that promise, so an
    /// [`AdmittedLimit`] — which admits zero-maximum families on purpose — is
    /// not enough evidence here, and the bound is read off [`PositiveLimit`].
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

    /// Carry as many supplied items as the family's admitted maximum holds,
    /// and hand back how many it could not carry.
    ///
    /// A checked constructor refuses the whole value, which is wrong for a
    /// report: the issues an over-bound pass established are each true on
    /// their own, so this road carries the prefix the admitted magnitude
    /// holds and reports the dropped count beside it.
    ///
    /// It is crate-internal because the carry and the count must never be
    /// pairable with anything else:
    /// a body truncated by one pass wearing the count another pass dropped
    /// would be two honest values and one lie.
    /// Its one consumer is [`crate::refusal::AdmittedPrefix`], which takes
    /// both in the construction that produced them and never lets them apart.
    /// It is total because the witness is [`PositiveLimit`]: the maximum is at
    /// least one, so the first item always fits and the prefix is never
    /// empty.
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

    /// The fixed-arity collection: a total structural constructor whose
    /// "at least one" and "fits the bound" proofs are both discharged at
    /// compile time.
    ///
    /// The first item is a separate parameter, so emptiness is
    /// unrepresentable, and the remainder's arity is `N`, so the `const`
    /// block settles the bound question before the program runs.
    ///
    /// # Nonclaims
    ///
    /// Both proofs are facts about this call; neither claims the family's
    /// magnitude was admitted — [`NonEmptyBounded::admitted_const`] is that
    /// road.
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

    /// The one-item collection: a total structural constructor whose
    /// "at least one" proof is discharged at compile time.
    ///
    /// The only way a single item could exceed a family's maximum is a
    /// `MAX = 0` family, and the `const` block rejects that instantiation at
    /// const evaluation — a `const` item refuses under `cargo check`, a
    /// function-body call refuses at codegen — so the failing case never
    /// reaches a running program, and a caller assembling a one-issue refusal
    /// body has no impossible error branch to fabricate a value for.
    ///
    /// # Nonclaims
    ///
    /// The proof is local positivity off the declaration; it claims nothing
    /// about admission under any plane's ceiling — [`PositiveLimit`] seats
    /// both facts together for the road that needs both.
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
    /// Checked construction against a schema-minted runtime witness that
    /// admits an item.
    ///
    /// [`Bounded::admitted`]'s magnitude authority, with
    /// [`NonEmptyBounded::admitted_const`]'s evidence bar.
    /// It takes the stronger runtime witness because its signature promises
    /// an inhabitant: a zero selection would refuse every call, so a bare
    /// [`LimitWitness`] — which admits a zero selection on purpose for the
    /// empty-only seat — is not enough evidence here.
    /// The separate first parameter makes emptiness unrepresentable in the
    /// result; the magnitude promise still needs the evidence.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyBoundedConstruction::OverLimit`] when the total item
    /// count exceeds the witnessed maximum.
    pub fn admitted(
        first: T,
        rest: Vec<T>,
        witness: &PositiveLimitWitness<L>,
    ) -> Result<Self, NonEmptyBoundedConstruction>
    where
        L: EvidenceSelectedLimit,
    {
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

    /// Always `false`: the shape holds at least one item.
    /// Present because the `len`/`is_empty` pair is conventional; the
    /// constant answer *is* the law.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Read the held values, the guaranteed first item ahead of the rest.
    /// Read-only by construction: the collection is borrowed, not consumed,
    /// and no mutable or positional road exists beside this one — no
    /// `iter_mut`, no `Index`, no slice escape.
    ///
    /// # Ordering
    ///
    /// Iteration order may influence semantic meaning only where the owner
    /// type explicitly declares ordering as semantic; identity-bearing
    /// generation over order-insensitive collections canonicalizes by an
    /// owner-declared order or key first.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        core::iter::once(&self.first).chain(self.rest.iter())
    }
}

// ---------------------------------------------------------------------------
// Freshness: root-admitted axis. Current and Stale are TYPES, not variants — an API
// that requires fresh input demands `Current<T>` and stale data does not typecheck.
// ---------------------------------------------------------------------------

/// The invalidation coordinate of one claim family.
/// Every family implements this with its *own* coordinate; there is no
/// universal cut, and a flattened anything-freshness substrate is refused by
/// construction.
pub trait EvidenceCut {}

/// The uninhabited coordinate: a claim family with no admitted invalidation
/// coordinate parameterizes over [`Never`], which makes its `Stale` form
/// unrepresentable rather than runtime-checked.
pub enum Never {}

impl EvidenceCut for Never {}

/// A value proven applicable now.
/// Minted only by the evidence-producing boundary; there is no public
/// constructor, so holding a `Current<T>` *is* the proof.
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
/// against.
/// Staleness changes present admissibility, never the earlier claim's truth.
/// Returning to [`Current`] happens only through a named re-assessment
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
/// assessment has not yet branched.
/// The types are primary; this sum only carries them to the branch point.
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

/// What one verification run established about one claim.
/// Its own axis — never a terminal variant, never a rank, and (by decision)
/// not a knowledge axis: no variant means "not yet".
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

/// Completeness over an owner-specific domain.
/// The domain parameter is non-erasable, so a complete query can never
/// masquerade as complete verification.
/// Owners instantiate it under their own names — source closure in
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

/// Whether a referent is currently reachable.
/// A non-identifying runtime fact carried on a reference.
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
/// (non-identifying runtime facts).
/// The claim marker is defined by the owner making the claim, never
/// centrally.
/// This is a reference, not a container — no value comes out of it.
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

/// One dispatch outcome.
/// Generic over the owner's refusal family — the root grammar names no
/// concrete refusal type.
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

/// The transition-system closure bar.
/// A conformance contract, not a universal state type: each machine
/// implements it and proves six obligations — exact initial posture; every
/// declared state reachable; every transition naming exact source,
/// destination, and firing input; terminals that no transition leaves;
/// deterministic dispatch or declared ambiguity with a named resolution; and
/// total typed refusal for every unmatched pair.
/// Closure is evidenced, not asserted: the machine and its judge never share
/// the dispatch path being judged.
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

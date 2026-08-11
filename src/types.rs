//! The root shape calculus: the generic composition shapes every home instantiates,
//! plus the two axes admitted to root by explicit ruling. Nothing here is a semantic
//! noun beyond those two — a semantic noun lives at root only by an explicit root
//! admission ruling from the repository owner.
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
//! By explicit ruling, [`Freshness`] and [`ProofDisposition`] are *evidence facts*,
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

/// A runtime magnitude for the limit family `L`, minted only by schema validation.
/// Carrying the family as a type parameter keeps runtime-limited and compile-limited
/// values in the same shape without confusing their authorities.
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
    /// Checked construction against the family's compile-time maximum.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the items exceed
    /// `L::MAX`.
    pub fn admitted_const(items: Vec<T>) -> Result<Self, BoundedConstruction> {
        if items.len() <= L::MAX {
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
    /// Checked construction against the family's compile-time maximum. The
    /// first item is a separate parameter — emptiness is unrepresentable, not
    /// refused.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyBoundedConstruction::OverLimit`] when the total item
    /// count exceeds `L::MAX`.
    pub fn admitted_const(first: T, rest: Vec<T>) -> Result<Self, NonEmptyBoundedConstruction> {
        if rest.len().saturating_add(1) <= L::MAX {
            Ok(Self {
                first,
                rest,
                _family: PhantomData,
            })
        } else {
            Err(NonEmptyBoundedConstruction::OverLimit)
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
    /// Owed reversal (red twin): a trybuild fixture instantiating this road
    /// under a family declaring `MAX = 0` must fail to compile.
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
/// terminal variant, never a rank, and (by ruling) not a knowledge axis: no variant
/// means "not yet".
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

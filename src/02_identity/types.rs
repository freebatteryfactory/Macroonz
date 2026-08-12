//! The identity class calculus: the six classes, the two-column law, the
//! derived-seat law, and the scope guards. This home owns the *shapes*; every
//! concrete identity in the machine lives with its owner home and instantiates
//! them. There is one class law, not one register document.
//!
//! # The two-column law
//!
//! Every identity binds two independent columns: its **class** (which question it
//! answers) and its **creation law** (how an instance is minted). The class never
//! implies the creation law — an identity is designed by classification plus one
//! named minting rule, never by taste.
//!
//! # The derived-seat law (Class D gate)
//!
//! Derived minting is admitted only where it earns two seats: a named consumer of
//! convergence (replay, retry, or an independent route that must re-derive the
//! same identity without coordination), and preimage custody (every preimage
//! input admitted or owned by the minting authority). Where either seat is empty,
//! the identity is fresh — an absent preimage is a design answer, not a gap,
//! because fresh minting makes the computed-identity attack class unrepresentable
//! rather than defended. Class A/B commitments are computable by design — that is
//! their question — under their own guards: domain-tagged preimages, and
//! keyed-when-protected where the meaning is protected.
//!
//! # Seams and envelopes
//!
//! Internal seams speak refusal *family bodies* (like [`OrderComparison`]); the
//! universal envelope is the publication form, minted only where reasons are
//! registered. Canonical refusal and released refusal are different projections
//! of one fact — never two facts.

use crate::refusal::{FamilyShape, RefusalFamily};
use core::cmp::Ordering;
use core::marker::PhantomData;

/// Which question an identity answers. One of six, closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentityClass {
    /// Class A — "does this mean the same thing?"
    SemanticCommitment,
    /// Class B — "are these the same bytes?" Never substitutable for Class A.
    ByteDigest,
    /// Class C — "what position in one authority's order?"
    AuthorityOrder,
    /// Class D — "which happening?" — identity of an occurrence, not of content.
    Occurrence,
    /// Class E — "which referent, at which version?"
    TypedReference,
    /// Class F — an application-composed scope. The machine mints none.
    ApplicationScope,
}

/// How an instance is minted. Independent of class by law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CreationLaw {
    /// A domain-tagged digest of normalized meaning (Class A's computable law).
    DomainTaggedDigestOfMeaning,
    /// A digest of exact bytes (Class B's computable law).
    DigestOfExactBytes,
    /// Assigned by exactly one writer authority within one scope.
    AssignedByOneAuthority,
    /// Deterministically derived from an admitted, custodied preimage — lawful
    /// only under the derived-seat law's two seats.
    DerivedFromAdmittedPreimage,
    /// Fresh and opaque under a qualified minting profile; a reader parses no
    /// structure from the bytes.
    FreshOpaque,
    /// Bound pointer construction (Class E).
    BoundPointerConstruction,
    /// Composed by the application under a canonical composition normal form
    /// (Class F; the normal form is authored where `KeyScope` lands).
    ApplicationComposed,
}

impl CreationLaw {
    /// The identity class this creation law names in its OWN declaration, where
    /// it names one.
    ///
    /// Four of the seven creation laws are declared as one class's law and say
    /// so above: the domain-tagged digest is Class A's, the exact-byte digest is
    /// Class B's, bound pointer construction is Class E's, and application
    /// composition is Class F's. The remaining three name no class and are open
    /// to any — a fresh occurrence identity and a fresh schema family identity
    /// are the same creation law under two different questions.
    ///
    /// This reads in exactly one direction, and the direction matters. The
    /// two-column law says the CLASS never implies the creation law, and that
    /// stands untouched: [`IdentityClass::Occurrence`] admits derived minting
    /// and fresh minting both, which is why the columns are independent. What
    /// this answers is the other direction, for the four laws whose own
    /// declaration is class-specific.
    #[must_use]
    pub const fn declared_class(self) -> Option<IdentityClass> {
        match self {
            Self::DomainTaggedDigestOfMeaning => Some(IdentityClass::SemanticCommitment),
            Self::DigestOfExactBytes => Some(IdentityClass::ByteDigest),
            Self::BoundPointerConstruction => Some(IdentityClass::TypedReference),
            Self::ApplicationComposed => Some(IdentityClass::ApplicationScope),
            Self::AssignedByOneAuthority
            | Self::DerivedFromAdmittedPreimage
            | Self::FreshOpaque => None,
        }
    }
}

/// The two-column declaration every concrete identity makes: machine-readable
/// law, joined by tooling against the owner's README and (later) derived by the
/// macros crate rather than hand-written.
pub trait IdentityRole {
    /// Which question this identity answers.
    const CLASS: IdentityClass;
    /// How instances are minted.
    const CREATION: CreationLaw;
}

/// How admitting one identity role's two-column declaration refuses.
///
/// One inhabited cause, so no cause-selection rule is owed. It is a single-cause
/// family rather than a collection because there is exactly one join to run.
#[must_use = "an admission refusal carries the established reason a role's declaration was not \
              admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentityRoleAdmission {
    /// The declared creation law names one class in its own declaration, and
    /// the declared class is a different one.
    NotClassCoherent,
}

impl RefusalFamily for IdentityRoleAdmission {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["NotClassCoherent"];
}

/// Evidence that one identity role's two-column declaration was admitted.
///
/// # Why a declaration is not yet a machine fact
///
/// [`IdentityRole`] is an extension point: every concrete identity in the
/// machine declares its own two columns, and nothing in the type system makes
/// the pair coherent. A road that reads either constant and acts on it is
/// trusting two declarations nobody joined. This witness is that join, and it is
/// opaque and constructor-free, so holding one *is* the evidence.
///
/// # What the mint establishes, and the claim ceiling
///
/// [`admitted`](Self::admitted) runs the one join the home's own declarations
/// support: where the declared creation law names a class in its own
/// declaration — see [`CreationLaw::declared_class`] — the declared class must
/// be that class.
///
/// That is a narrow claim and it is stated narrowly. Admission establishes
/// NOTHING about the three class-open creation laws: a role declaring
/// `AssignedByOneAuthority`, `DerivedFromAdmittedPreimage`, or `FreshOpaque`
/// passes this join under any class, because the two-column law says the class
/// does not imply the creation law and there is no declared fact to join
/// against. It establishes nothing about the derived-seat law's two seats — a
/// named consumer of convergence and preimage custody are facts about a
/// deployment's design, not about a pair of constants, and no road here can see
/// them. And it establishes nothing about any minter's conduct: whether a
/// concrete mint actually follows the creation law it declared is a behavioral
/// claim, it is owed, and it opens when minters exist.
#[must_use = "an admitted role is the evidence a two-column declaration passed its join; \
              dropping it discards the only proof a road may act on that declaration"]
pub struct AdmittedIdentityRole<T: IdentityRole> {
    columns: AdmittedIdentityColumns,
    _role: PhantomData<T>,
}

impl<T: IdentityRole> AdmittedIdentityRole<T> {
    /// Admit one role's two-column declaration.
    ///
    /// # Errors
    ///
    /// Returns [`IdentityRoleAdmission::NotClassCoherent`] when the declared
    /// creation law names a class and the declared class is a different one.
    pub fn admitted() -> Result<Self, IdentityRoleAdmission> {
        match T::CREATION.declared_class() {
            Some(named) if named != T::CLASS => Err(IdentityRoleAdmission::NotClassCoherent),
            _ => Ok(Self {
                columns: AdmittedIdentityColumns {
                    class: T::CLASS,
                    creation: T::CREATION,
                },
                _role: PhantomData,
            }),
        }
    }
}

/// One ADMITTED identity role's two columns, read as a value with the role
/// erased.
///
/// # The reification is reachable only from the witness
///
/// The columns are trait constants: anything can read `T::CLASS` directly, and
/// nothing stops it. What this type is for is the road in the other direction —
/// turning the declaration into a VALUE that travels, that a projection can
/// carry, that a diagnostic can name — and that road demands the admission
/// witness. A declaration that has not passed its join never becomes a value
/// the machine passes around.
///
/// # Why the name says admitted, and why the role parameter goes
///
/// This is the projection that ERASES `T`, and a name is owed to what a value
/// actually is rather than to where it came from. Every one of these was read
/// off an [`AdmittedIdentityRole`] — there is no other road — so what a reader
/// holds is admitted columns, and the name says so. The role parameter is gone
/// on purpose: once read, the two columns ARE the facts, and a reader deciding
/// by them is deciding by the declaration rather than by which Rust type
/// declared it. `AdmittedIdentityRole<T>` keeps both its name and its `T`,
/// because a witness is exactly a statement about one role; the erasing
/// projection is the thing that stopped being about one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdmittedIdentityColumns {
    class: IdentityClass,
    creation: CreationLaw,
}

impl AdmittedIdentityColumns {
    /// Read one admitted role's two columns.
    ///
    /// The columns come off the WITNESS rather than off a fresh read of the
    /// trait, so what travels is the reading admission actually joined. A second
    /// read here would be a second value that could disagree with the one the
    /// join ran over.
    #[must_use]
    pub fn of<T: IdentityRole>(admitted: &AdmittedIdentityRole<T>) -> Self {
        admitted.columns
    }

    /// Which question this identity answers.
    #[must_use]
    pub const fn class(self) -> IdentityClass {
        self.class
    }

    /// How instances are minted.
    #[must_use]
    pub const fn creation(self) -> CreationLaw {
        self.creation
    }
}

/// Class A shape: an opaque domain-tagged commitment over normalized meaning.
/// The domain is a type parameter, so commitments from different domains never
/// unify at compile time; the domain tag also lives in the preimage, so they
/// never collide at runtime either.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Commitment<Domain> {
    bytes: [u8; 32],
    _domain: PhantomData<Domain>,
}

impl<Domain> Commitment<Domain> {
    /// In-crate mint for laws. Test-gated until digest derivation exists.
    #[cfg(test)]
    pub(crate) const fn raw(bytes: [u8; 32]) -> Self {
        Self {
            bytes,
            _domain: PhantomData,
        }
    }

    /// The declared raw-byte storage order of this identity.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

/// Class B shape: an opaque digest of exact bytes, role-tagged so different
/// byte-digest roles never unify. A different question than [`Commitment`],
/// never substitutable for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ByteIdentity<Role> {
    bytes: [u8; 32],
    _role: PhantomData<Role>,
}

impl<Role> ByteIdentity<Role> {
    /// In-crate mint for laws. Test-gated until digest derivation exists.
    #[cfg(test)]
    pub(crate) const fn raw(bytes: [u8; 32]) -> Self {
        Self {
            bytes,
            _role: PhantomData,
        }
    }

    /// The declared raw-byte storage order of this identity.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

/// The single-cause family for order comparison: one dependent check. Comparing
/// positions across scopes is a category error, not a runtime edge case — the
/// lawful cross-scope expression is a cut vector. Its treatment is do-not-retry:
/// repeating the same comparison is unlawful.
#[must_use = "a comparison refusal carries the lawful reason two positions were not ranked"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderComparison {
    /// The two positions do not share one scope.
    NotSameScope,
}

impl RefusalFamily for OrderComparison {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["NotSameScope"];
}

/// Class C shape: one position in one authority's order, carrying its scope
/// binding in the value itself. There is no `Ord` and no `PartialOrd` — `a < b`
/// does not typecheck; the only comparison is [`Self::try_cmp_same_scope`], and
/// cross-scope order is a cut vector, never integers. The scope is generic and
/// may be a tuple (two-part scopes exist).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthorityPosition<Scope> {
    scope: Scope,
    position: u64,
}

impl<Scope: Eq> AuthorityPosition<Scope> {
    /// The authority-side mint: assigned by exactly one writer authority.
    /// Test-gated until an owner home carries the real admission path — the gate
    /// comes off the moment a lawful minter exists, never before.
    #[cfg(test)]
    pub(crate) const fn assigned(scope: Scope, position: u64) -> Self {
        Self { scope, position }
    }

    /// The one lawful comparison: total within one scope, refused across scopes.
    ///
    /// # Errors
    ///
    /// Returns the [`OrderComparison`] family body when the two positions do not
    /// share one scope.
    pub fn try_cmp_same_scope(&self, other: &Self) -> Result<Ordering, OrderComparison> {
        if self.scope == other.scope {
            Ok(self.position.cmp(&other.position))
        } else {
            Err(OrderComparison::NotSameScope)
        }
    }
}

/// The two byte forms of a Class-D identity, per the class byte law: a derived
/// occurrence is a 32-byte domain-tagged preimage digest; a fresh occurrence is
/// 16 entropy bytes with no meaning in the bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OccurrenceForm {
    /// Derived: 32-byte domain-tagged preimage digest (the derived-seat law's
    /// two seats earned).
    Derived([u8; 32]),
    /// Fresh: 16 opaque entropy bytes. Which layout a deployment mints them
    /// under is host and admission policy; the reader contract binds either
    /// way — no reader parses structure out of these bytes.
    Fresh([u8; 16]),
}

/// Class D shape: an opaque occurrence identity in one of the two lawful byte
/// forms. A reader parses no structure from the bytes regardless of form;
/// whether a role mints derived or fresh is its declared creation law, gated by
/// the derived-seat law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Occurrence<Role> {
    form: OccurrenceForm,
    _role: PhantomData<Role>,
}

impl<Role> Occurrence<Role> {
    /// In-crate mint for laws. Test-gated until lawful minters exist.
    #[cfg(test)]
    pub(crate) const fn for_laws(form: OccurrenceForm) -> Self {
        Self {
            form,
            _role: PhantomData,
        }
    }

    /// The identity's byte form.
    #[must_use]
    pub fn form(&self) -> &OccurrenceForm {
        &self.form
    }
}

/// Class E shape: a typed reference identified by exactly its referent and
/// version. Availability and integrity are runtime facts carried alongside a
/// reference, never inside its identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedRef<To> {
    referent: To,
    version: u64,
}

impl<To> TypedRef<To> {
    /// Bound pointer construction. Test-gated until an owner home carries the
    /// real admission path — the gate comes off the moment a lawful minter
    /// exists, never before.
    #[cfg(test)]
    pub(crate) const fn bound(referent: To, version: u64) -> Self {
        Self { referent, version }
    }

    /// The referent this reference is bound to.
    #[must_use]
    pub fn referent(&self) -> &To {
        &self.referent
    }

    /// The referent version this reference is bound to.
    #[must_use]
    pub fn version(&self) -> u64 {
        self.version
    }
}

/// Class F contract: an application-composed scope under a canonical composition
/// normal form. The machine mints none; the normal form is authored where
/// `KeyScope` lands (the authority home), which Class A's keyed-when-protected
/// rule depends on.
pub trait ApplicationScope {}

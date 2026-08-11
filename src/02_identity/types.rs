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

/// The two-column declaration every concrete identity makes: machine-readable
/// law, joined by tooling against the owner's README and (later) derived by the
/// macros crate rather than hand-written.
pub trait IdentityRole {
    /// Which question this identity answers.
    const CLASS: IdentityClass;
    /// How instances are minted.
    const CREATION: CreationLaw;
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

/// The three fresh-minting layout families the old book names for Class-D fresh
/// identities. Which family a deployment uses is admission policy selected by
/// evidence; the reader contract holds regardless: 16 opaque bytes, and no
/// reader may parse structure out of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MintingProfile {
    /// All sixteen bytes from admitted entropy.
    FullyRandom,
    /// A time-prefixed layout (structure exists; readers still parse none).
    TimePrefixed,
    /// A writer-counter layout (structure exists; readers still parse none).
    WriterCounter,
}

/// The two byte forms of a Class-D identity, per the class byte law: a derived
/// occurrence is a 32-byte domain-tagged preimage digest; a fresh occurrence is
/// 16 entropy bytes with no meaning in the bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OccurrenceForm {
    /// Derived: 32-byte domain-tagged preimage digest (the derived-seat law's
    /// two seats earned).
    Derived([u8; 32]),
    /// Fresh: 16 opaque entropy bytes under an admitted [`MintingProfile`].
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

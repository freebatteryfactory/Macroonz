//! Public contract types.

use core::marker::PhantomData;
use core::num::NonZeroUsize;

#[path = "type_guard.rs"]
mod guard;

/// The authority that supplies a limit family's capacity.
pub trait CapacityAuthority {}

/// The authority of a magnitude declared in source.
pub enum DeclaredMagnitude {}

impl CapacityAuthority for DeclaredMagnitude {}

/// The authority of a limit family with no declared capacity road.
pub enum UnstatedMagnitude {}

impl CapacityAuthority for UnstatedMagnitude {}

/// A type-level family that identifies which limit governs a bounded value.
pub trait Limit {
    /// The authority that supplies this family's capacity.
    type Authority: CapacityAuthority;
}

/// A limit family whose magnitude is declared in source.
pub trait ConstLimit: Limit<Authority = DeclaredMagnitude> {
    /// The maximum number of items this family admits.
    const MAX: usize;
}

/// A profile that admits compile-time magnitudes under one ceiling.
pub trait LimitAdmissionProfile {
    /// The widest declared magnitude admitted by this profile.
    const MAX_DECLARED_LIMIT: usize;
}

/// Evidence that one declared limit stands under one profile's ceiling.
#[must_use = "an admitted limit is the evidence that a declared magnitude passed admission"]
pub struct AdmittedLimit<L: Limit, P: LimitAdmissionProfile> {
    max: usize,
    _family: PhantomData<L>,
    _profile: PhantomData<P>,
}

impl<L: ConstLimit, P: LimitAdmissionProfile> AdmittedLimit<L, P> {
    /// Admits one compile-time magnitude under one profile's ceiling.
    ///
    /// The comparison is evaluated at compile time, so this constructor has no runtime refusal.
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
    /// The admitted maximum carried by this witness.
    #[must_use]
    pub const fn max(&self) -> usize {
        self.max
    }
}

/// Evidence that one admitted declared limit can hold at least one item.
#[must_use = "a positive limit is the evidence that an admitted declared magnitude can hold an item"]
pub struct PositiveLimit<L: Limit, P: LimitAdmissionProfile> {
    admitted: AdmittedLimit<L, P>,
}

impl<L: ConstLimit, P: LimitAdmissionProfile> PositiveLimit<L, P> {
    /// Admits one compile-time magnitude and establishes that it can hold an item.
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
    /// The admitted maximum carried by this witness.
    #[must_use]
    pub const fn max(&self) -> usize {
        self.admitted.max()
    }
}

/// How construction of a bounded collection refuses.
#[must_use = "a bounded construction refusal names why construction did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundedConstruction {
    /// The supplied items exceed the admitted maximum.
    OverLimit,
}

/// A collection that carries its governing limit family in its type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bounded<T, L: Limit> {
    items: Vec<T>,
    _family: PhantomData<L>,
}

impl<T, L: ConstLimit> Bounded<T, L> {
    /// Constructs a fixed-arity collection whose bound is checked at compile time.
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

    /// Constructs a collection under an admitted compile-time magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedConstruction::OverLimit`] when the supplied items exceed the admitted maximum.
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
    /// Constructs the empty collection without consulting a magnitude.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            items: Vec::new(),
            _family: PhantomData,
        }
    }

    /// Returns the number of held items.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Reports whether this collection holds no item.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterates over the held items without exposing a mutable or owned collection.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

/// How construction of a non-empty bounded collection refuses.
#[must_use = "a non-empty bounded construction refusal names why construction did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonEmptyBoundedConstruction {
    /// The supplied items exceed the admitted maximum.
    OverLimit,
}

/// A bounded collection that structurally contains at least one item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyBounded<T, L: Limit> {
    first: T,
    rest: Vec<T>,
    _family: PhantomData<L>,
}

impl<T, L: ConstLimit> NonEmptyBounded<T, L> {
    /// Constructs a non-empty collection under an admitted positive compile-time magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyBoundedConstruction::OverLimit`] when the total item count exceeds the admitted maximum.
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

    fn admitted_prefix<P: LimitAdmissionProfile>(
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

    /// Constructs a fixed-arity non-empty collection whose bound is checked at compile time.
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

    /// Constructs a one-item collection whose positivity is checked at compile time.
    #[must_use]
    pub const fn singleton(value: T) -> Self {
        const {
            assert!(L::MAX >= 1, "a limit family admitting no item at all");
        }
        Self {
            first: value,
            rest: Vec::new(),
            _family: PhantomData,
        }
    }
}

impl<T, L: Limit> NonEmptyBounded<T, L> {
    /// Returns the guaranteed first item.
    #[must_use]
    pub const fn first(&self) -> &T {
        &self.first
    }

    /// Returns the number of held items.
    #[must_use]
    #[expect(
        clippy::len_without_is_empty,
        reason = "this collection structurally contains a first item, so an emptiness query would have only one answer"
    )]
    pub fn len(&self) -> usize {
        self.rest.len().saturating_add(1)
    }

    /// Iterates over the guaranteed first item followed by the remaining items.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        core::iter::once(&self.first).chain(self.rest.iter())
    }
}

/// The stable opaque identity of one refusal reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReasonId([u8; 32]);

impl ReasonId {
    /// Returns the identity's declared raw-byte storage order.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// The declared bound that stopped an enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StopBound {
    /// The declared issue bound stopped enumeration.
    DeclaredIssueBound,
    /// The declared work bound stopped enumeration.
    DeclaredWorkBound,
}

/// The exact established-issue count omitted by one bounded report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReportTruncation {
    stopped_at: StopBound,
    omitted: NonZeroUsize,
}

/// What one collection-shaped refusal body says about its coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompletionPosture {
    /// Every declared site was examined and every established issue is carried.
    Complete,
    /// Enumeration stopped before every declared site was examined.
    EarlyStopped {
        /// The bound that stopped enumeration.
        stopped_at: StopBound,
    },
    /// Every declared site was examined and the body omits an exact established-issue count.
    ReportTruncated(ReportTruncation),
}

/// A non-empty bounded refusal body bound to the coverage posture produced by the same construction.
#[must_use = "an admitted prefix carries established issues and the posture of the same construction"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdmittedPrefix<T, L: Limit> {
    carried: NonEmptyBounded<T, L>,
    completion: CompletionPosture,
}

/// The body shape declared by a refusal family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyShape {
    /// One cause selected from dependent checks.
    SingleCause,
    /// A bounded non-empty collection of independent issues.
    IssueCollection,
    /// Exactly two questions that have no lawful meaning apart.
    InseparablePair,
}

/// The stable identity of one refusal family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalFamilyId(&'static str);

impl RefusalFamilyId {
    /// Declares one stable family identity.
    #[must_use]
    pub const fn declared(identity: &'static str) -> Self {
        Self(identity)
    }

    /// Returns the declared identity.
    #[must_use]
    pub const fn as_declared(self) -> &'static str {
        self.0
    }
}

/// One cause's stable key within its family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalCauseKey(&'static str);

impl LocalCauseKey {
    /// Declares one family-local cause key.
    #[must_use]
    pub const fn declared(key: &'static str) -> Self {
        Self(key)
    }

    /// Returns the declared local key.
    #[must_use]
    pub const fn as_declared(self) -> &'static str {
        self.0
    }
}

/// The stable identity of one cause within one refusal family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauseId {
    family: RefusalFamilyId,
    local: LocalCauseKey,
}

impl CauseId {
    /// Declares one cause identity from its family and family-local key.
    #[must_use]
    pub const fn declared(family: RefusalFamilyId, local: LocalCauseKey) -> Self {
        Self { family, local }
    }

    /// Returns the family that declares this cause.
    #[must_use]
    pub const fn family(self) -> RefusalFamilyId {
        self.family
    }

    /// Returns this cause's family-local key.
    #[must_use]
    pub const fn local(self) -> LocalCauseKey {
        self.local
    }

    /// Projects this identity into its canonical text form.
    #[must_use]
    pub fn canonical_text(self) -> String {
        let mut text = String::from(self.family.as_declared());
        text.push('.');
        text.push_str(self.local.as_declared());
        text
    }
}

/// The typed zero-based position of one cause in a declared order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauseOrdinal(u16);

impl CauseOrdinal {
    /// Returns the zero-based position represented by this ordinal.
    #[must_use]
    pub const fn position(self) -> u16 {
        self.0
    }
}

/// One declared cause together with its current Rust spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredCause {
    id: CauseId,
    spelling: &'static str,
}

impl DeclaredCause {
    /// Declares one cause from its stable identity and Rust spelling.
    #[must_use]
    pub const fn declared(id: CauseId, spelling: &'static str) -> Self {
        Self { id, spelling }
    }

    /// Returns this cause's stable identity.
    #[must_use]
    pub const fn id(self) -> CauseId {
        self.id
    }

    /// Returns the Rust spelling that projects this cause identity.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        self.spelling
    }
}

/// The typed canonical order declared by one refusal family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredCauseOrder {
    causes: &'static [DeclaredCause],
}

impl DeclaredCauseOrder {
    /// Declares a canonical cause order, first cause first.
    #[must_use]
    pub const fn declared(causes: &'static [DeclaredCause]) -> Self {
        Self { causes }
    }

    /// Declares that a family has no canonical cause order.
    #[must_use]
    pub const fn none() -> Self {
        Self { causes: &[] }
    }

    /// Returns the number of declared causes.
    #[must_use]
    pub const fn len(self) -> usize {
        self.causes.len()
    }

    /// Reports whether this declaration orders no causes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.causes.is_empty()
    }

    /// Iterates over the declared causes in canonical order.
    #[must_use]
    pub fn iter(self) -> impl ExactSizeIterator<Item = DeclaredCause> {
        self.causes.iter().copied()
    }

    /// Returns the ordinal of one cause identity in this order.
    #[must_use]
    pub fn ordinal_of(self, id: CauseId) -> Option<CauseOrdinal> {
        self.causes
            .iter()
            .position(|cause| cause.id == id)
            .and_then(|index| u16::try_from(index).ok())
            .map(CauseOrdinal)
    }

    /// Returns the cause identity at one ordinal.
    #[must_use]
    pub fn identity_at(self, ordinal: CauseOrdinal) -> Option<CauseId> {
        self.causes
            .get(usize::from(ordinal.position()))
            .map(|cause| cause.id)
    }

    /// Reports whether one textual order is exactly this typed order's projection.
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

/// The shape contract implemented by every refusal family.
///
/// Macroonz derives emit the textual selection order as an inherent projection beside the typed [`CauseOrderDeclaration::DECLARED_ORDER`].
pub trait RefusalFamily {
    /// The family's body shape.
    const SHAPE: FamilyShape;
}

/// The typed canonical cause order declared by one refusal family.
pub trait CauseOrderDeclaration: RefusalFamily {
    /// The family's canonical order over stable cause identities.
    const DECLARED_ORDER: DeclaredCauseOrder;
}

/// How admission of a typed refusal-family order refuses.
#[must_use = "a family admission refusal names why the typed declaration was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyAdmission {
    /// The declared shape and typed cause order contradict each other.
    NotShapeCoherent,
    /// The typed cause order does not project to the textual selection order.
    NotProjected,
}

/// The joins established by one refusal-family admission witness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[expect(
    clippy::enum_variant_names,
    reason = "each variant names the exact admission coverage it witnesses, and the shared prefix is the relationship this roster distinguishes"
)]
pub enum FamilyAdmissionCoverage {
    /// The family declares one of the closed body shapes.
    ShapeCoherence,
    /// The family shape and typed cause order were found coherent.
    ShapeCoherenceAndTypedOrder,
    /// Shape coherence and typed-to-text order projection were both established.
    ShapeCoherenceAndOrderProjection,
}

mod sealed {
    #[expect(
        unnameable_types,
        reason = "the sealed supertrait is deliberately unnameable outside this crate so downstream code cannot declare admission coverage"
    )]
    pub trait Sealed {}
}

/// The type-level coverage of shape admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShapeCoherent;

/// The type-level coverage of typed-order admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedOrderCoherent;

/// The type-level coverage of typed-to-text projection admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderProjected;

/// The minimum coverage required by a shape-sensitive consumer.
pub trait ShapeAdmission: sealed::Sealed {
    /// The inspection projection of this type-level coverage.
    const INSPECTION: FamilyAdmissionCoverage;
}

/// The stronger coverage required by an order-sensitive consumer.
pub trait OrderAdmission: ShapeAdmission {}

impl sealed::Sealed for ShapeCoherent {}

impl ShapeAdmission for ShapeCoherent {
    const INSPECTION: FamilyAdmissionCoverage = FamilyAdmissionCoverage::ShapeCoherence;
}

impl sealed::Sealed for TypedOrderCoherent {}

impl ShapeAdmission for TypedOrderCoherent {
    const INSPECTION: FamilyAdmissionCoverage =
        FamilyAdmissionCoverage::ShapeCoherenceAndTypedOrder;
}

impl OrderAdmission for TypedOrderCoherent {}

impl sealed::Sealed for OrderProjected {}

impl ShapeAdmission for OrderProjected {
    const INSPECTION: FamilyAdmissionCoverage =
        FamilyAdmissionCoverage::ShapeCoherenceAndOrderProjection;
}

impl OrderAdmission for OrderProjected {}

/// Evidence that one refusal-family declaration passed the admission represented by `Coverage`.
#[must_use = "an admitted family is the evidence that its declaration passed the required joins"]
pub struct AdmittedRefusalFamily<F: RefusalFamily, Coverage: ShapeAdmission> {
    _family: PhantomData<F>,
    _coverage: PhantomData<Coverage>,
}

impl<F: RefusalFamily, Coverage: ShapeAdmission> AdmittedRefusalFamily<F, Coverage> {
    /// Returns this witness's inspection coverage.
    #[must_use]
    #[expect(
        clippy::unused_self,
        reason = "the receiver is the admission witness, while the answer is projected from its coverage parameter"
    )]
    pub const fn coverage(&self) -> FamilyAdmissionCoverage {
        Coverage::INSPECTION
    }
}

/// Admits one refusal family's closed body shape.
pub fn admit_shape<F: RefusalFamily>() -> AdmittedRefusalFamily<F, ShapeCoherent> {
    AdmittedRefusalFamily {
        _family: PhantomData,
        _coverage: PhantomData,
    }
}

/// Admits one refusal family's typed cause order.
///
/// # Errors
///
/// Returns [`FamilyAdmission::NotShapeCoherent`] when the declared shape and typed cause order contradict each other.
pub fn admit_order<F: CauseOrderDeclaration>()
-> Result<AdmittedRefusalFamily<F, TypedOrderCoherent>, FamilyAdmission> {
    if !shape_coheres::<F>() {
        return Err(FamilyAdmission::NotShapeCoherent);
    }
    Ok(AdmittedRefusalFamily {
        _family: PhantomData,
        _coverage: PhantomData,
    })
}

/// Admits a generated textual projection of one refusal family's typed cause order.
///
/// # Errors
///
/// Returns [`FamilyAdmission::NotShapeCoherent`] when the family shape and typed order contradict each other. Returns [`FamilyAdmission::NotProjected`] when the supplied textual projection differs from the typed declaration.
pub fn admit_order_projection<F: CauseOrderDeclaration>(
    textual: &[&str],
) -> Result<AdmittedRefusalFamily<F, OrderProjected>, FamilyAdmission> {
    if !shape_coheres::<F>() {
        return Err(FamilyAdmission::NotShapeCoherent);
    }
    if F::DECLARED_ORDER.projects_to(textual) {
        Ok(AdmittedRefusalFamily {
            _family: PhantomData,
            _coverage: PhantomData,
        })
    } else {
        Err(FamilyAdmission::NotProjected)
    }
}

fn shape_coheres<F: CauseOrderDeclaration>() -> bool {
    matches!(F::SHAPE, FamilyShape::SingleCause) != F::DECLARED_ORDER.is_empty()
}

impl<F: CauseOrderDeclaration, Coverage: OrderAdmission> AdmittedRefusalFamily<F, Coverage> {
    /// Returns the family's typed cause order to an order-admitted consumer.
    #[must_use]
    #[expect(
        clippy::unused_self,
        reason = "the receiver is the order-admission witness, while the order is read from the family declaration"
    )]
    pub const fn cause_order(&self) -> DeclaredCauseOrder {
        F::DECLARED_ORDER
    }
}

/// An opaque domain-tagged commitment over normalized meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Commitment<Domain> {
    bytes: [u8; 32],
    _domain: PhantomData<Domain>,
}

impl<Domain> Commitment<Domain> {
    /// Returns the commitment's declared raw-byte storage order.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

/// A field's declared cardinality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldCardinality {
    /// Exactly one value is present.
    Required,
    /// Zero or one value is present.
    Optional,
    /// Zero or more values are present.
    Repeated,
}

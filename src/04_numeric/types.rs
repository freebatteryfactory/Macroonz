//! The exact numeric families, their constructor-axis ladder, interval decisions,
//! quantization, rounding, and the numeric-honesty layer.
//!
//! # The frozen semantic requirements
//!
//! Scale explicit; unit explicit; range checked; scale increase exact when
//! representable; scale reduction requires an explicit rounding mode; overflow is
//! a typed refusal — no saturation, no silent truncation, no implicit unit
//! conversion; decimal source text parses directly into coefficient and scale and
//! never passes through binary floating point (`12.34` → coefficient `1234`,
//! scale `2`).
//!
//! # Authority
//!
//! Exactness is necessary for authority, never sufficient — the owning policy
//! grants it. Numeric confidence never becomes authority; approximation never
//! silently acquires authority; a float is never the authority default. No
//! constructor family is an authority gate: successful construction grants
//! nothing.
//!
//! # Dimensional arithmetic (law now, operations later)
//!
//! Add/subtract: same compatible unit only. Multiply: dimensional ×
//! dimensionless. Divide like dimensions → `ExactRatio`. `Money × Money` and
//! cross-currency arithmetic without an explicit conversion function and a
//! receipted rate source are rejected. The everyday surface is plain verb
//! methods; Rust operators are admitted only on same-unit exact values; division,
//! rounding, and cross-unit/currency crossings are never an operator.
//!
//! # Admitted approximation
//!
//! Approximate observation is first-class but quarantined:
//! approximate arithmetic is admitted only under an explicitly selected profile
//! with sound error or interval propagation; a typed wrapper around host floating
//! point is not an admitted approximate operation.
//!
//! # Declared incompleteness (owed, not hidden)
//!
//! Ranges are schema-declared — this home fixes none (`Percent`'s range is decided
//! fully schema-side; 0–100 is not implied by the name). Currency, time-unit, and
//! unit-domain designation members await the schema home's designation types;
//! structs carrying them say so on the missing member. Refusal-cause payloads are
//! documented per cause and materialize with those same types. The interval
//! truth tables are documented law on [`IntervalRelation`]; their executable form
//! lands with the interval family roster.

use crate::identity::Commitment;
use crate::logic::Truth;
use crate::types::{EvidenceRef, Limit};
use crate::value::BoundedText;

/// The four constructor axes — the value-shape axes of exact construction. A
/// category whose unit is fixed by the type carries no unit member and owns no
/// unit cause; a category with no scale owns no scale cause: a family that lists
/// a cause for an axis it does not carry has copied a neighbour rather than
/// stated itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstructorAxis {
    /// The per-value unit member, admitted by the declaring schema.
    Unit,
    /// The per-value decimal scale, admitted by the declared profile.
    Scale,
    /// The admitted bound — always schema/profile-declared, never fixed here.
    Range,
    /// A composite witness family's obligation to hold its parts consistent.
    WitnessCoherence,
}

/// The dependency-ordered ladder: unit → scale → range → witness coherence.
/// This IS every family's deterministic cause-selection rule — Rust variant
/// order is never the rule; repair direction is the ladder itself.
pub const CONSTRUCTOR_AXIS_LADDER: [ConstructorAxis; 4] = [
    ConstructorAxis::Unit,
    ConstructorAxis::Scale,
    ConstructorAxis::Range,
    ConstructorAxis::WitnessCoherence,
];

// ---------------------------------------------------------------------------
// Width-independent carriers. The i128 carrier is realization, never value law:
// widths are evidence-selected, and the wide-exact seam preserves over-wide
// exact results rather than refusing them.
// ---------------------------------------------------------------------------

/// An exact integer coefficient. Carrier `i128` is the day-one realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExactCoefficient(i128);

impl ExactCoefficient {
    /// In-crate mint for laws. Test-gated until a lawful minter exists.
    #[cfg(test)]
    pub(crate) const fn raw(value: i128) -> Self {
        Self(value)
    }

    /// The exact value under the day-one carrier.
    #[must_use]
    pub fn value(&self) -> i128 {
        self.0
    }
}

/// An exact nonnegative decimal scale ordinal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimalScale(u32);

impl DecimalScale {
    /// In-crate mint for laws. Test-gated until a lawful minter exists.
    #[cfg(test)]
    pub(crate) const fn raw(value: u32) -> Self {
        Self(value)
    }

    /// The scale ordinal.
    #[must_use]
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// The limit family for numeric designation text (currency and unit
/// designations). Magnitude is schema-witnessed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DesignationLimit;
impl Limit for DesignationLimit {}

/// A currency designation as supplied and schema-admitted. Seated here (not at
/// the schema home) because the band graph demands it: `Money` at band 04
/// cannot import band 08. Admission of *values* stays the declaring schema's.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyDesignation {
    text: BoundedText<DesignationLimit>,
}

impl CurrencyDesignation {
    /// The designation text.
    #[must_use]
    pub fn text(&self) -> &BoundedText<DesignationLimit> {
        &self.text
    }
}

/// A unit-domain designation as supplied and schema-admitted (time units,
/// measurement units). Same seating argument as [`CurrencyDesignation`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitDesignation {
    text: BoundedText<DesignationLimit>,
}

impl UnitDesignation {
    /// The designation text.
    #[must_use]
    pub fn text(&self) -> &BoundedText<DesignationLimit> {
        &self.text
    }
}

/// The domain marker for decimal-profile identity commitments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimalProfileDomain;

/// The identity of one declared decimal profile — a Class-A commitment over the
/// profile declaration (AUTHORED seating: the profile identity is a type
/// here, not a bare name).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimalProfileId(Commitment<DecimalProfileDomain>);

impl DecimalProfileId {
    /// In-crate mint for laws. Test-gated until profile declaration exists.
    #[cfg(test)]
    pub(crate) const fn default_for_laws() -> Self {
        Self(Commitment::raw([3; 32]))
    }
}

// ---------------------------------------------------------------------------
// The exact families — role-distinct opaque newtypes with checked construction,
// never one shared numeric type. Constructors land when schema profiles exist
// (every range bound is schema-declared).
// ---------------------------------------------------------------------------

/// Exact integer. No unit member, no scale member; range alone is its axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExactInteger(ExactCoefficient);

impl ExactInteger {
    /// The exact value.
    #[must_use]
    pub fn value(&self) -> i128 {
        self.0.value()
    }
}

/// Exact fixed-point decimal: coefficient plus explicit scale. Canonical form
/// retains the authored scale exactly — trailing zeros are significant, never
/// normalized away — so no non-canonical cause exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedDecimal {
    coefficient: ExactCoefficient,
    scale: DecimalScale,
}

impl FixedDecimal {
    /// In-crate mint for laws. Test-gated until a lawful minter exists.
    #[cfg(test)]
    pub(crate) const fn raw(coefficient: ExactCoefficient, scale: DecimalScale) -> Self {
        Self { coefficient, scale }
    }

    /// The coefficient.
    #[must_use]
    pub fn coefficient(&self) -> ExactCoefficient {
        self.coefficient
    }

    /// The authored scale.
    #[must_use]
    pub fn scale(&self) -> DecimalScale {
        self.scale
    }
}

/// Exact ratio. Canonical form: nonzero positive denominator, gcd-reduced, zero
/// as `0/1` — all three normalizations are exact and total, so none is a cause,
/// and refused overflow and checked range name one event, not two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExactRatio {
    numerator: ExactCoefficient,
    denominator: ExactCoefficient,
}

impl ExactRatio {
    /// In-crate mint for laws (canonical form assumed by the caller).
    /// Test-gated until a lawful minter exists.
    #[cfg(test)]
    pub(crate) const fn raw(numerator: ExactCoefficient, denominator: ExactCoefficient) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// The canonical numerator.
    #[must_use]
    pub fn numerator(&self) -> ExactCoefficient {
        self.numerator
    }

    /// The canonical (positive) denominator.
    #[must_use]
    pub fn denominator(&self) -> ExactCoefficient {
        self.denominator
    }
}

/// Money: a per-value currency member plus an amount at that currency's
/// declared minor-unit scale. The canonical form IS the minor-unit scale, so no
/// non-canonical cause exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Money {
    amount: FixedDecimal,
    currency: CurrencyDesignation,
}

impl Money {
    /// The amount at the currency's minor-unit scale.
    #[must_use]
    pub fn amount(&self) -> FixedDecimal {
        self.amount
    }

    /// The per-value currency member.
    #[must_use]
    pub fn currency(&self) -> &CurrencyDesignation {
        &self.currency
    }
}

/// Percent: unit fixed by the type — no unit member, no unit cause. Its range is
/// declared by the admitting schema (decided fully schema-side); 0–100 is not
/// implied by the name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Percent(FixedDecimal);

impl Percent {
    /// The percent value against its own declared profile.
    #[must_use]
    pub fn value(&self) -> FixedDecimal {
        self.0
    }
}

/// Percentage points: same axes as [`Percent`], its own family — the parallel
/// roster is the consequence of identical axes, never evidence of one family.
/// Substituting it for `Percent` is a compile-time wrong-role refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PercentagePoints(FixedDecimal);

impl PercentagePoints {
    /// The percentage-point value against its own declared profile.
    #[must_use]
    pub fn value(&self) -> FixedDecimal {
        self.0
    }
}

/// Duration: a general exact quantity of time with its per-value time-unit
/// member. The time home's deadline-policy type is deliberately not this type,
/// and none of that policy's law flows in.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Duration {
    amount: FixedDecimal,
    unit: UnitDesignation,
}

impl Duration {
    /// The amount in the declared time unit.
    #[must_use]
    pub fn amount(&self) -> FixedDecimal {
        self.amount
    }

    /// The per-value time-unit member.
    #[must_use]
    pub fn unit(&self) -> &UnitDesignation {
        &self.unit
    }
}

/// Count: nonnegative exact integers, lower bound zero, schema-declared upper
/// bound. A negative value is out of range, not a second cause; one cause is the
/// honest roster, not a gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Count(u64);

impl Count {
    /// The count value.
    #[must_use]
    pub fn value(&self) -> u64 {
        self.0
    }
}

/// The distance a margin preserves: a point or a distance-interval. This is the
/// structural resolution of the margin-or-interval question — the alternative
/// lives inside the witness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistanceWitness {
    /// A scalar distance.
    Point(FixedDecimal),
    /// A distance interval.
    Interval {
        /// The lower distance bound.
        lower: FixedDecimal,
        /// The upper distance bound.
        upper: FixedDecimal,
    },
}

/// Which side of the threshold the margin lies on (authored spelling; the old
/// book states direction as a preserved fact without naming its values).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarginDirection {
    /// The interval lies above the threshold.
    Above,
    /// The interval lies below the threshold.
    Below,
}

/// Whether zero lies inside the preserved distance — the fact that makes a
/// margin conclusive or lawfully inconclusive. Never a `bool`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZeroPosture {
    /// Zero lies inside: no conclusive side — a lawful inconclusive witness,
    /// never a cause.
    ZeroInside,
    /// Zero lies outside: the direction is conclusive.
    ZeroOutside,
}

/// Typed margin: a composite witness preserving unit, direction, distance or
/// distance-interval, and whether zero lies inside. A margin minted by the
/// decide operation is a total function of validated operands — its direction
/// is computed from the distance, so contradiction is unrepresentable on that
/// route.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedMargin {
    unit: UnitDesignation,
    direction: MarginDirection,
    distance: DistanceWitness,
    zero: ZeroPosture,
}

impl TypedMargin {
    /// The preserved unit domain.
    #[must_use]
    pub fn unit(&self) -> &UnitDesignation {
        &self.unit
    }

    /// The preserved direction.
    #[must_use]
    pub fn direction(&self) -> MarginDirection {
        self.direction
    }

    /// The preserved distance or distance-interval.
    #[must_use]
    pub fn distance(&self) -> DistanceWitness {
        self.distance
    }

    /// Whether zero lies inside the distance.
    #[must_use]
    pub fn zero(&self) -> ZeroPosture {
        self.zero
    }
}

// ---------------------------------------------------------------------------
// Constructor refusal families. All single-cause closed enums riding the
// declared ladder; per-cause payloads are documented on each variant and
// materialize with the schema home's designation and profile identity types.
// Every family maps every inhabited cause value to its own stable ReasonId.
// ---------------------------------------------------------------------------

use crate::refusal::{FamilyShape, RefusalFamily};

/// `ExactInteger` construction: range alone. Payload owed: supplied exact value;
/// declared bound and which side was exceeded; the admitting profile's identity.
#[must_use = "a construction refusal carries the lawful reason the value was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExactIntegerConstruction {
    /// The value lies outside the admitted bound.
    RangeExceeded,
}

impl RefusalFamily for ExactIntegerConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["RangeExceeded"];
}

/// `FixedDecimal` construction: scale → range. On the derived quantization route
/// no cause is reachable — the crossing establishes and releases through
/// `QuantizeCrossing`, and one crossing never releases two families.
#[must_use = "a construction refusal carries the lawful reason the value was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixedDecimalConstruction {
    /// The supplied scale is not in the declared decimal profile's admitted set.
    /// Payload owed: supplied scale; profile identity.
    ScaleNotAdmitted,
    /// The value determined by coefficient and scale falls outside the admitted
    /// range. Payload owed: supplied coefficient and scale; declared bound.
    RangeExceeded,
}

impl RefusalFamily for FixedDecimalConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["ScaleNotAdmitted", "RangeExceeded"];
}

/// `ExactRatio` construction: denominator admissibility → range.
#[must_use = "a construction refusal carries the lawful reason the value was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExactRatioConstruction {
    /// The denominator is zero — there is no ratio to canonicalize. Payload
    /// owed: the supplied numerator, as evidence of what was attempted.
    ZeroDenominator,
    /// A supplied term already exceeds the admitted exact range. Payload owed:
    /// supplied pair; canonical pair; declared bound.
    RangeExceeded,
}

impl RefusalFamily for ExactRatioConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["ZeroDenominator", "RangeExceeded"];
}

/// `Money` construction: currency → scale → range. On the derived receipted
/// conversion route only `RangeExceeded` is reachable — a rate multiplication is
/// not magnitude-bounded by its operands.
#[must_use = "a construction refusal carries the lawful reason the value was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoneyConstruction {
    /// Currency unstated or not admitted by the declaring schema. Payload owed:
    /// supplied designation (or its absence); schema identity.
    CurrencyNotAdmitted,
    /// The amount's scale differs from the admitted currency's declared minor
    /// unit. Payload owed: both scales.
    ScaleNotAdmitted,
    /// The amount lies outside the currency's declared range. Payload owed:
    /// amount; bound.
    RangeExceeded,
}

impl RefusalFamily for MoneyConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] =
        &["CurrencyNotAdmitted", "ScaleNotAdmitted", "RangeExceeded"];
}

/// `Percent` construction: scale → range, against the percent's own declared
/// profile.
#[must_use = "a construction refusal carries the lawful reason the value was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PercentConstruction {
    /// The supplied scale is not admitted by the percent's own profile.
    ScaleNotAdmitted,
    /// The value lies outside the percent's own declared range.
    RangeExceeded,
}

impl RefusalFamily for PercentConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["ScaleNotAdmitted", "RangeExceeded"];
}

/// `PercentagePoints` construction: same two axes, its own family, against its
/// own declared profile and range.
#[must_use = "a construction refusal carries the lawful reason the value was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PercentagePointsConstruction {
    /// The supplied scale is not admitted by this family's own profile.
    ScaleNotAdmitted,
    /// The value lies outside this family's own declared range.
    RangeExceeded,
}

impl RefusalFamily for PercentagePointsConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["ScaleNotAdmitted", "RangeExceeded"];
}

/// `Duration` construction: time unit → scale → range.
#[must_use = "a construction refusal carries the lawful reason the value was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurationConstruction {
    /// Time unit unstated or not admitted by the declaring schema. Payload
    /// owed: supplied designation; schema identity.
    UnitNotAdmitted,
    /// The scale is not admitted by the unit's declared profile.
    ScaleNotAdmitted,
    /// The value lies outside the unit's declared range.
    RangeExceeded,
}

impl RefusalFamily for DurationConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] =
        &["UnitNotAdmitted", "ScaleNotAdmitted", "RangeExceeded"];
}

/// `Count` construction: range alone — `[0, schema-declared upper]`.
#[must_use = "a construction refusal carries the lawful reason the value was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountConstruction {
    /// The value lies outside the admitted range. Payload owed: supplied value;
    /// declared bound and the side exceeded; the admitting schema's identity.
    RangeExceeded,
}

impl RefusalFamily for CountConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["RangeExceeded"];
}

/// `TypedMargin` construction: unit domain → scale → range → witness coherence.
/// All four reachable only on the authored schema-admission route; none on the
/// derived interval-decision route (direction is computed from the distance, so
/// contradiction is unrepresentable; the distance cannot exceed its own domain's
/// span).
#[must_use = "a construction refusal carries the lawful reason the margin was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypedMarginConstruction {
    /// The unit is not admitted by the declaring schema's unit domain.
    UnitNotAdmitted,
    /// The scale is not admitted.
    ScaleNotAdmitted,
    /// The value lies outside the admitted range.
    RangeExceeded,
    /// A declared direction contradicts the sign of the declared distance, or a
    /// declared zero-inside posture contradicts the declared distance interval.
    /// Payload owed: declared direction; derived sign; zero-inside posture.
    IncoherentWitness,
}

impl RefusalFamily for TypedMarginConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "UnitNotAdmitted",
        "ScaleNotAdmitted",
        "RangeExceeded",
        "IncoherentWitness",
    ];
}

// ---------------------------------------------------------------------------
// Operation families — the operations' own refusals, never the constructors'.
// ---------------------------------------------------------------------------

/// Money arithmetic: operand admissibility → currency compatibility. The family
/// owns no repair it can perform — a refusal is never a licence to select a
/// rate, widen a currency domain, or convert by convenience. The operation
/// family and the value family are never released for one event.
#[must_use = "an arithmetic refusal carries the lawful reason the operation did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoneyArithmetic {
    /// A money operand multiplied by another dimensional operand — `Money ×
    /// Money` is the named case. Payload owed: the attempted operator and both
    /// operand families; never a result, because no result was ever formed.
    UnsupportedProduct,
    /// An operation over two currencies lacking an explicit conversion
    /// function, a rate source, or a receipt for the rate source. Payload owed:
    /// both currency designations and which of the three requirements is
    /// absent.
    CrossCurrencyWithoutReceiptedRate,
}

impl RefusalFamily for MoneyArithmetic {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] =
        &["UnsupportedProduct", "CrossCurrencyWithoutReceiptedRate"];
}

/// Interval comparison: unit domain → scale. Unit precedes scale because a
/// scale comparison across two unit domains has no meaning to be wrong about. A
/// scale difference is never an incompatibility — only a finer threshold scale
/// refuses, because entering the domain would be a scale reduction, and
/// `decide` neither takes a rounding mode nor may invent one.
#[must_use = "a comparison refusal carries the lawful reason no truth was established"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntervalComparison {
    /// The threshold's unit domain is not the interval's. Payload owed: both
    /// unit-domain designations.
    ThresholdUnitIncompatible,
    /// The threshold's scale is finer than the interval's domain admits.
    /// Payload owed: the interval's bound scale, the threshold's scale, the
    /// domain's admitted scale set.
    ThresholdScaleNotAdmitted,
}

impl RefusalFamily for IntervalComparison {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] =
        &["ThresholdUnitIncompatible", "ThresholdScaleNotAdmitted"];
}

// ---------------------------------------------------------------------------
// Interval decisions.
// ---------------------------------------------------------------------------

/// Which comparison one interval decision performs — first-class data, because a
/// relation hidden inside method dispatch could not be carried or replayed. The
/// truth tables (documented law; executable when the interval roster lands),
/// over interval `[lo, hi]` versus threshold `t` in one compatible unit domain:
///
/// | Relation | True | False | Pending |
/// |---|---|---|---|
/// | `Is` | `lo = hi = t` | `t < lo` or `t > hi` | `lo ≤ t ≤ hi` and `lo < hi` |
/// | `IsNot` | `t < lo` or `t > hi` | `lo = hi = t` | `lo ≤ t ≤ hi` and `lo < hi` |
/// | `LessThan` | `hi < t` | `lo ≥ t` | `lo < t ≤ hi` |
/// | `AtMost` | `hi ≤ t` | `lo > t` | `lo ≤ t < hi` |
/// | `MoreThan` | `lo > t` | `hi ≤ t` | `lo ≤ t < hi` |
/// | `AtLeast` | `lo ≥ t` | `hi < t` | `lo < t ≤ hi` |
///
/// (`IsNot` is enumerated cell-for-cell as the exact complement of `Is` —
/// derived from stated law, marked as such.) General rule: whole interval
/// satisfies →
/// `True`; contradicts → `False`; overlaps the boundary → `Pending`. A
/// `Pending` from boundary overlap is a lawful outcome, never a refusal; a
/// decided `Truth` carries no authority — a policy alone maps it to a decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntervalRelation {
    /// Equality against the threshold.
    Is,
    /// The exact complement of `Is`.
    IsNot,
    /// Strictly less than the threshold.
    LessThan,
    /// At most the threshold.
    AtMost,
    /// Strictly more than the threshold.
    MoreThan,
    /// At least the threshold.
    AtLeast,
}

/// The decided result: the established truth plus its typed margin witness. Not
/// a second three-valued enum — its truth IS the canonical `Truth`. Travels
/// first-class into normalized form, explanation, evidence, and independent
/// replay. The time home's temporal comparisons route through this same family.
#[must_use = "a decided result carries the established truth and the margin that decided it"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntervalDecision {
    /// The established truth.
    pub truth: Truth,
    /// The typed margin witness.
    pub margin: TypedMargin,
}

// ---------------------------------------------------------------------------
// Quantization: the crossing from admitted approximation to exact
// representation. It never invents a point; loss grants no authority to the
// mechanism that performed it.
// ---------------------------------------------------------------------------

/// Whether a crossing was exact or lossy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantizeDisposition {
    /// The crossing lost nothing.
    Exact,
    /// The crossing discarded a remainder, recorded in the evidence.
    Inexact,
}

/// The six standard rounding modes — named explicitly by authority-changing
/// operations; no default mode is ever selected; every rounding event is
/// recorded. General associativity/distributivity are not claimed for
/// operations that round at intermediate boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoundingMode {
    /// Round half to even.
    HalfEven,
    /// Round half away from zero.
    HalfAwayFromZero,
    /// Truncate toward zero.
    TowardZero,
    /// Round away from zero.
    AwayFromZero,
    /// Round toward negative infinity.
    Floor,
    /// Round toward positive infinity.
    Ceiling,
}

/// What one crossing discarded — the remainder is exactly representable as a
/// ratio (authored carrier).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiscardedRemainder {
    /// The crossing was exact; nothing was discarded.
    NothingDiscarded,
    /// The exact discarded remainder.
    Discarded(ExactRatio),
}

/// Declared interval or error evidence carried by an approximate observation or
/// a crossing (AUTHORED shape: the fact carries a type here, never bare
/// prose).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorEvidence {
    /// A finite exact enclosing interval.
    EnclosingInterval {
        /// The lower bound.
        lower: FixedDecimal,
        /// The upper bound.
        upper: FixedDecimal,
    },
    /// A declared error bound.
    ErrorBound(FixedDecimal),
}

/// The claim marker for quantize-crossing provenance references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantizeProvenance;

/// Quantization evidence — the nine mandatory facts, none omittable:
/// disposition · source representation · source uncertainty · target profile ·
/// target scale · rounding mode · discarded remainder · error evidence ·
/// provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuantizeEvidence {
    /// Whether the crossing was exact or lossy.
    pub disposition: QuantizeDisposition,
    /// The source's format identity.
    pub source_representation: FloatFormatId,
    /// The source's declared uncertainty.
    pub source_uncertainty: ErrorEvidence,
    /// The resolved target profile.
    pub target_profile: DecimalProfileId,
    /// The target scale.
    pub target_scale: DecimalScale,
    /// The admitted rounding mode — recorded whether or not the crossing was
    /// lossy.
    pub rounding: RoundingMode,
    /// What the crossing discarded.
    pub discarded_remainder: DiscardedRemainder,
    /// The crossing's own error evidence.
    pub error: ErrorEvidence,
    /// Provenance of the crossing.
    pub provenance: EvidenceRef<QuantizeProvenance>,
}

/// A quantized exact point: the value plus its mandatory evidence.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuantizePoint {
    /// The exact value produced by the crossing.
    pub value: FixedDecimal,
    /// The mandatory attached evidence.
    pub evidence: QuantizeEvidence,
}

/// A quantized enclosing interval: lower bound `Floor`, upper bound `Ceiling`
/// by law, so the target contains every source value — nearest rounding is
/// refused where it could shrink the possibility set. It supplies no rounding
/// mode at all because its bounds are fixed by law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantizeInterval {
    /// The floor-rounded lower bound.
    pub lower: FixedDecimal,
    /// The ceiling-rounded upper bound.
    pub upper: FixedDecimal,
}

/// The one family for both quantize crossings: crossing inputs → source
/// admissibility → domain compatibility → target admissibility →
/// representability. There is no missing-rounding-mode cause — structurally: a
/// fact supplied at the call site is structurally mandatory and has no absence
/// cause, while a fact the crossing must resolve can genuinely be absent.
#[must_use = "a crossing refusal carries the lawful reason the quantization did not proceed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantizeCrossing {
    /// The target profile was not resolvable. Payload owed: the identity of the
    /// input it did not receive.
    MissingTargetProfile,
    /// The source's error evidence was absent. Payload owed: the identity of
    /// the input it did not receive.
    MissingErrorEvidence,
    /// The source is NaN — no crossing may invent a point for a value that is
    /// not one. Payload owed: source format identity and classification.
    SourceIsNaN,
    /// The source is infinite. Payload owed: source format identity and
    /// classification.
    SourceIsInfinite,
    /// Source and target unit domains differ. Payload owed: both designations.
    UnitMismatch,
    /// The requested scale is not in the target profile's admitted set.
    /// Payload owed: requested scale; admitted scale set.
    UnsupportedScale,
    /// The exact source magnitude exceeds the target's declared bound. Payload
    /// owed: declared bound; exact source magnitude.
    RangeOverflow,
}

impl RefusalFamily for QuantizeCrossing {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "MissingTargetProfile",
        "MissingErrorEvidence",
        "SourceIsNaN",
        "SourceIsInfinite",
        "UnitMismatch",
        "UnsupportedScale",
        "RangeOverflow",
    ];
}

// ---------------------------------------------------------------------------
// Admitted approximation observations.
// ---------------------------------------------------------------------------

/// A registered float-format identity (u16-registered, authored seating).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatFormatId(u16);

impl FloatFormatId {
    /// In-crate mint for laws. Test-gated until the format register exists.
    #[cfg(test)]
    pub(crate) const fn registered(id: u16) -> Self {
        Self(id)
    }

    /// The registered identity.
    #[must_use]
    pub fn value(&self) -> u16 {
        self.0
    }
}

/// A registered approximation-profile identity (u16-registered, authored
/// seating). Approximate arithmetic is admitted only under an explicitly
/// selected profile with sound error or interval propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApproximationProfileId(u16);

impl ApproximationProfileId {
    /// In-crate mint for laws. Test-gated until the profile register exists.
    #[cfg(test)]
    pub(crate) const fn registered(id: u16) -> Self {
        Self(id)
    }

    /// The registered identity.
    #[must_use]
    pub fn value(&self) -> u16 {
        self.0
    }
}

/// The format's exact observed bit pattern (day-one carrier `u64`, covering the
/// required Binary32/Binary64 compatibility formats; carrier is realization).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatBitPattern(u64);

impl FloatBitPattern {
    /// In-crate mint for laws. Test-gated until observation ingress exists.
    #[cfg(test)]
    pub(crate) const fn raw(bits: u64) -> Self {
        Self(bits)
    }

    /// The exact observed bits.
    #[must_use]
    pub fn bits(&self) -> u64 {
        self.0
    }
}

/// Approximation taint (AUTHORED roster): whether the observation is
/// as-observed or has propagated through
/// admitted approximate operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApproximationTaint {
    /// The bits are as observed at ingress.
    DirectlyObserved,
    /// The value propagated through admitted approximate operations under its
    /// profile.
    Propagated,
}

/// The claim marker for approximate-observation provenance references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApproxProvenance;

/// An observed approximate value — first-class but quarantined from implicit
/// authority. Evidence equality is exact format + raw bits (the identifying
/// pair; equality and hashing use exactly these two); class, profile, error,
/// taint, and provenance are carried facts. Numeric equality is a separate
/// question answered by admitted operations, never by `==`: `PositiveZero`
/// equals `NegativeZero` numerically but stays a distinct evidence identity,
/// and NaN numeric equality is a typed refusal. No ordinary total order exists.
#[derive(Debug, Clone)]
pub struct ApproxObservation {
    /// The format identity.
    pub format: FloatFormatId,
    /// The exact observed bit pattern.
    pub raw_bits: FloatBitPattern,
    /// The exact classification.
    pub class: FloatClass,
    /// The admitted approximation profile.
    pub profile: ApproximationProfileId,
    /// Declared interval or error evidence.
    pub error: ErrorEvidence,
    /// The approximation taint.
    pub taint: ApproximationTaint,
    /// Provenance of the observation.
    pub provenance: EvidenceRef<ApproxProvenance>,
}

impl PartialEq for ApproxObservation {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format && self.raw_bits == other.raw_bits
    }
}

impl Eq for ApproxObservation {}

impl core::hash::Hash for ApproxObservation {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.format.hash(state);
        self.raw_bits.hash(state);
    }
}

/// The exact classification of an observed float — exactly six. Stable identity
/// belongs only to profile/format identities, never to individual NaNs,
/// infinities, zero signs, or decimal positions. `PositiveZero` equals
/// `NegativeZero` numerically but stays a distinct evidence identity; NaN
/// numeric equality is a typed refusal; there is no ordinary total order over
/// an approximate observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatClass {
    /// A finite value.
    Finite,
    /// Positive zero.
    PositiveZero,
    /// Negative zero.
    NegativeZero,
    /// Positive infinity.
    PositiveInfinity,
    /// Negative infinity.
    NegativeInfinity,
    /// Not a number.
    NaN,
}

// ---------------------------------------------------------------------------
// The numeric-honesty layer. Applications own their domain content; this home
// owns only its honesty — bindings, non-collapse, bounds, refusals.
// ---------------------------------------------------------------------------

/// The six terminals of checking an evidence requirement — each carries the
/// terms that produced it (payloads owed to their owners). Composition may
/// never turn `Unresolved` or `SourceIncomplete` into rejection, nor let a
/// decisive branch hide a term's invalidity.
#[must_use = "a disposition is what checking the evidence requirement concluded"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementDisposition {
    /// The requirement conclusively held.
    ConclusivelySatisfied,
    /// The requirement conclusively failed.
    ConclusivelyRejected,
    /// Not resolvable from what is admitted.
    Unresolved,
    /// The requirement itself is invalid.
    Invalid,
    /// The required source region is incomplete.
    SourceIncomplete,
    /// The proof route is unavailable.
    ProofUnavailable,
}

/// Result finality — its own axis, orthogonal to truth, decision, freshness,
/// and proof, applying only to result families whose claims can extend or
/// require closure: an operation exposes only the axes it actually answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Finality<Cut> {
    /// A partial result over a monotone operator may safely extend as later
    /// facts arrive at higher cuts.
    MonotoneExtendable,
    /// Absence, proven no-route, final ordering, truncation, negation, and
    /// aggregate finality require exact source closure at the named cut —
    /// completeness comes from source law and operator law, never from "the
    /// stream kept going".
    ClosedAt(Cut),
}

/// Every declared exact-to-estimate information-loss crossing kind — the same
/// discipline as quantize, generalized. No implicit collapse; no default that
/// silently widens or sharpens a claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LossKind {
    /// Exact value to interval estimate.
    ExactToInterval,
    /// Exact value to distribution estimate.
    ExactToDistribution,
    /// Estimate to estimate.
    EstimateToEstimate,
}

/// Neutral inspection projection over the estimate families — carries no
/// authority; the families are the real types, and there is no bare
/// point-estimate accessor anywhere: any move to a less-certain family goes
/// through a declared evidence-bearing loss crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EstimateFamily {
    /// Exact representation, no information loss.
    Exact,
    /// Finite exact-bounded interval, decided via interval decisions.
    Interval,
    /// Bounded distribution/sample/histogram role.
    Distribution,
}

/// An exact estimate: an exact representation with no information loss, over
/// whichever exact family the claim uses. Authority only where an owning policy
/// admits it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExactEstimate<V> {
    /// The exact value.
    pub value: V,
}

/// An interval estimate: a finite exact-bounded interval, decided via
/// [`IntervalDecision`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntervalEstimate {
    /// The lower bound.
    pub lower: FixedDecimal,
    /// The upper bound.
    pub upper: FixedDecimal,
}

/// The role one distribution estimate plays — the stated shape ("bounded
/// distribution / sample / histogram role").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistributionRole {
    /// A bounded distribution.
    BoundedDistribution,
    /// A bounded sample.
    Sample,
    /// A bounded histogram.
    Histogram,
}

/// A distribution estimate carrying its declared role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DistributionEstimate {
    /// The declared role.
    pub role: DistributionRole,
}

/// The claim marker for information-loss evidence references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LossEvidence;

/// A declared, typed exact-to-estimate information-loss crossing carrying
/// mandatory loss evidence — the quantize discipline generalized. `#[must_use]`
/// by law: an unobserved crossing is a silent collapse.
#[must_use = "a crossing carries the loss it disclosed; an unobserved crossing is the silent \
              collapse the type exists to prevent"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationLossCrossing<From, To> {
    /// The crossing kind.
    pub kind: LossKind,
    /// The value crossed from.
    pub from: From,
    /// The estimate produced.
    pub to: To,
    /// The mandatory loss evidence.
    pub evidence: EvidenceRef<LossEvidence>,
}

/// The wide-exact seam's day-one carrier: preserves an exact result exceeding a
/// bounded inline representation without silent approximation. Carrier is
/// realization (authored: `ExactRatio` day-one); crossing back to bounded exact
/// requires checked range, explicit target profile/scale, explicit rounding if
/// lossy, and conversion evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WideExact(ExactRatio);

impl WideExact {
    /// The preserved exact value.
    #[must_use]
    pub fn value(&self) -> ExactRatio {
        self.0
    }
}

/// The 2026-08-09 decision, machine-readable: knowledge-axis offer checking
/// selects its single reported cause in this order.
pub const KNOWLEDGE_AXIS_SELECTION_ORDER: [&str; 4] = [
    "truth-coverage disagreement",
    "estimate-witness-family availability",
    "axis-presence disagreement in canonical result-axis order",
    "estimate-family disagreement",
];

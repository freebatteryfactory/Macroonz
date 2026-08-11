//! Band 04 — numeric: the exact families, the constructor-axis ladder, interval
//! decisions, quantization, rounding, and the numeric-honesty layer including
//! `Finality`.

pub mod types;

pub use types::{
    ApproxObservation, ApproxProvenance, ApproximationProfileId, ApproximationTaint,
    CONSTRUCTOR_AXIS_LADDER, ConstructorAxis, Count, CountConstruction, CurrencyDesignation,
    DecimalProfileDomain, DecimalProfileId, DecimalScale, DesignationLimit, DiscardedRemainder,
    DistanceWitness, DistributionEstimate, DistributionRole, Duration, DurationConstruction,
    ErrorEvidence, EstimateFamily, ExactCoefficient, ExactEstimate, ExactInteger,
    ExactIntegerConstruction, ExactRatio, ExactRatioConstruction, Finality, FixedDecimal,
    FixedDecimalConstruction, FloatBitPattern, FloatClass, FloatFormatId, InformationLossCrossing,
    IntervalComparison, IntervalDecision, IntervalEstimate, IntervalRelation,
    KNOWLEDGE_AXIS_SELECTION_ORDER, LossEvidence, LossKind, MarginDirection, Money,
    MoneyArithmetic, MoneyConstruction, Percent, PercentConstruction, PercentagePoints,
    PercentagePointsConstruction, QuantizeCrossing, QuantizeDisposition, QuantizeEvidence,
    QuantizeInterval, QuantizePoint, QuantizeProvenance, RequirementDisposition, RoundingMode,
    TypedMargin, TypedMarginConstruction, UnitDesignation, WideExact, ZeroPosture,
};

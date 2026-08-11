//! Band 20 — derived: `DataBlock` law, the two-seat row-domain identity,
//! selection masks, payload locators, materialization, physical plans.

pub mod types;

pub use types::{
    BindingEntry, BindingEntryRef, Column, DATA_MECHANISM_DIAGNOSTICS, DATA_SEMANTIC_WORK,
    DERIVATION_PRIMITIVES, DERIVED_REFUSAL_CLASSES, DataBlockState, ExtentEntry, ExtentEntryRef,
    KERNEL_ADMISSION_GATE, LayoutId, MaskRepresentation, MaterializationAppliedCut,
    MaterializationAvailability, MaterializationCoverage, MaterializationGeneration,
    MaterializationId, MaterializationPresence, MaterializationSourceCuts, OccurrenceDigest,
    OccurrenceId, PLAN_CANNOT, PROTECTED_INDEX_STANDING_BAR, PayloadLocator, PlanBinding,
    PlanTemplate, RowDomainId, SelectionMask, SelectionMaskConstruction, SourceBinding,
    ValidityCondition,
};

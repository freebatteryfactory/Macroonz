//! The declaration nucleus: every constructor that can refuse and every private-field reader.

use super::{
    BenchMeasurement, BenchReferences, BenchRow, BenchRowKey, BenchRowRefusal, BenchTableName,
    ComplexityClaimRef, ContentionPosture, DeclaredBudgets, DeclaredBudgetsRefusal, ExactRatio,
    InputSizeAxis, InputSizeAxisRefusal, PlantedWorseRef, PreflightRef, WorkFormula,
    WorkFormulaRefusal, WorkObservationRef, WorkloadRef,
};
use crate::bench::declaration::encode::derive_row_key;
use crate::descriptor::{NameRefusal, NamespacedName};
use crate::identity::ContentAddress;
use std::collections::BTreeMap;
use std::num::NonZeroU32;

/// The two roads and the one reader every namespaced reference here shares, written once.
///
/// Each reference is a type of its own so a workload cannot be handed in where a complexity claim was meant.
/// All they share is how a name is parsed, and that law belongs in one place.
macro_rules! namespaced_reference {
    ($($reference:ident),+ $(,)?) => {
        $(
            impl $reference {
                /// This reference, from the namespace that declares it and the spelling it carries.
                ///
                /// # Errors
                ///
                /// Refuses an empty namespace, then an empty stem.
                pub const fn named(
                    namespace: &'static str,
                    stem: &'static str,
                ) -> Result<Self, NameRefusal> {
                    match NamespacedName::named(namespace, stem) {
                        Ok(name) => Ok(Self(name)),
                        Err(refusal) => Err(refusal),
                    }
                }

                /// This reference, over a name already parsed.
                #[must_use]
                pub const fn over(name: NamespacedName) -> Self {
                    Self(name)
                }

                /// The name this reference carries.
                #[must_use]
                pub const fn name(self) -> NamespacedName {
                    self.0
                }
            }
        )+
    };
}

namespaced_reference!(
    WorkloadRef,
    PreflightRef,
    PlantedWorseRef,
    ComplexityClaimRef,
    WorkObservationRef,
    BenchTableName,
);

impl InputSizeAxis {
    /// At least two distinct sizes, keeping the order they were authored in.
    ///
    /// # Errors
    ///
    /// Refuses an axis shorter than two, then the first repeated size.
    pub fn declared(sizes: Vec<u64>) -> Result<Self, InputSizeAxisRefusal> {
        if sizes.len() < 2usize {
            return Err(InputSizeAxisRefusal::TooShort { found: sizes.len() });
        }
        let mut seen = BTreeMap::new();
        for (duplicate, size) in sizes.iter().copied().enumerate() {
            if let Some(first) = seen.insert(size, duplicate) {
                return Err(InputSizeAxisRefusal::DuplicateSize {
                    size,
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self(sizes))
    }

    /// The sizes, in authored order.
    #[must_use]
    pub fn sizes(&self) -> &[u64] {
        &self.0
    }
}

impl ExactRatio {
    /// The numerator.
    #[must_use]
    pub const fn numerator(self) -> u64 {
        self.numerator
    }

    /// The denominator.
    #[must_use]
    pub const fn denominator(self) -> u64 {
        self.denominator
    }
}

impl DeclaredBudgets {
    /// One positive sample count, any warmup count, and one positive gap ratio.
    ///
    /// # Errors
    ///
    /// Refuses zero samples, then a zero numerator, then a zero denominator.
    pub fn declared(
        samples: u32,
        warmups: u32,
        ratio_numerator: u64,
        ratio_denominator: u64,
    ) -> Result<Self, DeclaredBudgetsRefusal> {
        let Some(samples) = NonZeroU32::new(samples) else {
            return Err(DeclaredBudgetsRefusal::NoSamples);
        };
        if ratio_numerator == 0u64 {
            return Err(DeclaredBudgetsRefusal::ZeroRatioNumerator);
        }
        if ratio_denominator == 0u64 {
            return Err(DeclaredBudgetsRefusal::ZeroRatioDenominator);
        }
        Ok(Self {
            samples,
            warmups,
            ratio: ExactRatio {
                numerator: ratio_numerator,
                denominator: ratio_denominator,
            },
        })
    }

    /// How many samples each pass takes.
    #[must_use]
    pub const fn samples(self) -> u32 {
        self.samples.get()
    }

    /// How many calls the timed pass discards first.
    #[must_use]
    pub const fn warmups(self) -> u32 {
        self.warmups
    }

    /// The gap ratio handed to the judge.
    #[must_use]
    pub const fn ratio(self) -> ExactRatio {
        self.ratio
    }
}

impl WorkFormula {
    /// One present formula, in whatever bytes the owner spells it.
    ///
    /// # Errors
    ///
    /// Refuses empty bytes, since a row with no formula carries `None` instead.
    pub fn encoded(bytes: Vec<u8>) -> Result<Self, WorkFormulaRefusal> {
        if bytes.is_empty() {
            return Err(WorkFormulaRefusal::Empty);
        }
        Ok(Self { bytes })
    }

    /// The bytes, exactly as they were declared.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl BenchRowKey {
    pub(in crate::bench) const fn derived(address: ContentAddress) -> Self {
        Self(address)
    }

    /// The address this identity carries.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl BenchReferences {
    /// The four names one row joins.
    #[must_use]
    pub const fn declared(
        workload: WorkloadRef,
        preflight: PreflightRef,
        planted_worse: PlantedWorseRef,
        complexity: ComplexityClaimRef,
    ) -> Self {
        Self {
            workload,
            preflight,
            planted_worse,
            complexity,
        }
    }

    /// The workload being measured.
    #[must_use]
    pub const fn workload(self) -> WorkloadRef {
        self.workload
    }

    /// The correctness preflight.
    #[must_use]
    pub const fn preflight(self) -> PreflightRef {
        self.preflight
    }

    /// The deliberately worse control.
    #[must_use]
    pub const fn planted_worse(self) -> PlantedWorseRef {
        self.planted_worse
    }

    /// The complexity claim.
    #[must_use]
    pub const fn complexity(self) -> ComplexityClaimRef {
        self.complexity
    }
}

impl BenchMeasurement {
    /// The four measurement facts one row declares.
    #[must_use]
    pub fn declared(
        input_sizes: InputSizeAxis,
        budgets: DeclaredBudgets,
        contention: ContentionPosture,
        formula: Option<WorkFormula>,
    ) -> Self {
        Self {
            input_sizes,
            budgets,
            contention,
            formula,
        }
    }

    /// The input-size axis.
    #[must_use]
    pub const fn input_sizes(&self) -> &InputSizeAxis {
        &self.input_sizes
    }

    /// The sample, warmup, and ratio budgets.
    #[must_use]
    pub const fn budgets(&self) -> DeclaredBudgets {
        self.budgets
    }

    /// The declared contention posture.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.contention
    }

    /// The formula bytes, where the row declared any.
    #[must_use]
    pub const fn formula(&self) -> Option<&WorkFormula> {
        self.formula.as_ref()
    }
}

impl BenchRow {
    /// One row, with its identity derived from the whole declaration.
    ///
    /// # Identity
    ///
    /// The preimage is the workload name, authored axis length and values, preflight name, planted-worse name, sample and warmup counts, exact ratio, contention tag, optional formula, and complexity name, in that order.
    /// Names encode as length-prefixed namespace and stem bytes, lengths and axis values are big-endian `u64`, sample and warmup counts are big-endian `u32`, ratio members are big-endian `u64`, contention is its declared one-byte tag, and formula absence or presence is `0` or `1` followed on presence by length-prefixed bytes.
    ///
    /// # Errors
    ///
    /// Refuses only where the descriptor adapter cannot hold a member's length in the public encoding width before delegating its bytes to shared identity framing.
    /// That refusal is unreachable on every supported target and remains as a compatibility ceiling.
    pub fn declared(
        references: BenchReferences,
        measurement: BenchMeasurement,
    ) -> Result<Self, BenchRowRefusal> {
        let key = derive_row_key(references, &measurement).map_err(BenchRowRefusal::Encoding)?;
        Ok(Self {
            references,
            measurement,
            key,
        })
    }

    /// The workload being measured.
    #[must_use]
    pub const fn workload(&self) -> WorkloadRef {
        self.references.workload
    }

    /// The input-size axis.
    #[must_use]
    pub const fn input_sizes(&self) -> &InputSizeAxis {
        &self.measurement.input_sizes
    }

    /// The correctness preflight.
    #[must_use]
    pub const fn preflight(&self) -> PreflightRef {
        self.references.preflight
    }

    /// The deliberately worse control.
    #[must_use]
    pub const fn planted_worse(&self) -> PlantedWorseRef {
        self.references.planted_worse
    }

    /// The sample, warmup, and ratio budgets.
    #[must_use]
    pub const fn budgets(&self) -> DeclaredBudgets {
        self.measurement.budgets
    }

    /// The declared contention posture.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.measurement.contention
    }

    /// The formula bytes, where the row declared any.
    #[must_use]
    pub const fn formula(&self) -> Option<&WorkFormula> {
        self.measurement.formula.as_ref()
    }

    /// The complexity claim.
    #[must_use]
    pub const fn complexity(&self) -> ComplexityClaimRef {
        self.references.complexity
    }

    /// The identity derived from the whole declaration.
    #[must_use]
    pub const fn key(&self) -> BenchRowKey {
        self.key
    }
}

//! The bench home's invariant nucleus: every road that reaches a private field.
//!
//! A work formula is encoded here, so a formula carrying no bytes is not a value anybody can hold.
//! An axis is admitted here, so a growth class is never read off a single point.
//! A table's lens namespace is closed here, so one carrier never asks for one target-owned expression twice.

use super::{
    BENCH_ROW_LIMIT, BenchCaptureError, BenchmarkDeclaration, INPUT_SIZE_LIMIT, Measurement,
    References, Reporter, Row, WORK_FORMULA_LIMIT, WORK_OBSERVATION_LIMIT, WorkFormula,
};
use crate::bounded::{Bounded, NonEmpty};
use crate::descriptor::{
    CaptureCause, DeclarationError, FunctionName, Grammar, HelperRefusal, ModuleName, Name, Seat,
    SupportName,
};
use crate::token::SpanHandle;
use std::collections::BTreeSet;

impl WorkFormula {
    /// One declared work formula, over the declaration's own encoded bytes.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where no bytes were supplied — an operation that declares no formula states that by carrying none, so an empty one is a formula nobody wrote — and [`DeclarationError::Unbounded`] where the bytes outgrow [`WORK_FORMULA_LIMIT`].
    pub fn encoded(bytes: Vec<u8>) -> Result<Self, DeclarationError> {
        if bytes.is_empty() {
            return Err(DeclarationError::Absent {
                seat: Seat::WorkFormulaByte,
            });
        }
        let offered = bytes.len();
        let encoded = Bounded::new(bytes).map_err(|_| {
            DeclarationError::unbounded(Seat::WorkFormulaByte, WORK_FORMULA_LIMIT, offered)
        })?;
        Ok(Self { encoded })
    }

    /// The formula's encoded bytes, in the order the declaration wrote them.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        self.encoded.as_slice()
    }
}

impl Row {
    /// Declare one bench row.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::NotACurve`] where the axis states fewer than two sizes, [`DeclarationError::Doubled`] where two positions state one axis size or observation, [`DeclarationError::Absent`] where no observation was declared, and [`DeclarationError::Unbounded`] where either roster outgrows its declared magnitude.
    /// The checks run in the signature's semantic order and retain the first established cause.
    pub fn declared(
        lens: FunctionName,
        references: References,
        axis: Vec<u64>,
        measurement: Measurement,
        observations: Vec<Name>,
    ) -> Result<Self, DeclarationError> {
        let offered = axis.len();
        if offered < 2 {
            return Err(DeclarationError::NotACurve {
                observed: u64::try_from(offered).unwrap_or(u64::MAX),
            });
        }
        let distinct: BTreeSet<&u64> = axis.iter().collect();
        if distinct.len() != offered {
            return Err(DeclarationError::Doubled {
                seat: Seat::AxisSize,
            });
        }
        let admitted_axis = Bounded::new(axis)
            .map_err(|_| DeclarationError::unbounded(Seat::AxisSize, INPUT_SIZE_LIMIT, offered))?;
        if observations.is_empty() {
            return Err(DeclarationError::Absent {
                seat: Seat::WorkObservation,
            });
        }
        let mut distinct_observations: BTreeSet<&Name> = BTreeSet::new();
        for observation in &observations {
            if !distinct_observations.insert(observation) {
                return Err(DeclarationError::Doubled {
                    seat: Seat::WorkObservation,
                });
            }
        }
        let observation_count = observations.len();
        let admitted_observations = Bounded::new(observations).map_err(|_| {
            DeclarationError::unbounded(
                Seat::WorkObservation,
                WORK_OBSERVATION_LIMIT,
                observation_count,
            )
        })?;
        Ok(Self {
            lens,
            references,
            axis: admitted_axis,
            measurement,
            observations: admitted_observations,
        })
    }

    /// The lens the carrier's target-owned expressions are matched under.
    #[must_use]
    pub const fn lens(&self) -> &FunctionName {
        &self.lens
    }

    /// The four namespaced references this row states about itself.
    #[must_use]
    pub const fn references(&self) -> &References {
        &self.references
    }

    /// The declared input-size axis; structurally at least two distinct sizes.
    ///
    /// # Ordering
    ///
    /// This order is meaning: the axis is the curve a growth class is read off, and the rendering writes the sizes in exactly the order they were declared.
    #[must_use]
    pub fn axis(&self) -> &[u64] {
        self.axis.as_slice()
    }

    /// What this row declares about how it is measured.
    #[must_use]
    pub const fn measurement(&self) -> &Measurement {
        &self.measurement
    }

    /// The work observations this row's target-owned callables may record, in authored order.
    #[must_use]
    pub fn observations(&self) -> &[Name] {
        self.observations.as_slice()
    }
}

impl Reporter {
    /// Declare the module that carries the target-supplied report reader.
    #[must_use]
    pub const fn declared(module: ModuleName) -> Self {
        Self { module }
    }

    /// The module the report-reader value is rendered inside.
    #[must_use]
    pub const fn module(&self) -> &ModuleName {
        &self.module
    }
}

impl BenchmarkDeclaration {
    /// Declare the complete neutral benchmark payload one delivery is written from.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where no row was supplied, [`DeclarationError::Doubled`] where the table function and reporter module share one generated-item spelling or two rows carry one lens spelling, and [`DeclarationError::Unbounded`] where the rows outgrow [`BENCH_ROW_LIMIT`].
    pub fn declared(
        support: SupportName,
        table_function: FunctionName,
        table: Name,
        rows: Vec<Row>,
        reporter: Reporter,
    ) -> Result<Self, DeclarationError> {
        if rows.is_empty() {
            return Err(DeclarationError::Absent { seat: Seat::Row });
        }
        if table_function.spelling() == reporter.module().spelling() {
            return Err(DeclarationError::Doubled {
                seat: Seat::GeneratedItem,
            });
        }
        lens_namespace_closed(&rows)?;
        let offered = rows.len();
        let admitted = NonEmpty::new(rows)
            .map_err(|_| DeclarationError::unbounded(Seat::Row, BENCH_ROW_LIMIT, offered))?;
        Ok(Self {
            support,
            table_function,
            table,
            rows: admitted,
            reporter,
        })
    }

    /// The exported name a consumption target invokes this delivery's carrier by.
    #[must_use]
    pub const fn support(&self) -> &SupportName {
        &self.support
    }

    /// The function the benchmark-table stamp writes.
    #[must_use]
    pub const fn table_function(&self) -> &FunctionName {
        &self.table_function
    }

    /// The bench table's own namespaced name.
    #[must_use]
    pub const fn table(&self) -> &Name {
        &self.table
    }

    /// The rows, in the order they were declared; structurally at least one.
    #[must_use]
    pub fn rows(&self) -> &NonEmpty<Row, BENCH_ROW_LIMIT> {
        &self.rows
    }

    /// The module that carries the target-supplied report reader.
    #[must_use]
    pub const fn reporter(&self) -> &Reporter {
        &self.reporter
    }
}

impl BenchCaptureError {
    /// One refusal the bench grammar's own reading established.
    pub const fn grammar_refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> Self {
        Self(HelperRefusal::grammar_refused(grammar, cause, at))
    }

    /// One refusal the vocabulary established over a value this grammar read.
    pub const fn vocabulary_refused(
        grammar: Grammar,
        refusal: DeclarationError,
        at: SpanHandle,
    ) -> Self {
        Self(HelperRefusal::vocabulary_refused(grammar, refusal, at))
    }

    /// The refusal itself.
    pub const fn refusal(&self) -> &HelperRefusal {
        &self.0
    }
}

/// The carrier matcher's one lens namespace, closed: every lens spelling across every row, distinct.
///
/// Refused here rather than left to the consumer's compiler, which would report a duplicate definition inside an expansion nobody wrote.
fn lens_namespace_closed(rows: &[Row]) -> Result<(), DeclarationError> {
    let mut taken: BTreeSet<&str> = BTreeSet::new();
    for row in rows {
        if !taken.insert(row.lens().spelling()) {
            return Err(DeclarationError::Doubled { seat: Seat::Lens });
        }
    }
    Ok(())
}

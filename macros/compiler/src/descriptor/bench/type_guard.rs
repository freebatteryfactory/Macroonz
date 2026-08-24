//! The bench home's invariant nucleus: every road that reaches a private field.
//!
//! A work formula is encoded here, so a formula carrying no bytes is not a value anybody can hold.
//! An axis is admitted here, so a growth class is never read off a single point.
//! A backend is named here, so the adapter's one swap point cannot be handed a spelling that is not a Rust identifier.
//! A table's lens namespace is closed here, so the rendered adapter never declares one function twice.

use super::{
    Adapter, Attachment, BENCH_ROW_LIMIT, Backend, Benches, INPUT_SIZE_LIMIT, Measurement,
    References, Row, WORK_FORMULA_LIMIT, WORK_OBSERVATION_LIMIT, WorkFormula,
};
use crate::bounded::{Bounded, NonEmpty};
use crate::descriptor::{
    BoundPath, DeclarationError, FunctionName, ModuleName, Name, Seat, rendered_identifier,
};
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

impl Attachment {
    /// What makes one row measurable: the three callables the host order invokes, and the work observations it reads.
    ///
    /// The three callables are required by the signature, so a row that would be benchmarked without its two gates is unrepresentable rather than refused.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Unbounded`] where the observations outgrow [`WORK_OBSERVATION_LIMIT`].
    pub fn measuring(
        measured: BoundPath,
        planted_worse: BoundPath,
        preflight: BoundPath,
        observations: Vec<BoundPath>,
    ) -> Result<Self, DeclarationError> {
        let offered = observations.len();
        let admitted = Bounded::new(observations).map_err(|_| {
            DeclarationError::unbounded(Seat::WorkObservation, WORK_OBSERVATION_LIMIT, offered)
        })?;
        Ok(Self {
            measured,
            planted_worse,
            preflight,
            observations: admitted,
        })
    }

    /// The work observations this row reads, in the order they were declared.
    #[must_use]
    pub fn observations(&self) -> &[BoundPath] {
        self.observations.as_slice()
    }
}

impl Row {
    /// Declare one bench row.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::NotACurve`] where the axis states fewer than two sizes — a growth class is read off a curve and never off a point — [`DeclarationError::Doubled`] where two positions state one size, and [`DeclarationError::Unbounded`] where the axis outgrows [`INPUT_SIZE_LIMIT`].
    /// The checks are dependent and in that order, so exactly one cause is true of any refused row.
    pub fn declared(
        lens: FunctionName,
        references: References,
        axis: Vec<u64>,
        measurement: Measurement,
        attachment: Attachment,
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
        let admitted = Bounded::new(axis)
            .map_err(|_| DeclarationError::unbounded(Seat::AxisSize, INPUT_SIZE_LIMIT, offered))?;
        Ok(Self {
            lens,
            references,
            axis: admitted,
            measurement,
            attachment,
        })
    }

    /// The lens the rendered adapter registers this row under.
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

    /// What makes this row measurable.
    #[must_use]
    pub const fn attachment(&self) -> &Attachment {
        &self.attachment
    }
}

impl Backend {
    /// The backend a consumer named, under whatever it reaches the dependency by.
    ///
    /// There is no default: a backend this compiler chose would be a dependency the consumer never asked for, and the adapter reads its name from the value it was handed.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::NotAnIdentifier`] where the spelling is not one Rust identifier: it is written in path position, so a spelling that is not one renders tokens the consumer's compiler reads as something else.
    pub fn named(spelling: &str) -> Result<Self, DeclarationError> {
        if rendered_identifier(spelling) {
            Ok(Self(spelling.to_owned()))
        } else {
            Err(DeclarationError::NotAnIdentifier)
        }
    }

    /// The spelling every backend-naming token in the adapter is written from.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.0.as_str()
    }
}

impl Adapter {
    /// Declare the one-file reporter adapter.
    ///
    /// Total: both parts were admitted by their own roads before they reached this one.
    #[must_use]
    pub const fn declared(module: ModuleName, backend: Backend) -> Self {
        Self { module, backend }
    }

    /// The module the adapter is rendered as.
    #[must_use]
    pub const fn module(&self) -> &ModuleName {
        &self.module
    }

    /// The one swap point: the backend the neutral table is bound to.
    #[must_use]
    pub const fn backend(&self) -> &Backend {
        &self.backend
    }
}

impl Benches {
    /// Declare the complete payload one bench delivery is written from.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where no row was supplied, [`DeclarationError::Doubled`] where two rows carry one lens spelling — the rendered adapter puts every lens in one namespace, so a collision would be a duplicate definition inside an expansion nobody wrote — and [`DeclarationError::Unbounded`] where the rows outgrow [`BENCH_ROW_LIMIT`].
    pub fn declared(
        module: ModuleName,
        table: Name,
        rows: Vec<Row>,
        adapter: Adapter,
    ) -> Result<Self, DeclarationError> {
        if rows.is_empty() {
            return Err(DeclarationError::Absent { seat: Seat::Row });
        }
        lens_namespace_closed(&rows)?;
        let offered = rows.len();
        let admitted = NonEmpty::new(rows)
            .map_err(|_| DeclarationError::unbounded(Seat::Row, BENCH_ROW_LIMIT, offered))?;
        Ok(Self {
            module,
            table,
            rows: admitted,
            adapter,
        })
    }

    /// The module the rendered table is written as.
    #[must_use]
    pub const fn module(&self) -> &ModuleName {
        &self.module
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

    /// The adapter that binds these rows to a measurement backend.
    #[must_use]
    pub const fn adapter(&self) -> &Adapter {
        &self.adapter
    }
}

/// The rendered adapter's ONE namespace, closed: every lens spelling across every row, distinct.
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

//! The host nucleus: preflight binding, table admission, invocation facts, and report minting.

use super::{
    BenchAttachment, BenchBinding, BenchBindingRefusal, BenchInvocation, BenchOutcome,
    BenchReading, BenchReport, BenchRow, BenchRowKey, BenchStage, BenchTable, BenchTableName,
    BenchTableRefusal, BenchVerdictRefusal, ContentionPosture, PreflightRef, PreflightTrial,
};
use crate::clock::HarnessClock;
use crate::descriptor::Provenance;
use crate::report::{TargetBinding, TrialReport};
use crate::runner::{Invocation, TrialBinding};
use std::collections::BTreeMap;

impl PreflightTrial {
    /// Bind this home's preflight name to a real trial and the invocation it runs under.
    #[must_use]
    pub fn bound(reference: PreflightRef, binding: TrialBinding, invocation: Invocation) -> Self {
        Self {
            reference,
            binding,
            invocation,
        }
    }

    /// The preflight name a row must agree with.
    #[must_use]
    pub const fn reference(&self) -> PreflightRef {
        self.reference
    }

    /// The trial that runs.
    #[must_use]
    pub const fn binding(&self) -> &TrialBinding {
        &self.binding
    }

    /// The invocation it runs under.
    #[must_use]
    pub const fn invocation(&self) -> &Invocation {
        &self.invocation
    }
}

impl BenchBinding {
    /// Join one row to its callables and its preflight.
    ///
    /// # Errors
    ///
    /// Refuses a workload, control, preflight, then complexity name that disagrees, in that order.
    pub fn bound(
        row: BenchRow,
        attachment: BenchAttachment,
        preflight: PreflightTrial,
    ) -> Result<Self, BenchBindingRefusal> {
        if row.workload() != attachment.workload() {
            return Err(BenchBindingRefusal::Workload {
                row: row.workload(),
                attachment: attachment.workload(),
            });
        }
        if row.planted_worse() != attachment.planted_worse_ref() {
            return Err(BenchBindingRefusal::PlantedWorse {
                row: row.planted_worse(),
                attachment: attachment.planted_worse_ref(),
            });
        }
        if row.preflight() != preflight.reference() {
            return Err(BenchBindingRefusal::Preflight {
                row: row.preflight(),
                trial: preflight.reference(),
            });
        }
        if row.complexity() != attachment.judge().complexity() {
            return Err(BenchBindingRefusal::Complexity {
                row: row.complexity(),
                judge: attachment.judge().complexity(),
            });
        }
        Ok(Self {
            row,
            attachment,
            preflight,
        })
    }

    /// The row.
    #[must_use]
    pub const fn row(&self) -> &BenchRow {
        &self.row
    }

    /// The callables and the judge bound to it.
    #[must_use]
    pub const fn attachment(&self) -> &BenchAttachment {
        &self.attachment
    }

    /// The correctness preflight.
    #[must_use]
    pub const fn preflight(&self) -> &PreflightTrial {
        &self.preflight
    }
}

impl BenchTable {
    /// One nonempty table, keeping the order the bindings were authored in.
    ///
    /// # Errors
    ///
    /// Refuses an empty table, then the first repeated row identity.
    pub fn authored(
        name: BenchTableName,
        provenance: Provenance,
        bindings: Vec<BenchBinding>,
    ) -> Result<Self, BenchTableRefusal> {
        if bindings.is_empty() {
            return Err(BenchTableRefusal::Empty);
        }
        let mut seen = BTreeMap::new();
        for (duplicate, binding) in bindings.iter().enumerate() {
            let row = binding.row().key();
            if let Some(first) = seen.insert(row, duplicate) {
                return Err(BenchTableRefusal::DuplicateRow {
                    row,
                    first,
                    duplicate,
                });
            }
        }
        Ok(Self {
            name,
            provenance,
            bindings,
        })
    }

    /// The table's name.
    #[must_use]
    pub const fn name(&self) -> BenchTableName {
        self.name
    }

    /// Whether these rows were written by hand or produced.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// Every binding, in authored order.
    #[must_use]
    pub fn bindings(&self) -> &[BenchBinding] {
        &self.bindings
    }

    /// How many rows this table admitted.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Always false, since an empty table is not admitted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

impl BenchInvocation {
    /// Declare the target, the clock, and the contention posture for one whole table run.
    #[must_use]
    pub fn declared(
        target: TargetBinding,
        clock: HarnessClock,
        contention: ContentionPosture,
    ) -> Self {
        Self {
            target,
            clock,
            contention,
        }
    }

    /// The target and toolchain this run declares.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The clock the timed pass reads.
    #[must_use]
    pub const fn clock(&self) -> HarnessClock {
        self.clock
    }

    /// The contention posture this run declares.
    #[must_use]
    pub const fn contention(&self) -> ContentionPosture {
        self.contention
    }
}

impl BenchOutcome {
    /// The stage this outcome occupies, without its evidence.
    #[must_use]
    pub const fn stage(&self) -> BenchStage {
        match self {
            Self::PreflightRefused => BenchStage::PreflightRefused,
            Self::PlantedWorseNotDistinguished { .. } => BenchStage::PlantedWorseNotDistinguished,
            Self::PrimaryWorkRefused { .. } => BenchStage::PrimaryWorkRefused,
            Self::Qualified { .. } => BenchStage::Qualified,
        }
    }
}

impl BenchReading {
    pub(in crate::bench) fn recorded(
        row: &BenchRow,
        target: TargetBinding,
        preflight: TrialReport,
        outcome: BenchOutcome,
    ) -> Self {
        Self {
            row: row.clone(),
            target,
            preflight,
            outcome,
        }
    }

    /// The whole row this reading executed.
    #[must_use]
    pub const fn row(&self) -> &BenchRow {
        &self.row
    }

    /// The target it stood on.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The correctness preflight's own report.
    #[must_use]
    pub const fn preflight(&self) -> &TrialReport {
        &self.preflight
    }

    /// How far the row got, and with what evidence.
    #[must_use]
    pub const fn outcome(&self) -> &BenchOutcome {
        &self.outcome
    }
}

impl BenchReport {
    pub(in crate::bench) fn recorded(
        table: BenchTableName,
        provenance: Provenance,
        readings: Vec<BenchReading>,
    ) -> Self {
        Self {
            table,
            provenance,
            readings,
        }
    }

    /// The table this report records.
    #[must_use]
    pub const fn table(&self) -> BenchTableName {
        self.table
    }

    /// Whether that table was written by hand or produced.
    #[must_use]
    pub const fn provenance(&self) -> Provenance {
        self.provenance
    }

    /// One reading per authored binding, in table order.
    #[must_use]
    pub fn readings(&self) -> &[BenchReading] {
        &self.readings
    }

    /// How many readings this report holds.
    #[must_use]
    pub fn denominator(&self) -> usize {
        self.readings.len()
    }
}

impl BenchVerdictRefusal {
    pub(in crate::bench) const fn row_not_qualified(row: BenchRowKey, stage: BenchStage) -> Self {
        Self { row, stage }
    }

    /// The first row that did not qualify.
    #[must_use]
    pub const fn row(self) -> BenchRowKey {
        self.row
    }

    /// Where that row stopped.
    #[must_use]
    pub const fn stage(self) -> BenchStage {
        self.stage
    }
}

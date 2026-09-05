//! The bench home's stated tables: what the kind is, where its two units land, the question it owes, and the arm a contention posture is emitted under.

use super::{
    BenchAnswer, BenchCaptureError, BenchQuestion, BenchRole, BenchTable, BenchmarkDeclaration,
    ContentionPosture,
};
use crate::descriptor::Name;
use crate::descriptor::vocabulary::HarnessName;
use crate::diagnostic::BENCH_HELPER_FAMILY;
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{Answer, CanonicalContent, Destination, Kind, Question, Role};

impl CanonicalContent for BenchmarkDeclaration {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.support().spelling().as_bytes(), into);
        encode_bytes(self.table_function().spelling().as_bytes(), into);
        encode_name(self.table(), into);
        encode_length(self.rows().count(), into);
        for row in self.rows() {
            let mut encoded = Vec::new();
            encode_bytes(row.lens().spelling().as_bytes(), &mut encoded);
            let references = row.references();
            encode_name(&references.workload, &mut encoded);
            encode_name(&references.correctness_preflight, &mut encoded);
            encode_name(&references.planted_worse, &mut encoded);
            encode_name(&references.complexity_claim, &mut encoded);
            encode_length(row.axis().len(), &mut encoded);
            for size in row.axis() {
                encoded.extend_from_slice(&size.to_be_bytes());
            }
            let measurement = row.measurement();
            encoded.extend_from_slice(&measurement.budgets.samples.to_be_bytes());
            encoded.extend_from_slice(&measurement.budgets.warmups.to_be_bytes());
            encoded.extend_from_slice(&measurement.budgets.ratio_numerator.to_be_bytes());
            encoded.extend_from_slice(&measurement.budgets.ratio_denominator.to_be_bytes());
            encoded.push(match measurement.contention {
                ContentionPosture::NoDeclaredContention => 0,
            });
            match &measurement.work_formula {
                None => encoded.push(0),
                Some(formula) => {
                    encoded.push(1);
                    encode_bytes(formula.bytes(), &mut encoded);
                }
            }
            encode_length(row.observations().len(), &mut encoded);
            for observation in row.observations() {
                encode_name(observation, &mut encoded);
            }
            encode_bytes(&encoded, into);
        }
        encode_bytes(self.reporter().module().spelling().as_bytes(), into);
    }
}

fn encode_name(name: &Name, into: &mut Vec<u8>) {
    encode_bytes(name.namespace().as_bytes(), into);
    encode_bytes(name.stem().as_bytes(), into);
}

impl Kind for BenchTable {
    const NAME: &'static str = "bench-table";

    type Content = BenchmarkDeclaration;
    type Role = BenchRole;
    type Question = BenchQuestion;
}

impl Role for BenchRole {
    const ALL: &'static [Self] = &[Self::Table, Self::Reporter];

    fn name(self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::Reporter => "reporter",
        }
    }

    fn destination(self) -> Destination {
        match self {
            Self::Table => Destination::DeclarationSite,
            Self::Reporter => Destination::BenchCarrier,
        }
    }
}

impl Question for BenchQuestion {
    const ALL: &'static [Self] = &[Self::WhichBenchmarksMeasure];

    type Answer = BenchAnswer;

    fn name(self) -> &'static str {
        match self {
            Self::WhichBenchmarksMeasure => "which-benchmarks-measure",
        }
    }
}

impl Answer for BenchAnswer {
    type Question = BenchQuestion;

    fn question(&self) -> BenchQuestion {
        match *self {
            Self::MeasuringBenchmarks { .. } => BenchQuestion::WhichBenchmarksMeasure,
        }
    }

    fn encode_into(&self, into: &mut Vec<u8>) {
        match *self {
            Self::MeasuringBenchmarks { ref table, rows } => {
                encode_bytes(table.namespace().as_bytes(), into);
                encode_bytes(table.stem().as_bytes(), into);
                into.extend_from_slice(&rows.to_be_bytes());
            }
        }
    }

    fn human(&self) -> String {
        match *self {
            Self::MeasuringBenchmarks { ref table, rows } => {
                let namespace = table.namespace();
                let stem = table.stem();
                format!("the table `{namespace}`/`{stem}` declares {rows} rows that measure it")
            }
        }
    }
}

impl ContentionPosture {
    /// The arm this posture is emitted under, exactly as the harness's schema declares the closed choice.
    ///
    /// A constant answer over a closed roster, so a second posture admitted later stops the compiler here until somebody says what it is called at the address.
    #[must_use]
    pub const fn arm(self) -> HarnessName {
        match self {
            Self::NoDeclaredContention => HarnessName::NoDeclaredContention,
        }
    }
}

impl BenchmarkDeclaration {
    /// How many rows stand under this table.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows().count()
    }
}

crate::descriptor::impl_helper_capture_contract!(BenchCaptureError, BENCH_HELPER_FAMILY, canonical);

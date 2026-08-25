//! The bench home's stated tables: what the kind is, where its two units land, the question it owes, the arm a posture is emitted under, the backend's road spellings, and the order the three named tolerances occupy in a positional roster.

use super::{
    BackendRoad, BenchAnswer, BenchCaptureError, BenchQuestion, BenchRole, BenchTable, Benches,
    ContentionPosture,
};
use crate::bounded::Bounded;
use crate::descriptor::vocabulary::HarnessName;
use crate::descriptor::{BoundPath, Name};
use crate::diagnostic::{
    BENCH_HELPER_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused,
    Repair,
};
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{Answer, CanonicalContent, Destination, Kind, Question, Role};

impl CanonicalContent for Benches {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.support().spelling().as_bytes(), into);
        encode_bytes(self.module().spelling().as_bytes(), into);
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
            encoded.extend_from_slice(&measurement.budgets.warmup.to_be_bytes());
            encoded.extend_from_slice(&measurement.budgets.ratio_threshold.to_be_bytes());
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
            let attachment = row.attachment();
            encode_path(&attachment.measured, &mut encoded);
            encode_path(&attachment.planted_worse, &mut encoded);
            encode_path(&attachment.preflight, &mut encoded);
            encode_length(attachment.observations().len(), &mut encoded);
            for observation in attachment.observations() {
                encode_path(observation, &mut encoded);
            }
            encode_bytes(&encoded, into);
        }
        encode_bytes(self.adapter().module().spelling().as_bytes(), into);
        encode_bytes(self.adapter().backend().spelling().as_bytes(), into);
    }
}

fn encode_path(path: &BoundPath, into: &mut Vec<u8>) {
    encode_bytes(path.binding().name().as_bytes(), into);
    encode_length(path.segments().count(), into);
    for segment in path.segments() {
        encode_bytes(segment.as_bytes(), into);
    }
}

fn encode_name(name: &Name, into: &mut Vec<u8>) {
    encode_bytes(name.namespace().as_bytes(), into);
    encode_bytes(name.stem().as_bytes(), into);
}

impl Kind for BenchTable {
    const NAME: &'static str = "bench-table";

    type Content = Benches;
    type Role = BenchRole;
    type Question = BenchQuestion;
}

impl Role for BenchRole {
    const ALL: &'static [Self] = &[Self::Table, Self::Adapter];

    fn name(self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::Adapter => "adapter",
        }
    }

    fn destination(self) -> Destination {
        match self {
            Self::Table => Destination::DeclarationSite,
            Self::Adapter => Destination::BenchCarrier,
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

impl BackendRoad {
    /// The spelling this road is published under.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::Bench => "bench",
            Self::Args => "args",
            Self::BlackBox => "black_box",
            Self::Main => "main",
        }
    }
}

impl Benches {
    /// How many rows stand under this table.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows().count()
    }

    /// The backend spelling every backend-naming token of the rendering is written from.
    #[must_use]
    pub fn backend(&self) -> &str {
        self.adapter().backend().spelling()
    }
}

impl Refused for BenchCaptureError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = BENCH_HELPER_FAMILY;

    fn class(&self) -> RefusalClass {
        self.refusal().class()
    }

    fn first(&self) -> String {
        self.refusal().first()
    }

    fn observed(&self) -> Observed {
        self.refusal().classified()
    }

    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    fn related(&self) -> Vec<Vec<u8>> {
        vec![self.refusal().canonical_bytes()]
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        self.refusal().repairs()
    }
}

impl core::fmt::Display for BenchCaptureError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let refusal = self.refusal();
        write!(into, "{refusal}")
    }
}

impl core::error::Error for BenchCaptureError {}

/// The named tolerance each position of the schema's positional budget roster carries, in the order the rendering writes them.
///
/// The schema declares budgets as a roster of counts and this home declares three named seats, so the mapping is a stated table rather than an order a reader infers from a rendering.
///
/// It is a table for a READER and never a lookup the rendering depends on: the rendered counts are literals, so nothing here elects a name at rendering time, and moving a row of this table is a change to what the table SAYS.
/// The order the emission writes and the order stated here move together, or this table has stopped describing the emission.
pub const BUDGET_ORDER: [&str; 3] = ["samples", "warmup", "ratio-threshold"];

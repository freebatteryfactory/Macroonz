//! The bench home's stated tables: what the kind is, where its two units land, the question it owes, the arm a posture is emitted under, the backend's road spellings, and the order the three named tolerances occupy in a positional roster.

use super::{
    BackendRoad, BenchAnswer, BenchQuestion, BenchRole, BenchTable, Benches, ContentionPosture,
};
use crate::descriptor::vocabulary::HarnessName;
use crate::identity::encode_bytes;
use crate::kind::{Answer, Destination, Kind, Question, Role};

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
            Self::Table | Self::Adapter => Destination::BenchCarrier,
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

/// The named tolerance each position of the schema's positional budget roster carries, in the order the rendering writes them.
///
/// The schema declares budgets as a roster of counts and this home declares three named seats, so the mapping is a stated table rather than an order a reader infers from a rendering.
///
/// It is a table for a READER and never a lookup the rendering depends on: the rendered counts are literals, so nothing here elects a name at rendering time, and moving a row of this table is a change to what the table SAYS.
/// The order the emission writes and the order stated here move together, or this table has stopped describing the emission.
pub const BUDGET_ORDER: [&str; 3] = ["samples", "warmup", "ratio-threshold"];

//! Historical payloads have neither conversions to current verdicts nor a conclusion operation.

use macroonz_harness::oracle::{
    CompilationVerdict, CompiledVerdict, StructuralVerdict, TranscriptVerdict, VectorVerdict,
};
use macroonz_harness::oracle::archive::{ArchivedOracle, ArchivedVerdict};
use macroonz_harness::report::{FindingLocation, TrialConclusion};

fn vector(historical: ArchivedVerdict) -> VectorVerdict {
    historical.into()
}

fn transcript(historical: ArchivedVerdict) -> TranscriptVerdict {
    historical.into()
}

fn structural(historical: ArchivedVerdict) -> StructuralVerdict {
    historical.into()
}

fn compiled(historical: ArchivedVerdict) -> CompiledVerdict {
    historical.into()
}

fn compilation(historical: ArchivedVerdict) -> CompilationVerdict {
    historical.into()
}

fn conclusion(historical: ArchivedOracle) -> TrialConclusion {
    historical.into()
}

fn conclude(historical: &ArchivedOracle) -> TrialConclusion {
    historical.verdict().concluded(FindingLocation::at("declared.rs", 1))
}

fn main() {}

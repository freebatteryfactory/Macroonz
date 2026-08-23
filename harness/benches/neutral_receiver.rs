//! One backend-free handwritten benchmark target over the public receiver.

#[path = "../tests/bench_receiver/fixture.rs"]
mod fixture;

use std::fmt;
use threadpak_testpak::bench::{
    BenchRunRefusal, BenchStampRefusal, BenchVerdictRefusal, bench_verdict, run_all,
};

enum BenchTargetFailure {
    Stamp(BenchStampRefusal),
    Run(BenchRunRefusal),
    Verdict(BenchVerdictRefusal),
}

impl fmt::Debug for BenchTargetFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stamp(refusal) => formatter.debug_tuple("Stamp").field(refusal).finish(),
            Self::Run(refusal) => formatter.debug_tuple("Run").field(refusal).finish(),
            Self::Verdict(refusal) => formatter.debug_tuple("Verdict").field(refusal).finish(),
        }
    }
}

impl From<BenchStampRefusal> for BenchTargetFailure {
    fn from(refusal: BenchStampRefusal) -> Self {
        Self::Stamp(refusal)
    }
}

impl From<BenchRunRefusal> for BenchTargetFailure {
    fn from(refusal: BenchRunRefusal) -> Self {
        Self::Run(refusal)
    }
}

impl From<BenchVerdictRefusal> for BenchTargetFailure {
    fn from(refusal: BenchVerdictRefusal) -> Self {
        Self::Verdict(refusal)
    }
}

fn main() -> Result<(), BenchTargetFailure> {
    let table = fixture::lawful_table()?;
    let report = run_all(&table, &fixture::invocation())?;
    bench_verdict(&report)?;
    fixture::render(&report);
    Ok(())
}

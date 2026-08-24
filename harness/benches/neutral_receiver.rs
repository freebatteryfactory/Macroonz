//! One hand-written benchmark target over the public receiver, with no framework behind it.

#[path = "../tests/bench_receiver/fixture.rs"]
mod fixture;

use macroonz_harness::bench::{
    BenchRunRefusal, BenchStampRefusal, BenchVerdictRefusal, bench_verdict, run_all,
};

/// Whichever of the three roads refused, carried out of `main` as this target's failure.
enum TargetFailure {
    Stamp(BenchStampRefusal),
    Run(BenchRunRefusal),
    Verdict(BenchVerdictRefusal),
}

/// Written by hand rather than derived, because a derived `Debug` does not count as reading a field and the wall denies an unread one.
impl core::fmt::Debug for TargetFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Stamp(refusal) => formatter.debug_tuple("Stamp").field(refusal).finish(),
            Self::Run(refusal) => formatter.debug_tuple("Run").field(refusal).finish(),
            Self::Verdict(refusal) => formatter.debug_tuple("Verdict").field(refusal).finish(),
        }
    }
}

impl From<BenchStampRefusal> for TargetFailure {
    fn from(refusal: BenchStampRefusal) -> Self {
        Self::Stamp(refusal)
    }
}

impl From<BenchRunRefusal> for TargetFailure {
    fn from(refusal: BenchRunRefusal) -> Self {
        Self::Run(refusal)
    }
}

impl From<BenchVerdictRefusal> for TargetFailure {
    fn from(refusal: BenchVerdictRefusal) -> Self {
        Self::Verdict(refusal)
    }
}

fn main() -> Result<(), TargetFailure> {
    let table = fixture::lawful_table()?;
    let report = run_all(&table, &fixture::invocation())?;
    bench_verdict(&report)?;
    fixture::render(&report);
    Ok(())
}

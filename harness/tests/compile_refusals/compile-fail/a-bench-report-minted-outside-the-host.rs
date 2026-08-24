//! A caller cannot mint the benchmark host's complete report or its qualified outcome.

use macroonz_harness::bench::{
    BenchOutcome, BenchReading, BenchReport, SecondaryObservation, WorkCurve, WorkJudgment,
};

fn work_curve() -> WorkCurve {
    loop {}
}

fn main() {
    let _ = BenchReport::recorded;
    let _ = BenchReading::recorded;

    let _ = BenchOutcome::Qualified {
        measured: work_curve(),
        planted_worse: work_curve(),
        judgment: work_judgment(),
        secondary: secondary_observation(),
    };
}

fn work_judgment() -> WorkJudgment {
    loop {}
}

fn secondary_observation() -> SecondaryObservation {
    loop {}
}

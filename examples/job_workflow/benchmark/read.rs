//! Independent report expectations for qualified and undistinguished Job work.

use super::judge::counts;
use macroonz::harness::bench::{
    BenchOutcome, BenchReport, BenchStage, BenchVerdictRefusal, bench_verdict,
};
use macroonz::harness::clock::MeasurementReading;

pub(super) fn qualified(report: &BenchReport) {
    assert!(bench_verdict(report).is_ok());
    assert_eq!(report.denominator(), 1);
    for reading in report.readings() {
        assert_eq!(reading.outcome().stage(), BenchStage::Qualified);
        if let BenchOutcome::Qualified {
            measured,
            planted_worse,
            secondary,
            ..
        } = reading.outcome()
        {
            assert_eq!(counts(measured), Some(vec![(2, 8), (4, 16), (8, 32)]));
            assert_eq!(
                counts(planted_worse),
                Some(vec![(2, 16), (4, 64), (8, 256)])
            );
            assert_eq!(
                secondary.measurements(),
                &[MeasurementReading::Unavailable; 6]
            );
        }
    }
}

pub(super) fn undistinguished(report: &BenchReport) {
    assert_eq!(report.denominator(), 1);
    assert_eq!(
        bench_verdict(report).err().map(BenchVerdictRefusal::stage),
        Some(BenchStage::PlantedWorseNotDistinguished)
    );
    for reading in report.readings() {
        if let BenchOutcome::PlantedWorseNotDistinguished {
            measured,
            planted_worse,
            ..
        } = reading.outcome()
        {
            assert_eq!(counts(measured), Some(vec![(2, 8), (4, 16), (8, 32)]));
            assert_eq!(counts(planted_worse), Some(vec![(2, 8), (4, 16), (8, 32)]));
        }
    }
}

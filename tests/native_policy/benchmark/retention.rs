//! Retention keeps the whole benchmark denominator and each independent failure axis.

use super::{fixture::report, specimen::mapped};
use macroonz::harness::bench::archive::{
    ArchivedBenchOutcome, ArchivedWorkConclusion, ArchivedWorkGap, BenchArchiveLimits,
    BenchArchiveRefusal, retain_report,
};
use macroonz::harness::bench::{BenchStage, bench_verdict};
use macroonz::harness::clock::{ClockAttribution, ClockReadRefusal, HarnessClock};
use macroonz::harness::report::archive::{ArchiveLimits, ArchiveRefusal, ArchivedMeasurement};
use macroonz::native_storage::{
    StorageArtifact, StorageBatch, StorageError, StorageLimits, StorageName, StorageRoot,
    StorageTransaction,
};
use macroonz::workflow::benchmark::{self, RetentionLimits, RetentionRefusal};
use std::path::Path;

const LIMITS: RetentionLimits = RetentionLimits {
    archive: BenchArchiveLimits::declared(ArchiveLimits::declared(65_536, 16_384), 8, 8, 4, 32),
    storage: StorageLimits {
        artifacts: 4,
        bytes: 65_536,
    },
};

fn observe(action: impl FnOnce(&StorageRoot, &Path) -> Result<(), String>) -> Result<(), String> {
    let path = crate::check::scratch()?;
    let root = mapped(StorageRoot::open(&path))?;
    let result = action(&root, &path);
    drop(root);
    let cleanup = std::fs::remove_dir_all(path).map_err(|error| error.to_string());
    result.and(cleanup)
}

#[test]
fn all_reached_axes_and_original_order_survive_storage_without_reexecution() -> Result<(), String> {
    observe(|root, _path| {
        super::fixture::reset();
        let report = report(macroonz::native_clock::source())?;
        assert!(bench_verdict(&report).is_err());
        assert_eq!(super::fixture::calls(), (15, 6, 2, 1));
        let key = mapped(StorageName::informed("all-rows"))?;
        mapped(benchmark::retain(&report, root, &key, LIMITS))?;
        super::fixture::reset();
        let saved = mapped(benchmark::load(root, &key, LIMITS))?;
        assert_eq!(super::fixture::calls(), (0, 0, 0, 0));
        assert_eq!(saved, mapped(retain_report(&report, LIMITS.archive))?);
        assert_eq!(saved.denominator(), 5);
        assert_eq!(
            saved
                .readings()
                .iter()
                .map(|row| row.outcome().stage())
                .collect::<Vec<_>>(),
            [
                BenchStage::Qualified,
                BenchStage::PreflightRefused,
                BenchStage::PlantedWorseNotDistinguished,
                BenchStage::PrimaryWorkRefused,
                BenchStage::PlantedWorseNotDistinguished,
            ]
        );
        let ArchivedBenchOutcome::PlantedWorseNotDistinguished { judgment, .. } =
            saved.readings().last().ok_or("last row absent")?.outcome()
        else {
            return Err("lost failure stage".to_owned());
        };
        let ArchivedWorkConclusion::Refused(measured) = judgment.measured() else {
            return Err("lost measured refusal".to_owned());
        };
        assert_eq!(measured.local(), "wrong-measured");
        assert_eq!(judgment.planted_worse(), &ArchivedWorkConclusion::Satisfied);
        let ArchivedWorkGap::NotDistinguished(gap) = judgment.gap() else {
            return Err("lost gap refusal".to_owned());
        };
        assert_eq!(gap.local(), "missing-gap");
        Ok(())
    })
}

#[test]
fn unavailable_zero_and_failed_clock_readings_remain_distinct_in_storage() -> Result<(), String> {
    observe(|root, _path| {
        let mut readings = Vec::new();
        for (name, clock, attribution) in [
            (
                "unavailable",
                HarnessClock::unavailable(),
                ClockAttribution::Unspecified,
            ),
            (
                "zero",
                HarnessClock::reading_as(|| 0, ClockAttribution::Synthetic),
                ClockAttribution::Synthetic,
            ),
            (
                "failed",
                HarnessClock::fallible_as(
                    || Err(ClockReadRefusal::Refused),
                    ClockAttribution::Monotonic,
                ),
                ClockAttribution::Monotonic,
            ),
        ] {
            let report = report(clock)?;
            let name = mapped(StorageName::informed(name))?;
            mapped(benchmark::retain(&report, root, &name, LIMITS))?;
            let saved = mapped(benchmark::load(root, &name, LIMITS))?;
            assert_eq!(saved, mapped(retain_report(&report, LIMITS.archive))?);
            let ArchivedBenchOutcome::Qualified { secondary, .. } = saved
                .readings()
                .first()
                .ok_or("first row absent")?
                .outcome()
            else {
                return Err("qualified row absent".to_owned());
            };
            assert_eq!(secondary.clock_attribution(), attribution);
            assert_eq!(secondary.measurements().len(), 6);
            readings.push(secondary.measurements().to_vec());
        }
        let [unavailable, zero, failed] = readings.as_slice() else {
            return Err("clock cases absent".to_owned());
        };
        assert_eq!(unavailable, &vec![ArchivedMeasurement::Unavailable; 6]);
        assert_ne!(unavailable, zero);
        assert_ne!(unavailable, failed);
        assert_ne!(zero, failed);
        Ok(())
    })
}

#[test]
fn valid_storage_with_missing_or_extra_benchmark_members_refuses() -> Result<(), String> {
    observe(|root, _path| {
        let report = report(HarnessClock::unavailable())?;
        let archive = mapped(retain_report(&report, LIMITS.archive))?;
        for (batch_name, names) in [
            ("missing", vec!["different"]),
            ("extra", vec!["benchmark", "extra"]),
        ] {
            let key = mapped(StorageName::informed(batch_name))?;
            let members = names
                .iter()
                .map(|name| mapped(StorageName::informed(name)))
                .collect::<Result<Vec<_>, _>>()?;
            let artifacts = members
                .iter()
                .map(|name| StorageArtifact {
                    name,
                    bytes: archive.encoded(),
                })
                .collect::<Vec<_>>();
            let batch = mapped(StorageBatch::informed(&artifacts, LIMITS.storage))?;
            let mut transaction = mapped(StorageTransaction::begin(root, &key, batch))?;
            while mapped(transaction.write_next())?.is_some() {}
            mapped(transaction.commit())?;
            assert!(matches!(
                benchmark::load(root, &key, LIMITS),
                Err(RetentionRefusal::Members)
            ));
        }
        Ok(())
    })
}

#[test]
fn bounds_refuse_before_reservation_and_collision_preserves_the_report() -> Result<(), String> {
    observe(|root, path| {
        let report = report(HarnessClock::unavailable())?;
        let key = mapped(StorageName::informed("bounded"))?;
        let cases = [
            RetentionLimits {
                storage: StorageLimits {
                    artifacts: 0,
                    ..LIMITS.storage
                },
                ..LIMITS
            },
            RetentionLimits {
                storage: StorageLimits {
                    bytes: 1,
                    ..LIMITS.storage
                },
                ..LIMITS
            },
            RetentionLimits {
                archive: BenchArchiveLimits::declared(LIMITS.archive.bytes(), 1, 8, 4, 32),
                ..LIMITS
            },
        ];
        for (limits, (owner, cause)) in cases.into_iter().zip([
            ("storage", "artifact-bound"),
            ("storage", "byte-bound"),
            ("archive", "too-many-rows"),
        ]) {
            let refusal = benchmark::retain(&report, root, &key, limits)
                .err()
                .ok_or("bounded retention succeeded")?;
            let shown = crate::presentation_formats::parsed(
                &macroonz::presentation::benchmark_retention_refusal(&refusal),
            )?;
            assert_eq!(
                crate::presentation_formats::field(&shown, "/record/kind")?,
                owner
            );
            assert_eq!(
                crate::presentation_formats::field(&shown, "/record/value/kind")?,
                cause
            );
            assert!(!path.join("item-bounded").exists());
        }
        mapped(benchmark::retain(&report, root, &key, LIMITS))?;
        assert!(matches!(
            benchmark::retain(&report, root, &key, LIMITS),
            Err(RetentionRefusal::Storage(StorageError::Collision))
        ));
        assert!(matches!(
            benchmark::recover_retention(&report, root, &key, LIMITS),
            Err(RetentionRefusal::Storage(StorageError::Collision))
        ));
        assert_eq!(report.denominator(), 5);
        assert!(bench_verdict(&report).is_err());
        Ok(())
    })
}

#[test]
fn partial_retention_requires_explicit_recovery_and_corruption_refuses_loading()
-> Result<(), String> {
    observe(|root, path| {
        let report = report(HarnessClock::unavailable())?;
        let archive = mapped(retain_report(&report, LIMITS.archive))?;
        let key = mapped(StorageName::informed("interrupted"))?;
        let member = mapped(StorageName::informed("benchmark"))?;
        let artifacts = [StorageArtifact {
            name: &member,
            bytes: archive.encoded(),
        }];
        let batch = mapped(StorageBatch::informed(&artifacts, LIMITS.storage))?;
        let mut transaction = mapped(StorageTransaction::begin(root, &key, batch))?;
        assert!(mapped(transaction.write_next())?.is_some());
        drop(transaction);
        assert!(matches!(
            benchmark::load(root, &key, LIMITS),
            Err(RetentionRefusal::Storage(StorageError::Incomplete))
        ));
        mapped(benchmark::recover_retention(&report, root, &key, LIMITS))?;
        assert_eq!(mapped(benchmark::load(root, &key, LIMITS))?, archive);
        let mut bytes = archive.encoded().to_vec();
        *bytes.last_mut().ok_or("empty archive")? ^= 1;
        std::fs::write(path.join("item-interrupted/item-benchmark"), bytes)
            .map_err(|error| error.to_string())?;
        let corruption = benchmark::load(root, &key, LIMITS)
            .err()
            .ok_or("corrupt benchmark loaded")?;
        assert!(matches!(
            &corruption,
            RetentionRefusal::Archive(BenchArchiveRefusal::Canonical(
                ArchiveRefusal::AddressMismatch
            ))
        ));
        let corruption_display = crate::presentation_formats::parsed(
            &macroonz::presentation::benchmark_retention_refusal(&corruption),
        )?;
        assert_eq!(
            crate::presentation_formats::field(&corruption_display, "/record/value/value/kind")?,
            "address-mismatch"
        );
        Ok(())
    })
}

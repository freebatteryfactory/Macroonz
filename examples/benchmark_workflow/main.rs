//! Execute real counted work, retain every reading and inspect historical bytes.

mod declaration;
mod specimen;

use macroonz::harness::bench::{archive::BenchArchiveLimits, bench_verdict, run_all};
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::native_storage::{StorageLimits, StorageName, StorageRoot};
use macroonz::workflow::benchmark::{self, RetentionLimits};
use specimen::mapped;

fn main() -> Result<(), String> {
    let table = declaration::table(vec![declaration::binding(
        "count-nonzero",
        specimen::measured,
        specimen::worse,
        specimen::judge,
        specimen::preflight,
    )?])?;
    let report = mapped(run_all(
        &table,
        &declaration::invocation(macroonz::native_clock::source()),
    ))?;
    let path = std::path::Path::new("target/qualification/benchmark-workflow-example");
    std::fs::create_dir_all(path.parent().ok_or("output parent absent")?)
        .map_err(|error| error.to_string())?;
    std::fs::create_dir(path).map_err(|error| error.to_string())?;
    let root = mapped(StorageRoot::open(path))?;
    let name = mapped(StorageName::informed("counting"))?;
    let limits = RetentionLimits {
        archive: BenchArchiveLimits::declared(ArchiveLimits::declared(65_536, 16_384), 8, 8, 4, 32),
        storage: StorageLimits {
            artifacts: 1,
            bytes: 65_536,
        },
    };
    mapped(benchmark::retain(&report, &root, &name, limits))?;
    let historical = mapped(benchmark::load(&root, &name, limits))?;
    assert_eq!(historical.denominator(), 1);
    drop(root);
    std::fs::remove_dir_all(path).map_err(|error| error.to_string())?;
    mapped(bench_verdict(&report))
}

//! Complete historical benchmark records observed independently of the archive implementation.

mod claims;
mod declarations;
mod effects;
mod fields;
mod fixture;
mod hostile;
mod process;
mod types;
mod wire;

pub(super) const LIMITS: macroonz_harness::bench::archive::BenchArchiveLimits =
    macroonz_harness::bench::archive::BenchArchiveLimits::declared(
        macroonz_harness::report::archive::ArchiveLimits::declared(65536, 8192),
        8,
        8,
        4,
        32,
    );

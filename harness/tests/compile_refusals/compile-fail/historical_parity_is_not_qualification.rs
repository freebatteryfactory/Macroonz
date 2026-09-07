//! Historical parity cannot become a live reading or qualification.

use macroonz_harness::muterprater::interpretation_archive::ArchivedParity;
use macroonz_harness::muterprater::{NoMutationParityQualification, NoMutationParityReading};

fn qualification(historical: ArchivedParity) -> NoMutationParityQualification<'static, 'static, (), ()> {
    historical
}

fn reading(historical: ArchivedParity) -> NoMutationParityReading<'static, 'static, (), ()> {
    historical
}

fn main() {}

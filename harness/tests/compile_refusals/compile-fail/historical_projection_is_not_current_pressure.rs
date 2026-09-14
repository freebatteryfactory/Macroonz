//! Historical projection records cannot create current compiled pressure or selection.

use macroonz_harness::muterprater::specimen_archive::{ArchivedProjectionPressure, ArchivedSpecimenStanding};
use macroonz_harness::muterprater::discovery_archive::ArchivedSelection;
use macroonz_harness::muterprater::{ActiveSelection, CompiledProjectionPressure, CompiledSpecimenStanding};

fn pressure(historical: ArchivedProjectionPressure) -> CompiledProjectionPressure<'static, 'static, 'static, (), ()> {
    historical
}

fn standing(historical: ArchivedSpecimenStanding) -> CompiledSpecimenStanding {
    historical
}

fn selection(historical: ArchivedSelection) -> ActiveSelection {
    historical
}

fn main() {}

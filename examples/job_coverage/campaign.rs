use macroonz::harness::descriptor::{
    DerivedRevision, NamespacedName, PopulationRef, RevisionBinding,
};
use macroonz::harness::fuzz::{CoverageBudgets, CoverageCampaign, CoverageProfile};
use macroonz::harness::report::{ByteBudget, CaseBudget};
use std::path::Path;

pub(super) fn declared(directory: &Path, declaration: &Path) -> Result<CoverageCampaign, String> {
    let material = [
        std::fs::read(directory.join("fixtures/coverage-record.rs")).map_err(debug)?,
        std::fs::read(declaration).map_err(debug)?,
        std::fs::read(directory.join("Cargo.lock")).map_err(debug)?,
    ]
    .concat();
    Ok(CoverageCampaign::declared(
        PopulationRef::named("neutral-job", "record-inputs").map_err(debug)?,
        RevisionBinding::derived(DerivedRevision::from_material(&material)),
        CoverageProfile::declared(
            NamespacedName::named("neutral-job", "record-reader-lines").map_err(debug)?,
            1,
        ),
        CoverageBudgets::declared(
            CaseBudget::declared(3),
            ByteBudget::declared(18),
            4_194_304,
            4096,
            CaseBudget::declared(2),
            ByteBudget::declared(17),
        )
        .map_err(debug)?,
    ))
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}

//! Historical suite claims cannot mint current pressure, qualification or source custody.

use macroonz_harness::muterprater::backend_archive::{ArchivedBackendManifest, ArchivedSuitePressure};
use macroonz_harness::muterprater::{AdapterQualification, CompiledSuiteArtifactCustody, CompiledSuitePressure};

fn pressure(historical: ArchivedSuitePressure) -> CompiledSuitePressure {
    historical
}

fn qualification(historical: ArchivedSuitePressure) -> AdapterQualification {
    historical
}

fn custody(historical: ArchivedBackendManifest) -> CompiledSuiteArtifactCustody {
    historical
}

fn main() {}

//! A caller cannot attach current-source standing to an imported artifact without the exact source-revision comparison.

use macroonz_harness::muterprater::{
    CompiledSuiteArtifactCustody, CompiledSuiteArtifactManifest,
};

fn remint(manifest: CompiledSuiteArtifactManifest) -> CompiledSuiteArtifactCustody {
    CompiledSuiteArtifactCustody { manifest }
}

fn main() {}

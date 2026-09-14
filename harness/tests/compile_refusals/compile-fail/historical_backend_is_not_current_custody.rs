//! Historical backend data cannot replace current custody or adapter evidence.

use macroonz_harness::muterprater::backend_archive::{ArchivedBackendInvocation, ArchivedBackendManifest, ArchivedBackendSource, ArchivedAdapterProfile};
use macroonz_harness::muterprater::{AdapterProfile, AdapterQualification, ArtifactCustodyRefusal, CompiledSuiteArtifactCustody, CompiledSuiteArtifactManifest, GrammarStanding, MutationBackendInvocation, MutationSourceRevision, QualificationRefusal};

fn promote_manifest(historical: ArchivedBackendManifest) -> CompiledSuiteArtifactManifest { historical }
fn promote_profile(historical: ArchivedAdapterProfile) -> AdapterProfile { historical }
fn promote_invocation(historical: ArchivedBackendInvocation) -> MutationBackendInvocation { historical }
fn promote_source(historical: ArchivedBackendSource) -> MutationSourceRevision { historical }
fn borrow_custody(historical: ArchivedBackendManifest) -> Result<CompiledSuiteArtifactCustody, ArtifactCustodyRefusal> {
    CompiledSuiteArtifactCustody::current(historical, Vec::new())
}
fn borrow_qualification(historical: &ArchivedBackendManifest, standing: GrammarStanding) -> Result<AdapterQualification, QualificationRefusal> {
    AdapterQualification::of(historical, standing)
}

fn main() {}

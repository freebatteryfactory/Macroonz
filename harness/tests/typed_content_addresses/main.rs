//! The typed content-address reader contract, observed across every public wrapper.

use macroonz_harness::bench::BenchRowKey;
use macroonz_harness::corpus::SeedPackAddress;
use macroonz_harness::descriptor::{
    GeneratedSupportSchemaId, ProposalId, ReplayRef, RevisionBinding, TrialKey,
};
use macroonz_harness::generate::ByteSourceAddress;
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::muterprater::{
    AlternativeId, ArtifactContentId, BackendOutputId, EvaluationSurfaceId, MutantId,
    MutationDiscoveryId, MutationPolicyId, MutationSourceRevisionId,
};
use macroonz_harness::network::TranscriptAddress;
use macroonz_harness::report::{CheckRevisionId, RowRevisionId, SubjectRevisionId, TrialId};

fn value_reader<Type>(_reader: fn(Type) -> ContentAddress) {}

fn borrowed_reader<Type>(_reader: for<'value> fn(&'value Type) -> &'value ContentAddress) {}

#[test]
fn every_typed_address_keeps_its_established_reader_contract() {
    value_reader(BenchRowKey::address);
    value_reader(SeedPackAddress::address);
    value_reader(ProposalId::address);
    value_reader(ReplayRef::address);
    value_reader(TrialKey::address);
    value_reader(GeneratedSupportSchemaId::address);
    value_reader(ByteSourceAddress::address);
    value_reader(TranscriptAddress::address);
    value_reader(MutationPolicyId::address);
    value_reader(MutationDiscoveryId::address);
    value_reader(AlternativeId::address);
    value_reader(EvaluationSurfaceId::address);
    value_reader(BackendOutputId::address);
    value_reader(MutationSourceRevisionId::address);
    value_reader(ArtifactContentId::address);

    borrowed_reader(TrialId::address);
    borrowed_reader(RowRevisionId::address);
    borrowed_reader(SubjectRevisionId::address);
    borrowed_reader(CheckRevisionId::address);
    borrowed_reader(MutantId::address);
}

#[test]
fn both_reader_postures_return_the_exact_wrapped_address() {
    let address = ContentAddress::derived(
        DomainTag::declared("typed-address-reader", IdentityProfileVersion::declared(1)),
        b"typed-address-reader-vector",
    );
    assert_eq!(ProposalId::over(address).address(), address);

    let revision = SubjectRevisionId::of_binding(RevisionBinding::declared(address));
    assert_eq!(revision.address(), &address);
}

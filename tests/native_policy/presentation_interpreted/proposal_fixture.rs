//! Actual in-memory offers over the independently authored byte witness.

use super::fixture;
use crate::presentation_parity::{binding, datum, invocation, mapped, odd, pair_for};
use macroonz::harness::descriptor::{
    AuthoredTableName, Binding, Classification, DerivedRevision, ExecutableAttachment,
    ExecutionSuite, GeneratedSupportSchemaId, Origin, PopulationRef, Provenance, RevisionBinding,
    Role, Row, SynthesisFacts, Tag,
};
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz::harness::muterprater::{
    Demonstration, InterpreterAvailability, MutantKilledProposal, MutationWitness,
    NoMutationParityStanding, ProposalDestination, SpecimenMaterializerBinding,
    interpret::{availability, execute_active, observe_no_mutation, qualify_no_mutation},
    propose::{offer_mutant_kill, prove_candidate},
    specimen::demonstrate_compiled_projection,
};
use macroonz::harness::report::{
    Fingerprint, GenerationProfile, MinimizationProfile, ReplayCapsule, TrialConclusion,
};
use macroonz::harness::runner::{TrialBinding, TrialTable, trial_identity};

pub(super) fn candidate() -> Result<TrialBinding, String> {
    let original = binding()?;
    let row = original.row();
    let row = mapped(Row::declared(
        row.claim(),
        row.execution_suite(),
        mapped(Classification::authored(
            vec![
                mapped(Role::named("offer", "witness"))?,
                mapped(Role::named("offer", "control"))?,
            ],
            vec![mapped(Tag::named("offer", "<foreign>| tag"))?],
        ))?,
        row.subject(),
        row.check(),
        mapped(PopulationRef::named("offer", "candidate"))?,
        Origin::Candidate(SynthesisFacts::ProofGap),
    ))?;
    let original_attachment = original.attachment();
    let attachment = ExecutableAttachment::attached(
        row.subject(),
        row.check(),
        original_attachment.subject_revision(),
        original_attachment.check_revision(),
        |_invocation| odd(&datum(2)),
    );
    mapped(Binding::bound(row, attachment, Provenance::Unproduced))
}

pub(super) fn destination() -> Result<ProposalDestination, String> {
    Ok(ProposalDestination::naming(mapped(ExecutionSuite::named(
        "review",
        "destination",
    ))?))
}

pub(super) fn kill() -> Result<MutantKilledProposal, String> {
    let surface = fixture::surface()?;
    let pair = pair_for(&surface, fixture::KILLED)?;
    let input = datum(1);
    let candidate = candidate()?;
    let witness = mapped(MutationWitness::bound(
        candidate.clone(),
        candidate.row().check(),
        odd,
    ))?;
    let invocation = invocation();
    let NoMutationParityStanding::Qualified(parity) = qualify_no_mutation(mapped(
        observe_no_mutation(&pair, witness, &input, &invocation),
    )?) else {
        return Err("proposal no-mutation parity refused".into());
    };
    let selection = surface
        .selections()
        .last()
        .copied()
        .ok_or("selection missing")?;
    let projection = mapped(demonstrate_compiled_projection(
        &surface,
        &parity,
        &SpecimenMaterializerBinding::bound(&pair, fixture::MATERIALIZER),
        selection,
        &invocation,
        fixture::HOST,
    ))?;
    let suite = fixture::suite()?;
    let InterpreterAvailability::Available(trust) =
        availability(Some(&surface), Some(&suite), Some(&projection))
    else {
        return Err("proposal trust inputs did not join".into());
    };
    let evidence = mapped(execute_active(&trust, &invocation))?;
    let parent = mapped(TrialTable::authored(
        mapped(AuthoredTableName::named("offer", "parent"))?,
        Provenance::Unproduced,
        vec![binding()?],
    ))?;
    let demonstration = mapped(prove_candidate(
        &parent,
        candidate.clone(),
        evidence.mutation().target(),
        &invocation,
    ))?;
    let capsule = capsule(&demonstration)?;
    let prior = Fingerprint::of(
        trial_identity(binding()?.row()),
        demonstration.rejection().finding(),
    );
    mapped(offer_mutant_kill(
        candidate.row().clone(),
        evidence.mutation(),
        capsule,
        demonstration,
        vec![prior, prior],
        destination()?,
    ))
}

fn probe(bytes: &[u8]) -> ProbeOutcome {
    if bytes != [255u8, 0, 13] {
        return ProbeOutcome::NoFailure;
    }
    let Ok(candidate) = candidate() else {
        return ProbeOutcome::NoFailure;
    };
    match candidate.attachment().conclude(&invocation()) {
        TrialConclusion::Refused(finding) => {
            ProbeOutcome::Reproduced(Fingerprint::of(trial_identity(candidate.row()), &finding))
        }
        TrialConclusion::Passed => ProbeOutcome::NoFailure,
    }
}

fn capsule(demonstration: &Demonstration) -> Result<ReplayCapsule, String> {
    let revision = DerivedRevision::from_material(b"offer probe");
    let binding = mapped(ReductionProbeBinding::bound(
        demonstration.trial_report(),
        GenerationProfile::declared("offer-generation", 4),
        GeneratedSupportSchemaId::over(revision.revision()),
        RevisionBinding::derived(revision),
        probe,
    ))?;
    let plan = mapped(ReductionPlan::declared(
        MinimizationProfile::declared("offer-minimization", 5),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(1),
    ))?;
    Ok(capture_replay(&mapped(reduce(
        &plan,
        &[255, 0, 13],
        &binding,
    ))?))
}

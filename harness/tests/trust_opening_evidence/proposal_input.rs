//! Actual typed candidate execution, with complete census and refusal controls.
//!
//! The imported target supplies selection coordinates only; these controls establish no fresh backend activation or human admission.

use super::support::{
    BACKEND_CONSOLE, CompiledRosterMeaning, OWNER, check, compiled_reading, trial_binding_with,
};
use arbitrary::Unstructured;
use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    AuthoredTableName, DerivedRevision, MutationPointRef, NamespacedName, Origin, Provenance,
    RevisionBinding, StagedTableRefusal, SynthesisFacts, TablePosture,
};
use macroonz_harness::input::{BoundInput, InputBinding, InputLimits, InputProfile, pack};
use macroonz_harness::muterprater::propose::prove_candidate;
use macroonz_harness::muterprater::wrap::read_output;
use macroonz_harness::muterprater::{BackendVersionPosture, MutationTarget, ProofRefusal};
use macroonz_harness::report::{
    ByteBudget, CacheEligibility, CaseBudget, InvocationProfile, NotSelectedReason, ReplayPosture,
    SelectionDisposition, TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity,
    TrialConclusion, TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding, TrialTable, trial_identity};
use std::cell::RefCell;

struct Specimen {
    seed: u8,
    trace: RefCell<Vec<u16>>,
}

fn decode(source: &mut Unstructured<'_>) -> arbitrary::Result<Specimen> {
    let [seed] = source.bytes(1)? else {
        return Err(arbitrary::Error::NotEnoughData);
    };
    Ok(Specimen {
        seed: *seed,
        trace: RefCell::new(Vec::new()),
    })
}

fn revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(
        &[
            include_bytes!("proposal_input.rs").as_slice(),
            include_bytes!("support.rs").as_slice(),
        ]
        .concat(),
    ))
}

fn invocation(
    seed: u8,
    cases: u32,
    bytes: u64,
    decoder: RevisionBinding,
) -> Result<Invocation<BoundInput<Specimen>>, ()> {
    let profile = InputProfile::declared(
        NamespacedName::named(OWNER, "candidate-seed").map_err(|_| ())?,
        1,
        DerivedRevision::from_material(b"one byte").revision(),
    );
    let input = InputBinding::declared(profile, decoder, decode)
        .decode(pack(profile, &[seed], InputLimits::declared(256, 1)).map_err(|_| ())?)
        .map_err(|_| ())?;
    Ok(Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(cases),
            ByteBudget::declared(bytes),
            TimeBudget::declared(0),
        ),
        TargetBinding::bound(
            TargetTriple::declared("synthetic-candidate-target"),
            ToolchainIdentity::declared("synthetic-candidate-toolchain"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "typed-candidate"),
        HarnessClock::unavailable(),
    )
    .with_input(input))
}

fn candidate_call(invocation: &Invocation<BoundInput<Specimen>>) -> TrialConclusion {
    let input = invocation.input().value();
    input.trace.borrow_mut().push(u16::from(input.seed) * 3);
    if input.seed == 0 {
        TrialConclusion::Passed
    } else {
        check(&CompiledRosterMeaning::Unstated)
    }
}

fn parent_call(invocation: &Invocation<BoundInput<Specimen>>) -> TrialConclusion {
    let input = invocation.input().value();
    input.trace.borrow_mut().push(u16::from(input.seed) + 1);
    TrialConclusion::Passed
}

fn candidate(stem: &'static str) -> Result<TrialBinding<BoundInput<Specimen>>, ()> {
    trial_binding_with(
        stem,
        Origin::Candidate(SynthesisFacts::Survivor(
            MutationPointRef::named(OWNER, "comparison-edge").map_err(|_| ())?,
        )),
        revision(),
        candidate_call,
    )
    .map_err(|_| ())
}

fn parent(stem: &'static str) -> Result<TrialTable<BoundInput<Specimen>>, ()> {
    TrialTable::authored(
        AuthoredTableName::named(OWNER, "typed-parent").map_err(|_| ())?,
        Provenance::Unproduced,
        vec![
            trial_binding_with(stem, Origin::HandWritten, revision(), parent_call)
                .map_err(|_| ())?,
        ],
    )
    .map_err(|_| ())
}

fn target() -> Result<MutationTarget, ()> {
    Ok(compiled_reading()
        .map_err(|_| ())?
        .run()
        .reports()
        .first()
        .ok_or(())?
        .target()
        .clone())
}

#[test]
fn distinct_candidate_inputs_retain_actual_refusals_and_the_whole_parent() -> Result<(), ()> {
    let parent = parent("parent-behaviour")?;
    let binding = candidate("comparison-behaviour")?;
    let trial = trial_identity(binding.row());
    let target = target()?;
    let first = invocation(2, 1, 1, revision())?;
    let second = invocation(7, 1, 1, revision())?;
    let before = prove_candidate(&parent, binding.clone(), &target, &first).map_err(|_| ())?;
    let after = prove_candidate(&parent, binding, &target, &second).map_err(|_| ())?;
    assert_eq!(*first.input().value().trace.borrow(), vec![6]);
    assert_eq!(*second.input().value().trace.borrow(), vec![21]);
    assert_eq!(
        before.rejection().fingerprint(),
        after.rejection().fingerprint()
    );
    assert_ne!(
        before.trial_report().standing().key().address(),
        after.trial_report().standing().key().address()
    );
    for (demonstration, invocation) in [(&before, &first), (&after, &second)] {
        let report = demonstration.report();
        assert_eq!(
            report.posture(),
            TablePosture::Staged {
                parent: parent.name()
            }
        );
        assert_eq!(report.denominator(), 2);
        assert_eq!(report.input(), invocation.input_standing());
        let [authored, overlaid] = report.census() else {
            return Err(());
        };
        assert_eq!(
            authored.trial(),
            trial_identity(parent.bindings().first().ok_or(())?.row())
        );
        assert!(matches!(
            authored.disposition(),
            SelectionDisposition::NotSelected {
                trial: _,
                reason: NotSelectedReason::OutsideSelection
            }
        ));
        assert_eq!(overlaid.trial(), trial);
        assert_eq!(
            overlaid.disposition().report(),
            Some(demonstration.trial_report())
        );
        assert_eq!(
            demonstration.trial_report().standing().key().input(),
            invocation.input_standing()
        );
        assert_eq!(demonstration.rejection().trial(), trial);
    }
    Ok(())
}

#[test]
fn unmapped_targets_execute_the_authored_parent_before_the_candidate() -> Result<(), ()> {
    let reading = read_output(
        BACKEND_CONSOLE,
        BackendVersionPosture::Unstated,
        |_| None,
        |_, _| None,
    )
    .map_err(|_| ())?;
    let target = reading.run().reports().first().ok_or(())?.target();
    assert_eq!(target.owning_claim(), None);
    let invocation = invocation(7, 1, 1, revision())?;
    let demonstration = prove_candidate(
        &parent("parent-behaviour")?,
        candidate("comparison-behaviour")?,
        target,
        &invocation,
    )
    .map_err(|_| ())?;
    assert_eq!(*invocation.input().value().trace.borrow(), vec![8, 21]);
    assert_eq!(demonstration.report().denominator(), 2);
    assert!(
        demonstration
            .report()
            .census()
            .iter()
            .all(|row| row.disposition().report().is_some())
    );
    Ok(())
}

#[test]
fn skipped_passing_and_unselected_candidates_cannot_demonstrate_a_rejection() -> Result<(), ()> {
    let parent = parent("parent-behaviour")?;
    let target = target()?;
    for (seed, cases, bytes, stem, expected, trace) in [
        (
            7,
            0,
            1,
            "comparison-behaviour",
            ProofRefusal::CandidateDidNotExecute,
            vec![],
        ),
        (
            7,
            1,
            0,
            "comparison-behaviour",
            ProofRefusal::CandidateDidNotExecute,
            vec![],
        ),
        (
            0,
            1,
            1,
            "comparison-behaviour",
            ProofRefusal::CandidateDidNotRefuse,
            vec![0],
        ),
        (
            7,
            1,
            1,
            "other-candidate",
            ProofRefusal::CandidateNotSelected,
            vec![],
        ),
    ] {
        let invocation = invocation(seed, cases, bytes, revision())?;
        assert_eq!(
            prove_candidate(&parent, candidate(stem)?, &target, &invocation).err(),
            Some(expected)
        );
        assert_eq!(*invocation.input().value().trace.borrow(), trace);
    }
    Ok(())
}

#[test]
fn staging_refusals_do_no_typed_candidate_work() -> Result<(), ()> {
    let parent = parent("comparison-behaviour")?;
    let invocation = invocation(7, 1, 1, revision())?;
    let candidate = candidate("comparison-behaviour")?;
    let key = candidate.trial_key();
    assert_eq!(
        prove_candidate(&parent, candidate, &target()?, &invocation).err(),
        Some(ProofRefusal::StagingRefused(
            StagedTableRefusal::DuplicateTrial(key)
        ))
    );
    assert_eq!(
        prove_candidate(
            &parent,
            parent.bindings().first().ok_or(())?.clone(),
            &target()?,
            &invocation
        )
        .err(),
        Some(ProofRefusal::StagingRefused(
            StagedTableRefusal::NotACandidate(key)
        ))
    );
    assert!(invocation.input().value().trace.borrow().is_empty());
    Ok(())
}

#[test]
fn a_typed_demonstration_keeps_its_decoder_ceiling() -> Result<(), ()> {
    let parent = parent("parent-behaviour")?;
    let target = target()?;
    let derived = revision();
    for (decoder, replay, cache) in [
        (
            derived,
            ReplayPosture::ExactDerived,
            CacheEligibility::Eligible,
        ),
        (
            RevisionBinding::declared(derived.revision()),
            ReplayPosture::DeclaredByAuthor,
            CacheEligibility::NeverEligible,
        ),
        (
            RevisionBinding::untracked(derived.revision()),
            ReplayPosture::UnavailableBecauseUntracked,
            CacheEligibility::NeverEligible,
        ),
    ] {
        let invocation = invocation(7, 1, 1, decoder)?;
        let demonstration = prove_candidate(
            &parent,
            candidate("comparison-behaviour")?,
            &target,
            &invocation,
        )
        .map_err(|_| ())?;
        let standing = demonstration.trial_report().standing();
        assert_eq!(standing.replay(), replay);
        assert_eq!(standing.cache_eligibility(), cache);
        assert_eq!(standing.key().input(), invocation.input_standing());
        assert_eq!(*invocation.input().value().trace.borrow(), vec![21]);
    }
    Ok(())
}

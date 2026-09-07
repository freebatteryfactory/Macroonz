//! Typed input crosses the actual runner, with independent traces and execution-standing controls.

use super::support::{binding, invocation, passes, refused, world};
use arbitrary::Unstructured;
use macroonz_harness::clock::{ClockReadRefusal, HarnessClock, MeasurementReading};
use macroonz_harness::descriptor::{DerivedRevision, NamespacedName, RevisionBinding};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::input::{BoundInput, InputBinding, InputLimits, InputProfile, pack};
use macroonz_harness::report::{
    Baseline, ByteBudget, CacheEligibility, CaseBudget, Fingerprint, InvocationProfile,
    ReplayPosture, ReportComparison, RunAttempt, SelectionOutcome, SkipReason, TimeBudget,
    TrialConclusion, TrialSite, compare,
};
use macroonz_harness::runner::{
    Invocation, Selection, SelectionPlan, execution_key, lens_verdict, run_all, run_one,
    seat_verdict, trial_identity,
};
use std::cell::RefCell;
use std::collections::BTreeSet;

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

fn profile(version: u32) -> Result<InputProfile, ()> {
    Ok(InputProfile::declared(
        NamespacedName::named("runner-input", "seed").map_err(|_| ())?,
        version,
        ContentAddress::derived(
            DomainTag::declared("runner-input-schema", IdentityProfileVersion::declared(1)),
            b"one byte",
        ),
    ))
}

fn revision() -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(include_bytes!(
        "input_execution.rs"
    )))
}

fn input(seed: u8, version: u32, revision: RevisionBinding) -> Result<BoundInput<Specimen>, ()> {
    let profile = profile(version)?;
    InputBinding::declared(profile, revision, decode)
        .decode(pack(profile, &[seed], InputLimits::declared(256, 1)).map_err(|_| ())?)
        .map_err(|_| ())
}

fn increment(invocation: &Invocation<BoundInput<Specimen>>) -> TrialConclusion {
    let input = invocation.input().value();
    input.trace.borrow_mut().push(u16::from(input.seed) + 1);
    TrialConclusion::Passed
}

fn triple(invocation: &Invocation<BoundInput<Specimen>>) -> TrialConclusion {
    let input = invocation.input().value();
    input.trace.borrow_mut().push(u16::from(input.seed) * 3);
    TrialConclusion::Passed
}

#[test]
fn both_witnesses_reach_every_subject_and_keep_trial_identity() -> Result<(), ()> {
    let table = world(vec![
        binding("increment", increment).map_err(|_| ())?,
        binding("triple", triple).map_err(|_| ())?,
    ])
    .map_err(|_| ())?;
    let first = invocation().with_input(input(2, 1, revision())?);
    let second = invocation().with_input(input(7, 1, revision())?);
    let selection = SelectionPlan::of(Selection::All);
    let before = run_all(&table.view(), &selection, &first);
    let after = run_all(&table.view(), &selection, &second);
    assert_eq!(*first.input().value().trace.borrow(), vec![3, 6]);
    assert_eq!(*second.input().value().trace.borrow(), vec![8, 21]);
    assert!(seat_verdict(&before).is_ok());
    assert!(seat_verdict(&after).is_ok());
    assert_eq!(before.denominator(), 2);
    assert_eq!(after.denominator(), 2);
    for (left, right) in before.census().iter().zip(after.census()) {
        assert_eq!(left.trial(), right.trial());
        let left = left.disposition().report().ok_or(())?;
        let right = right.disposition().report().ok_or(())?;
        assert_ne!(
            left.standing().key().address(),
            right.standing().key().address()
        );
        assert_eq!(left.standing().key().input(), before.input());
        assert_eq!(right.standing().key().input(), after.input());
    }
    let ReportComparison::Compared(diff) = compare(Baseline::Previous(&before), &after) else {
        return Err(());
    };
    let moved = diff.execution().input().ok_or(())?;
    assert_eq!(*moved.before(), before.input());
    assert_eq!(*moved.after(), after.input());
    assert!(diff.execution().flips().is_empty());
    assert!(diff.population().revised().is_empty());
    Ok(())
}

#[test]
fn selection_keeps_the_full_census_and_absence_is_not_execution() -> Result<(), ()> {
    let selected = binding("increment", increment).map_err(|_| ())?;
    let trial = trial_identity(selected.row());
    let table =
        world(vec![selected, binding("triple", triple).map_err(|_| ())?]).map_err(|_| ())?;
    let invocation = invocation().with_input(input(2, 1, revision())?);
    let report = run_all(
        &table.view(),
        &SelectionPlan::of(Selection::ByTrialIds(BTreeSet::from([trial]))),
        &invocation,
    );
    assert_eq!(*invocation.input().value().trace.borrow(), vec![3]);
    assert_eq!(report.denominator(), 2);
    assert_eq!(
        report
            .census()
            .iter()
            .filter(|entry| entry.disposition().report().is_some())
            .count(),
        1
    );
    let absent = run_all(
        &table.view(),
        &SelectionPlan::of(Selection::ByTrialIds(BTreeSet::new())),
        &invocation,
    );
    assert_eq!(
        absent.selection(),
        SelectionOutcome::UnsatisfiedByEmptySelection
    );
    assert!(seat_verdict(&absent).is_err());
    assert_eq!(absent.input(), report.input());
    assert_eq!(*invocation.input().value().trace.borrow(), vec![3]);
    Ok(())
}

fn refusing_clock() -> Result<u64, ClockReadRefusal> {
    Err(ClockReadRefusal::Refused)
}

#[test]
fn case_and_byte_budgets_refuse_before_the_subject_or_clock() -> Result<(), ()> {
    let binding = binding("increment", increment).map_err(|_| ())?;
    for (cases, bytes) in [(0, 1), (1, 0), (1, 1)] {
        let facts = invocation();
        let invocation = Invocation::declared(
            InvocationProfile::declared(
                CaseBudget::declared(cases),
                ByteBudget::declared(bytes),
                TimeBudget::declared(0),
            ),
            facts.target().clone(),
            facts.site(),
            if cases == 1 && bytes == 1 {
                HarnessClock::unavailable()
            } else {
                HarnessClock::fallible(refusing_clock)
            },
        )
        .with_input(input(2, 1, revision())?);
        let report = run_one(&binding, &invocation);
        assert_eq!(report.measurement(), MeasurementReading::Unavailable);
        if cases == 1 && bytes == 1 {
            assert_eq!(*invocation.input().value().trace.borrow(), vec![3]);
            assert!(lens_verdict(&report).is_ok());
        } else {
            assert!(invocation.input().value().trace.borrow().is_empty());
            assert_eq!(
                report.attempt(),
                &RunAttempt::SkippedWithReason(SkipReason::BudgetExhausted)
            );
            assert!(lens_verdict(&report).is_err());
        }
    }
    Ok(())
}

#[test]
fn decoder_posture_limits_the_complete_replay_and_cache_standing() -> Result<(), ()> {
    let binding = binding("increment", increment).map_err(|_| ())?;
    let derived = revision();
    let mut keys = BTreeSet::new();
    for (revision, replay, cache) in [
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
        let invocation = invocation().with_input(input(2, 1, revision)?);
        let report = run_one(&binding, &invocation);
        assert_eq!(report.standing().replay(), replay);
        assert_eq!(report.standing().cache_eligibility(), cache);
        assert!(keys.insert(report.standing().key().address()));
        assert_eq!(
            report.standing().key().input().ok_or(())?.decoder(),
            derived.revision()
        );
    }
    Ok(())
}

fn below_five(invocation: &Invocation<BoundInput<Specimen>>) -> TrialConclusion {
    if invocation.input().value().seed < 5 {
        TrialConclusion::Passed
    } else {
        refused()
    }
}

#[test]
fn a_passing_other_witness_cannot_discharge_the_saved_failure() -> Result<(), ()> {
    let binding = binding("below-five", below_five).map_err(|_| ())?;
    let passed = run_one(&binding, &invocation().with_input(input(2, 1, revision())?));
    let first = run_one(&binding, &invocation().with_input(input(7, 1, revision())?));
    let second = run_one(&binding, &invocation().with_input(input(9, 1, revision())?));
    assert!(lens_verdict(&passed).is_ok());
    assert!(lens_verdict(&first).is_err());
    assert!(lens_verdict(&second).is_err());
    let RunAttempt::Executed(TrialConclusion::Refused(left)) = first.attempt() else {
        return Err(());
    };
    let RunAttempt::Executed(TrialConclusion::Refused(right)) = second.attempt() else {
        return Err(());
    };
    assert_eq!(
        Fingerprint::of(first.trial(), left),
        Fingerprint::of(second.trial(), right)
    );
    assert_ne!(
        first.standing().key().address(),
        second.standing().key().address()
    );
    assert_ne!(
        first.standing().key().address(),
        passed.standing().key().address()
    );
    Ok(())
}

#[test]
fn unit_input_keeps_its_declared_budget_and_clone_surface() -> Result<(), ()> {
    let binding = binding("unit", passes).map_err(|_| ())?;
    let facts = invocation();
    let unit = Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(0),
            ByteBudget::declared(0),
            TimeBudget::declared(0),
        ),
        facts.target().clone(),
        facts.site(),
        facts.clock(),
    );
    let report = run_one(&binding, &unit.clone());
    assert!(lens_verdict(&report).is_ok());
    assert_eq!(report.standing().key().input(), None);
    Ok(())
}

#[test]
fn input_profile_and_decoder_changes_are_visible_even_without_selected_rows() -> Result<(), ()> {
    let table = world(vec![binding("increment", increment).map_err(|_| ())?]).map_err(|_| ())?;
    let selection = SelectionPlan::of(Selection::ByTrialIds(BTreeSet::new()));
    let before = run_all(
        &table.view(),
        &selection,
        &invocation().with_input(input(2, 1, revision())?),
    );
    for input in [
        input(2, 2, revision())?,
        input(
            2,
            1,
            RevisionBinding::derived(DerivedRevision::from_material(b"changed decoder")),
        )?,
    ] {
        let after = run_all(&table.view(), &selection, &invocation().with_input(input));
        let ReportComparison::Compared(diff) = compare(Baseline::Previous(&before), &after) else {
            return Err(());
        };
        assert!(diff.execution().input().is_some());
        assert!(diff.execution().revisions().is_empty());
        assert!(diff.execution().flips().is_empty());
        assert_eq!(after.denominator(), 1);
    }
    Ok(())
}

#[test]
fn moving_the_invocation_site_preserves_input_execution_identity() -> Result<(), ()> {
    let binding = binding("increment", increment).map_err(|_| ())?;
    let facts = invocation();
    let moved = Invocation::declared(
        facts.profile(),
        facts.target().clone(),
        TrialSite::located("moved", "elsewhere.rs", 8, "renamed"),
        facts.clock(),
    );
    let before = run_one(&binding, &facts.with_input(input(2, 1, revision())?));
    let after = run_one(&binding, &moved.with_input(input(2, 1, revision())?));
    assert_eq!(before.standing(), after.standing());
    assert_ne!(before.site(), after.site());
    Ok(())
}

#[test]
fn input_execution_domain_preserves_unit_bytes_and_extends_an_independent_vector() -> Result<(), ()>
{
    let unit_binding = binding("increment", passes).map_err(|_| ())?;
    let typed_binding = binding("increment", increment).map_err(|_| ())?;
    let facts = invocation();
    let unit = execution_key(&unit_binding, &facts.clone());
    let typed = execution_key(&typed_binding, &facts.with_input(input(2, 1, revision())?));
    let mut expected = Vec::new();
    for address in [
        unit.trial().address(),
        unit.subject().address(),
        unit.check().address(),
    ] {
        expected.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x20");
        expected.extend_from_slice(address.as_bytes());
    }
    expected.extend_from_slice(
        b"\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x40\x00\x00\x00\x00\x00\x0f\x42\x40",
    );
    expected.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x16runner-evidence-target\x00\x00\x00\x00\x00\x00\x00\x0crustc-1.98.0");
    assert_eq!(
        unit.address().as_bytes(),
        &blake3::derive_key("macroonz/harness-identity/execution-key/v1", &expected)
    );
    let standing = typed.input().ok_or(())?;
    for address in [standing.case().address(), standing.decoder()] {
        expected.extend_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x20");
        expected.extend_from_slice(address.as_bytes());
    }
    expected.push(0);
    assert_eq!(
        typed.address().as_bytes(),
        &blake3::derive_key(
            "macroonz/harness-identity/input-execution-key/v1",
            &expected
        )
    );
    assert_ne!(unit.address(), typed.address());
    Ok(())
}

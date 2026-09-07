//! Complete reduction inspection remains historical across byte and process boundaries.

use super::{
    fixture, process,
    reduction_vector::ReductionVector,
    vector::{InputKind, hash},
};
use macroonz_harness::descriptor::{DerivedRevision, RevisionBinding};
use macroonz_harness::generate::reduce::archive::{
    ArchivedReduction, ReductionArchiveLimits, ReductionArchiveRefusal, read_reduction,
    retain_reduction,
};
use macroonz_harness::generate::{
    ByteReducerExecution, ByteReducerId, ReductionHalt, SemanticCandidateRefusal,
    SemanticCandidates, SemanticReducerBinding, SemanticReducerId, capture_replay,
};
use macroonz_harness::report::ReplayPosture;
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};
use std::error::Error;
use std::io::Read as _;

pub(super) const LIMITS: ReductionArchiveLimits =
    ReductionArchiveLimits::declared(ArchiveLimits::declared(8192, 4096), 8);

fn candidates(input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    SemanticCandidates::proposed(input, vec![vec![1, 2], vec![1]])
}

fn empty(input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    SemanticCandidates::proposed(input, Vec::new())
}

fn semantic(
    stem: &'static str,
    revision: RevisionBinding,
    call: fn(&[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal>,
) -> Result<SemanticReducerBinding, ()> {
    Ok(SemanticReducerBinding::bound(
        SemanticReducerId::named("archive", stem).map_err(|_| ())?,
        revision,
        call,
    ))
}

#[test]
fn independent_reduction_vectors_preserve_the_complete_historical_account() -> Result<(), ()> {
    for kind in [InputKind::Unit, InputKind::Bound] {
        let vector = ReductionVector::declared(kind);
        let owned = read_reduction(&vector.encoded(), LIMITS).map_err(|_| ())?;
        assert_eq!(
            owned.address().as_bytes(),
            &hash("historical-reduction/v1", &vector.body())
        );
        assert_eq!(owned.capsule().encoded(), vector.capsule.encoded());
        assert_eq!(owned.budget().probes(), 20);
        assert_eq!(owned.report_posture(), ReplayPosture::ExactDerived);
        assert_eq!(owned.probe_revision().as_bytes(), &[9; 32]);
        assert_eq!(owned.probe_posture(), ReplayPosture::DeclaredByAuthor);
        assert_eq!(owned.census().accepted(), 3);
        assert_eq!(owned.census().fingerprint_moved(), 4);
        assert_eq!(owned.census().no_failure(), 5);
        assert_eq!(owned.census().probes(), 12);
        assert_eq!(owned.halt(), ReductionHalt::FixedPointReached);
        assert_eq!(
            owned.byte_reducer(),
            ByteReducerExecution::Executed(ByteReducerId::ChunkRemovalAndZeroing)
        );
        let [first, second] = owned.semantic_reducers() else {
            return Err(());
        };
        assert_eq!(first.name().namespace(), "outside");
        assert_eq!(first.name().stem(), "first");
        assert_eq!(second.name().stem(), "second");
        assert_eq!(first.revision().as_bytes(), &[8; 32]);
        assert_eq!(first.claimed_posture(), ReplayPosture::ExactDerived);
        assert_eq!((first.candidates(), first.probes()), (3, 3));
        assert_eq!((second.candidates(), second.probes()), (2, 2));
        drop(vector);
        assert_eq!(owned.capsule().input(), &[1]);
    }
    Ok(())
}

#[test]
fn actual_budget_is_retained_without_renaming_an_equal_capsule() -> Result<(), ()> {
    let first = fixture::reduction(&[1], 2, Vec::new())?;
    let second = fixture::reduction(&[1], 20, Vec::new())?;
    assert_eq!(first.budget().probes(), 2);
    assert_eq!(second.budget().probes(), 20);
    assert_eq!(first.outcome(), second.outcome());
    assert_eq!(first.outcome().census().probes(), 2);
    assert_eq!(first.outcome().halt(), ReductionHalt::FixedPointReached);
    assert_eq!(capture_replay(&first), capture_replay(&second));
    let a = retain_reduction(&first, LIMITS).map_err(|_| ())?;
    let b = retain_reduction(&second, LIMITS).map_err(|_| ())?;
    assert_eq!(a.capsule(), b.capsule());
    assert_ne!(a.address(), b.address());
    assert_eq!(a.budget().probes(), 2);
    assert_eq!(b.budget().probes(), 20);
    drop(first);
    drop(second);
    assert_eq!(read_reduction(a.encoded(), LIMITS).map_err(|_| ())?, a);
    Ok(())
}

#[test]
fn actual_unit_and_typed_reductions_preserve_original_case_and_reached_witness() -> Result<(), ()> {
    let unit = fixture::unit_reduction(&[1, 2, 3], 32)?;
    let typed = fixture::reduction(&[1, 2, 3], 32, Vec::new())?;
    for evidence in [&unit, &typed] {
        let archived = retain_reduction(evidence, LIMITS).map_err(|_| ())?;
        assert_eq!(archived.capsule().input(), &[1]);
        assert_eq!(
            archived.capsule().key().address(),
            evidence.standing().key().address()
        );
        assert_eq!(
            archived.probe_revision().as_bytes(),
            evidence.probe_revision().revision().as_bytes()
        );
        assert_eq!(
            archived.census().accepted(),
            evidence.outcome().census().accepted()
        );
        assert_eq!(
            archived.census().fingerprint_moved(),
            evidence.outcome().census().fingerprint_moved()
        );
        assert_eq!(
            archived.census().no_failure(),
            evidence.outcome().census().no_failure()
        );
        assert_eq!(
            archived.census().probes(),
            evidence.outcome().census().probes()
        );
    }
    assert!(
        retain_reduction(&unit, LIMITS)
            .map_err(|_| ())?
            .capsule()
            .key()
            .input()
            .is_none()
    );
    let record = retain_reduction(&typed, LIMITS).map_err(|_| ())?;
    let original = fixture::report(&[1, 2, 3])?;
    let witness = fixture::report(&[1])?;
    assert_eq!(
        record.capsule().key().input().ok_or(())?.case().as_bytes(),
        original
            .standing()
            .key()
            .input()
            .ok_or(())?
            .case()
            .address()
            .as_bytes()
    );
    assert_ne!(
        record.capsule().key().address(),
        witness.standing().key().address()
    );
    Ok(())
}

#[test]
fn semantic_budget_stop_and_invoked_empty_reducers_keep_distinct_custody() -> Result<(), ()> {
    let revision = RevisionBinding::derived(DerivedRevision::from_material(include_bytes!(
        "reductions.rs"
    )));
    let untracked = RevisionBinding::untracked(revision.revision());
    let record = fixture::reduction(
        &[1, 2, 3],
        1,
        vec![
            semantic("first", revision, candidates)?,
            semantic("not-reached", untracked, empty)?,
        ],
    )?;
    let saved = retain_reduction(&record, LIMITS).map_err(|_| ())?;
    let [first] = saved.semantic_reducers() else {
        return Err(());
    };
    assert_eq!((first.candidates(), first.probes()), (2, 1));
    assert_eq!(saved.capsule().input(), &[1, 2]);
    assert_eq!(
        saved.capsule().claimed_posture(),
        ReplayPosture::ExactDerived
    );
    assert_eq!(
        saved.byte_reducer(),
        ByteReducerExecution::NotReachedBecauseBudgetSpent
    );
    assert_eq!(saved.halt(), ReductionHalt::BudgetExhausted);
    let empty_record =
        fixture::reduction(&[1], 20, vec![semantic("invoked-empty", untracked, empty)?])?;
    let empty_saved = retain_reduction(&empty_record, LIMITS).map_err(|_| ())?;
    let [called] = empty_saved.semantic_reducers() else {
        return Err(());
    };
    assert_eq!((called.candidates(), called.probes()), (0, 0));
    assert_eq!(
        empty_saved.capsule().claimed_posture(),
        ReplayPosture::UnavailableBecauseUntracked
    );
    assert_eq!(
        empty_saved.byte_reducer(),
        ByteReducerExecution::Executed(ByteReducerId::ChunkRemovalAndZeroing)
    );
    assert_eq!(empty_saved.halt(), ReductionHalt::FixedPointReached);
    Ok(())
}

#[test]
fn actual_generic_budget_stop_is_not_an_unreached_generic_phase() -> Result<(), ()> {
    let evidence = fixture::reduction(&[1, 2, 3], 1, Vec::new())?;
    let record = retain_reduction(&evidence, LIMITS).map_err(|_| ())?;
    assert_eq!(record.census().probes(), 1);
    assert_eq!(record.halt(), ReductionHalt::BudgetExhausted);
    assert_eq!(
        record.byte_reducer(),
        ByteReducerExecution::Executed(ByteReducerId::ChunkRemovalAndZeroing)
    );
    Ok(())
}

#[test]
fn retention_and_loading_obey_independent_envelope_field_and_roster_bounds() -> Result<(), ()> {
    let evidence = fixture::reduction(&[1, 2, 3], 32, Vec::new())?;
    let record = retain_reduction(&evidence, LIMITS).map_err(|_| ())?;
    let exact = ReductionArchiveLimits::declared(
        ArchiveLimits::declared(record.encoded().len(), record.capsule().encoded().len()),
        0,
    );
    assert_eq!(retain_reduction(&evidence, exact).map_err(|_| ())?, record);
    assert_eq!(
        read_reduction(record.encoded(), exact).map_err(|_| ())?,
        record
    );
    for (bytes, refusal) in [
        (
            ArchiveLimits::declared(record.encoded().len().saturating_sub(1), 4096),
            ArchiveRefusal::EnvelopeTooLarge,
        ),
        (
            ArchiveLimits::declared(8192, record.capsule().encoded().len().saturating_sub(1)),
            ArchiveRefusal::FieldTooLarge,
        ),
    ] {
        let limits = ReductionArchiveLimits::declared(bytes, 0);
        assert_eq!(
            retain_reduction(&evidence, limits),
            Err(ReductionArchiveRefusal::Archive(refusal))
        );
        assert_eq!(
            read_reduction(record.encoded(), limits),
            Err(ReductionArchiveRefusal::Archive(refusal))
        );
    }
    let with_semantic = fixture::reduction(
        &[1],
        20,
        vec![semantic("empty", fixture::revision(), empty)?],
    )?;
    assert_eq!(
        retain_reduction(&with_semantic, exact),
        Err(ReductionArchiveRefusal::TooManyReducers)
    );
    let saved = retain_reduction(&with_semantic, LIMITS).map_err(|_| ())?;
    let no_rows = ReductionArchiveLimits::declared(LIMITS.bytes(), 0);
    assert_eq!(
        read_reduction(saved.encoded(), no_rows),
        Err(ReductionArchiveRefusal::TooManyReducers)
    );
    Ok(())
}

#[test]
#[ignore = "driven by the reduction process-boundary claim"]
fn child_loads_reduction() -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin().lock().take(8193).read_to_end(&mut bytes)?;
    let record =
        read_reduction(&bytes, LIMITS).map_err(|r| std::io::Error::other(format!("{r:?}")))?;
    drop(bytes);
    process::publish(record.encoded())
}

#[test]
fn actual_and_independent_reductions_survive_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let evidence = fixture::reduction(&[1, 2, 3], 32, Vec::new())
        .map_err(|()| std::io::Error::other("fixture refused"))?;
    let record =
        retain_reduction(&evidence, LIMITS).map_err(|r| std::io::Error::other(format!("{r:?}")))?;
    drop(evidence);
    for encoded in [
        record.encoded().to_vec(),
        ReductionVector::declared(InputKind::Unit).encoded(),
        ReductionVector::declared(InputKind::Bound).encoded(),
    ] {
        let returned = process::round_trip(&encoded, "archive::reductions::child_loads_reduction")?;
        assert_eq!(returned, encoded);
        let owned: ArchivedReduction = read_reduction(&returned, LIMITS)
            .map_err(|r| std::io::Error::other(format!("{r:?}")))?;
        assert_eq!(owned.encoded(), encoded);
    }
    Ok(())
}

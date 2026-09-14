//! Hostile mutation fields cannot hide behind recomputed envelope integrity.

use super::mutation_vector::MutationVector;
use super::trial_vector;
use macroonz_harness::muterprater::verdict_archive::{
    ArchivedMutationOutcome, ArchivedRejection, MutationArchiveRefusal, read_mutation,
};
use macroonz_harness::report::FOREIGN_TEXT_MAX_BYTES;
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal, ArchivedTruncation};

const LIMITS: ArchiveLimits = ArchiveLimits::declared(65_536, 32_768);

#[test]
fn unsupported_mutation_headers_and_axis_slots_refuse() {
    for (header, refusal) in [
        ([2, 1, 0], ArchiveRefusal::UnsupportedFormat { found: 2 }),
        ([1, 2, 0], ArchiveRefusal::WrongKind { found: 2 }),
        ([1, 1, 1], ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        let mut vector = MutationVector::demonstrated();
        vector.header = header;
        assert_eq!(
            read_mutation(&vector.encoded(), LIMITS),
            Err(MutationArchiveRefusal::Record(refusal))
        );
    }
    let mut cases = Vec::new();
    let mut baseline = MutationVector::demonstrated();
    baseline.baseline = 3;
    cases.push(baseline);
    let mut materialization = MutationVector::demonstrated();
    materialization.materialization = 3;
    cases.push(materialization);
    let mut activation = MutationVector::demonstrated();
    activation.activation = vec![3];
    cases.push(activation);
    let mut execution = MutationVector::demonstrated();
    execution.execution = 5;
    cases.push(execution);
    let mut equivalence = MutationVector::demonstrated();
    equivalence.equivalence = 4;
    cases.push(equivalence);
    for outcome in [vec![3], vec![0, 2], vec![2, 6]] {
        let mut vector = MutationVector::demonstrated();
        vector.outcome = outcome;
        cases.push(vector);
    }
    for vector in cases {
        assert_eq!(
            read_mutation(&vector.encoded(), LIMITS),
            Err(MutationArchiveRefusal::Record(ArchiveRefusal::InvalidSlot))
        );
    }
}

#[test]
fn backend_rejection_requires_present_text_with_truthful_loss_markers() -> Result<(), ()> {
    let mut vector = MutationVector::backend();
    vector.outcome = vec![0, 1, 0];
    assert_eq!(
        read_mutation(&vector.encoded(), LIMITS),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::InvalidForeignText
        ))
    );
    vector.outcome = vec![0, 1];
    vector
        .outcome
        .extend(trial_vector::foreign(&[255], &[0, 0]));
    assert_eq!(
        read_mutation(&vector.encoded(), LIMITS),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::InvalidForeignText
        ))
    );
    let bytes = vec![b'x'; FOREIGN_TEXT_MAX_BYTES];
    let admitted = u64::try_from(bytes.len()).map_err(|_| ())?;
    let mut tail = vec![1];
    tail.extend_from_slice(&admitted.to_be_bytes());
    tail.extend_from_slice(&u64::MAX.to_be_bytes());
    tail.push(0);
    vector.outcome = vec![0, 1];
    vector.outcome.extend(trial_vector::foreign(&bytes, &tail));
    let record = read_mutation(&vector.encoded(), LIMITS).map_err(|_| ())?;
    let ArchivedMutationOutcome::Killed(ArchivedRejection::ReportedByBackend(text)) =
        record.outcome()
    else {
        return Err(());
    };
    assert_eq!(text.bytes(), bytes);
    assert_eq!(
        text.truncation(),
        ArchivedTruncation::TruncatedAt {
            admitted,
            offered: u64::MAX
        }
    );
    let mut false_tail = vec![1];
    false_tail.extend_from_slice(&admitted.to_be_bytes());
    false_tail.extend_from_slice(&admitted.to_be_bytes());
    false_tail.push(0);
    vector.outcome = vec![0, 1];
    vector
        .outcome
        .extend(trial_vector::foreign(&bytes, &false_tail));
    assert_eq!(
        read_mutation(&vector.encoded(), LIMITS),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::InvalidForeignText
        ))
    );
    Ok(())
}

//! Complete historical mutation claims challenged through independent bytes and live parser output.

use super::mutation_vector::{self, MutationVector};
use super::proposal_vector;
use macroonz_harness::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutationOutcome, ArchivedRejection, MutationArchiveRefusal,
    read_mutation, retain_mutation,
};
use macroonz_harness::muterprater::{
    BackendVersionPosture, BaselineAxis, EquivalenceAxis, ExecutionAxis, InconclusiveCause,
    IntendedRejection, MaterializationAxis, MutationOutcome, MutationVerdict,
};
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};
use macroonz_harness::report::{FailureClass, TextFidelity};
use std::error::Error;
use std::io::Read as _;

const LIMITS: ArchiveLimits = ArchiveLimits::declared(65_536, 32_768);

#[test]
fn independent_mutation_retains_every_axis_and_demonstrated_finding() -> Result<(), ()> {
    let vector = MutationVector::demonstrated();
    let record = read_mutation(&vector.encoded(), LIMITS).map_err(|_| ())?;
    assert_eq!(record.encoded(), vector.encoded());
    assert_eq!(record.baseline(), BaselineAxis::Qualified);
    assert_eq!(record.materialization(), MaterializationAxis::Built);
    assert_eq!(record.execution(), ExecutionAxis::Completed);
    assert_eq!(record.equivalence(), EquivalenceAxis::Refuted);
    assert_eq!(
        MutationVerdict::from(record.outcome()),
        MutationVerdict::Killed
    );
    let ArchivedActivation::Observed(activation) = record.activation() else {
        return Err(());
    };
    assert_eq!(activation.firings(), 7);
    assert_eq!(activation.witness().as_bytes(), &[12; 32]);
    assert_eq!(activation.surface().as_bytes(), &[21; 32]);
    assert_eq!(activation.alternative().as_bytes(), &[11; 32]);
    assert_eq!(activation.point().stem(), "point");
    let ArchivedMutationOutcome::Killed(ArchivedRejection::Demonstrated(finding)) =
        record.outcome()
    else {
        return Err(());
    };
    assert_eq!(finding.fingerprint().trial().as_bytes(), &[1; 32]);
    assert_eq!(finding.fingerprint().family(), "fixture");
    assert_eq!(finding.fingerprint().local(), "disagrees");
    assert_eq!(
        finding.fingerprint().class(),
        FailureClass::PropertyDisagreement
    );
    assert_eq!(finding.file(), "check.rs");
    assert_eq!(finding.line(), 29);
    assert_eq!(finding.foreign().ok_or(())?.bytes(), &[b'a', 255, b'b']);
    assert_eq!(
        finding.foreign().ok_or(())?.fidelity(),
        TextFidelity::LossyReplacement
    );
    assert_eq!(
        record.target().family(),
        Some("historical-family-outside-current-bank")
    );
    // The live owner establishes no extra activation-witness/finding-trial join.
    assert_ne!(activation.witness(), finding.fingerprint().trial());
    Ok(())
}

#[test]
fn backend_rejection_keeps_text_without_fingerprint_or_activation() -> Result<(), ()> {
    let record = read_mutation(&MutationVector::backend().encoded(), LIMITS).map_err(|_| ())?;
    assert_eq!(
        record.activation(),
        &ArchivedActivation::UnobservableUnderBackend
    );
    let ArchivedMutationOutcome::Killed(ArchivedRejection::ReportedByBackend(text)) =
        record.outcome()
    else {
        return Err(());
    };
    assert_eq!(text.bytes(), b"caught");
    assert_eq!(text.fidelity(), TextFidelity::Exact);
    assert!(record.target().family().is_none());
    assert!(record.target().owner().is_none());
    Ok(())
}

#[test]
fn historical_outcomes_refuse_only_the_live_owners_axis_ceiling() -> Result<(), ()> {
    for outcome in [MutationVector::demonstrated().outcome, vec![1]] {
        for (baseline, materialization, activation, execution) in [
            (1, 0, proposal_vector::activation(1), 0),
            (2, 0, proposal_vector::activation(1), 0),
            (0, 1, proposal_vector::activation(1), 0),
            (0, 2, proposal_vector::activation(1), 0),
            (0, 0, vec![1], 0),
            (0, 0, proposal_vector::activation(1), 1),
            (0, 0, proposal_vector::activation(1), 2),
            (0, 0, proposal_vector::activation(1), 3),
            (0, 0, proposal_vector::activation(1), 4),
        ] {
            let mut vector = MutationVector::demonstrated();
            vector.baseline = baseline;
            vector.materialization = materialization;
            vector.activation = activation;
            vector.execution = execution;
            vector.outcome.clone_from(&outcome);
            assert_eq!(
                read_mutation(&vector.encoded(), LIMITS),
                Err(MutationArchiveRefusal::OutcomeAxesMismatch)
            );
        }
    }
    let mut survivor = MutationVector::demonstrated();
    survivor.outcome = vec![1];
    let record = read_mutation(&survivor.encoded(), LIMITS).map_err(|_| ())?;
    assert_eq!(record.outcome(), &ArchivedMutationOutcome::Survived);
    survivor.activation = vec![2];
    assert_eq!(
        read_mutation(&survivor.encoded(), LIMITS),
        Err(MutationArchiveRefusal::OutcomeAxesMismatch)
    );
    Ok(())
}

#[test]
fn inconclusive_records_retain_unusual_axis_combinations_and_all_causes() -> Result<(), ()> {
    for (slot, cause) in [
        (0, InconclusiveCause::BaselineNotQualified),
        (1, InconclusiveCause::NotMaterialized),
        (2, InconclusiveCause::NotActivated),
        (3, InconclusiveCause::WitnessIncomplete),
        (4, InconclusiveCause::UnobservableAndUnrejected),
        (5, InconclusiveCause::ProvenEquivalentInScope),
    ] {
        let mut vector = MutationVector::demonstrated();
        vector.baseline = 2;
        vector.materialization = 2;
        vector.activation = vec![1];
        vector.execution = 3;
        vector.equivalence = 1;
        vector.outcome = vec![2, slot];
        let record = read_mutation(&vector.encoded(), LIMITS).map_err(|_| ())?;
        assert_eq!(
            record.outcome(),
            &ArchivedMutationOutcome::Inconclusive(cause)
        );
        assert_eq!(record.baseline(), BaselineAxis::NotRun);
        assert_eq!(record.materialization(), MaterializationAxis::ToolFailed);
        assert_eq!(record.activation(), &ArchivedActivation::NotObserved);
        assert_eq!(record.execution(), ExecutionAxis::Crashed);
        assert_eq!(record.equivalence(), EquivalenceAxis::ProvenInScope);
    }
    Ok(())
}

#[test]
fn mutation_integrity_bounds_and_complete_framing_are_independent() -> Result<(), ()> {
    let vector = MutationVector::backend();
    let encoded = vector.encoded();
    let exact = ArchiveLimits::declared(encoded.len(), vector.target.len());
    assert_eq!(
        read_mutation(&encoded, exact).map_err(|_| ())?.encoded(),
        encoded
    );
    assert_eq!(
        read_mutation(
            &encoded,
            ArchiveLimits::declared(encoded.len().saturating_sub(1), 32_768)
        ),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::EnvelopeTooLarge
        ))
    );
    assert_eq!(
        read_mutation(
            &encoded,
            ArchiveLimits::declared(65_536, vector.target.len().saturating_sub(1))
        ),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::FieldTooLarge
        ))
    );
    let mut corrupt = encoded;
    *corrupt.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_mutation(&corrupt, LIMITS),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::AddressMismatch
        ))
    );
    let body = vector.body();
    for length in 0..body.len() {
        assert!(
            read_mutation(
                &mutation_vector::envelope(body.get(..length).ok_or(())?),
                LIMITS
            )
            .is_err()
        );
    }
    let mut trailing = body;
    trailing.push(0);
    assert_eq!(
        read_mutation(&mutation_vector::envelope(&trailing), LIMITS),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::TrailingBytes
        ))
    );
    Ok(())
}

#[test]
fn historical_axis_slots_preserve_every_successfully_loaded_reading() -> Result<(), ()> {
    let readings = [
        (
            0,
            BaselineAxis::Qualified,
            0,
            MaterializationAxis::Built,
            0,
            ExecutionAxis::Completed,
            0,
            EquivalenceAxis::NotAssessed,
        ),
        (
            1,
            BaselineAxis::Failed,
            1,
            MaterializationAxis::Unviable,
            1,
            ExecutionAxis::NotExecuted,
            1,
            EquivalenceAxis::ProvenInScope,
        ),
        (
            2,
            BaselineAxis::NotRun,
            2,
            MaterializationAxis::ToolFailed,
            2,
            ExecutionAxis::TimedOut,
            2,
            EquivalenceAxis::Refuted,
        ),
        (
            0,
            BaselineAxis::Qualified,
            0,
            MaterializationAxis::Built,
            3,
            ExecutionAxis::Crashed,
            3,
            EquivalenceAxis::Inconclusive,
        ),
        (
            0,
            BaselineAxis::Qualified,
            0,
            MaterializationAxis::Built,
            4,
            ExecutionAxis::InfrastructureFailed,
            0,
            EquivalenceAxis::NotAssessed,
        ),
    ];
    for (
        baseline,
        expected_baseline,
        materialization,
        expected_materialization,
        execution,
        expected_execution,
        equivalence,
        expected_equivalence,
    ) in readings
    {
        let mut vector = MutationVector::demonstrated();
        vector.outcome = vec![2, 0];
        vector.baseline = baseline;
        vector.materialization = materialization;
        vector.execution = execution;
        vector.equivalence = equivalence;
        let record = read_mutation(&vector.encoded(), LIMITS).map_err(|_| ())?;
        assert_eq!(record.baseline(), expected_baseline);
        assert_eq!(record.materialization(), expected_materialization);
        assert_eq!(record.execution(), expected_execution);
        assert_eq!(record.equivalence(), expected_equivalence);
    }
    Ok(())
}

/// Caller-supplied synthetic console text exercises the real parser, without asserting backend execution.
#[test]
fn live_backend_records_round_trip_with_exact_writer_limits() -> Result<(), ()> {
    let reading = macroonz_harness::muterprater::wrap::read_output(
        "ok Unmutated baseline\ncaught sample.rs:1:1: replace true with false\nmissed sample.rs:2:1: replace true with false\n",
        BackendVersionPosture::Unstated,
        |_| None,
        |_, _| None,
    ).map_err(|_| ())?;
    assert_eq!(reading.run().reports().len(), 2);
    for report in reading.run().reports() {
        let archived = retain_mutation(report, LIMITS).map_err(|_| ())?;
        assert_eq!(MutationVerdict::from(archived.outcome()), report.verdict());
        assert_eq!(archived.baseline(), report.baseline());
        assert_eq!(archived.execution(), report.execution());
        assert_eq!(archived.materialization(), report.materialization());
        assert_eq!(archived.equivalence(), report.equivalence());
        if let MutationOutcome::Killed(IntendedRejection::ReportedByBackend { stated }) =
            report.outcome()
        {
            let ArchivedMutationOutcome::Killed(ArchivedRejection::ReportedByBackend(retained)) =
                archived.outcome()
            else {
                return Err(());
            };
            assert_eq!(retained.bytes(), stated.bytes());
        }
        let exact = ArchiveLimits::declared(archived.encoded().len(), LIMITS.field());
        assert_eq!(retain_mutation(report, exact).map_err(|_| ())?, archived);
        let short =
            ArchiveLimits::declared(archived.encoded().len().saturating_sub(1), LIMITS.field());
        assert_eq!(
            retain_mutation(report, short),
            Err(MutationArchiveRefusal::Record(
                ArchiveRefusal::EnvelopeTooLarge
            ))
        );
        assert_eq!(
            retain_mutation(report, ArchiveLimits::declared(65_536, 31)),
            Err(MutationArchiveRefusal::Record(
                ArchiveRefusal::FieldTooLarge
            ))
        );
    }
    Ok(())
}

#[test]
fn complete_mutation_readback_owns_data_across_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let encoded = MutationVector::demonstrated().encoded();
    let returned =
        crate::archive_process::round_trip(&encoded, "archive::mutations::mutation_archive_child")?;
    assert_eq!(returned, encoded);
    Ok(())
}

#[test]
#[ignore = "invoked by the parent with a bounded historical mutation envelope"]
fn mutation_archive_child() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(65_537)
        .read_to_end(&mut encoded)?;
    let record = read_mutation(&encoded, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(encoded);
    crate::archive_process::publish(record.encoded())
}

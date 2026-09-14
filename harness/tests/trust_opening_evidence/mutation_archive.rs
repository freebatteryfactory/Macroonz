//! Actual interpreted and compiled projection reports retain their historical fields.

use super::interpretation::interpreted_survivor;
use super::support::{
    EVALUATION, SELECTED_OPERATION, active_selection, family, interpreted_kill,
    lock_specimen_tests, pair, qualification_of, qualified_no_mutation, standard_projection,
    surface_with, witness,
};
use macroonz_harness::muterprater::verdict_archive::{
    ArchivedActivation, ArchivedMutation, ArchivedMutationIdentity, ArchivedMutationOutcome,
    ArchivedRejection, read_mutation, retain_mutation,
};
use macroonz_harness::muterprater::{
    ActivationDisposition, IntendedRejection, MutationIdentity, MutationOutcome, MutationReport,
    MutationVerdict,
};
use macroonz_harness::report::ForeignText;
use macroonz_harness::report::archive::{ArchiveLimits, ArchivedForeignText};

const LIMITS: ArchiveLimits = ArchiveLimits::declared(65_536, 32_768);

#[test]
fn interpreted_kill_and_survivor_retain_the_actual_report_fields() -> Result<(), String> {
    let killed = interpreted_kill().map_err(|error| format!("{error:?}"))?;
    assert_eq!(killed.verdict(), MutationVerdict::Killed);
    compare(&killed)?;
    let survivor = interpreted_survivor().map_err(|error| format!("{error:?}"))?;
    assert_eq!(survivor.verdict(), MutationVerdict::Survived);
    compare(&survivor)
}

#[test]
fn separately_compiled_projection_retains_demonstrated_rejection_at_its_backend_ceiling()
-> Result<(), String> {
    let _guard = lock_specimen_tests().map_err(|error| format!("{error:?}"))?;
    let owner = family("archived-compiled-projection").map_err(|error| format!("{error:?}"))?;
    let surface =
        surface_with(owner, vec![SELECTED_OPERATION]).map_err(|error| format!("{error:?}"))?;
    let pair = pair(owner, &surface, EVALUATION).map_err(|error| format!("{error:?}"))?;
    let input = [1, 0, 0];
    let standing = qualified_no_mutation(
        &pair,
        witness().map_err(|error| format!("{error:?}"))?,
        &input,
    )
    .map_err(|error| format!("{error:?}"))?;
    let qualification = qualification_of(&standing).map_err(|error| format!("{error:?}"))?;
    let selection = active_selection(&surface).map_err(|error| format!("{error:?}"))?;
    let projection = standard_projection(&surface, qualification, &pair, selection)
        .map_err(|error| format!("{error:?}"))?;
    assert_eq!(projection.mutation().verdict(), MutationVerdict::Killed);
    assert_eq!(
        projection.mutation().activation(),
        ActivationDisposition::UnobservableUnderBackend
    );
    compare(projection.mutation())
}

fn compare(report: &MutationReport) -> Result<(), String> {
    let archived = retain_mutation(report, LIMITS).map_err(|error| format!("{error:?}"))?;
    assert_eq!(archived.baseline(), report.baseline());
    assert_eq!(archived.materialization(), report.materialization());
    assert_eq!(archived.execution(), report.execution());
    assert_eq!(archived.equivalence(), report.equivalence());
    assert_eq!(MutationVerdict::from(archived.outcome()), report.verdict());
    compare_target(report, &archived)?;
    compare_activation(report, &archived)?;
    compare_rejection(report, &archived)?;
    let exact = ArchiveLimits::declared(archived.encoded().len(), LIMITS.field());
    assert_eq!(
        retain_mutation(report, exact).map_err(|error| format!("{error:?}"))?,
        archived
    );
    let restored =
        read_mutation(archived.encoded(), LIMITS).map_err(|error| format!("{error:?}"))?;
    drop(archived);
    assert_eq!(MutationVerdict::from(restored.outcome()), report.verdict());
    Ok(())
}

fn compare_target(report: &MutationReport, archived: &ArchivedMutation) -> Result<(), String> {
    match (report.target().identity(), archived.target().identity()) {
        (
            MutationIdentity::Interpreted { point, alternative },
            ArchivedMutationIdentity::Interpreted {
                point: retained,
                alternative: claim,
            },
        )
        | (
            MutationIdentity::CompiledProjection { point, alternative },
            ArchivedMutationIdentity::CompiledProjection {
                point: retained,
                alternative: claim,
            },
        ) => {
            assert_eq!(retained.namespace(), point.name().namespace().written());
            assert_eq!(retained.stem(), point.name().stem().written());
            assert_eq!(claim.as_bytes(), alternative.address().as_bytes());
            Ok(())
        }
        _ => Err("the actual selected projection changed its identity road".to_owned()),
    }
}

fn compare_activation(report: &MutationReport, archived: &ArchivedMutation) -> Result<(), String> {
    match (report.activation(), archived.activation()) {
        (ActivationDisposition::Observed(actual), ArchivedActivation::Observed(retained)) => {
            assert_eq!(
                retained.surface().as_bytes(),
                actual.selection().surface().address().as_bytes()
            );
            assert_eq!(
                retained.point().namespace(),
                actual.point().name().namespace().written()
            );
            assert_eq!(
                retained.point().stem(),
                actual.point().name().stem().written()
            );
            assert_eq!(
                retained.alternative().as_bytes(),
                actual.selection().alternative().address().as_bytes()
            );
            assert_eq!(
                retained.witness().as_bytes(),
                actual.witness().address().as_bytes()
            );
            assert_eq!(retained.firings(), actual.firings());
            Ok(())
        }
        (
            ActivationDisposition::UnobservableUnderBackend,
            ArchivedActivation::UnobservableUnderBackend,
        ) => Ok(()),
        _ => Err("the actual projection changed its activation ceiling".to_owned()),
    }
}

fn compare_rejection(report: &MutationReport, archived: &ArchivedMutation) -> Result<(), String> {
    match (report.outcome(), archived.outcome()) {
        (
            MutationOutcome::Killed(IntendedRejection::Demonstrated(actual)),
            ArchivedMutationOutcome::Killed(ArchivedRejection::Demonstrated(retained)),
        ) => {
            assert_eq!(
                retained.fingerprint().trial().as_bytes(),
                actual.trial().address().as_bytes()
            );
            assert_eq!(
                retained.fingerprint().address(),
                actual.fingerprint().address()
            );
            assert_eq!(
                retained.fingerprint().family(),
                actual.finding().cause().family()
            );
            assert_eq!(
                retained.fingerprint().local(),
                actual.finding().cause().local()
            );
            assert_eq!(retained.fingerprint().class(), actual.finding().class());
            assert_eq!(retained.file(), actual.finding().located().file());
            assert_eq!(retained.line(), actual.finding().located().line());
            assert_eq!(
                retained.foreign().map(ArchivedForeignText::bytes),
                actual.finding().foreign().map(ForeignText::bytes)
            );
            Ok(())
        }
        (MutationOutcome::Survived, ArchivedMutationOutcome::Survived) => Ok(()),
        _ => Err("the actual projection changed its rejection road".to_owned()),
    }
}

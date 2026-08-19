//! The seat's one reading: what an aggregate seat concludes from a run report,
//! and what a named lens concludes from one trial report.
//!
//! Both are folds over typed values a run already wrote. Neither reads a word
//! of prose: a failure is described by carrying the record's own typed fields,
//! so nothing here matches on a message anybody rendered, and a refusal that
//! renamed itself would still be the same typed arm.
//!
//! # Why the readings are the engine's
//!
//! The stamp could write this fold into every module it stamps, and it
//! deliberately does not. A calculator standing in as many places as there are
//! invocations is as many places for it to drift, and two seats disagreeing
//! about what "the run passed" means is exactly the disagreement a harness
//! exists to make impossible. One home, one reading, however many seats call
//! it.
//!
//! # Nonclaims
//!
//! The table posture is not read here. Claim coverage refuses a staged report,
//! because a candidate run's numbers must never pass as an authored world's;
//! whether the trials CONCLUDED is the same question in both worlds, so a
//! verdict answers it in both.
//!
//! Neither reading compares against anything. A verdict is a statement about
//! one report and its own census; a difference between two runs is the record
//! home's comparison, over what this one read.

use super::types::{FailedTrial, SeatFailure, SeatOutcome, SeatRefusal};
use crate::report::{RunAttempt, RunReport, SelectionOutcome, TrialConclusion, TrialReport};

/// The verdict one aggregate seat takes over one run.
///
/// # Authority
///
/// Two readings, in a declared order. The run's own selection outcome is read
/// first, because a run that exercised nothing has no census rows to read for a
/// conclusion and the reason it exercised nothing is a fact the run already
/// recorded — this reading translates that fact, it does not re-derive it from
/// an empty roster. The trials come second, over the census in census order and
/// over nothing else: a row the selection passed over is not a failure —
/// narrowing a run has never been an outcome, and the census states the
/// pass-over in the open — so only the rows the selection admitted are read for
/// a conclusion.
///
/// A run whose selection matched nothing under an ADMITTED empty expectation
/// answers with [`SeatOutcome::NoWorkAsStated`], carrying the reason its caller
/// stated. That is a zero-work result and it is named as one: no arm of the
/// answer says a trial passed, because none ran.
///
/// # Errors
///
/// Refuses when the selection named no row of the denominator and the run
/// expected at least one. A run that exercised nothing is not a run that passed,
/// and on the stamped road this is precisely the pairing a stamp cannot check
/// without reading inside a row expression: a suite group whose declared suite
/// is no row's own selects nothing, and the seat says so instead of reporting
/// success over a world it never touched.
///
/// Refuses when any selected trial did not conclude lawfully, carrying every
/// one of them with both identity rails and the typed fact that says what it
/// did instead, alongside how many rows were selected and how many rows the run
/// was stated over.
pub fn seat_verdict(report: &RunReport) -> Result<SeatOutcome, SeatRefusal> {
    let denominator = report.denominator();
    match report.selection() {
        SelectionOutcome::Satisfied => {}
        SelectionOutcome::UnsatisfiedByEmptySelection => {
            return Err(SeatRefusal::NothingSelected { denominator });
        }
        SelectionOutcome::EmptyAsStated(reason) => {
            return Ok(SeatOutcome::NoWorkAsStated {
                reason,
                denominator,
            });
        }
    }
    let selected: Vec<&TrialReport> = report
        .census()
        .iter()
        .filter_map(|accounting| accounting.disposition().report())
        .collect();
    let failed: Vec<FailedTrial> = selected.iter().copied().filter_map(failed_trial).collect();
    if failed.is_empty() {
        Ok(SeatOutcome::EveryTrialConcluded {
            selected: selected.len(),
            denominator,
        })
    } else {
        Err(SeatRefusal::RunFailed {
            failed,
            selected: selected.len(),
            denominator,
        })
    }
}

/// The verdict one named lens takes over the single trial it ran.
///
/// # Authority
///
/// A lens runs one binding, so there is no census to read and no selection to
/// account for: the whole question is what that one trial did, answered from
/// the same reading the aggregate seat folds over its own selected rows.
///
/// It answers with nothing rather than with a [`SeatOutcome`], and that is the
/// asymmetry stated rather than hidden: a lens has no selection, so it has no
/// expectation to satisfy and no zero-work result to render. Exactly one trial
/// ran, and whether it concluded lawfully is the entire finding.
///
/// # Errors
///
/// Refuses when the trial did not conclude lawfully, carrying its two identity
/// rails and the typed fact that says what it did instead.
pub fn lens_verdict(report: &TrialReport) -> Result<(), SeatRefusal> {
    match failed_trial(report) {
        None => Ok(()),
        Some(failed) => Err(SeatRefusal::trial_failed(failed)),
    }
}

/// What one trial's record says it did instead of concluding lawfully, or
/// nothing at all where it concluded lawfully.
///
/// The one place a report becomes a failure. Both readings above go through it,
/// so an aggregate seat and a named lens describe the same trial's failure in
/// the same words — which is what makes the two spellings of a table
/// comparable at all.
///
/// The satisfied arm is the ONLY lawful conclusion: a skip, a reached budget,
/// and a harness fault each state that the check's question went unanswered,
/// and a seat that passed over any of them would be green about evidence
/// nobody produced.
fn failed_trial(report: &TrialReport) -> Option<FailedTrial> {
    let failure = match report.attempt() {
        RunAttempt::Executed(TrialConclusion::Passed) => return None,
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            SeatFailure::CheckRefused(finding.clone())
        }
        RunAttempt::SkippedWithReason(reason) => SeatFailure::NotRun(*reason),
        RunAttempt::TimedOut(budget) => SeatFailure::PastTimeBudget(*budget),
        RunAttempt::InfrastructureFailed(fault) => SeatFailure::HarnessFailed(*fault),
    };
    Some(FailedTrial::recorded(report.trial(), report.site(), failure))
}

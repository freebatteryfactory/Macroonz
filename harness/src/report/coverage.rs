//! Claim coverage: did every claim the denominator names actually get
//! exercised.
//!
//! A reading of one report, computed and never hand-counted, over the
//! denominator that report already recorded. The intermediate tally is private
//! to the reading; the counts leave as typed values.

use super::{ClaimCoverage, ClaimExercise, CoverageRefusal, Exercise, RunReport};
use crate::descriptor::{ClaimRef, TablePosture};

/// One claim's running counts while the census is walked.
type Tally = (ClaimRef, usize, usize);

/// Read what one run exercised, per claim, over the denominator it recorded.
///
/// # Authority
///
/// Exercise is execution: a row the invocation selected and then skipped counts
/// as unexercised, because a coverage number that counted it would be claiming
/// evidence nobody produced.
///
/// # Errors
///
/// Refuses a report standing over a staged view. Coverage admits
/// authored-posture reports only, so a candidate run never enters the numbers a
/// gate reads — by refusal, not by declaration.
pub fn claim_coverage(report: &RunReport) -> Result<ClaimCoverage, CoverageRefusal> {
    match report.posture() {
        TablePosture::Authored => {}
        TablePosture::Staged { parent } => {
            return Err(CoverageRefusal::StagedPosture { parent });
        }
    }

    let mut tallies: Vec<Tally> = Vec::new();
    for entry in report.census() {
        let exercise = entry.disposition().exercise();
        let claim = entry.claim();
        match tallies.iter_mut().find(|tally| tally.0 == claim) {
            Some(tally) => counted_into(tally, exercise),
            None => tallies.push(opened(claim, exercise)),
        }
    }

    Ok(ClaimCoverage::read(
        tallies
            .into_iter()
            .map(|(claim, exercised, unexercised)| {
                ClaimExercise::counted(claim, exercised, unexercised)
            })
            .collect(),
    ))
}

/// The first row seen for one claim.
fn opened(claim: ClaimRef, exercise: Exercise) -> Tally {
    match exercise {
        Exercise::Exercised => (claim, 1usize, 0usize),
        Exercise::Unexercised => (claim, 0usize, 1usize),
    }
}

/// One more row for a claim already seen.
fn counted_into(tally: &mut Tally, exercise: Exercise) {
    match exercise {
        Exercise::Exercised => tally.1 = tally.1.saturating_add(1),
        Exercise::Unexercised => tally.2 = tally.2.saturating_add(1),
    }
}

//! The ground roads: how a kill is demonstrated, how proof is measured, how a duplicate is refused, and the three grounds a proposal stands on.

use crate::descriptor::{ClaimRef, TablePosture};
use crate::muterprater::proposal::types::{
    ClaimPinnedGround, Demonstration, DischargeEvidence, DuplicateRefusal, FailureComparison,
    MutantKilledGround, NoComparison, NoComparisonReason, ObligationComparison,
    ObligationDischargedGround, OwedClaim, ProofDelta, ProofDeltaRefusal, ProofRefusal,
};
use crate::muterprater::{ActivationDisposition, DemonstratedRejection, MutationTarget};
use crate::report::{
    Fingerprint, ReplayCapsule, RunAttempt, RunReport, TrialConclusion, TrialId, TrialReport,
};

// ---------------------------------------------------------------------------
// Demonstration, and the duplicate comparisons.
// ---------------------------------------------------------------------------

impl Demonstration {
    /// Read a demonstrated kill out of the report a staged run wrote.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a report standing over the authored world rather than a staged view, a census that does not carry the candidate, a candidate the selection passed over, a candidate that did not execute, and a candidate that executed and did not refuse.
    pub fn read(report: RunReport, candidate: TrialId) -> Result<Self, ProofRefusal> {
        require_staged(&report)?;
        let trial_report = candidate_report(&report, candidate)?;
        let rejection = demonstrated_rejection(&trial_report, candidate)?;
        Ok(Self {
            report,
            trial_report,
            rejection,
        })
    }

    /// The report the staged run wrote.
    #[must_use]
    pub const fn report(&self) -> &RunReport {
        &self.report
    }

    /// The candidate trial report the rejection was read from.
    #[must_use]
    pub const fn trial_report(&self) -> &TrialReport {
        &self.trial_report
    }

    /// The rejection read out of it.
    #[must_use]
    pub const fn rejection(&self) -> &DemonstratedRejection {
        &self.rejection
    }
}

/// Require the table posture one candidate demonstration stands over.
fn require_staged(report: &RunReport) -> Result<(), ProofRefusal> {
    match report.posture() {
        TablePosture::Staged { parent: _ } => Ok(()),
        TablePosture::Authored => Err(ProofRefusal::NotStaged),
    }
}

/// Read the selected candidate's trial report from the complete staged census.
fn candidate_report(report: &RunReport, candidate: TrialId) -> Result<TrialReport, ProofRefusal> {
    let Some(entry) = report
        .census()
        .iter()
        .find(|accounting| accounting.trial() == candidate)
    else {
        return Err(ProofRefusal::CandidateNotInCensus);
    };
    let Some(executed) = entry.disposition().report() else {
        return Err(ProofRefusal::CandidateNotSelected);
    };
    Ok(executed.clone())
}

/// Read the demonstrated rejection from one selected candidate report.
fn demonstrated_rejection(
    report: &TrialReport,
    candidate: TrialId,
) -> Result<DemonstratedRejection, ProofRefusal> {
    match report.attempt() {
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => Ok(
            DemonstratedRejection::demonstrated(candidate, finding.clone()),
        ),
        RunAttempt::Executed(TrialConclusion::Passed) => Err(ProofRefusal::CandidateDidNotRefuse),
        RunAttempt::SkippedWithReason(_)
        | RunAttempt::TimedOut
        | RunAttempt::InfrastructureFailed(_) => Err(ProofRefusal::CandidateDidNotExecute),
    }
}

impl ProofDelta {
    /// How much proof one candidate added to the claim it pins.
    ///
    /// # Errors
    ///
    /// Refuses a pair that does not move, because a candidate that leaves the exercised count where it was pins nothing.
    pub const fn between(before: usize, after: usize) -> Result<Self, ProofDeltaRefusal> {
        if after <= before {
            return Err(ProofDeltaRefusal::NoProofAdded { before, after });
        }
        Ok(Self { before, after })
    }

    /// The exercised count before the candidate ran.
    #[must_use]
    pub const fn before(self) -> usize {
        self.before
    }

    /// The exercised count after it ran.
    #[must_use]
    pub const fn after(self) -> usize {
        self.after
    }
}

impl FailureComparison {
    /// The comparison a failure-bearing ground offers.
    ///
    /// # Errors
    ///
    /// Refuses a candidate whose fingerprint the known roster already carries.
    pub fn compared(
        candidate: Fingerprint,
        known: Vec<Fingerprint>,
    ) -> Result<Self, DuplicateRefusal> {
        if known.contains(&candidate) {
            return Err(DuplicateRefusal::FingerprintAlreadyKnown(candidate));
        }
        Ok(Self { candidate, known })
    }

    /// The fingerprint this candidate carries.
    #[must_use]
    pub const fn candidate(&self) -> Fingerprint {
        self.candidate
    }

    /// The fingerprints already known, in the order they were compared.
    pub fn known(&self) -> impl Iterator<Item = &Fingerprint> {
        self.known.iter()
    }
}

impl ObligationComparison {
    /// The comparison a discharge ground offers.
    ///
    /// # Errors
    ///
    /// Refuses an owed claim that already carries a discharge, naming the first one recorded for it.
    pub fn compared(owed: ClaimRef, discharges: &[TrialId]) -> Result<Self, DuplicateRefusal> {
        if let Some(first) = discharges.first() {
            return Err(DuplicateRefusal::ObligationAlreadyDischarged(*first));
        }
        Ok(Self { owed })
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(&self) -> ClaimRef {
        self.owed
    }
}

impl NoComparison {
    /// The statement a ground with no comparable subject makes.
    #[must_use]
    pub const fn stated(reason: NoComparisonReason) -> Self {
        Self { reason }
    }

    /// Why nothing was compared.
    #[must_use]
    pub const fn reason(self) -> NoComparisonReason {
        self.reason
    }
}

// ---------------------------------------------------------------------------
// The three proposal grounds.
// ---------------------------------------------------------------------------

impl MutantKilledGround {
    /// The ground a demonstrated kill stands on.
    #[must_use]
    pub(in crate::muterprater) const fn shown(
        target: MutationTarget,
        activation: ActivationDisposition,
        capsule: ReplayCapsule,
        demonstration: Demonstration,
    ) -> Self {
        Self {
            target,
            activation,
            capsule,
            demonstration,
        }
    }

    /// What was damaged.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// What the damage's activation was.
    #[must_use]
    pub const fn activation(&self) -> ActivationDisposition {
        self.activation
    }

    /// The reproduction account of the demonstrating run.
    #[must_use]
    pub const fn capsule(&self) -> &ReplayCapsule {
        &self.capsule
    }

    /// The demonstrated kill.
    #[must_use]
    pub const fn demonstration(&self) -> &Demonstration {
        &self.demonstration
    }
}

impl ClaimPinnedGround {
    /// The ground a pin stands on.
    #[must_use]
    pub const fn moved(claim: ClaimRef, capsule: ReplayCapsule, delta: ProofDelta) -> Self {
        Self {
            claim,
            capsule,
            delta,
        }
    }

    /// The claim pinned.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// The reproduction account of the pinning run.
    #[must_use]
    pub const fn capsule(&self) -> &ReplayCapsule {
        &self.capsule
    }

    /// What the pin added to the claim's proof.
    #[must_use]
    pub const fn delta(&self) -> ProofDelta {
        self.delta
    }
}

impl ObligationDischargedGround {
    /// The ground a discharge stands on.
    #[must_use]
    pub const fn discharged(owed: OwedClaim, discharge: DischargeEvidence) -> Self {
        Self { owed, discharge }
    }

    /// The owed claim's identity.
    #[must_use]
    pub const fn owed(&self) -> &OwedClaim {
        &self.owed
    }

    /// What discharged it.
    #[must_use]
    pub const fn discharge(&self) -> &DischargeEvidence {
        &self.discharge
    }
}

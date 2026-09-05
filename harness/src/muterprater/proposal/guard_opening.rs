//! The opening roads: how a survivor is explained, how an owed claim comes due, and what discharged it.

use crate::descriptor::{
    CheckRef, ClaimRef, Classification, ExecutionSuite, PopulationRef, SubjectRoute,
};
use crate::muterprater::proposal::types::{
    CandidateSketch, CheckGap, DischargeEvidence, ExplanationRefusal, InferredObligation,
    ObligationLane, OracleClass, OwedClaim, OwedClaimRefusal, OwedDeclaration, ProofShape,
    SurvivorExplanation,
};
use crate::muterprater::{MutationReport, MutationTarget, MutationVerdict};
use crate::report::{ClaimExercise, ExecutionKey, TrialId};

impl SurvivorExplanation {
    /// The explanation one survivor's record hands into synthesis.
    ///
    /// # Errors
    ///
    /// Refuses a record whose verdict is not survived, then a target whose owning claim is unmapped.
    pub fn of(
        report: &MutationReport,
        missing: OracleClass,
        closing: CheckRef,
    ) -> Result<Self, ExplanationRefusal> {
        let verdict = report.verdict();
        match verdict {
            MutationVerdict::Survived => {}
            MutationVerdict::Killed | MutationVerdict::Inconclusive => {
                return Err(ExplanationRefusal::NotASurvivor(verdict));
            }
        }
        let Some(claim) = report.target().owning_claim() else {
            return Err(ExplanationRefusal::OwnerUnmapped);
        };
        Ok(Self {
            target: report.target().clone(),
            claim,
            missing,
            closing,
        })
    }

    /// The target that survived.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// The claim that owns it.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// The oracle class no check of that claim supplies.
    #[must_use]
    pub const fn missing(&self) -> OracleClass {
        self.missing
    }

    /// The check reference that would close the opening.
    #[must_use]
    pub const fn closing(&self) -> CheckRef {
        self.closing
    }
}

impl CheckGap {
    /// The finding a synthesis raises where the closing check has no attachment.
    pub const fn found(claim: ClaimRef, check: CheckRef, missing: OracleClass) -> Self {
        Self {
            claim,
            check,
            missing,
        }
    }

    /// The claim the opening belongs to.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// The check reference nobody has written an attachment for.
    #[must_use]
    pub const fn check(self) -> CheckRef {
        self.check
    }

    /// The oracle class the missing check would supply.
    #[must_use]
    pub const fn missing(self) -> OracleClass {
        self.missing
    }
}

impl CandidateSketch {
    /// The row coordinates the caller states beside an explanation.
    #[must_use]
    pub fn stated(
        suite: ExecutionSuite,
        classification: Classification,
        subject: SubjectRoute,
        population: PopulationRef,
    ) -> Self {
        Self {
            suite,
            classification,
            subject,
            population,
        }
    }

    /// The aggregate seat the candidate would run under.
    #[must_use]
    pub const fn suite(&self) -> ExecutionSuite {
        self.suite
    }

    /// How the candidate would be classified.
    #[must_use]
    pub const fn classification(&self) -> &Classification {
        &self.classification
    }

    /// What the candidate would exercise.
    #[must_use]
    pub const fn subject(&self) -> SubjectRoute {
        self.subject
    }

    /// The population that would supply its inputs.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.population
    }
}
impl OwedClaim {
    /// The owed posture a claim's declaration states.
    ///
    /// # Errors
    ///
    /// Refuses a posture naming no opening condition, because an obligation that never comes due is one nobody can discharge.
    pub const fn declared(
        claim: ClaimRef,
        opening_condition: &'static str,
    ) -> Result<Self, OwedClaimRefusal> {
        if opening_condition.is_empty() {
            return Err(OwedClaimRefusal::NoOpeningCondition);
        }
        Ok(Self {
            claim,
            opening_condition,
        })
    }

    /// The claim declared owed.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// The condition its declaration named as the opening.
    #[must_use]
    pub const fn opening_condition(self) -> &'static str {
        self.opening_condition
    }
}

impl OwedDeclaration {
    /// One owed claim and the shape of proof its opening asks for.
    #[must_use]
    pub const fn stated(owed: OwedClaim, shape: ProofShape) -> Self {
        Self { owed, shape }
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(self) -> OwedClaim {
        self.owed
    }

    /// The shape of proof it asks for.
    #[must_use]
    pub const fn shape(self) -> ProofShape {
        self.shape
    }
}

impl InferredObligation {
    /// One opening a coverage reading stated.
    #[must_use]
    pub const fn inferred(owed: OwedClaim, exercise: ClaimExercise, shape: ProofShape) -> Self {
        Self {
            owed,
            exercise,
            shape,
        }
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(self) -> OwedClaim {
        self.owed
    }

    /// The counts the coverage reading recorded for it.
    #[must_use]
    pub const fn exercise(self) -> ClaimExercise {
        self.exercise
    }

    /// The shape of proof the opening asks for.
    #[must_use]
    pub const fn shape(self) -> ProofShape {
        self.shape
    }
}

impl DischargeEvidence {
    /// What discharged one owed claim.
    #[must_use]
    pub fn recorded(lane: ObligationLane, trial: TrialId, key: ExecutionKey) -> Self {
        Self { lane, trial, key }
    }

    /// The lane the obligation was routed to.
    #[must_use]
    pub const fn lane(&self) -> ObligationLane {
        self.lane
    }

    /// The trial that discharged it.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The key that trial ran under.
    #[must_use]
    pub const fn key(&self) -> &ExecutionKey {
        &self.key
    }
}

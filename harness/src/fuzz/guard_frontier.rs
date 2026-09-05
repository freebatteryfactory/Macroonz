//! The frontier roads: the observation, the corpus with its reservation and admission, the interesting bytes it mints, and the profile result the admission consumes.

use crate::fuzz::types::{
    CoverageAdmission, CoverageAdmissionRefusal, CoverageCorpus, CoverageObservation,
    CoveragePoint, CoverageStanding, FuzzExecution, InterestingBytes, ReadyPreflight,
    RustcProfileRefusal, RustcProfileResult,
};
use std::collections::BTreeSet;

impl InterestingBytes {
    /// The exact interesting byte string.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl CoverageObservation {
    pub(crate) fn established(points: BTreeSet<CoveragePoint>) -> Self {
        Self {
            points: points.into_iter().collect(),
        }
    }

    pub(crate) const fn empty() -> Self {
        Self { points: Vec::new() }
    }

    /// The canonical covered points in lexical order.
    #[must_use]
    pub fn points(&self) -> &[CoveragePoint] {
        &self.points
    }
}

impl CoverageCorpus {
    /// Open an empty coverage frontier under one actively qualified campaign standing.
    #[must_use]
    pub fn opening(ready: &ReadyPreflight) -> Self {
        Self {
            standing: ready.standing().clone(),
            attempted_cases: 0,
            attempted_input_bytes: 0,
            observed: BTreeSet::new(),
            interesting: Vec::new(),
            retained_bytes: 0,
        }
    }

    pub(crate) fn reserve_execution(
        &mut self,
        ready: &ReadyPreflight,
        candidate_bytes: usize,
    ) -> Result<u32, RustcProfileRefusal> {
        if self.standing != *ready.standing() {
            return Err(RustcProfileRefusal::CampaignMismatch);
        }
        let budgets = self.standing.campaign().budgets();
        let case_bound = budgets.executions().cases();
        if self.attempted_cases >= case_bound {
            return Err(RustcProfileRefusal::CaseBudgetExhausted { bound: case_bound });
        }
        let candidate_bytes = u64::try_from(candidate_bytes).unwrap_or(u64::MAX);
        let attempted = self.attempted_input_bytes.saturating_add(candidate_bytes);
        let input_bound = budgets.input_bytes().bytes();
        if attempted > input_bound {
            return Err(RustcProfileRefusal::InputBudgetExhausted {
                bound: input_bound,
                attempted,
            });
        }
        let case = self.attempted_cases;
        self.attempted_cases = self.attempted_cases.saturating_add(1);
        self.attempted_input_bytes = attempted;
        Ok(case)
    }

    /// Compare one joined execution reading with the accumulated frontier.
    ///
    /// # Errors
    ///
    /// Refuses another campaign standing, a non-successful execution, an empty observation, or a point or retention ceiling.
    pub fn admit(
        &mut self,
        reading: RustcProfileResult,
    ) -> Result<CoverageAdmission, CoverageAdmissionRefusal> {
        if self.standing != reading.standing {
            return Err(CoverageAdmissionRefusal::CampaignMismatch);
        }
        if reading.execution != FuzzExecution::Success {
            return Err(CoverageAdmissionRefusal::Execution(reading.execution));
        }
        if reading.observation.points().is_empty() {
            return Err(CoverageAdmissionRefusal::EmptyObservation);
        }
        let novel_points = reading
            .observation
            .points()
            .iter()
            .filter(|point| !self.observed.contains(*point))
            .count();
        if novel_points == 0 {
            return Ok(CoverageAdmission::Known);
        }
        let attempted_points = u64::try_from(self.observed.len())
            .unwrap_or(u64::MAX)
            .saturating_add(u64::try_from(novel_points).unwrap_or(u64::MAX));
        let budgets = self.standing.campaign().budgets();
        if attempted_points > budgets.points() {
            return Err(CoverageAdmissionRefusal::PointBudgetExhausted {
                bound: budgets.points(),
                attempted: attempted_points,
            });
        }
        let retained_cases = u32::try_from(self.interesting.len()).unwrap_or(u32::MAX);
        let retained_case_bound = budgets.retained_cases().cases();
        if retained_cases >= retained_case_bound {
            return Err(CoverageAdmissionRefusal::RetainedCaseBudgetExhausted {
                bound: retained_case_bound,
            });
        }
        let candidate_bytes = u64::try_from(reading.candidate.len()).unwrap_or(u64::MAX);
        let retained_bytes = self.retained_bytes.saturating_add(candidate_bytes);
        let retained_byte_bound = budgets.retained_bytes().bytes();
        if retained_bytes > retained_byte_bound {
            return Err(CoverageAdmissionRefusal::RetainedByteBudgetExhausted {
                bound: retained_byte_bound,
                attempted: retained_bytes,
            });
        }
        self.observed
            .extend(reading.observation.points().iter().cloned());
        let interesting = InterestingBytes {
            bytes: reading.candidate,
        };
        self.interesting.push(interesting.clone());
        self.retained_bytes = retained_bytes;
        Ok(CoverageAdmission::Interesting(interesting))
    }

    /// The campaign standing this frontier accepts.
    #[must_use]
    pub const fn standing(&self) -> &CoverageStanding {
        &self.standing
    }

    /// How many candidate attempts this campaign has spent.
    #[must_use]
    pub const fn attempted_cases(&self) -> u32 {
        self.attempted_cases
    }

    /// How many candidate bytes this campaign has spent across attempts.
    #[must_use]
    pub const fn attempted_input_bytes(&self) -> u64 {
        self.attempted_input_bytes
    }

    /// Every point observed across admitted candidates.
    #[must_use]
    pub const fn observed(&self) -> &BTreeSet<CoveragePoint> {
        &self.observed
    }

    /// Interesting candidates in admission order.
    #[must_use]
    pub fn interesting(&self) -> &[InterestingBytes] {
        &self.interesting
    }

    /// The cumulative bytes retained by coverage novelty.
    #[must_use]
    pub const fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }
}

impl RustcProfileResult {
    pub(crate) const fn established(
        case: u32,
        candidate: Vec<u8>,
        execution: FuzzExecution,
        observation: CoverageObservation,
        standing: CoverageStanding,
    ) -> Self {
        Self {
            case,
            candidate,
            execution,
            observation,
            standing,
        }
    }

    /// The zero-based case ordinal reserved by the campaign.
    #[must_use]
    pub const fn case(&self) -> u32 {
        self.case
    }

    /// The exact candidate bytes that produced this reading.
    #[must_use]
    pub fn candidate(&self) -> &[u8] {
        &self.candidate
    }

    /// How the instrumented target process ended.
    #[must_use]
    pub const fn execution(&self) -> FuzzExecution {
        self.execution
    }

    /// Coverage the target flushed before it ended.
    #[must_use]
    pub const fn observation(&self) -> &CoverageObservation {
        &self.observation
    }

    /// The campaign, target, and toolchain under which this reading was produced.
    #[must_use]
    pub const fn standing(&self) -> &CoverageStanding {
        &self.standing
    }
}

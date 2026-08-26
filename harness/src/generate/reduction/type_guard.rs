//! Every road that reaches a private field of the reduction home, and every reader that hands one back.
//!
//! Declared inside `types.rs` as its own child, so it sees fields no sibling module does.
//! Empty reducer rosters, duplicate semantic reducers, zero budgets, non-descending candidates, and unbound probes are refused here, which makes those claims structural rather than remembered.

use super::{
    ByteReducerExecution, ByteReducerId, FingerprintPreservation, FingerprintProbe,
    ReductionBudget, ReductionCensus, ReductionEvidence, ReductionHalt, ReductionOutcome,
    ReductionPlan, ReductionPlanRefusal, ReductionProbeBinding, ReductionProbeRefusal,
    SemanticCandidateRefusal, SemanticCandidates, SemanticReducerBinding, SemanticReducerCall,
    SemanticReducerExecution, SemanticReducerId, ShrinkVerdict,
};
use crate::descriptor::{GeneratedSupportSchemaId, NameRefusal, NamespacedName, RevisionBinding};
use crate::report::{
    Fingerprint, GenerationProfile, MinimizationProfile, ReplayPosture, RunAttempt,
    TrialConclusion, TrialReport, TrialRunStanding,
};
use std::collections::BTreeSet;

// The reduction plan and the reducers it binds.

impl SemanticReducerId {
    /// This reducer, parsed from the owner that declares it and the spelling it carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem.
    pub fn named(namespace: &'static str, stem: &'static str) -> Result<Self, NameRefusal> {
        NamespacedName::named(namespace, stem).map(Self)
    }

    /// This reducer, over a name already parsed.
    #[must_use]
    pub const fn over(name: NamespacedName) -> Self {
        Self(name)
    }

    /// The namespaced name this reducer carries.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.0
    }
}

impl SemanticCandidates {
    /// The ordered candidates one semantic reducer proposes for this input.
    ///
    /// Empty is a lawful fixed point.
    /// Every present candidate must be strictly shorter than the input or candidate immediately before it, so a semantic reducer cannot move the shared reduction backwards or cycle it.
    ///
    /// # Errors
    ///
    /// Refuses the first candidate that does not strictly decrease the byte length, carrying its position and both lengths.
    pub fn proposed(
        input: &[u8],
        candidates: Vec<Vec<u8>>,
    ) -> Result<Self, SemanticCandidateRefusal> {
        let mut predecessor_bytes = input.len();
        for (position, candidate) in candidates.iter().enumerate() {
            let candidate_bytes = candidate.len();
            if candidate_bytes >= predecessor_bytes {
                return Err(SemanticCandidateRefusal::NotStrictlySmaller {
                    position,
                    predecessor_bytes,
                    candidate_bytes,
                });
            }
            predecessor_bytes = candidate_bytes;
        }
        Ok(Self { candidates })
    }

    /// The strictly descending candidates, in the reducer's declared order.
    #[must_use]
    pub fn candidates(&self) -> &[Vec<u8>] {
        &self.candidates
    }

    /// The candidates, taken by the shared reduction engine.
    pub(crate) fn into_candidates(self) -> Vec<Vec<u8>> {
        self.candidates
    }
}

impl SemanticReducerBinding {
    /// Bind one semantic reducer identity and revision to the callable executed for it.
    #[must_use]
    pub const fn bound(
        reducer: SemanticReducerId,
        revision: RevisionBinding,
        call: SemanticReducerCall,
    ) -> Self {
        Self {
            reducer,
            revision,
            call,
        }
    }

    /// The semantic reducer this binding names.
    #[must_use]
    pub const fn reducer(self) -> SemanticReducerId {
        self.reducer
    }

    /// The revision posture bound to the callable.
    #[must_use]
    pub const fn revision(self) -> RevisionBinding {
        self.revision
    }

    /// Invoke the bound reducer over one current best input.
    pub(crate) fn call(self, input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
        (self.call)(input)
    }
}

impl SemanticReducerExecution {
    /// Retain one reducer invocation and how much candidate work it offered.
    #[must_use]
    pub(crate) const fn recorded(
        binding: SemanticReducerBinding,
        candidates: usize,
        probes: usize,
    ) -> Self {
        Self {
            reducer: binding.reducer(),
            revision: binding.revision(),
            candidates,
            probes,
        }
    }

    /// The semantic reducer invoked.
    #[must_use]
    pub const fn reducer(self) -> SemanticReducerId {
        self.reducer
    }

    /// The revision standing the invocation ran under.
    #[must_use]
    pub const fn revision(self) -> RevisionBinding {
        self.revision
    }

    /// How many candidates the reducer returned.
    #[must_use]
    pub const fn candidates(self) -> usize {
        self.candidates
    }

    /// How many of those candidates the shared engine probed before its budget stopped it.
    #[must_use]
    pub const fn probes(self) -> usize {
        self.probes
    }
}

impl ReductionBudget {
    /// The probe budget the plan's author declared.
    #[must_use]
    pub const fn declared(probes: u32) -> Self {
        Self(probes)
    }

    /// How many candidate probes the plan admits.
    #[must_use]
    pub const fn probes(self) -> u32 {
        self.0
    }
}

impl ReductionPlan {
    /// The plan its author declared.
    ///
    /// # Errors
    ///
    /// Refuses a zero budget, then a semantic reducer named more than once.
    pub fn declared(
        profile: MinimizationProfile,
        byte_reducer: ByteReducerId,
        semantic_reducers: Vec<SemanticReducerBinding>,
        preservation: FingerprintPreservation,
        budget: ReductionBudget,
    ) -> Result<Self, ReductionPlanRefusal> {
        if budget.probes() == 0 {
            return Err(ReductionPlanRefusal::ZeroReductionBudget);
        }
        let mut roster: BTreeSet<SemanticReducerId> = BTreeSet::new();
        for binding in &semantic_reducers {
            if !roster.insert(binding.reducer()) {
                return Err(ReductionPlanRefusal::DuplicateSemanticReducer(
                    binding.reducer(),
                ));
            }
        }
        Ok(Self {
            profile,
            byte_reducer,
            semantic_reducers,
            preservation,
            budget,
        })
    }

    /// The minimization profile and version the reduction runs under.
    #[must_use]
    pub const fn profile(&self) -> MinimizationProfile {
        self.profile
    }

    /// The generic byte reducer the plan binds.
    #[must_use]
    pub const fn byte_reducer(&self) -> ByteReducerId {
        self.byte_reducer
    }

    /// The semantic reducers the plan invokes, in authored order.
    #[must_use]
    pub fn semantic_reducers(&self) -> &[SemanticReducerBinding] {
        &self.semantic_reducers
    }

    /// That the reduction preserves the failure fingerprint.
    #[must_use]
    pub const fn preservation(&self) -> FingerprintPreservation {
        self.preservation
    }

    /// How many candidate probes the reduction admits.
    #[must_use]
    pub const fn budget(&self) -> ReductionBudget {
        self.budget
    }
}

impl ReductionProbeBinding {
    /// Bind a byte-input probe and its declared revision to one real refused report.
    ///
    /// The fingerprint derives from the report, so no caller supplies one beside the standing.
    /// The generation profile and schema are the caller's typed statements about the input road and stay under that declaration ceiling.
    ///
    /// # Errors
    ///
    /// Refuses a report that did not execute to a conclusion, then one whose conclusion passed and therefore carries no failure fingerprint.
    pub fn bound(
        report: &TrialReport,
        generation: GenerationProfile,
        schema: GeneratedSupportSchemaId,
        revision: RevisionBinding,
        probe: FingerprintProbe,
    ) -> Result<Self, ReductionProbeRefusal> {
        let finding = match report.attempt() {
            RunAttempt::Executed(TrialConclusion::Refused(finding)) => finding,
            RunAttempt::Executed(TrialConclusion::Passed) => {
                return Err(ReductionProbeRefusal::TrialPassed);
            }
            RunAttempt::SkippedWithReason(_)
            | RunAttempt::TimedOut
            | RunAttempt::InfrastructureFailed(_) => {
                return Err(ReductionProbeRefusal::TrialDidNotConclude);
            }
        };
        Ok(Self {
            standing: report.standing().clone(),
            preserved: Fingerprint::of(report.trial(), finding),
            generation,
            schema,
            revision,
            probe,
        })
    }

    /// The exact execution standing the reduction reproduces.
    #[must_use]
    pub const fn standing(&self) -> &TrialRunStanding {
        &self.standing
    }

    /// The report-derived failure fingerprint the reduction preserves.
    #[must_use]
    pub const fn preserved(&self) -> Fingerprint {
        self.preserved
    }

    /// The generation profile declared for the input road.
    #[must_use]
    pub const fn generation(&self) -> GenerationProfile {
        self.generation
    }

    /// The generated-support schema identity declared for the input road.
    #[must_use]
    pub const fn schema(&self) -> GeneratedSupportSchemaId {
        self.schema
    }

    /// The revision posture bound to the probe adapter.
    #[must_use]
    pub const fn revision(&self) -> RevisionBinding {
        self.revision
    }

    /// The byte-input probe bound to this exact report standing.
    #[must_use]
    pub(crate) const fn probe(&self) -> FingerprintProbe {
        self.probe
    }

    /// The replay ceiling after the report and the probe adapter meet.
    #[must_use]
    pub(crate) fn replay_posture(&self) -> ReplayPosture {
        self.standing
            .replay()
            .meet_revision(self.revision.posture())
    }
}

// What a reduction counts and leaves behind.

impl ReductionCensus {
    /// An accounting opened with every seat at zero.
    #[must_use]
    pub const fn opening() -> Self {
        Self {
            accepted: 0,
            fingerprint_moved: 0,
            no_failure: 0,
        }
    }

    /// Count one candidate under the verdict it earned.
    pub fn count(&mut self, verdict: ShrinkVerdict) {
        match verdict {
            ShrinkVerdict::Accepted => self.accepted = self.accepted.saturating_add(1),
            ShrinkVerdict::RejectedFingerprintMoved { found: _ } => {
                self.fingerprint_moved = self.fingerprint_moved.saturating_add(1);
            }
            ShrinkVerdict::RejectedNoFailure => {
                self.no_failure = self.no_failure.saturating_add(1);
            }
        }
    }

    /// How many candidates carried the fingerprint through.
    #[must_use]
    pub const fn accepted(self) -> u32 {
        self.accepted
    }

    /// How many candidates failed under a different fingerprint.
    ///
    /// Every one of these is a shrink the reduction refused, so the count is the evidence that minimization stayed on the bug it started from.
    #[must_use]
    pub const fn fingerprint_moved(self) -> u32 {
        self.fingerprint_moved
    }

    /// How many candidates stopped failing.
    #[must_use]
    pub const fn no_failure(self) -> u32 {
        self.no_failure
    }

    /// How many candidate probes were spent, over every seat.
    #[must_use]
    pub const fn probes(self) -> u32 {
        self.accepted
            .saturating_add(self.fingerprint_moved)
            .saturating_add(self.no_failure)
    }
}

impl ReductionOutcome {
    /// What one reduction produced.
    #[must_use]
    pub fn reduced(
        input: Vec<u8>,
        fingerprint: Fingerprint,
        census: ReductionCensus,
        halt: ReductionHalt,
    ) -> Self {
        Self {
            input,
            fingerprint,
            census,
            halt,
        }
    }

    /// The smallest input the reduction reached.
    #[must_use]
    pub fn input(&self) -> &[u8] {
        &self.input
    }

    /// The fingerprint the reduced input still carries.
    ///
    /// Carried here so an outcome is readable without the call that produced it.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// The accounting over every candidate.
    #[must_use]
    pub const fn census(&self) -> ReductionCensus {
        self.census
    }

    /// Why the reduction stopped.
    #[must_use]
    pub const fn halt(&self) -> ReductionHalt {
        self.halt
    }

    /// The reduced input, taken by a caller that consumes it.
    #[must_use]
    pub fn into_input(self) -> Vec<u8> {
        self.input
    }
}

impl ReductionEvidence {
    /// Retain one completed reduction and every authority-bearing input it ran under.
    #[must_use]
    pub(crate) fn recorded(
        probe: &ReductionProbeBinding,
        minimization: MinimizationProfile,
        semantic_reducers: Vec<SemanticReducerExecution>,
        byte_reducer: ByteReducerExecution,
        outcome: ReductionOutcome,
        replay: ReplayPosture,
    ) -> Self {
        Self {
            standing: probe.standing().clone(),
            generation: probe.generation(),
            schema: probe.schema(),
            probe_revision: probe.revision(),
            minimization,
            semantic_reducers,
            byte_reducer,
            outcome,
            replay,
        }
    }

    /// The exact execution standing reproduced by the reduction probe.
    #[must_use]
    pub const fn standing(&self) -> &TrialRunStanding {
        &self.standing
    }

    /// The generation profile declared for the reduced input.
    #[must_use]
    pub const fn generation(&self) -> GenerationProfile {
        self.generation
    }

    /// The generated-support schema identity declared for the reduced input.
    #[must_use]
    pub const fn schema(&self) -> GeneratedSupportSchemaId {
        self.schema
    }

    /// The revision posture bound to the re-execution probe.
    #[must_use]
    pub const fn probe_revision(&self) -> RevisionBinding {
        self.probe_revision
    }

    /// The minimization profile the reduction plan declared.
    #[must_use]
    pub const fn minimization(&self) -> MinimizationProfile {
        self.minimization
    }

    /// The semantic reducers actually invoked, in execution order.
    #[must_use]
    pub fn semantic_reducers(&self) -> &[SemanticReducerExecution] {
        &self.semantic_reducers
    }

    /// Whether the generic byte reducer was reached.
    #[must_use]
    pub const fn byte_reducer(&self) -> ByteReducerExecution {
        self.byte_reducer
    }

    /// The reduced input, fingerprint, census, and halt posture.
    #[must_use]
    pub const fn outcome(&self) -> &ReductionOutcome {
        &self.outcome
    }

    /// The replay ceiling after the report, the probe, and every invoked reducer meet.
    #[must_use]
    pub const fn replay_posture(&self) -> ReplayPosture {
        self.replay
    }
}

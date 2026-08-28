//! Smart constructors and readers for the fuzz home.

use super::{
    CoverageAdmission, CoverageAdmissionRefusal, CoverageBudgetRefusal, CoverageBudgets,
    CoverageCampaign, CoverageCorpus, CoverageObservation, CoveragePoint, CoverageProfile,
    CoverageSource, CoverageSourceRoot, CoverageSourceRootRefusal, CoverageStanding, FuzzExecution,
    InstrumentedTarget, InterestingBytes, MutationCandidate, MutationKind, MutationPlan,
    MutationPlanRefusal, ReadyPreflight, RustcCoverageTools, RustcProfileRefusal,
    RustcProfileRequest, RustcProfileRequestRefusal, RustcProfileResult,
};
use crate::descriptor::NamespacedName;
use crate::report::{ByteBudget, CaseBudget, TargetBinding};
use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};

impl CoverageSourceRoot {
    /// Declare one logical source root and its absolute checkout seat.
    ///
    /// # Errors
    ///
    /// Refuses an empty, relative, traversing, or non-UTF-8 checkout path.
    pub fn declared(
        logical: NamespacedName,
        checkout: PathBuf,
    ) -> Result<Self, CoverageSourceRootRefusal> {
        if checkout.as_os_str().is_empty() {
            return Err(CoverageSourceRootRefusal::EmptyCheckout);
        }
        if !checkout.is_absolute() {
            return Err(CoverageSourceRootRefusal::RelativeCheckout);
        }
        if checkout
            .components()
            .any(|component| matches!(component, Component::ParentDir))
        {
            return Err(CoverageSourceRootRefusal::CheckoutTraversal);
        }
        if checkout.to_str().is_none() {
            return Err(CoverageSourceRootRefusal::NonUtf8Checkout);
        }
        Ok(Self { logical, checkout })
    }

    pub(crate) const fn logical(&self) -> NamespacedName {
        self.logical
    }

    pub(crate) fn checkout(&self) -> &Path {
        &self.checkout
    }
}

impl CoverageSource {
    pub(crate) const fn established(root: NamespacedName, relative: String) -> Self {
        Self { root, relative }
    }

    /// The caller-declared logical root.
    #[must_use]
    pub const fn root(&self) -> NamespacedName {
        self.root
    }

    /// The canonical slash-separated path beneath the logical root.
    #[must_use]
    pub fn relative(&self) -> &str {
        &self.relative
    }
}

impl CoverageProfile {
    /// Declare the named interpretation and version applied to coverage exports.
    #[must_use]
    pub const fn declared(name: NamespacedName, version: u32) -> Self {
        Self { name, version }
    }

    /// The profile's declared name.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.name
    }

    /// The profile's declared version.
    #[must_use]
    pub const fn version(self) -> u32 {
        self.version
    }
}

impl CoverageBudgets {
    /// Declare every resource ceiling of one coverage campaign.
    ///
    /// # Errors
    ///
    /// Refuses a zero ceiling in any dimension.
    pub const fn declared(
        executions: CaseBudget,
        input_bytes: ByteBudget,
        export_bytes: u64,
        points: u64,
        retained_cases: CaseBudget,
        retained_bytes: ByteBudget,
    ) -> Result<Self, CoverageBudgetRefusal> {
        if executions.cases() == 0 {
            return Err(CoverageBudgetRefusal::Executions);
        }
        if input_bytes.bytes() == 0 {
            return Err(CoverageBudgetRefusal::InputBytes);
        }
        if export_bytes == 0 {
            return Err(CoverageBudgetRefusal::ExportBytes);
        }
        if points == 0 {
            return Err(CoverageBudgetRefusal::Points);
        }
        if retained_cases.cases() == 0 {
            return Err(CoverageBudgetRefusal::RetainedCases);
        }
        if retained_bytes.bytes() == 0 {
            return Err(CoverageBudgetRefusal::RetainedBytes);
        }
        Ok(Self {
            executions,
            input_bytes,
            export_bytes,
            points,
            retained_cases,
            retained_bytes,
        })
    }

    /// The candidate-attempt ceiling.
    #[must_use]
    pub const fn executions(self) -> CaseBudget {
        self.executions
    }

    /// The cumulative executed candidate-byte ceiling.
    #[must_use]
    pub const fn input_bytes(self) -> ByteBudget {
        self.input_bytes
    }

    /// The per-execution coverage-export byte ceiling.
    #[must_use]
    pub const fn export_bytes(self) -> u64 {
        self.export_bytes
    }

    /// The accumulated canonical coverage-point ceiling.
    #[must_use]
    pub const fn points(self) -> u64 {
        self.points
    }

    /// The retained coverage-novel case ceiling.
    #[must_use]
    pub const fn retained_cases(self) -> CaseBudget {
        self.retained_cases
    }

    /// The cumulative retained candidate-byte ceiling.
    #[must_use]
    pub const fn retained_bytes(self) -> ByteBudget {
        self.retained_bytes
    }
}

impl CoverageCampaign {
    /// Declare one population, target revision, coverage interpretation, and resource ceiling.
    #[must_use]
    pub const fn declared(
        population: crate::descriptor::PopulationRef,
        revision: crate::descriptor::RevisionBinding,
        profile: CoverageProfile,
        budgets: CoverageBudgets,
    ) -> Self {
        Self {
            population,
            revision,
            profile,
            budgets,
        }
    }

    /// The population whose candidates this campaign executes.
    #[must_use]
    pub const fn population(self) -> crate::descriptor::PopulationRef {
        self.population
    }

    /// The declared revision of the instrumented subject.
    #[must_use]
    pub const fn revision(self) -> crate::descriptor::RevisionBinding {
        self.revision
    }

    /// The coverage interpretation applied to exports.
    #[must_use]
    pub const fn profile(self) -> CoverageProfile {
        self.profile
    }

    /// The campaign's closed resource ceiling.
    #[must_use]
    pub const fn budgets(self) -> CoverageBudgets {
        self.budgets
    }
}

impl CoverageStanding {
    pub(crate) const fn established(campaign: CoverageCampaign, target: TargetBinding) -> Self {
        Self { campaign, target }
    }

    /// The caller-declared campaign facts.
    #[must_use]
    pub const fn campaign(&self) -> CoverageCampaign {
        self.campaign
    }

    /// The target and toolchain established by active preflight.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }
}

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

impl MutationPlan {
    /// Declare one bounded deterministic neighboring-input plan.
    ///
    /// # Errors
    ///
    /// Refuses a zero budget, zero byte ceiling, or empty dictionary token.
    pub fn declared(
        budget: u32,
        byte_limit: usize,
        dictionary: Vec<Vec<u8>>,
    ) -> Result<Self, MutationPlanRefusal> {
        if budget == 0 {
            return Err(MutationPlanRefusal::ZeroBudget);
        }
        if byte_limit == 0 {
            return Err(MutationPlanRefusal::ZeroByteLimit);
        }
        if let Some(at) = dictionary.iter().position(Vec::is_empty) {
            return Err(MutationPlanRefusal::EmptyDictionaryToken { at });
        }
        Ok(Self {
            budget,
            byte_limit,
            dictionary,
        })
    }

    pub(crate) const fn budget(&self) -> u32 {
        self.budget
    }

    pub(crate) const fn byte_limit(&self) -> usize {
        self.byte_limit
    }

    pub(crate) fn dictionary(&self) -> &[Vec<u8>] {
        &self.dictionary
    }
}

impl MutationCandidate {
    pub(crate) const fn established(kind: MutationKind, bytes: Vec<u8>) -> Self {
        Self { kind, bytes }
    }

    /// The operation that produced this neighbor.
    #[must_use]
    pub const fn kind(&self) -> MutationKind {
        self.kind
    }

    /// The exact neighboring bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl RustcCoverageTools {
    pub(crate) const fn established(profdata: PathBuf, cov: PathBuf) -> Self {
        Self { profdata, cov }
    }

    pub(crate) fn profdata(&self) -> &Path {
        &self.profdata
    }

    pub(crate) fn cov(&self) -> &Path {
        &self.cov
    }
}

impl InstrumentedTarget {
    /// Declare one already-instrumented target executable.
    ///
    /// # Errors
    ///
    /// Refuses an empty or relative executable path.
    pub fn declared(
        executable: PathBuf,
        arguments: Vec<String>,
    ) -> Result<Self, RustcProfileRequestRefusal> {
        if executable.as_os_str().is_empty() {
            return Err(RustcProfileRequestRefusal::Target);
        }
        if !executable.is_absolute() {
            return Err(RustcProfileRequestRefusal::RelativeTarget);
        }
        Ok(Self {
            executable,
            arguments,
        })
    }

    pub(crate) fn executable(&self) -> &Path {
        &self.executable
    }

    pub(crate) fn arguments(&self) -> &[String] {
        &self.arguments
    }
}

impl RustcProfileRequest {
    /// Declare one profile observation request.
    ///
    /// # Errors
    ///
    /// Refuses an empty or relative rustc or scratch path.
    pub fn declared(
        rustc: PathBuf,
        target: InstrumentedTarget,
        source_root: CoverageSourceRoot,
        scratch: PathBuf,
        campaign: CoverageCampaign,
    ) -> Result<Self, RustcProfileRequestRefusal> {
        if rustc.as_os_str().is_empty() {
            return Err(RustcProfileRequestRefusal::Rustc);
        }
        if !rustc.is_absolute() {
            return Err(RustcProfileRequestRefusal::RelativeRustc);
        }
        if scratch.as_os_str().is_empty() {
            return Err(RustcProfileRequestRefusal::Scratch);
        }
        if !scratch.is_absolute() {
            return Err(RustcProfileRequestRefusal::RelativeScratch);
        }
        Ok(Self {
            rustc,
            target,
            source_root,
            scratch,
            campaign,
        })
    }

    pub(crate) fn rustc(&self) -> &Path {
        &self.rustc
    }

    pub(crate) const fn target(&self) -> &InstrumentedTarget {
        &self.target
    }

    pub(crate) const fn source_root(&self) -> &CoverageSourceRoot {
        &self.source_root
    }

    pub(crate) const fn campaign(&self) -> CoverageCampaign {
        self.campaign
    }
}

impl ReadyPreflight {
    pub(crate) const fn target(&self) -> &InstrumentedTarget {
        &self.request.target
    }

    pub(crate) const fn tools(&self) -> &RustcCoverageTools {
        &self.tools
    }

    pub(crate) const fn source_root(&self) -> &CoverageSourceRoot {
        &self.source_root
    }

    pub(crate) fn scratch(&self) -> &Path {
        &self.request.scratch
    }

    /// The declared campaign joined to the target and toolchain established by preflight.
    #[must_use]
    pub const fn standing(&self) -> &CoverageStanding {
        &self.standing
    }

    /// The qualified compiler-reported rustc sysroot that owns the matching LLVM tools.
    #[must_use]
    pub fn sysroot(&self) -> &Path {
        &self.sysroot
    }

    /// The stable rustc release established by preflight.
    #[must_use]
    pub fn release(&self) -> &str {
        &self.release
    }

    /// The rustc host tuple established by preflight.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// The LLVM version shared by rustc and its matching tools.
    #[must_use]
    pub fn llvm_version(&self) -> &str {
        &self.llvm_version
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

//! Smart constructors and readers for the fuzz home.

use super::{
    BackendSelection, BackendSelectionRefusal, CoverageAdmission, CoverageAdmissionRefusal,
    CoverageCorpus, CoverageObservation, CoveragePoint, FuzzExecution, HostDisposition,
    InstrumentedTarget, InterestingBytes, InterestingBytesRefusal, MutationCandidate, MutationKind,
    MutationPlan, MutationPlanRefusal, NamedCeiling, PreflightCapability, PreflightFact,
    PreflightIncomplete, PreflightStatus, ReadyPreflight, RustcCoverageTools, RustcProfileRequest,
    RustcProfileRequestRefusal, RustcProfileResult, SelectedBackend,
};
use crate::descriptor::NamespacedName;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub(crate) const REQUIRED_RUSTC_COVERAGE: &[PreflightCapability] = &[
    PreflightCapability::RustcMsrv,
    PreflightCapability::RustcHostTuple,
    PreflightCapability::RustcSysroot,
    PreflightCapability::LlvmReported,
    PreflightCapability::LlvmToolsPreview,
    PreflightCapability::LlvmProfdata,
    PreflightCapability::LlvmCov,
    PreflightCapability::InstrumentCoverage,
];

pub(crate) const REQUIRED_RUSTC_CEILINGS: &[NamedCeiling] = &[
    NamedCeiling::FreshProcessPerCandidate,
    NamedCeiling::InstrumentedSourceTargetRequired,
    NamedCeiling::LlvmCoverageToolsRequired,
    NamedCeiling::CallerSuppliesProcessSupervisor,
];

pub(crate) const REQUIRED_RUSTC_HOSTS: &[HostDisposition] = &[
    HostDisposition::ObservedWindows,
    HostDisposition::UnexecutedLinux,
    HostDisposition::UnexecutedMacOs,
];

impl PreflightFact {
    /// Record one capability observation the caller already established.
    #[must_use]
    pub const fn declared(capability: PreflightCapability, status: PreflightStatus) -> Self {
        Self { capability, status }
    }

    /// The capability this fact names.
    #[must_use]
    pub const fn capability(self) -> PreflightCapability {
        self.capability
    }

    /// Whether the capability was available.
    #[must_use]
    pub const fn status(self) -> PreflightStatus {
        self.status
    }
}

impl ReadyPreflight {
    /// Judge caller-supplied facts for one selected backend.
    ///
    /// # Errors
    ///
    /// Refuses when a required capability is missing, duplicated, contradictory, or unavailable.
    pub fn from_facts(
        backend: SelectedBackend,
        facts: &[PreflightFact],
    ) -> Result<Self, PreflightIncomplete> {
        for capability in REQUIRED_RUSTC_COVERAGE {
            let fact = unique_required_fact(facts, *capability)?;
            if !matches!(fact.status(), PreflightStatus::Available) {
                return Err(PreflightIncomplete::Unavailable(*capability));
            }
        }
        Ok(Self { backend })
    }

    /// The backend this ready roster was judged for.
    #[must_use]
    pub const fn backend(self) -> SelectedBackend {
        self.backend
    }
}

impl BackendSelection {
    /// Select stable rustc coverage with its complete ceiling and host roster.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster or any selection that omits a required ceiling or host disposition.
    pub fn rustc_coverage(
        name: NamespacedName,
        ceilings: Vec<NamedCeiling>,
        hosts: Vec<HostDisposition>,
    ) -> Result<Self, BackendSelectionRefusal> {
        if ceilings.is_empty() {
            return Err(BackendSelectionRefusal::NoCeiling);
        }
        if hosts.is_empty() {
            return Err(BackendSelectionRefusal::NoHostDisposition);
        }
        for required in REQUIRED_RUSTC_CEILINGS {
            if !ceilings.iter().any(|ceiling| ceiling == required) {
                return Err(BackendSelectionRefusal::MissingRequiredCeiling(*required));
            }
        }
        for required in REQUIRED_RUSTC_HOSTS {
            if !hosts.iter().any(|host| host == required) {
                return Err(BackendSelectionRefusal::MissingRequiredHost(*required));
            }
        }
        Ok(Self {
            name,
            backend: SelectedBackend::RustcInstrumentCoverage,
            ceilings,
            hosts,
        })
    }

    /// The namespaced campaign name bound to this selection.
    #[must_use]
    pub const fn name(&self) -> NamespacedName {
        self.name
    }

    /// The selected backend.
    #[must_use]
    pub const fn backend(&self) -> SelectedBackend {
        self.backend
    }

    /// The named ceilings retained with the selection.
    #[must_use]
    pub fn ceilings(&self) -> &[NamedCeiling] {
        &self.ceilings
    }

    /// The host dispositions retained with the selection.
    #[must_use]
    pub fn hosts(&self) -> &[HostDisposition] {
        &self.hosts
    }
}

impl InterestingBytes {
    /// Admit nonempty bytes a coverage observation marked interesting.
    ///
    /// # Errors
    ///
    /// Refuses an empty byte string.
    pub fn admitted(bytes: Vec<u8>) -> Result<Self, InterestingBytesRefusal> {
        if bytes.is_empty() {
            return Err(InterestingBytesRefusal::Empty);
        }
        Ok(Self { bytes })
    }

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
    /// Open an empty coverage frontier.
    #[must_use]
    pub const fn opening() -> Self {
        Self {
            observed: BTreeSet::new(),
            interesting: Vec::new(),
        }
    }

    /// Compare one candidate observation with the accumulated frontier.
    ///
    /// # Errors
    ///
    /// Refuses an empty observation or empty candidate.
    pub fn admit(
        &mut self,
        candidate: Vec<u8>,
        observation: &CoverageObservation,
    ) -> Result<CoverageAdmission, CoverageAdmissionRefusal> {
        if observation.points().is_empty() {
            return Err(CoverageAdmissionRefusal::EmptyObservation);
        }
        if candidate.is_empty() {
            return Err(CoverageAdmissionRefusal::EmptyCandidate);
        }
        let adds_point = observation
            .points()
            .iter()
            .any(|point| !self.observed.contains(point));
        if !adds_point {
            return Ok(CoverageAdmission::Known);
        }
        self.observed.extend(observation.points().iter().cloned());
        let interesting = InterestingBytes { bytes: candidate };
        self.interesting.push(interesting.clone());
        Ok(CoverageAdmission::Interesting(interesting))
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
    /// Declare the exact matching LLVM profile tools.
    ///
    /// # Errors
    ///
    /// Refuses an empty tool path.
    pub fn declared(profdata: PathBuf, cov: PathBuf) -> Result<Self, RustcProfileRequestRefusal> {
        if profdata.as_os_str().is_empty() {
            return Err(RustcProfileRequestRefusal::Profdata);
        }
        if cov.as_os_str().is_empty() {
            return Err(RustcProfileRequestRefusal::Cov);
        }
        Ok(Self { profdata, cov })
    }

    pub(crate) fn profdata(&self) -> &std::path::Path {
        &self.profdata
    }

    pub(crate) fn cov(&self) -> &std::path::Path {
        &self.cov
    }
}

impl InstrumentedTarget {
    /// Declare one already-instrumented target executable.
    ///
    /// # Errors
    ///
    /// Refuses an empty executable path.
    pub fn declared(
        executable: PathBuf,
        arguments: Vec<String>,
    ) -> Result<Self, RustcProfileRequestRefusal> {
        if executable.as_os_str().is_empty() {
            return Err(RustcProfileRequestRefusal::Target);
        }
        Ok(Self {
            executable,
            arguments,
        })
    }

    pub(crate) fn executable(&self) -> &std::path::Path {
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
    /// Refuses an empty scratch path.
    pub fn declared(
        target: InstrumentedTarget,
        tools: RustcCoverageTools,
        scratch: PathBuf,
    ) -> Result<Self, RustcProfileRequestRefusal> {
        if scratch.as_os_str().is_empty() {
            return Err(RustcProfileRequestRefusal::Scratch);
        }
        Ok(Self {
            target,
            tools,
            scratch,
        })
    }

    pub(crate) const fn target(&self) -> &InstrumentedTarget {
        &self.target
    }

    pub(crate) const fn tools(&self) -> &RustcCoverageTools {
        &self.tools
    }

    pub(crate) fn scratch(&self) -> &std::path::Path {
        &self.scratch
    }
}

impl RustcProfileResult {
    pub(crate) const fn established(
        execution: FuzzExecution,
        observation: CoverageObservation,
    ) -> Self {
        Self {
            execution,
            observation,
        }
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
}

fn unique_required_fact(
    facts: &[PreflightFact],
    capability: PreflightCapability,
) -> Result<PreflightFact, PreflightIncomplete> {
    let matches: Vec<PreflightFact> = facts
        .iter()
        .copied()
        .filter(|fact| fact.capability() == capability)
        .collect();
    match matches.as_slice() {
        [] => Err(PreflightIncomplete::Missing(capability)),
        [only] => Ok(*only),
        [first, rest @ ..] => {
            let contradictory = rest.iter().any(|fact| fact.status() != first.status());
            if contradictory {
                Err(PreflightIncomplete::Contradictory(capability))
            } else {
                Err(PreflightIncomplete::Duplicate(capability))
            }
        }
    }
}

//! Smart constructors and readers for the fuzz home.

use super::{
    BackendSelection, BackendSelectionRefusal, FridaCampaign, FridaCampaignRefusal,
    FridaCampaignResult, FridaModuleName, FridaTarget, FridaTargetRefusal, FuzzExecution,
    HostDisposition, InterestingBytes, InterestingBytesRefusal, NamedCeiling, PreflightCapability,
    PreflightFact, PreflightIncomplete, PreflightStatus, ReadyPreflight, SelectedBackend,
};
use crate::descriptor::NamespacedName;

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

pub(crate) const REQUIRED_FRIDA_WINDOWS: &[PreflightCapability] = &[
    PreflightCapability::VsWhere,
    PreflightCapability::VcVarsAll,
    PreflightCapability::ComposedMsvcSdkEnv,
    PreflightCapability::RustcMsrv,
    PreflightCapability::RustcHostTuple,
    PreflightCapability::RustcSysroot,
    PreflightCapability::RustcTargetLibdir,
    PreflightCapability::RustStdDll,
    PreflightCapability::LlvmReported,
    PreflightCapability::FridaGumLib,
    PreflightCapability::FridaGumHeader,
    PreflightCapability::FridaDevkitHash,
];

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
            let contradictory = rest
                .iter()
                .any(|fact| fact.status() != first.status());
            if contradictory {
                Err(PreflightIncomplete::Contradictory(capability))
            } else {
                Err(PreflightIncomplete::Duplicate(capability))
            }
        }
    }
}

impl ReadyPreflight {
    /// Judge caller-supplied facts for one selected backend.
    ///
    /// # Errors
    ///
    /// Refuses when a required capability is missing, duplicated, contradictory, or marked unavailable.
    pub fn from_facts(
        backend: SelectedBackend,
        facts: &[PreflightFact],
    ) -> Result<Self, PreflightIncomplete> {
        let required = match backend {
            SelectedBackend::LibAflFrida => REQUIRED_FRIDA_WINDOWS,
        };
        for capability in required {
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

/// Every named ceiling the F0 Frida accept receipt retains with the selection.
pub(crate) const REQUIRED_F0_CEILINGS: &[NamedCeiling] = &[
    NamedCeiling::Lnk4098Coexistence,
    NamedCeiling::LibAppendMsvcSdk,
    NamedCeiling::RustStdDllOnPath,
    NamedCeiling::LinuxMacOsUnexecutedUntilWaveF,
];

/// Every host disposition the F0 Frida accept receipt retains with the selection.
pub(crate) const REQUIRED_F0_HOSTS: &[HostDisposition] = &[
    HostDisposition::ObservedWindows,
    HostDisposition::CredibleUnexecutedLinux,
    HostDisposition::CredibleUnexecutedMacOs,
];

impl BackendSelection {
    /// Select `LibAFL` plus Frida with the complete F0 ceiling and host roster.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster or any selection that omits a required F0 ceiling or host disposition.
    pub fn libafl_frida(
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
        for required in REQUIRED_F0_CEILINGS {
            if !ceilings.iter().any(|ceiling| ceiling == required) {
                return Err(BackendSelectionRefusal::MissingRequiredCeiling(*required));
            }
        }
        for required in REQUIRED_F0_HOSTS {
            if !hosts.iter().any(|host| host == required) {
                return Err(BackendSelectionRefusal::MissingRequiredHost(*required));
            }
        }
        Ok(Self {
            name,
            backend: SelectedBackend::LibAflFrida,
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
    /// Admit nonempty bytes a coverage backend marked interesting.
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

impl FuzzExecution {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::LawfulSuccess => 0,
            Self::TypedRefusal => 1,
            Self::NotUtf8 => 2,
            Self::Crash => 3,
            Self::Timeout => 4,
            Self::ResourceExhaustion => 5,
            Self::AmbiguousPartialAcceptance => 6,
        }
    }
}

impl FridaTarget {
    /// Declare one loaded module by its exact runtime name.
    ///
    /// # Errors
    ///
    /// Refuses an empty module name.
    pub fn named(name: impl Into<String>) -> Result<Self, FridaTargetRefusal> {
        FridaModuleName::declared(name).map(Self::NamedModule)
    }
}

impl FridaModuleName {
    fn declared(name: impl Into<String>) -> Result<Self, FridaTargetRefusal> {
        let name = name.into();
        if name.is_empty() {
            return Err(FridaTargetRefusal::EmptyModuleName);
        }
        Ok(Self { name })
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.name
    }
}

impl FridaCampaign {
    /// Declare one deterministic bounded native Frida campaign.
    ///
    /// # Errors
    ///
    /// Refuses missing seeds or zero execution, mutation, or timeout bounds.
    pub fn declared(
        target: FridaTarget,
        seeds: Vec<Vec<u8>>,
        handoff: FuzzExecution,
        random_seed: u64,
        iterations: u64,
        mutation_iterations: usize,
        timeout: std::time::Duration,
    ) -> Result<Self, FridaCampaignRefusal> {
        if seeds.is_empty() {
            return Err(FridaCampaignRefusal::NoSeeds);
        }
        if iterations == 0 {
            return Err(FridaCampaignRefusal::ZeroIterations);
        }
        if mutation_iterations == 0 {
            return Err(FridaCampaignRefusal::ZeroMutationIterations);
        }
        if timeout.is_zero() {
            return Err(FridaCampaignRefusal::ZeroTimeout);
        }
        Ok(Self {
            target,
            seeds,
            handoff,
            random_seed,
            iterations,
            mutation_iterations,
            timeout,
        })
    }

    pub(crate) const fn target(&self) -> &FridaTarget {
        &self.target
    }

    pub(crate) fn seeds(&self) -> &[Vec<u8>] {
        &self.seeds
    }

    pub(crate) const fn handoff(&self) -> FuzzExecution {
        self.handoff
    }

    pub(crate) const fn random_seed(&self) -> u64 {
        self.random_seed
    }

    pub(crate) const fn iterations(&self) -> u64 {
        self.iterations
    }

    pub(crate) const fn mutation_iterations(&self) -> usize {
        self.mutation_iterations
    }

    pub(crate) const fn timeout(&self) -> std::time::Duration {
        self.timeout
    }
}

impl FridaCampaignResult {
    pub(crate) const fn established(
        corpus_after_seeds: usize,
        corpus_after_loop: usize,
        nonempty_edge_entries: u64,
        monitor_events: usize,
        execution_counts: [u64; 7],
        interesting: InterestingBytes,
    ) -> Self {
        Self {
            corpus_after_seeds,
            corpus_after_loop,
            nonempty_edge_entries,
            monitor_events,
            execution_counts,
            interesting,
        }
    }

    /// The corpus population after declared seeds were evaluated.
    #[must_use]
    pub const fn corpus_after_seeds(&self) -> usize {
        self.corpus_after_seeds
    }

    /// The corpus population after the bounded mutational loop.
    #[must_use]
    pub const fn corpus_after_loop(&self) -> usize {
        self.corpus_after_loop
    }

    /// Nonzero entries in the final target-relative edge map.
    #[must_use]
    pub const fn nonempty_edge_entries(&self) -> u64 {
        self.nonempty_edge_entries
    }

    /// Events emitted by the `LibAFL` monitor during the campaign.
    #[must_use]
    pub const fn monitor_events(&self) -> usize {
        self.monitor_events
    }

    /// Executions observed under one caller-supplied classification.
    #[must_use]
    pub fn executions(&self, execution: FuzzExecution) -> u64 {
        self.execution_counts
            .get(execution.index())
            .copied()
            .unwrap_or(0)
    }

    /// The exact evolved bytes selected for Macroonz reduction and replay.
    #[must_use]
    pub const fn interesting(&self) -> &InterestingBytes {
        &self.interesting
    }
}

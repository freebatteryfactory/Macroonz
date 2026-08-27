//! Smart constructors and readers for the fuzz home.

use super::{
    BackendSelection, BackendSelectionRefusal, HostDisposition, InterestingBytes,
    InterestingBytesRefusal, NamedCeiling, PreflightCapability, PreflightFact, PreflightIncomplete,
    PreflightStatus, ReadyPreflight, SelectedBackend,
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

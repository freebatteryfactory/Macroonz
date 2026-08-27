//! Every public type of the fuzz home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`.

use crate::descriptor::NamespacedName;

#[path = "type_guard.rs"]
mod guard;

/// Exact `LibAFL` crate pin the F0 selection established.
pub const LIBAFL_PIN: &str = "0.16.1";

/// Exact `frida-gum` crate pin the F0 selection established.
pub const FRIDA_GUM_CRATE_PIN: &str = "0.17.2";

/// Exact Frida Gum Windows x86-64 devkit pin the F0 selection established.
pub const FRIDA_GUM_WINDOWS_X86_64_DEVKIT: &str = "17.9.5";

/// Which native coverage backend the campaign selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectedBackend {
    /// `LibAFL` plus Frida under the named F0 ceilings.
    LibAflFrida,
}

/// One named Windows/process choreography ceiling retained with Frida selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamedCeiling {
    /// Residual LNK4098 / mixed defaultlib coexistence after structural CRT attempts.
    Lnk4098Coexistence,
    /// `LIB` must append MSVC and Windows SDK roots rather than replace them with the Frida devkit alone.
    LibAppendMsvcSdk,
    /// Rust 1.98 `std-*.dll` directory must be on `PATH` when the driver and target import those DLLs.
    RustStdDllOnPath,
    /// Linux and macOS Macroonz receipts remain unexecuted until Wave F hosts run them.
    LinuxMacOsUnexecutedUntilWaveF,
}

/// What one host class established for the selected backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostDisposition {
    /// Native execution was observed on Windows.
    ObservedWindows,
    /// Upstream supports the host; Macroonz has not yet executed a native receipt.
    CredibleUnexecutedLinux,
    /// Upstream supports the host; Macroonz has not yet executed a native receipt.
    CredibleUnexecutedMacOs,
}

/// One capability a Frida Windows cold-shell preflight may name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreflightCapability {
    /// Visual Studio locator (`vswhere`).
    VsWhere,
    /// MSVC environment script (`vcvarsall` / equivalent).
    VcVarsAll,
    /// Composed MSVC and Windows SDK search paths.
    ComposedMsvcSdkEnv,
    /// Rustc at the product MSRV.
    RustcMsrv,
    /// Host triple from `rustc --print`.
    RustcHostTuple,
    /// Sysroot from `rustc --print`.
    RustcSysroot,
    /// Target libdir from `rustc --print`.
    RustcTargetLibdir,
    /// Rust standard-library DLL search directory.
    RustStdDll,
    /// LLVM version rustc reports.
    LlvmReported,
    /// Frida Gum static library present at the declared path.
    FridaGumLib,
    /// Frida Gum header present at the declared path.
    FridaGumHeader,
    /// Pinned Frida devkit archive hash matched.
    FridaDevkitHash,
}

/// Whether one declared preflight capability was available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreflightStatus {
    /// The capability was present under the caller's declared fact.
    Available,
    /// The capability was absent under the caller's declared fact.
    Unavailable,
}

/// One caller-supplied preflight observation.
///
/// The harness never discovers these facts; the driver or adopter hands them in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PreflightFact {
    capability: PreflightCapability,
    status: PreflightStatus,
}

/// Why a preflight roster was incomplete.
#[must_use = "a refusal is the reason fuzz preflight was not ready"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightIncomplete {
    /// At least one required capability was unavailable.
    Unavailable(PreflightCapability),
    /// A required capability was not present in the declared roster.
    Missing(PreflightCapability),
    /// The same capability was declared more than once.
    Duplicate(PreflightCapability),
    /// The same capability was declared with disagreeing availability.
    Contradictory(PreflightCapability),
}

/// A preflight roster in which every required capability was declared Available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReadyPreflight {
    backend: SelectedBackend,
}

/// The selected backend together with the ceilings and host dispositions it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendSelection {
    name: NamespacedName,
    backend: SelectedBackend,
    ceilings: Vec<NamedCeiling>,
    hosts: Vec<HostDisposition>,
}

/// Why a backend selection was refused.
#[must_use = "a refusal is the reason a fuzz backend selection was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendSelectionRefusal {
    /// No named ceiling was declared, so the selection would hide the F0 coexistence facts.
    NoCeiling,
    /// No host disposition was declared.
    NoHostDisposition,
    /// Windows observation was claimed without the LNK4098 coexistence ceiling.
    WindowsWithoutLnk4098Ceiling,
    /// A Linux or macOS credible-unexecuted host was claimed without the Wave F ceiling.
    CrossHostWithoutWaveFCeiling,
}

/// Exact bytes a coverage backend admitted as interesting for Macroonz reduction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterestingBytes {
    bytes: Vec<u8>,
}

/// Why interesting bytes were refused.
#[must_use = "a refusal is the reason interesting bytes were not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterestingBytesRefusal {
    /// The byte string was empty, so it cannot seed a reduction.
    Empty,
}

/// Why compose-reduce-replay refused.
#[must_use = "a refusal is the reason fuzz compose did not mint a replay capsule"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposeRefusal {
    /// The reduction road refused the seed under the bound probe.
    Reduction(crate::generate::ReductionRefusal),
    /// Replay did not re-establish the preserved fingerprint.
    ReplayFingerprintMoved,
    /// Replay established no failure.
    ReplayNoFailure,
}

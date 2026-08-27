//! Every public type of the fuzz home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`.

use crate::descriptor::NamespacedName;
use std::time::Duration;

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
    /// A required F0 ceiling was absent from the declared roster.
    MissingRequiredCeiling(NamedCeiling),
    /// A required F0 host disposition was absent from the declared roster.
    MissingRequiredHost(HostDisposition),
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

/// One outcome class the caller's subject reports to the native fuzz mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FuzzExecution {
    /// The subject admitted the bytes lawfully.
    LawfulSuccess,
    /// The subject established a typed refusal.
    TypedRefusal,
    /// The bytes never entered the subject because they were not UTF-8.
    NotUtf8,
    /// The subject or its process crashed.
    Crash,
    /// The subject exceeded its execution-time bound.
    Timeout,
    /// A supervising process established resource exhaustion.
    ResourceExhaustion,
    /// Execution ended without a complete semantic classification.
    AmbiguousPartialAcceptance,
}

/// The loaded module whose relative blocks enter the Frida edge map.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FridaTarget {
    /// Observe the executable that owns the current process.
    MainExecutable,
    /// Observe one explicitly named loaded module.
    NamedModule(FridaModuleName),
}

/// One nonempty loaded-module name used for target-relative Frida observation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FridaModuleName {
    name: String,
}

/// Why a Frida target declaration was refused.
#[must_use = "a refusal is the reason a Frida target was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FridaTargetRefusal {
    /// A named module must have a nonempty name.
    EmptyModuleName,
}

/// One deterministic bounded `LibAFL` plus Frida campaign declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FridaCampaign {
    target: FridaTarget,
    seeds: Vec<Vec<u8>>,
    handoff: FuzzExecution,
    random_seed: u64,
    iterations: u64,
    mutation_iterations: usize,
    timeout: Duration,
}

/// Why a Frida campaign declaration was refused.
#[must_use = "a refusal is the reason a Frida campaign was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FridaCampaignRefusal {
    /// At least one declared seed is required.
    NoSeeds,
    /// The bounded fuzz loop must execute at least once.
    ZeroIterations,
    /// Each mutation stage must execute at least once.
    ZeroMutationIterations,
    /// The executor timeout must be positive.
    ZeroTimeout,
}

/// Why the selected `LibAFL` plus Frida mechanism could not complete a campaign.
#[must_use = "a refusal is the reason the Frida campaign did not complete"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FridaRunRefusal {
    /// The selected feature was invoked on a host outside its native support posture.
    UnsupportedHost,
    /// Frida Stalker is unavailable on this host.
    StalkerUnavailable,
    /// The declared target module was not loaded.
    TargetModuleUnavailable,
    /// Frida event delivery overlapped an edge-trace borrow.
    ObservationBorrowCollision,
    /// A declared seed produced no target-relative edge observation.
    EmptyObservation,
    /// Repeating the same seed changed its edge map.
    UnstableObservation,
    /// `LibAFL` or Frida refused an internal engine operation.
    Engine(String),
    /// Coverage feedback did not grow the corpus beyond its admitted seeds.
    CorpusDidNotGrow,
    /// No evolved corpus entry had the requested handoff classification.
    NoEvolvedHandoff,
    /// The selected evolved entry could not become nonempty interesting bytes.
    InterestingBytes(InterestingBytesRefusal),
}

/// The bounded facts retained from one completed native Frida campaign.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FridaCampaignResult {
    corpus_after_seeds: usize,
    corpus_after_loop: usize,
    nonempty_edge_entries: u64,
    monitor_events: usize,
    execution_counts: [u64; 7],
    interesting: InterestingBytes,
}

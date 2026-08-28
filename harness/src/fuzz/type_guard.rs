//! Smart constructors and readers for the fuzz home.

use super::{
    CoverageAdmission, CoverageAdmissionRefusal, CoverageCorpus, CoverageObservation,
    CoveragePoint, CoverageSource, CoverageSourceRoot, CoverageSourceRootRefusal, FuzzExecution,
    InstrumentedTarget, InterestingBytes, InterestingBytesRefusal, MutationCandidate, MutationKind,
    MutationPlan, MutationPlanRefusal, ReadyPreflight, RustcCoverageTools, RustcProfileRequest,
    RustcProfileRequestRefusal, RustcProfileResult,
};
use crate::descriptor::NamespacedName;
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

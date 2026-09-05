//! The host roads: the tools derived from one sysroot, the instrumented target, the profile request, and the ready preflight that joins them.

use crate::fuzz::types::{
    AbsolutePath, CoverageCampaign, CoverageSourceRoot, CoverageStanding, InstrumentedTarget,
    ReadyPreflight, RustcCoverageTools, RustcProfileRequest, RustcProfileRequestRefusal,
};
use std::path::{Path, PathBuf};

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
        let executable = AbsolutePath::informed(
            executable,
            RustcProfileRequestRefusal::Target,
            RustcProfileRequestRefusal::RelativeTarget,
        )?
        .into_path();
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
        let rustc = AbsolutePath::informed(
            rustc,
            RustcProfileRequestRefusal::Rustc,
            RustcProfileRequestRefusal::RelativeRustc,
        )?
        .into_path();
        let scratch = AbsolutePath::informed(
            scratch,
            RustcProfileRequestRefusal::Scratch,
            RustcProfileRequestRefusal::RelativeScratch,
        )?
        .into_path();
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

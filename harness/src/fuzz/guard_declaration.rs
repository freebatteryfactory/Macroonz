//! The declaration roads: the absolute-path boundary, the source root and identity, the profile, the budgets, the campaign, and the standing preflight joins it to.

use crate::descriptor::NamespacedName;
use crate::fuzz::types::{
    AbsolutePath, CoverageBudgetRefusal, CoverageBudgets, CoverageCampaign, CoverageProfile,
    CoverageSource, CoverageSourceRoot, CoverageSourceRootRefusal, CoverageStanding,
};
use crate::report::{ByteBudget, CaseBudget, TargetBinding};
use std::path::{Component, Path, PathBuf};

impl AbsolutePath {
    pub(super) fn informed<Refusal>(
        path: PathBuf,
        empty: Refusal,
        relative: Refusal,
    ) -> Result<Self, Refusal> {
        if path.as_os_str().is_empty() {
            return Err(empty);
        }
        if !path.is_absolute() {
            return Err(relative);
        }
        Ok(Self(path))
    }

    pub(super) fn into_path(self) -> PathBuf {
        self.0
    }
}

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
        let checkout = AbsolutePath::informed(
            checkout,
            CoverageSourceRootRefusal::EmptyCheckout,
            CoverageSourceRootRefusal::RelativeCheckout,
        )?
        .into_path();
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

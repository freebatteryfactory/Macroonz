//! Assembly constructors and readers.
use super::super::establish::{
    carried_axes, consumption_issues, destination_issues, form_issues, root_issues,
};
use super::{ASSEMBLY_ISSUE_LIMIT, AssemblyError, AssemblyIssue, SupportAssembly};
use crate::bounded::{Capped, Capping, NonEmpty};
use crate::expansion::Expansion;
use crate::identity::{self, ClosedExpansionId, Identity};
use crate::kind::{Destination, Kind};
use crate::support::cargo::{
    AxisCargo, CargoAxis, CargoProofIssue, DeclaredCargo, DeferredCargo, ProvedCargo, SupportAxes,
};
use crate::support::{DeliveryForm, EXPECTED_SCHEMA_ID, SchemaId, SupportName};
use crate::token::GeneratedTree;
impl DeclaredCargo {
    /// Reads a stamped body from the terminal which proved it.
    ///
    /// # Errors
    /// Returns [`AssemblyIssue::CargoNotTheSourcesOwn`] when no stamped body was proved.
    pub fn stamped_from<K: Kind>(
        expansion: &Expansion<K>,
        matched: GeneratedTree,
    ) -> Result<Self, AssemblyError> {
        Self::proved_stamped_from(expansion, matched).map_err(project)
    }
}
impl ProvedCargo {
    /// Promotes one terminal's own delivery into proved cargo.
    ///
    /// # Errors
    /// Returns the established destination or delivery-ownership issue.
    pub fn carried<K: Kind>(
        expansion: &Expansion<K>,
        axis: CargoAxis,
        destination: Destination,
        cargo: DeferredCargo,
    ) -> Result<Self, AssemblyError> {
        Self::proved_carried(expansion, axis, destination, cargo).map_err(project)
    }
}
impl AssemblyError {
    /// Builds a refusal from one issue.
    pub fn of(issue: AssemblyIssue) -> Self {
        Self {
            body: Capped::all(NonEmpty::one(issue)),
        }
    }
    /// Builds a refusal from a complete issue pass.
    pub fn over(first: AssemblyIssue, rest: Vec<AssemblyIssue>) -> Self {
        Self {
            body: Capped::first_n(first, rest.into_iter()),
        }
    }
    /// Reads the primary issue.
    #[must_use]
    pub fn first_issue(&self) -> &AssemblyIssue {
        self.body.items().first()
    }
    /// Reads every carried issue.
    #[must_use]
    pub fn issues(&self) -> &NonEmpty<AssemblyIssue, ASSEMBLY_ISSUE_LIMIT> {
        self.body.items()
    }
    /// Reads the report capping.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.body.capping()
    }
}
impl SupportAssembly {
    /// Checks and assembles one complete axis set.
    ///
    /// # Errors
    /// Returns every independently established issue within the issue bound.
    pub fn assembled(
        root: Identity<identity::CapturedDeclaration>,
        address: Option<SupportName>,
        axes: SupportAxes,
    ) -> Result<Self, AssemblyError> {
        let mut issues = Vec::new();
        {
            let carried = carried_axes(&axes);
            issues.extend(root_issues(root, &carried));
            issues.extend(destination_issues(&carried));
            issues.extend(consumption_issues(&carried));
        }
        issues.extend(form_issues(&axes));
        if let Some(refusal) = refused(issues) {
            return Err(refusal);
        }
        Ok(Self {
            root,
            expectation: EXPECTED_SCHEMA_ID,
            address,
            declared: axes.declared,
            deferred: axes.deferred,
            bench: axes.bench,
        })
    }
    /// Reads the declaration root.
    #[must_use]
    pub const fn root(&self) -> Identity<identity::CapturedDeclaration> {
        self.root
    }
    /// Reads the pinned expectation.
    #[must_use]
    pub const fn expectation(&self) -> SchemaId {
        self.expectation
    }
    /// Reads the public address.
    #[must_use]
    pub const fn address(&self) -> Option<&SupportName> {
        self.address.as_ref()
    }
    /// Reads declaration cargo.
    pub const fn declared(&self) -> &AxisCargo<DeclaredCargo> {
        &self.declared
    }
    /// Reads test cargo.
    pub const fn deferred(&self) -> &AxisCargo<ProvedCargo> {
        &self.deferred
    }
    /// Reads benchmark cargo.
    pub const fn bench(&self) -> &AxisCargo<ProvedCargo> {
        &self.bench
    }
    /// Reads the form from the occupied proved axis.
    #[must_use]
    pub const fn form(&self) -> DeliveryForm {
        match &self.bench {
            AxisCargo::Carried(_) => DeliveryForm::Benches,
            AxisCargo::Absent { .. } => DeliveryForm::Trials,
        }
    }
    /// Iterates proving terminals in axis order.
    pub fn sources(&self) -> impl Iterator<Item = ClosedExpansionId> {
        [source(&self.deferred), source(&self.bench)]
            .into_iter()
            .flatten()
    }
}
fn project(issue: CargoProofIssue) -> AssemblyError {
    AssemblyError::of(match issue {
        CargoProofIssue::DestinationMismatch { axis, destination } => {
            AssemblyIssue::CargoReachesASecondDestination { axis, destination }
        }
        CargoProofIssue::NotSourcesOwn {
            source,
            destination,
        } => AssemblyIssue::CargoNotTheSourcesOwn {
            source,
            destination,
        },
    })
}
fn refused(issues: Vec<AssemblyIssue>) -> Option<AssemblyError> {
    let mut all = issues.into_iter();
    let first = all.next()?;
    Some(AssemblyError::over(first, all.collect()))
}
fn source(axis: &AxisCargo<ProvedCargo>) -> Option<ClosedExpansionId> {
    match axis {
        AxisCargo::Absent { .. } => None,
        AxisCargo::Carried(proved) => Some(proved.source()),
    }
}

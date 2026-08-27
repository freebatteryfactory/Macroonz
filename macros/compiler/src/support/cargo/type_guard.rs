//! Cargo invariant constructors and readers.
use super::{CargoAxis, CargoProofIssue, DeclaredCargo, DeferredCargo, ProvedCargo};
use crate::closure::PartitionCargo;
use crate::expansion::Expansion;
use crate::identity::{self, ClosedExpansionId, Identity};
use crate::kind::{Destination, Kind};
use crate::token::GeneratedTree;
impl DeferredCargo {
    /// Declares opaque cargo.
    pub const fn deferred(tokens: GeneratedTree) -> Self {
        Self { tokens }
    }
    /// Reads its tree.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tokens
    }
}
impl DeclaredCargo {
    /// Binds a stamped body to its matcher.
    pub const fn declared(matched: GeneratedTree, stamped: GeneratedTree) -> Self {
        Self { matched, stamped }
    }
    pub(in crate::support) fn proved_stamped_from<K: Kind>(
        expansion: &Expansion<K>,
        matched: GeneratedTree,
    ) -> Result<Self, CargoProofIssue> {
        let source = expansion.identity();
        let Some(PartitionCargo::Carried(proved)) =
            expansion.emission().joined(Destination::DeclarationSite)
        else {
            return Err(CargoProofIssue::NotSourcesOwn {
                source,
                destination: Destination::DeclarationSite,
            });
        };
        Ok(Self {
            matched,
            stamped: proved.tree().clone(),
        })
    }
    /// Reads matcher clauses.
    #[must_use]
    pub const fn matched(&self) -> &GeneratedTree {
        &self.matched
    }
    /// Reads stamped material.
    #[must_use]
    pub const fn stamped(&self) -> &GeneratedTree {
        &self.stamped
    }
}
impl ProvedCargo {
    pub(in crate::support) fn proved_carried<K: Kind>(
        expansion: &Expansion<K>,
        axis: CargoAxis,
        destination: Destination,
        cargo: DeferredCargo,
    ) -> Result<Self, CargoProofIssue> {
        let source = expansion.identity();
        if axis.reads_from() != Some(destination) {
            return Err(CargoProofIssue::DestinationMismatch { axis, destination });
        }
        let Some(PartitionCargo::Carried(proved)) = expansion.emission().joined(destination) else {
            return Err(CargoProofIssue::NotSourcesOwn {
                source,
                destination,
            });
        };
        if proved.tree() != cargo.tree() {
            return Err(CargoProofIssue::NotSourcesOwn {
                source,
                destination,
            });
        }
        Ok(Self {
            source,
            root: expansion.plan().account().commitment(),
            destination,
            digest: proved.digest(),
            cargo,
        })
    }
    /// Reads the proving terminal.
    #[must_use]
    pub const fn source(&self) -> ClosedExpansionId {
        self.source
    }
    /// Reads the terminal's declaration.
    #[must_use]
    pub const fn root(&self) -> Identity<identity::CapturedDeclaration> {
        self.root
    }
    /// Reads the proved destination.
    #[must_use]
    pub const fn destination(&self) -> Destination {
        self.destination
    }
    /// Reads the delivered-byte digest.
    #[must_use]
    pub const fn digest(&self) -> Identity<identity::OutputBytes> {
        self.digest
    }
    /// Reads the proved cargo.
    pub const fn cargo(&self) -> &DeferredCargo {
        &self.cargo
    }
}

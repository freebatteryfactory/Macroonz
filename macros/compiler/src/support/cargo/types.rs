//! Cargo declarations.
use crate::identity::{self, ClosedExpansionId, Identity};
use crate::kind::{Destination, Disposition};
use crate::token::GeneratedTree;
#[path = "type_guard.rs"]
mod guard;
crate::roster! {
    /// The cargo axes one carrier composes.
    pub enum CargoAxis {
        /// Declaration-grammar material.
        Declared = "declared",
        /// Proved test-target material.
        Deferred = "deferred",
        /// Proved benchmark-target material.
        Bench = "bench",
    }
}
/// The tokens one opaque seat receives before promotion.
#[must_use = "deferred cargo is the token tree one opaque seat receives"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeferredCargo {
    tokens: GeneratedTree,
}
/// One stamped body and the matcher clauses it consumes.
#[must_use = "declared cargo is one stamped body and the clauses its invocation must supply"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredCargo {
    matched: GeneratedTree,
    stamped: GeneratedTree,
}
/// One terminal's proved cargo and parentage.
#[must_use = "proved cargo is one terminal's own tokens and the parentage that reading established"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProvedCargo {
    source: ClosedExpansionId,
    root: Identity<identity::CapturedDeclaration>,
    destination: Destination,
    digest: Identity<identity::OutputBytes>,
    cargo: DeferredCargo,
}
/// What one axis carries, or why it carries nothing.
#[must_use = "an axis either carries its material or states what happened to whatever would have filled it"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AxisCargo<Material> {
    /// Nothing filled this axis.
    Absent {
        /// The disposition which left it empty.
        because: Disposition,
    },
    /// The informed material.
    Carried(Material),
}
/// The complete three-axis input to assembly.
#[must_use = "the axes are what one carrier is composed from, whole"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupportAxes {
    /// Declaration material.
    pub declared: AxisCargo<DeclaredCargo>,
    /// Test-carrier material.
    pub deferred: AxisCargo<ProvedCargo>,
    /// Benchmark-carrier material.
    pub bench: AxisCargo<ProvedCargo>,
}
/// A private promotion finding projected by assembly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::support) enum CargoProofIssue {
    /// The axis and destination disagree.
    DestinationMismatch {
        /// The requested axis.
        axis: CargoAxis,
        /// The requested destination.
        destination: Destination,
    },
    /// The delivery did not prove the supplied cargo.
    NotSourcesOwn {
        /// The terminal.
        source: ClosedExpansionId,
        /// The delivery.
        destination: Destination,
    },
}

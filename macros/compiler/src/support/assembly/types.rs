//! Assembly declarations.
use super::super::cargo::{AxisCargo, CargoAxis, DeclaredCargo, ProvedCargo};
use super::super::types::{DeliveryForm, SchemaId, SupportName};
use crate::bounded::Capped;
use crate::identity::{self, ClosedExpansionId, Identity, OwnerFact};
use crate::kind::Destination;
#[path = "type_guard.rs"]
mod guard;
/// Issues one assembly refusal carries before counting the rest.
pub const ASSEMBLY_ISSUE_LIMIT: usize = 8;
/// The fact this owner declares.
pub const ASSEMBLY_FACT: OwnerFact = OwnerFact {
    home: "support",
    name: "one-carrier-delivers-one-declarations-proved-cargo",
};
/// The verified whole one exported carrier is rendered from.
#[must_use = "an assembly is the verified whole one exported carrier is rendered from"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupportAssembly {
    root: Identity<identity::CapturedDeclaration>,
    expectation: SchemaId,
    address: Option<SupportName>,
    declared: AxisCargo<DeclaredCargo>,
    deferred: AxisCargo<ProvedCargo>,
    bench: AxisCargo<ProvedCargo>,
}
/// One way closed outputs do not compose into one carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssemblyIssue {
    /// A carried terminal stands over another declaration.
    RootsDisagree {
        /// The carried axis.
        axis: CargoAxis,
        /// The stated declaration.
        stated: Identity<identity::CapturedDeclaration>,
        /// The carried declaration.
        carried: Identity<identity::CapturedDeclaration>,
    },
    /// One delivery was consumed twice.
    CargoConsumedTwice {
        /// The terminal.
        source: ClosedExpansionId,
        /// The delivery.
        destination: Destination,
    },
    /// Cargo reached an axis for another destination.
    CargoReachesASecondDestination {
        /// The seated axis.
        axis: CargoAxis,
        /// The proved destination.
        destination: Destination,
    },
    /// The delivery did not prove the supplied cargo.
    CargoNotTheSourcesOwn {
        /// The terminal.
        source: ClosedExpansionId,
        /// The delivery.
        destination: Destination,
    },
    /// Both delivery forms were carried.
    TwoFormsCarried,
    /// A required stamped seat was absent.
    StampedCargoAbsent {
        /// The affected form.
        form: DeliveryForm,
    },
}
/// The complete assembly refusal.
#[must_use = "an assembly refusal carries every way the outputs did not compose"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssemblyError {
    body: Capped<AssemblyIssue, ASSEMBLY_ISSUE_LIMIT>,
}

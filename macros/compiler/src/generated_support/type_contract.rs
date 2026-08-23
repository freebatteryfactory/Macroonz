//! The assembly home's declarative surface: the shape its refusal family
//! declares, and the closed table each axis's delivery is read through.
//!
//! Both are declarations rather than computations. The family states its shape
//! and its selection order as constants; the axis roster states, per row, which
//! emission partition that axis delivers from. Nothing here decides anything:
//! the pass that establishes an assembly issue lives beside them in
//! `establish.rs`, and the roads that build a value live in `type_guard.rs`.

use super::{AssemblyIssue, CargoAxis, CarrierAssembly};
use crate::planning::EmissionPartition;
use macroonz::{FamilyShape, RefusalFamily};

impl RefusalFamily for CarrierAssembly {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
}

impl AssemblyIssue {
    /// The issue kind's position in the declared roster, written ahead of the
    /// issue's own material so two kinds never encode alike.
    ///
    /// A position is APPENDED and never renumbered: renumbering an occupied
    /// position re-encodes values that were already encoded, which renames every
    /// identity derived over them.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::RootsDisagree { .. } => 0,
            Self::SchemaExpectationNotPublished { .. } => 1,
            Self::CargoConsumedTwice { .. } => 2,
            Self::CargoReachesASecondDestination { .. } => 3,
            Self::CargoNotTheSourcesOwn { .. } => 4,
            Self::BenchVehicleNotOpen => 5,
            Self::CarrierRootIsNotTheAssemblys { .. } => 6,
        }
    }

    /// The issue rendered for a person. A projection of the typed value:
    /// nothing reads it back, and no decision consults it.
    ///
    /// Fixed sentences rather than composed ones, because what a reader needs is
    /// the CLASS of disagreement; the axis, the terminal, and the partition that
    /// disagreed are typed seats on the issue itself, and the road that composes
    /// a compiler line reads them from there.
    #[must_use]
    pub const fn described(&self) -> &'static str {
        match self {
            Self::RootsDisagree { .. } => {
                "an axis carries cargo from a terminal planned over another declaration"
            }
            Self::SchemaExpectationNotPublished { .. } => {
                "the carrier's gate would be pinned against an expectation these services do not \
                 publish"
            }
            Self::CargoConsumedTwice { .. } => {
                "two axes read one terminal's one partition, so one proved cargo is delivered twice"
            }
            Self::CargoReachesASecondDestination { .. } => {
                "an axis read a partition other than the one its own delivery names"
            }
            Self::CargoNotTheSourcesOwn { .. } => {
                "the cargo handed for an axis is not the cargo that terminal's partition proved"
            }
            Self::BenchVehicleNotOpen => {
                "the bench axis carries material and the carrier's published grammar writes no \
                 seat for it"
            }
            Self::CarrierRootIsNotTheAssemblys { .. } => {
                "the carrier's own plan stands under a declaration other than the one this \
                 assembly composed"
            }
        }
    }

    /// Which axis this issue is about, where it is about one.
    ///
    /// Answers with nothing for the issues about the assembly as a WHOLE — the
    /// expectation it stands under, the terminal partition two axes read, and
    /// the carrier plan the whole assembly would be rendered under — because
    /// there is no axis to name and electing one would be a stand-in nobody
    /// established.
    #[must_use]
    pub const fn axis(&self) -> Option<CargoAxis> {
        match self {
            Self::RootsDisagree { axis, .. }
            | Self::CargoReachesASecondDestination { axis, .. } => Some(*axis),
            Self::CargoConsumedTwice { .. }
            | Self::CargoNotTheSourcesOwn { .. }
            | Self::SchemaExpectationNotPublished { .. }
            | Self::CarrierRootIsNotTheAssemblys { .. } => None,
            Self::BenchVehicleNotOpen => Some(CargoAxis::Bench),
        }
    }

    /// Append this issue's canonical bytes: the kind's roster slot, then the
    /// typed material that kind carries.
    ///
    /// Exhaustive over the roster on purpose: an issue added to
    /// [`AssemblyIssue`] stops compiling HERE until somebody says what of it a
    /// diagnostic's related identity stands over.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        match self {
            Self::RootsDisagree {
                axis,
                stated,
                carried,
            } => {
                into.push(axis.slot());
                stated.encode_into(into);
                carried.encode_into(into);
            }
            Self::SchemaExpectationNotPublished { stated } => into.extend_from_slice(stated),
            Self::CargoConsumedTwice { source, partition }
            | Self::CargoNotTheSourcesOwn { source, partition } => {
                into.extend_from_slice(source.as_bytes());
                into.push(partition.slot());
            }
            Self::CargoReachesASecondDestination { axis, partition } => {
                into.push(axis.slot());
                into.push(partition.slot());
            }
            Self::BenchVehicleNotOpen => {}
            // Both roots, in the order the issue holds them — the assembly's
            // first, the plan's second — each through the anchoring's own
            // spelling, so a reader of the two knows which is which without the
            // encoding saying so twice.
            Self::CarrierRootIsNotTheAssemblys { stated, planned } => {
                stated.encode_into(into);
                planned.encode_into(into);
            }
        }
    }
}

impl CargoAxis {
    /// The emission partition this axis's cargo is read from, where the axis
    /// carries a terminal's PROVED cargo at all.
    ///
    /// # Authority
    ///
    /// **The mapping is stated once, here, and the reading is what makes "no unit
    /// reaches two destinations" checkable.** A caller names the partition it
    /// read; this table says which one the axis delivers from; a disagreement is
    /// the typed refusal. The declaration-site partition is the case that costs:
    /// its units are already compiled by the consumer's normal build, so
    /// carrying them again into a consumption target is the tax the wall's
    /// delivery vocabulary exists to refuse.
    ///
    /// The TRIAL axis answers with nothing, and the absence is the honest shape
    /// rather than a missing row: that axis carries a DECLARED payload — the
    /// harness's row vocabulary, refused seat by seat at the carrier's own door —
    /// rather than one terminal's proved cargo, so there is no partition for it
    /// to have been read from. An axis whose material is not a proof's has no
    /// proof to name.
    ///
    /// A constant answer over a closed roster, so an axis admitted later stops
    /// the compiler here until somebody says where its cargo comes from.
    #[must_use]
    pub const fn delivers_from(self) -> Option<EmissionPartition> {
        match self {
            Self::Trial => None,
            Self::Evaluation => Some(EmissionPartition::TestCarrier),
            Self::Bench => Some(EmissionPartition::BenchCarrier),
        }
    }
}

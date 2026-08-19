//! The remote-surface home's declarative surface: the tables and trait
//! implementations this home states rather than computes.
//!
//! Four declarations stand here.
//!
//! The LIMIT FAMILY: its capacity authority and its magnitude are written on
//! adjacent rows, so the family cannot be declared on the compile-time ladder
//! while wearing another road's authority — [`Limit::Authority`] resolves to one
//! type, and naming [`DeclaredMagnitude`] there is what makes [`ConstLimit`]
//! implementable at all. The family itself is declared beside the capacity it
//! governs in `types.rs`; what it is FOR is said there, and the number is said
//! here.
//!
//! The REFUSAL FAMILY's declared shape: a single cause with a declared selection
//! order, because every check on this road is dependent on the one before it and
//! exactly one cause is true of any refused composition. The order is written
//! here, in the order the checks establish them.
//!
//! The FACING TABLE: which of the pairing's two roads opens a surface and which
//! closes it, per declared direction. Stated as a constant answer over two closed
//! rosters rather than as a sentence, so "an inbound surface opens by reading the
//! wire" is a value a reader can read back and a match the compiler keeps
//! exhaustive.
//!
//! The PAIRING CONTRACT: what each of the pairing's roads is called with and
//! hands back, one row per road. It is a constant table over a closed roster
//! rather than a sentence in a README, so a reader can read the bill back and the
//! compiler keeps the roster and the bill the same length.

use super::{PairedCodecRoad, RemoteSurfaceIssue, SurfaceContractMint, SurfacePathSegmentLimit};
use crate::planning::SurfaceDirection;
use threadpak::refusal::{FamilyShape, RefusalFamily};
use threadpak::types::{ConstLimit, DeclaredMagnitude, Limit};

impl Limit for SurfacePathSegmentLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for SurfacePathSegmentLimit {
    const MAX: usize = 8;
}

impl RefusalFamily for RemoteSurfaceIssue {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    /// The order the composition road establishes its causes in: a member is
    /// found before its destination is read, a destination before the binding it
    /// stands under, and the binding before anything is rendered.
    const SELECTION_ORDER: &'static [&'static str] = &[
        "RoleNotPlanned",
        "DestinationNotIntegrationTarget",
        "TargetBindingFree",
        "SurfaceTreeUnbounded",
    ];
}

/// Which of a pairing's two roads a facing opens with and which it closes with.
///
/// # Authority
///
/// **A facing is a rendering ORDER and nothing else.** Both directions ride both
/// of the pairing's roads and both call the port's road between them; what an
/// inbound surface and an outbound surface disagree about is which end of the
/// wire they stand at, and that disagreement is exactly which road runs first.
/// Writing it as a table is what keeps the rendering from re-deciding it per
/// call site.
///
/// # Bounds
///
/// There is no seat for how many roads a facing rides, because both ride two, and
/// no seat for the port's road, because both call it in the middle. A table row
/// that carried either would be stating a constant as though it varied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceFacing {
    /// The pairing road the rendered surface calls on the value it was handed.
    pub opens_with: PairedCodecRoad,
    /// The pairing road the rendered surface calls on the port's answer.
    pub closes_with: PairedCodecRoad,
}

/// The facing one declared direction renders under.
///
/// A constant answer over two closed rosters, so a third direction or a third
/// pairing road admitted later stops the compiler here until somebody says which
/// side of this line it stands on.
#[must_use]
pub const fn facing(direction: SurfaceDirection) -> SurfaceFacing {
    match direction {
        SurfaceDirection::Inbound => SurfaceFacing {
            opens_with: PairedCodecRoad::Decode,
            closes_with: PairedCodecRoad::Encode,
        },
        SurfaceDirection::Outbound => SurfaceFacing {
            opens_with: PairedCodecRoad::Encode,
            closes_with: PairedCodecRoad::Decode,
        },
    }
}

/// One pairing road's bill: what the rendered surface calls it with, and what it
/// hands back.
///
/// # Authority
///
/// **The bill is stated and never worked around.** A pairing road the rendering
/// could not call end to end would be a surface whose bytes nobody could re-read,
/// so the rendering does not degrade — it writes the call and the integration
/// target's compiler answers. Where a road is absent or mis-shaped the failure
/// lands at that target as an ordinary unresolved method or type mismatch, which
/// is exactly where a missing road on the caller's own codec belongs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PairingContract {
    /// The pairing road this row is about.
    pub road: PairedCodecRoad,
    /// What the rendered surface hands the road.
    pub called_with: &'static str,
    /// What the road hands back, before the checked conversion.
    pub hands_back: &'static str,
}

/// The complete pairing contract, one row per admitted road, in the roster's own
/// order.
///
/// Two rows and no more, because the roster is two: a row added here without an
/// arm beside it, or an arm added without a row, is a length disagreement the
/// declaration itself carries.
pub const PAIRING_CONTRACT: [PairingContract; 2] = [
    PairingContract {
        road: PairedCodecRoad::Encode,
        called_with: "one value of the owner's own type",
        hands_back: "the wire material for it, or the road's own refusal",
    },
    PairingContract {
        road: PairedCodecRoad::Decode,
        called_with: "the wire material the surface stands over",
        hands_back: "one value of the owner's own type, or the road's own refusal",
    },
];

/// The standing of the mint that would let a caller outside these services bind
/// a host contract.
///
/// # Authority
///
/// **The road is stated as unopened rather than left to be discovered.** A host
/// contract reaches the plane as
/// [`OwnerIdentityRef`](crate::plane::OwnerIdentityRef) over the machine's
/// declaration-target domain, and the plane's only public road to one projects a
/// commitment the MACHINE minted. The machine's identity home carries no public
/// mint for a commitment today, so no caller outside this workspace can hold the
/// value a bound context requires — and a kind whose target requirement is a
/// bound host contract therefore has no outside caller yet.
///
/// This is not a defect in this home and it is not repaired here. A surface
/// rendered against a contract this home invented would be a surface bound to a
/// host nobody declared, which is exactly what
/// [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned) refuses
/// a target-free plan to prevent.
///
/// # Bounds
///
/// It is the same machine fact the host-wrapper home's own standing names, held
/// twice because a shared standing belongs on the plane beside the binding it is
/// about and putting it there is a decision neither home may make alone. The two
/// constants move together or the duplication has become a disagreement.
pub const REMOTE_SURFACE_CONTRACT_MINT: SurfaceContractMint =
    SurfaceContractMint::AwaitingOwnerMint {
        home: "the machine's identity home",
        seat: "a public mint for a domain-tagged commitment over a declaration target",
    };

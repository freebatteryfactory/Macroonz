//! The host-wrapper home's declarative surface: the tables and trait
//! implementations this home states rather than computes.
//!
//! Three declarations stand here.
//!
//! The REFUSAL FAMILY's declared shape: an issue collection, because a plan may
//! select several components nobody staged while a shape stages several nobody
//! selected, and a caller repairing a wrapper one component per attempt is a
//! caller this home failed.
//!
//! The STAGE CONTRACT: the one fact this home adds per wrapper component — the
//! local that component's stage binds its answer under in the rendered wrapper.
//! It is a `const fn` over the plane's own closed roster rather than a second
//! roster written beside it, so the compiler keeps every component answered and
//! no row can be forgotten or left stale.
//!
//! The CONTRACT MINT: whether a caller can be handed the machine's identity for
//! a host contract at all. It is a value rather than a sentence in a README, so
//! the honest answer travels with the vocabulary that needs it.

use super::{WrapperComposition, WrapperContractMint};
use crate::planning::{WRAPPER_COMPONENTS, WrapperComponent};
use threadpak::refusal::{FamilyShape, RefusalFamily};

impl RefusalFamily for WrapperComposition {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// What one wrapper component's stage binds its answer under in the rendered
/// wrapper.
///
/// # Authority
///
/// **This is the whole of what this home says about a component, and the
/// narrowness is the point.** What a component IS — admission, cancellation
/// carriage, receipt emission — is the plane's declaration, and a table here
/// that described one would be a second vocabulary standing beside the owner's.
/// What the plane does NOT say is the local a rendered wrapper binds that
/// component's answer under, because the plane renders nothing; that is this
/// home's fact and it is stated once, here.
///
/// # Bounds
///
/// There is no description seat and no stable-name seat. The plane's
/// [`WrapperComponent`] roster declares neither, so a name written here would be
/// these services legislating a spelling inside a vocabulary the plane owns —
/// and this home needs no such spelling, because a rendered binding is named by
/// this table rather than by the component. A home that later has to write a
/// component's NAME into prose owes the same seat the documentation home's facet
/// fact owes: a declared stable name on the plane's own roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StageContract {
    /// The component this row is about.
    pub component: WrapperComponent,
    /// The local the rendered wrapper binds this stage's answer under.
    pub carried_as: &'static str,
}

/// The stage contract for one component.
///
/// A `const fn` over a closed roster rather than an indexed table: the match is
/// exhaustive, so a component admitted to the plane's roster and not answered
/// here stops the compiler at this function instead of passing as a silently
/// missing row.
#[must_use]
pub const fn stage(component: WrapperComponent) -> StageContract {
    let carried_as = match component {
        WrapperComponent::Admission => "admitted",
        WrapperComponent::Decode => "decoded",
        WrapperComponent::Encode => "encoded",
        WrapperComponent::Cancellation => "cancellable",
        WrapperComponent::Receipt => "receipted",
        WrapperComponent::EffectDispatch => "dispatched",
        WrapperComponent::Observation => "observed",
        WrapperComponent::Explanation => "explained",
    };
    StageContract {
        component,
        carried_as,
    }
}

/// The complete stage contract, one row per component, in the plane's own roster
/// order.
///
/// It is [`stage`] read over
/// [`WRAPPER_COMPONENTS`](crate::planning::WRAPPER_COMPONENTS) rather than a
/// second roster written beside it, so there is no list anybody could forget to
/// add a row to and no length for two declarations to disagree on. It exists for
/// the reader with a question about the contract as a whole rather than about one
/// component.
pub fn stage_contract() -> impl Iterator<Item = StageContract> {
    WRAPPER_COMPONENTS.into_iter().map(stage)
}

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
/// This is not a defect in this home and it is not repaired here. A wrapper
/// rendered against a contract this home invented would be a wrapper bound to a
/// host nobody declared, which is exactly what
/// [`ProjectionPlan::planned`](crate::planning::ProjectionPlan::planned) refuses
/// a target-free plan to prevent.
///
/// # Bounds
///
/// It claims nothing about WHEN the mint lands and nothing about what shape it
/// takes; it names the home that owes it and the seat that closes it, which is
/// the whole of what this side knows. The flip to
/// [`WrapperContractMint::Minted`] is an edit to this one constant and to nothing
/// else in this home.
pub const HOST_WRAPPER_CONTRACT_MINT: WrapperContractMint =
    WrapperContractMint::AwaitingOwnerMint {
        home: "the machine's identity home",
        seat: "a public mint for a domain-tagged commitment over a declaration target",
    };

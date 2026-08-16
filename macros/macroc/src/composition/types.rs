//! The composition home's declarations: the descriptor kinds a provider may
//! compose, the declared providers themselves, how a root fails to be
//! declarable, and the root.
//!
//! Declarations only. The roads that reach a private seat — the root's provider
//! set and the refusal body's one seat — live in `type_guard.rs`, this file's own
//! child, which is what makes the duplicate-free claim structural rather than
//! reviewed.

use crate::plane::{
    DescriptorProviderLimit, DescriptorProviderSubject, OwnerFactRef, OwnerIdentityRef,
};
use threadpak::types::NonEmptyBounded;

#[path = "type_guard.rs"]
mod guard;

/// The closed roster of descriptor kinds a provider may compose.
///
/// Each kind is material derived from declared facts and carried to a consumer
/// that must see all of it at once — which is what makes composition, rather
/// than ambient collection, the only lawful way to gather it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DescriptorKind {
    /// A descriptor challenging one declared obligation.
    TestDescriptor,
    /// A descriptor measuring one declared work formula.
    BenchmarkDescriptor,
    /// A descriptor binding one surface to one host contract.
    HostBindingDescriptor,
    /// One entry of the documentation index.
    DocumentationIndexEntry,
    /// One row of the public-surface inventory.
    ApiInventoryRow,
    /// One entry of the remote-surface listing.
    RemoteSurfaceEntry,
}

/// The declared descriptor-kind roster, in the order this contract states it.
pub const DESCRIPTOR_KINDS: [DescriptorKind; 6] = [
    DescriptorKind::TestDescriptor,
    DescriptorKind::BenchmarkDescriptor,
    DescriptorKind::HostBindingDescriptor,
    DescriptorKind::DocumentationIndexEntry,
    DescriptorKind::ApiInventoryRow,
    DescriptorKind::RemoteSurfaceEntry,
];

/// One declared provider of descriptor material: which provider it is, which
/// owning home its facts come from, and which kind of descriptor it composes.
///
/// The owner-home seat is what keeps this a composition declaration rather than
/// a registry: a provider does not stand on its own authority, it stands on the
/// owner fact it derives from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DescriptorProvider {
    /// The provider's own identity.
    pub provider: OwnerIdentityRef<DescriptorProviderSubject>,
    /// The owning home whose fact this provider derives from.
    pub home: OwnerFactRef,
    /// The kind of descriptor material it composes.
    pub kind: DescriptorKind,
}

/// How a composition root fails to be declarable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositionRootIssue {
    /// One provider identity is declared more than once.
    DuplicateProvider {
        /// The doubled provider.
        provider: OwnerIdentityRef<DescriptorProviderSubject>,
    },
    /// The provider seat outran its declared magnitude.
    SeatBoundExceeded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
}

/// The composition-root declaration refusal family body, published from this
/// file and DECLARED in `type_guard.rs`'s `seat` module, beside the only roads
/// that reach its seat.
pub use guard::CompositionRootDeclaration;

/// The one composition root: every provider that participates, named once.
///
/// Structurally non-empty — a root with no provider is not a composition, it is
/// silence — and duplicate-free by construction, which is this home's end of
/// the bidirectional join stated in the home's README.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompositionRoot {
    providers: NonEmptyBounded<DescriptorProvider, DescriptorProviderLimit>,
}

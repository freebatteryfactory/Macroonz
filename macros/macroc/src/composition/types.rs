//! The composition home's declarations: the descriptor kinds a provider may
//! compose, the declared providers themselves, how a root fails to be
//! declarable, the root, and the two magnitudes this home's capacities are
//! governed by.
//!
//! Declarations only. The roads that reach a private seat — the root's provider
//! set and the refusal body's one seat — live in `type_guard.rs`, this file's own
//! child, which is what makes the duplicate-free claim structural rather than
//! reviewed.

use crate::plane::{DescriptorProviderSubject, OwnerFactRef, OwnerIdentityRef};
use threadpak::types::NonEmptyBounded;

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
//
// This home's own rows, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on every row below
// are this home's, declared beside the capacities they govern.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many descriptor providers one composition
    /// root may declare.
    ///
    /// # Bounds
    ///
    /// Sixty-four. The root is the ONE place a global fact's providers are
    /// declared, so this is how wide the declared surface of a workspace's
    /// composed descriptors may be — and past sixty-four the root has stopped
    /// being a list a reader audits in one sitting, which is the whole reason it
    /// is a declaration rather than a scan.
    DescriptorProviderLimit = 64,
    /// The magnitude governing how many issues one composition-root refusal body
    /// may carry.
    ///
    /// # Bounds
    ///
    /// Sixty-four — at most one issue per declared provider seat, because the
    /// duplicate scan establishes one issue about the provider it found and
    /// never two.
    ///
    /// # Nonclaims
    ///
    /// It is its own family and not [`DescriptorProviderLimit`], even though the
    /// two numbers agree and one is sized off the other's seats. That one bounds
    /// what a root DECLARES; this one bounds what a refused declaration
    /// CARRIES, and one family standing for both would be one authority
    /// answering two questions — the day the scan learns a second question per
    /// seat is the day that would show.
    CompositionIssueLimit = 64,
}

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

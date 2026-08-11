//! The descriptor composition root: the one place that says which providers of
//! descriptor material exist.
//!
//! # Why a hand-authored root is lawful and an unchecked inventory is not
//!
//! Composition carries meaning. Naming, in one declaration, exactly which
//! providers compose is a statement somebody made and can be held to. An
//! unchecked inventory carries none: it is a list that happens to be right
//! today, and nothing refuses when it stops being right. The machine's format
//! law bans hand-maintained inventories for precisely that reason, and this
//! root is not one — it is a composition declaration, and every claim it makes
//! is joined against derived facts.
//!
//! # Local facts generate local products; global facts compose through here
//!
//! A local fact — one obligation, one work formula, one port — generates its
//! own local product beside it. A global fact — the full set of test
//! descriptors, the documentation index, the public-surface inventory —
//! composes through THIS root and nowhere else. There is no ambient
//! registration, no scan of the tree, no link-time collection, and no attribute
//! that quietly enrols a provider: a provider that is not declared here does
//! not participate.
//!
//! # The bidirectional join this root is owed
//!
//! Detecting an omitted provider or one that exists only in the root is a JOIN,
//! and the join belongs to xtask, which already owns the derived-fact side. Its
//! shape is stated here so the obligation is not vague: a provider exists ↔ it
//! appears exactly once in this root ↔ it has a disposition ↔ it has an
//! obligation. Omission fails, phantom fails, duplicate fails. This module owns
//! the duplicate end of that join structurally — [`CompositionRoot::declared`]
//! refuses one — and the omission and phantom ends land with xtask when the
//! providers themselves exist. Sequencing the join is not deferring it: the
//! shape above is the check, written down.

use crate::plane::{
    CompositionIssueLimit, DescriptorProviderLimit, DescriptorProviderSubject, ExactIdentity,
    OwnerFactRef,
};
use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
use threadpak::types::{ConstLimit, NonEmptyBounded, NonEmptyBoundedConstruction};

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
    pub provider: ExactIdentity<DescriptorProviderSubject>,
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
        provider: ExactIdentity<DescriptorProviderSubject>,
    },
    /// The provider seat outran its declared magnitude.
    SeatBoundExceeded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
}

/// The composition-root declaration refusal family body.
///
/// Independent members: several providers may be doubled in one declaration,
/// and reporting one of them would leave a caller repairing the root one
/// provider per attempt.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompositionRootDeclaration {
    /// The established issues — at least one, at most the declared bound.
    pub issues: NonEmptyBounded<CompositionRootIssue, CompositionIssueLimit>,
    /// Whether every declared provider was examined.
    pub posture: CompletionPosture,
}

impl RefusalFamily for CompositionRootDeclaration {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl CompositionRootDeclaration {
    /// The body a declaration check refuses with. When the issues outrun the
    /// declared bound the body keeps the first and reports that examination
    /// stopped there.
    #[must_use]
    fn established(first: CompositionRootIssue, rest: Vec<CompositionRootIssue>) -> Self {
        match NonEmptyBounded::admitted_const(first, rest) {
            Ok(issues) => Self {
                issues,
                posture: CompletionPosture::Complete,
            },
            Err(NonEmptyBoundedConstruction::OverLimit) => Self {
                issues: NonEmptyBounded::singleton(first),
                posture: CompletionPosture::EarlyStopped {
                    stopped_at: StopBound::DeclaredIssueBound,
                },
            },
        }
    }
}

/// The one composition root: every provider that participates, named once.
///
/// Structurally non-empty — a root with no provider is not a composition, it is
/// silence — and duplicate-free by construction, which is this module's end of
/// the bidirectional join stated in the module documentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompositionRoot {
    providers: NonEmptyBounded<DescriptorProvider, DescriptorProviderLimit>,
}

impl CompositionRoot {
    /// Declare the complete provider set.
    ///
    /// # Errors
    ///
    /// Returns [`CompositionRootDeclaration`] naming every provider identity
    /// declared more than once, and the provider seat when the set outgrows its
    /// declared magnitude. Duplicates are refused rather than deduplicated:
    /// silently keeping one of two entries is how a root stops matching the
    /// providers that exist.
    pub fn declared(
        first: DescriptorProvider,
        rest: Vec<DescriptorProvider>,
    ) -> Result<Self, CompositionRootDeclaration> {
        let declared: Vec<DescriptorProvider> = core::iter::once(first)
            .chain(rest.iter().copied())
            .collect();
        let mut issues: Vec<CompositionRootIssue> = Vec::new();
        for (position, provider) in declared.iter().enumerate() {
            let earlier = declared
                .iter()
                .take(position)
                .any(|other| other.provider == provider.provider);
            let repeated = declared
                .iter()
                .skip(position.saturating_add(1))
                .any(|other| other.provider == provider.provider);
            if repeated && !earlier {
                issues.push(CompositionRootIssue::DuplicateProvider {
                    provider: provider.provider,
                });
            }
        }
        let mut established = issues.into_iter();
        if let Some(issue) = established.next() {
            return Err(CompositionRootDeclaration::established(
                issue,
                established.collect(),
            ));
        }
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(first, rest)
            .map(|providers| Self { providers })
            .map_err(|_| {
                CompositionRootDeclaration::established(
                    CompositionRootIssue::SeatBoundExceeded {
                        bound: u64::try_from(DescriptorProviderLimit::MAX).unwrap_or(u64::MAX),
                        observed: u64::try_from(observed).unwrap_or(u64::MAX),
                    },
                    Vec::new(),
                )
            })
    }

    /// The guaranteed first declared provider.
    #[must_use]
    pub fn first(&self) -> &DescriptorProvider {
        self.providers.first()
    }

    /// Read the declared providers.
    ///
    /// The order law applies: the provider set is keyed by provider identity,
    /// so nothing identity-bearing is derived from the order this yields — the
    /// join xtask owes matches by identity, never by position.
    pub fn iter(&self) -> impl Iterator<Item = &DescriptorProvider> {
        self.providers.iter()
    }

    /// The number of providers declared; structurally at least one.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Always `false`: a root with no provider is unrepresentable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}

#[cfg(test)]
mod laws {
    use super::{
        CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DESCRIPTOR_KINDS,
        DescriptorKind, DescriptorProvider,
    };
    use crate::plane::{ExactIdentity, OwnerFactRef};
    use threadpak::refusal::{FamilyShape, RefusalFamily};

    /// The closed descriptor-kind roster, proven closed by an exhaustive match:
    /// a new kind stops compiling here until it is placed.
    const fn kind_index(kind: DescriptorKind) -> usize {
        match kind {
            DescriptorKind::TestDescriptor => 0,
            DescriptorKind::BenchmarkDescriptor => 1,
            DescriptorKind::HostBindingDescriptor => 2,
            DescriptorKind::DocumentationIndexEntry => 3,
            DescriptorKind::ApiInventoryRow => 4,
            DescriptorKind::RemoteSurfaceEntry => 5,
        }
    }

    /// One owner fact, for laws that need a home citation.
    fn owner_fact(tag: u8) -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([tag; 32]),
            fact: ExactIdentity::decoded([tag.saturating_add(1); 32]),
        }
    }

    /// One provider of the named kind under the identity byte named.
    fn provider(kind: DescriptorKind, tag: u8) -> DescriptorProvider {
        DescriptorProvider {
            provider: ExactIdentity::decoded([tag; 32]),
            home: owner_fact(tag.saturating_add(50)),
            kind,
        }
    }

    /// law: composition.descriptor-kinds-are-six-and-closed — the kinds a
    /// provider may compose are a closed roster whose members are pairwise
    /// distinct and declared in one order.
    /// Owed reversal: adding a kind without placing it must break this law.
    #[test]
    fn descriptor_kinds_are_six_and_closed() {
        assert_eq!(DESCRIPTOR_KINDS.len(), 6);
        let indexes: Vec<usize> = DESCRIPTOR_KINDS.iter().copied().map(kind_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: composition.a-provider-names-its-home-and-its-kind — a declared
    /// provider carries the owning home its facts come from and the kind it
    /// composes, and the root hands both back on a read-only pass.
    /// Owed reversal: a provider standing on its own authority must break this
    /// law.
    #[test]
    fn a_provider_names_its_home_and_its_kind() {
        let root = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 1),
            vec![provider(DescriptorKind::ApiInventoryRow, 2)],
        );
        assert!(root.is_ok_and(|root| {
            let kinds: Vec<DescriptorKind> = root.iter().map(|held| held.kind).collect();
            let homes: Vec<OwnerFactRef> = root.iter().map(|held| held.home).collect();
            kinds
                == vec![
                    DescriptorKind::TestDescriptor,
                    DescriptorKind::ApiInventoryRow,
                ]
                && homes == vec![owner_fact(51), owner_fact(52)]
                && root.len() == 2
                && !root.is_empty()
                && root.first().provider == ExactIdentity::decoded([1; 32])
        }));
    }

    /// law: composition.a-root-refuses-a-duplicate-provider — one provider
    /// identity declared twice refuses naming that provider, and a root past
    /// its declared magnitude refuses naming the seat. Neither is deduplicated
    /// and neither is trimmed.
    /// Owed reversal (red twin): a root that silently kept one of two entries
    /// must break this law.
    #[test]
    fn a_root_refuses_a_duplicate_provider() {
        let doubled = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 3),
            vec![provider(DescriptorKind::BenchmarkDescriptor, 3)],
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            CompositionRootIssue::DuplicateProvider { .. }
        )));

        let overrun: Vec<DescriptorProvider> = (0..70_u8)
            .map(|tag| {
                provider(
                    DescriptorKind::DocumentationIndexEntry,
                    tag.saturating_add(100),
                )
            })
            .collect();
        let refused =
            CompositionRoot::declared(provider(DescriptorKind::RemoteSurfaceEntry, 4), overrun);
        assert!(refused.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            CompositionRootIssue::SeatBoundExceeded {
                bound: 64,
                observed: 71
            }
        )));
    }

    /// law: composition.the-root-family-is-an-issue-collection — the
    /// declaration family declares the collection shape, elects no primary
    /// issue, and reports every doubled provider at once.
    /// Owed reversal (red twin): declaring `SingleCause` with a collection body
    /// must break this law.
    #[test]
    fn the_root_family_is_an_issue_collection() {
        assert!(matches!(
            CompositionRootDeclaration::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(CompositionRootDeclaration::SELECTION_ORDER.is_empty());

        let refused = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 5),
            vec![
                provider(DescriptorKind::TestDescriptor, 5),
                provider(DescriptorKind::ApiInventoryRow, 6),
                provider(DescriptorKind::ApiInventoryRow, 6),
            ],
        );
        assert!(refused.is_err_and(|refusal| refusal.issues.len() == 2));
    }
}

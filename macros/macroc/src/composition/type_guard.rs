//! The composition home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! end of the bidirectional join structural. A root is built HERE, after the
//! duplicate scan agreed, so a root carrying one provider identity twice is a
//! value nobody can hold rather than a state a later check has to notice.

use super::super::establish::{duplicate_issues, refused};
use super::{
    CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DescriptorProvider,
};
use crate::plane::{AuthoringLimitProfile, DescriptorProviderLimit};
use threadpak::types::{ConstLimit, NonEmptyBounded, PositiveLimit};

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
        if let Some(refusal) = refused(duplicate_issues(&declared)) {
            return Err(refusal);
        }
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
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

//! The composition home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! end of the bidirectional join structural. A root is built HERE, after the
//! duplicate scan agreed, so a root carrying one provider identity twice is a
//! value nobody can hold rather than a state a later check has to notice. The
//! refusal BODY is built here for the same reason and by the same permission:
//! its seat is private, so this file is the only module in the workspace that
//! can spell the literal, and every refusal that exists came off the scan.
//!
//! # What a private seat does and does not exclude
//!
//! It excludes every SIBLING: `establish.rs` beside it, anywhere else in the
//! services, and any crate downstream cannot write the literal, and the compiler
//! says so with `E0451`. It does not exclude DESCENDANTS — a module declared
//! inside this one would construct as freely as these roads do, so a
//! `#[cfg(test)] mod` under the guard would reopen exactly what the guard closes,
//! and the reversals for this seat are testpak's compile-fail fixtures instead.

use super::super::establish::duplicate_issues;
use super::{
    CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DescriptorProvider,
};
use crate::plane::{AuthoringLimitProfile, CompositionIssueLimit, DescriptorProviderLimit};
use threadpak::refusal::{AdmittedPrefix, StopBound};
use threadpak::types::{ConstLimit, NonEmptyBounded, PositiveLimit};

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
fn refused(issues: Vec<CompositionRootIssue>) -> Option<CompositionRootDeclaration> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(CompositionRootDeclaration::established(
        first,
        established.collect(),
    ))
}

impl CompositionRootDeclaration {
    /// The body a declaration check refuses with.
    ///
    /// The duplicate scan runs the declared set to the end before a body exists,
    /// so the posture here is never about the scan: it is about the REPORT. Where
    /// every established issue fits the declared bound the body carries all of
    /// them and says `Complete`; where it does not, the body carries what the
    /// bound holds and names how many established issues stand outside it. A
    /// posture claiming the examination stopped would say nobody looked past the
    /// bound, and somebody did.
    fn established(first: CompositionRootIssue, rest: Vec<CompositionRootIssue>) -> Self {
        Self {
            body: AdmittedPrefix::examined_completely(
                first,
                rest,
                &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                StopBound::DeclaredIssueBound,
            ),
        }
    }

    /// The established issues and what this refusal says about its own coverage
    /// of them.
    ///
    /// Borrowed and never owned, for the reason band 00 borrows its carry: an
    /// owned body is a value a caller can seat under another refusal, which is
    /// the pairing the coupled seat exists to end.
    pub const fn body(&self) -> &AdmittedPrefix<CompositionRootIssue, CompositionIssueLimit> {
        &self.body
    }
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

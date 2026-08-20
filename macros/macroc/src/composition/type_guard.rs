//! The composition home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! end of the bidirectional join structural. A root is built HERE, after the
//! duplicate scan agreed, so a root carrying one provider identity twice is a
//! value nobody can hold rather than a state a later check has to notice. The
//! refusal BODY is built here for the same reason, so every refusal that exists
//! came off that scan.
//!
//! The body is DECLARED in the `seat` module below rather than in `types.rs`,
//! because Rust's privacy is MODULE-scoped and a seat declared beside the rest
//! of this home's declarations would put all of them inside the wall. That
//! module's entire content is the record and its inherent implementations, so
//! the module IS the complete set of roads that reach the private seat.
//!
//! A private seat excludes every SIBLING: the rest of this file, `types.rs`
//! above it, `establish.rs` beside it, anywhere else in the services, and any
//! crate downstream cannot write the literal, and the compiler says so with
//! `E0451`. It does not exclude DESCENDANTS, so the reversal for this seat is
//! testpak's compile-fail fixture.

use super::super::establish::duplicate_issues;
use super::{CompositionRoot, CompositionRootIssue, DescriptorProvider, DescriptorProviderLimit};
use crate::plane::AuthoringLimitProfile;
use threadpak::types::{ConstLimit, NonEmptyBounded, PositiveLimit};

pub use seat::CompositionRootDeclaration;

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

mod seat {
    use super::super::{CompositionIssueLimit, CompositionRootIssue};
    use crate::plane::AuthoringLimitProfile;
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The composition-root declaration refusal family body.
    ///
    /// Independent members: several providers may be doubled in one declaration,
    /// and reporting one of them would leave a caller repairing the root one
    /// provider per attempt.
    #[must_use = "a refusal family body carries every established issue with the root"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct CompositionRootDeclaration {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue the scan
        /// established or names how many stand outside that bound. One seat
        /// rather than two, because a coverage claim seated beside its body is a
        /// claim that can be swapped for another body's.
        ///
        /// Private for the same reason: a PUBLIC seat on a one-field record
        /// hands the whole record back as a literal, so any holder of a body
        /// built for one scan could write it into another scan's refusal. Read
        /// back through [`CompositionRootDeclaration::body`].
        body: AdmittedPrefix<CompositionRootIssue, CompositionIssueLimit>,
    }

    impl CompositionRootDeclaration {
        /// The body a declaration check refuses with.
        ///
        /// The duplicate scan runs the declared set to the end before a body
        /// exists, so the posture here is never about the scan: it is about the
        /// REPORT. Where every established issue fits the declared bound the
        /// body carries all of them and says `Complete`; where it does not, the
        /// body carries what the bound holds and names how many established
        /// issues stand outside it. A posture claiming the examination stopped
        /// would say nobody looked past the bound, and somebody did.
        ///
        /// Reaches the guard file and no further, so a body exists only where
        /// the scan beside it ran.
        pub(super) fn established(
            first: CompositionRootIssue,
            rest: Vec<CompositionRootIssue>,
        ) -> Self {
            Self {
                body: AdmittedPrefix::examined_completely(
                    first,
                    rest,
                    &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                    StopBound::DeclaredIssueBound,
                ),
            }
        }

        /// The established issues and what this refusal says about its own
        /// coverage of them.
        ///
        /// Borrowed and never owned, for the reason band 00 borrows its carry:
        /// an owned body is a value a caller can seat under another refusal,
        /// which is the pairing the coupled seat exists to end.
        pub const fn body(&self) -> &AdmittedPrefix<CompositionRootIssue, CompositionIssueLimit> {
            &self.body
        }
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
    /// join, when it lands, matches by identity, never by position.
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

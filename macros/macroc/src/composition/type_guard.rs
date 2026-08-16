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
//! # Why the body is DECLARED here and not in `types.rs`
//!
//! Rust's privacy is MODULE-scoped, so a seat declared in `types.rs` puts every
//! other item in that file inside the wall and leaves "did anybody write a road
//! out?" as a whole-file audit. The body is therefore declared in the `seat`
//! module below, whose entire content is that record and inherent
//! implementations of it and nothing else — the module is the complete set of
//! roads that can reach the private seat.
//!
//! # What a private seat does and does not exclude
//!
//! It excludes every SIBLING: the rest of this file, `types.rs` above it,
//! `establish.rs` beside it, anywhere else in the services, and any crate
//! downstream cannot write the literal, and the compiler says so with `E0451`.
//! It does not exclude DESCENDANTS — a module declared inside the seat would
//! construct as freely as these roads do, which is why the reversals for this
//! seat are testpak's compile-fail fixtures and why the law above refuses a
//! nested module in a `seat` module outright.

use super::super::establish::duplicate_issues;
use super::{CompositionRoot, CompositionRootIssue, DescriptorProvider};
use crate::plane::{AuthoringLimitProfile, DescriptorProviderLimit};
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
    use super::super::CompositionRootIssue;
    use crate::plane::{AuthoringLimitProfile, CompositionIssueLimit};
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
        /// claim that can be swapped for another body's. The scan itself always
        /// covers every declared provider, so the completion here never reports
        /// a halted examination.
        ///
        /// Private, and that is the second half of the same claim. The coupled
        /// seat keeps a carry and its posture together; a PUBLIC seat on a
        /// one-field record hands the whole record back as a literal, so any
        /// holder of a body built for one scan could write it into another
        /// scan's refusal. Read back through [`CompositionRootDeclaration::body`].
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
        /// Reaches the guard file and no further — `pub(super)` from inside the
        /// seat is exactly the module-private reach this road had before the
        /// declaration moved, and the passes that raise it are the ones beside
        /// it.
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

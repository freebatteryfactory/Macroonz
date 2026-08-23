//! Invariant-preserving constructors and readers for private contract seats.

use super::{AdmittedPrefix, CompletionPosture, ReportTruncation, StopBound};
use crate::{
    ConstLimit, Limit, LimitAdmissionProfile, NonEmptyBounded, NonEmptyBoundedConstruction,
    PositiveLimit,
};
use core::num::NonZeroUsize;

impl ReportTruncation {
    /// Returns the declared bound that the report reached.
    #[must_use]
    pub const fn stopped_at(self) -> StopBound {
        self.stopped_at
    }

    /// Returns the non-zero count of established issues omitted by the report.
    #[must_use]
    pub const fn omitted(self) -> NonZeroUsize {
        self.omitted
    }
}

impl<T, L: ConstLimit> AdmittedPrefix<T, L> {
    /// Constructs the report produced by a complete examination.
    ///
    /// The constructor performs the bounded prefix operation and derives the completion posture from the exact omitted count.
    pub fn examined_completely<P: LimitAdmissionProfile>(
        first: T,
        rest: Vec<T>,
        admitted: &PositiveLimit<L, P>,
        at: StopBound,
    ) -> Self {
        let (carried, omitted) = NonEmptyBounded::admitted_prefix(first, rest, admitted);
        let completion = match NonZeroUsize::new(omitted) {
            None => CompletionPosture::Complete,
            Some(omitted) => CompletionPosture::ReportTruncated(ReportTruncation {
                stopped_at: at,
                omitted,
            }),
        };
        Self {
            carried,
            completion,
        }
    }

    /// Constructs the report produced by an examination that stopped at a declared bound.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyBoundedConstruction::OverLimit`] when the supplied material exceeds the admitted maximum and would otherwise be dropped silently.
    pub fn stopped_early<P: LimitAdmissionProfile>(
        first: T,
        rest: Vec<T>,
        admitted: &PositiveLimit<L, P>,
        stopped_at: StopBound,
    ) -> Result<Self, NonEmptyBoundedConstruction> {
        NonEmptyBounded::admitted_const(first, rest, admitted).map(|carried| Self {
            carried,
            completion: CompletionPosture::EarlyStopped { stopped_at },
        })
    }

    /// Constructs the complete report produced by a seam that can establish exactly one issue.
    pub fn carrying_one(item: T) -> Self {
        Self {
            carried: NonEmptyBounded::singleton(item),
            completion: CompletionPosture::Complete,
        }
    }
}

impl<T, L: Limit> AdmittedPrefix<T, L> {
    /// Returns the established issues, structurally non-empty and bounded.
    #[must_use]
    pub const fn carried(&self) -> &NonEmptyBounded<T, L> {
        &self.carried
    }

    /// Returns the coverage posture produced with the carried issues.
    #[must_use]
    pub const fn completion(&self) -> CompletionPosture {
        self.completion
    }
}

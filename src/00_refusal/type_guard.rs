//! The refusal home's invariant nucleus: every road that reaches a report
//! package's private seats.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! central claim structural. A [`ReportTruncation`] is minted HERE and nowhere
//! else, by the road that performs the truncation it records, so the count a
//! posture writes down is the count that road just dropped. An
//! [`AdmittedPrefix`] is built HERE, so a coverage claim and the material it is
//! about leave the road as one value and there is no seam anywhere in the crate
//! that can assemble them apart. The readers travel with the mints because they
//! are the same private seats read back: a borrow of the carry and a copy of the
//! posture are what the marriage is allowed to hand out, and both are decided
//! beside the invariant rather than away from it.

use super::{AdmittedPrefix, CompletionPosture, ReportTruncation, StopBound};
use crate::types::{
    ConstLimit, Limit, LimitAdmissionProfile, NonEmptyBounded, NonEmptyBoundedConstruction,
    PositiveLimit,
};
use core::num::NonZeroUsize;

impl ReportTruncation {
    /// The declared bound the report stopped at.
    #[must_use]
    pub const fn stopped_at(self) -> StopBound {
        self.stopped_at
    }

    /// How many established issues the body does not carry; at least one, by
    /// shape — a truncation that omitted nothing is [`CompletionPosture::Complete`]
    /// and is unrepresentable here.
    #[must_use]
    pub const fn omitted(self) -> NonZeroUsize {
        self.omitted
    }
}

impl<T, L: ConstLimit> AdmittedPrefix<T, L> {
    /// The report a COMPLETE examination amounts to: the prefix the family's
    /// admitted magnitude holds, and the posture the same truncation selects.
    ///
    /// This is the one mint for [`ReportTruncation`], and the ACT is what
    /// selects the posture rather than the caller: material that fits is
    /// `Complete`, and material that does not is a truncation naming the bound
    /// and the exact count it just dropped. A pass that carried every issue
    /// therefore cannot claim it truncated, and a pass that dropped issues
    /// cannot claim completeness — neither is a discipline a site has to
    /// remember, because neither is a value a site can build.
    ///
    /// The road takes the material rather than a count, and that is the
    /// difference between a posture that MATCHES a body and one that merely
    /// describes it. A count is a value anybody can choose, so a road taking one
    /// would let a body carrying every issue it established mint a posture saying
    /// seven were dropped, and the type would be recording a caller's assertion
    /// rather than the construction's own act.
    ///
    /// The bound is named HERE, at the one call that also supplies the limit
    /// family, because which declared bound a magnitude stands for is the
    /// caller's own statement about the family it just handed over. Naming it
    /// later would make the posture a re-derivation, and two reads of one body
    /// could then name different bounds.
    ///
    /// It is total for [`NonEmptyBounded::singleton`]'s reason: the witness is
    /// [`PositiveLimit`], so the maximum is at least one, so the prefix is never
    /// empty and there is no failing case to return.
    ///
    /// A pass whose examination genuinely halted does not come here. Its road
    /// is [`AdmittedPrefix::stopped_early`], because the fact it reports is
    /// about the scan rather than about the report, and no truncation this road
    /// performs could witness it.
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

    /// The report a HALTED examination amounts to: the issues the halted pass
    /// handed over, married to the posture naming the declared bound it stopped
    /// at, in one construction.
    ///
    /// `EarlyStopped` is a claim about the EXAMINATION and not about the report,
    /// which is why it is a road of its own rather than an outcome
    /// [`AdmittedPrefix::examined_completely`] could select. A truncation is
    /// something a construction performs, so a construction can witness it; a
    /// halt happened before any construction was reached, and no arrangement of
    /// seats observes it.
    ///
    /// # The honesty ceiling
    ///
    /// This constructor structurally couples the body to the posture. It does
    /// not prove that an external examination truly halted — the family owner's
    /// algorithm establishes the behavioral claim, and its proof lives outside
    /// this crate.
    ///
    /// # It refuses where the other road truncates
    ///
    /// [`AdmittedPrefix::examined_completely`] is total because a truncation has
    /// a seat to record itself in. `EarlyStopped` has no such seat: it names the
    /// bound and nothing else, so material past the declared bound could only be
    /// dropped silently here — the one defect this package exists to make
    /// unwritable. A pass that stopped BECAUSE of a bound has by definition
    /// nothing past it, so handing over more than the admitted magnitude holds
    /// is a caller contradicting its own posture, and the road answers with the
    /// same typed cause every checked non-empty construction answers with.
    ///
    /// No caller exists today, because no scan in the machine halts. The first
    /// one arrives with the first honestly halting examination, and it arrives
    /// at the coupled seat rather than at a pair of loose values.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyBoundedConstruction::OverLimit`] when the handed
    /// material exceeds what the admitted magnitude holds.
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

    /// The report a seam that can establish exactly one issue amounts to.
    ///
    /// Total, and `Complete` by shape rather than by assertion: the road is
    /// handed one item and carries one item, so there is no material it could
    /// have dropped and no bound it could name. A one-issue seam therefore
    /// reaches the same package every other seam does, and never assembles a
    /// posture beside a body of its own.
    pub fn carrying_one(item: T) -> Self {
        Self {
            carried: NonEmptyBounded::singleton(item),
            completion: CompletionPosture::Complete,
        }
    }
}

impl<T, L: Limit> AdmittedPrefix<T, L> {
    /// The issues the body carries — at least one, at most the declared bound.
    ///
    /// Borrowed and never owned. An owned carry is half of the pair this type
    /// exists to keep together, and a caller holding it could seat it under
    /// another report's posture.
    #[must_use]
    pub const fn carried(&self) -> &NonEmptyBounded<T, L> {
        &self.carried
    }

    /// What this body says about its own coverage.
    #[must_use]
    pub const fn completion(&self) -> CompletionPosture {
        self.completion
    }
}

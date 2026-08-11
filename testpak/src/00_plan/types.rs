//! The plan home's public types: the denominators a verdict is stated over.
//!
//! Nothing here judges anything. A denominator is the honest half of every
//! claim the plane makes — "three of the twelve reversals are written" is a
//! statement; "the reversals pass" is not one — and it has to be a typed value
//! rather than a number in a sentence, because a number in a sentence can shrink
//! without anyone noticing.

/// The expected-versus-executed accounting for one population of red twins.
///
/// A green law names the reversal that would break it. That reversal is EXPECTED
/// from the moment the law is written and DISCHARGED only when the reversal
/// exists and runs. The gap between the two is the debt, and this record is
/// where the plane carries it instead of narrating it.
///
/// The fields are private and the only road forward is
/// [`RedTwinLedger::discharge`], which refuses past the denominator. That is the
/// whole point of the type: a ledger reporting more discharged than were ever
/// expected is not an error to detect afterwards, it is a value that cannot be
/// built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedTwinLedger {
    /// The red twins the green laws name. The denominator.
    expected: usize,
    /// Those of them that exist and run. Never more than `expected`.
    discharged: usize,
}

impl RedTwinLedger {
    /// Open a ledger over a declared denominator, with nothing yet discharged.
    #[must_use]
    pub const fn opened(expected: usize) -> Self {
        Self {
            expected,
            discharged: 0,
        }
    }

    /// The declared denominator.
    #[must_use]
    pub const fn expected(&self) -> usize {
        self.expected
    }

    /// How many are discharged.
    #[must_use]
    pub const fn discharged(&self) -> usize {
        self.discharged
    }

    /// How many are still owed.
    #[must_use]
    pub const fn outstanding(&self) -> usize {
        self.expected.saturating_sub(self.discharged)
    }

    /// Record one discharged red twin.
    ///
    /// Returns `None` where the ledger is already settled. Discharging past the
    /// denominator is not a large number to clamp — it means the denominator
    /// was wrong, and a ledger that quietly absorbed the overrun would be
    /// reporting a fiction in exactly the direction that flatters.
    #[must_use]
    pub const fn discharge(self) -> Option<Self> {
        if self.discharged >= self.expected {
            return None;
        }
        Some(Self {
            expected: self.expected,
            discharged: self.discharged.saturating_add(1),
        })
    }
}

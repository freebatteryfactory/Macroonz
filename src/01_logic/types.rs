//! Three-valued logic in general: the canonical truth values, their K3 (strong
//! Kleene) connectives, and the decision algebra that lives beside them.
//!
//! There is exactly one three-valued truth in the machine — no second
//! `True`/`False` enum exists anywhere, and `bool` is never a result axis. `Truth`
//! is one of the three knowledge axes permitted to say "not yet", which is why its
//! third value is spelled `Pending`. `OutcomeUnknown` is a different axis
//! (runtime's outcome knowledge), never a fourth truth value.
//!
//! `Decision` is never `Truth` wearing different names: no conversion between them
//! exists in either direction, and a decided value carries no authority — policy
//! maps evidence to a decision; the decision does not carry the policy.

/// The canonical K3 truth value. Produced first by interval comparison (the
/// numeric home), consumed by evidence, gates, and every place a question's
/// answer can honestly lag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Truth {
    /// Established true.
    True,
    /// Established false.
    False,
    /// Not yet establishable from what is admitted. A knowledge-axis value —
    /// lawful here and nowhere else.
    Pending,
}

impl Truth {
    /// K3 conjunction: `False` dominates, `True` is neutral, `Pending`
    /// propagates otherwise. A lagging answer can never hide a known failure:
    /// `Pending.and(False)` is `False`.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, Self::True) => Self::True,
            _ => Self::Pending,
        }
    }

    /// K3 disjunction: `True` dominates, `False` is neutral, `Pending`
    /// propagates otherwise.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, Self::False) => Self::False,
            _ => Self::Pending,
        }
    }

    /// K3 negation: swaps the established values; `Pending` stays `Pending`.
    #[must_use]
    pub const fn negate(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Pending => Self::Pending,
        }
    }
}

/// The normalized decision algebra, beside `Truth` and never convertible to or
/// from it. `Defer` means the gate declines to decide now — it is not `Pending`,
/// and it is not a refusal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Decision {
    /// The gate admits the subject.
    Allow,
    /// The gate denies the subject.
    Deny,
    /// The gate declines to decide now; deciding later is lawful.
    Defer,
}

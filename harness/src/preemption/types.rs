//! Every public type of the preemption home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`; target-specific execution lives behind the one explore road.

use crate::report::{FindingCause, ForeignText};

#[path = "type_guard.rs"]
mod guard;

/// The owner every cause this home cites is declared under.
const CAUSE_FAMILY: &str = "macroonz.preemption";

/// The cause an explicit model check refused under.
///
/// This cause is minted only from [`PreemptionModelFailure`], never inferred from a foreign backend unwind.
pub const MODEL_BROKE: FindingCause = FindingCause::named(CAUSE_FAMILY, "model-broke");

/// The exact loom requirement the workspace manifest declares.
///
/// A mirror of the manifest's `=`-pin, held here so evidence can spell which scheduler semantics a reading ran under; the preemption lane holds the two spellings together.
pub const LOOM_PIN: &str = "0.7.2";

/// How many preemptions one explored execution may spend.
///
/// Loom's search is exhaustive under this bound: two or three preemptions catch most real memory-model bugs, and `Exhaustive` removes the bound where the model is small enough to walk whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreemptionBound {
    /// No bound: every reachable interleaving, however many preemptions it takes.
    Exhaustive,
    /// At most this many preemptions per execution.
    AtMost(u32),
}

/// The declared budget one exploration runs under.
///
/// Both seats are the author's statement, never runner-tuned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PreemptionBounds {
    preemptions: PreemptionBound,
    branches: u32,
}

/// Why one bounds declaration was refused.
#[must_use = "a refusal is the reason preemption bounds were not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreemptionBoundsRefusal {
    /// The branch budget admits no branch, so no execution could take a single step.
    ZeroBranches,
}

/// One model-owned refusal returned as a value from a scheduled execution.
///
/// Its private report seat keeps the exact model-break mint on the typed return road; panicking with lookalike text cannot construct it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreemptionModelFailure {
    report: Option<ForeignText>,
}

/// The result one model returns on each scheduled execution.
pub type PreemptionModelResult = Result<(), PreemptionModelFailure>;

/// What one completed bounded exploration established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreemptionVerdict {
    /// Every execution under the declared bounds completed with every model check standing.
    AllInterleavingsHeld,
    /// One execution returned an explicit model-owned refusal.
    ModelBroke {
        /// The model's bounded report, where it supplied one.
        report: Option<ForeignText>,
    },
}

/// Why one requested exploration did not establish a model verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncompleteExploration {
    /// The pinned backend has no implementation for this compilation target.
    Unavailable,
    /// The backend refused before its explicitly configured exploration began.
    InitializationFailed {
        /// The backend's bounded report, where its payload was text.
        report: Option<ForeignText>,
    },
    /// The backend unwound after exploration began without a Macroonz-minted model refusal.
    ///
    /// Loom 0.7.2 does not type whether this was declared branch exhaustion, cleanup failure, an undeclared model panic, or a backend defect; the report remains foreign and is never parsed into invented authority.
    ExecutionUnresolved {
        /// The backend's bounded report, where its payload was text.
        report: Option<ForeignText>,
    },
}

/// Whether one exploration completed with a model verdict or stopped on the infrastructure rail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreemptionOutcome {
    /// The backend established a verdict about the bounded model space.
    Completed(PreemptionVerdict),
    /// The backend did not establish a model verdict.
    Incomplete(IncompleteExploration),
}

/// What one exploration produced: the bounds it was asked to run under and the strongest outcome the backend established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreemptionReading {
    bounds: PreemptionBounds,
    outcome: PreemptionOutcome,
}

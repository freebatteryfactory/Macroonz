//! Every public type of the preemption home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`; the one road that runs loom lives in `explore.rs`.

use crate::report::{FindingCause, ForeignText};

#[path = "type_guard.rs"]
mod guard;

/// The owner every cause this home cites is declared under.
const CAUSE_FAMILY: &str = "macroonz.preemption";

/// The cause a model that did not complete cleanly is cited under.
///
/// One cause for the whole broke-arm, because the boundary cannot type loom's report without parsing it — an assertion, a deadlock, and an overrun bound all conclude here, and loom's own words ride the finding as foreign text.
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

/// What one bounded exploration established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreemptionVerdict {
    /// Every execution under the declared bounds completed with every assertion standing.
    AllInterleavingsHeld,
    /// Some execution did not complete cleanly — an assertion failed, a deadlock was found, or the exploration overran a declared bound — and this is loom's own report of it.
    ModelBroke {
        /// Loom's report, bounded and carried one way; a payload of a foreign type reads as absent.
        report: Option<ForeignText>,
    },
}

/// What one exploration produced: the bounds it ran under, and the verdict they established.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreemptionReading {
    bounds: PreemptionBounds,
    verdict: PreemptionVerdict,
}

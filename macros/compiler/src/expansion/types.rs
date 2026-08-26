//! The expansion home's declarations: the fact a binding refusal cites, how binding refuses, the sealed account one request produces, and the complete disposition witness a door seats beside it.
//!
//! Declarations only, with every road that reaches a private field in `type_guard.rs`, this file's own child.

use crate::closure::Closure;
use crate::explanation::View;
use crate::identity::{ClosedExpansionId, ClosureId, OwnerFact, PlanId, Provenance};
use crate::kind::{DispositionSet, Kind, KindSet};
use crate::plan::Plan;

#[path = "type_guard.rs"]
mod guard;

/// The fact this home declares, and the one a binding refusal cites as its repair.
pub const BINDING_FACT: OwnerFact = OwnerFact {
    home: "expansion",
    name: "nothing-is-handed-out-that-did-not-bind",
};

/// How binding one expansion refuses.
///
/// Three values produced separately can disagree about their parentage in exactly three places, and each of the three is a different repair.
/// Every arm names both identities it holds and elects neither.
#[must_use = "a binding refusal names the two identities an expansion was asked to bind as one"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BindError {
    /// The closure proves a rendering against a plan other than the one handed in beside it.
    ClosureProvedAgainstAnotherPlan {
        /// The plan handed to the binding.
        planned: PlanId,
        /// The plan the closure was actually proved against.
        proved: PlanId,
    },
    /// The explanation was answered over a plan other than the one handed in beside it.
    ///
    /// Reachable under one kind, which is why the type parameter cannot catch it: two plans of one kind admit the same questions, so an explanation written over the other one covers its roster exactly.
    ExplanationAnsweredOverAnotherPlan {
        /// The plan handed to the binding.
        planned: PlanId,
        /// The plan the explanation was actually answered over.
        answered: PlanId,
    },
    /// The explanation was answered over a proof other than the one handed in beside it.
    ///
    /// Reachable on its own, because one plan may be proved by two renderings and an explanation over the other proof states a digest of bytes this expansion never emitted.
    ExplanationAnsweredOverAnotherClosure {
        /// The proof handed to the binding.
        proved: ClosureId,
        /// The proof the explanation was actually answered over.
        answered: ClosureId,
    },
}

/// Everything one request produced, bound under one identity, with tokens reachable from here and from nowhere else.
///
/// One cannot be held without a plan, a closure proved over that plan, and an explanation answered over the two, all having been produced and having agreed.
/// [`Expansion::plan`] and [`Expansion::closure`] are the same values the deliveries are read from, so what it says it did and what it did cannot drift.
#[must_use = "an expansion is the whole account one request produced, and the only road to tokens"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expansion<K: Kind> {
    identity: ClosedExpansionId,
    provenance: Provenance,
    plan: Plan<K>,
    closure: Closure<K::Role>,
    explanation: View<K>,
}

/// One door's complete account: the expansion it produced, and what happened to every kind of the set it answers for.
///
/// The expansion carries what was produced and is silent about every kind that produced nothing; the sealed witness beside it carries one disposition per declared kind.
/// Which row says generated about this expansion remains the door's decision, but an incomplete or overfull record cannot become the witness stored here.
#[must_use = "an accounted expansion is what one door produced and what happened to every kind it did not"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accounted<K: Kind, Set: KindSet> {
    expansion: Expansion<K>,
    dispositions: DispositionSet<Set>,
}

//! The nine answers every kind owes, composed off the plan and the proof one request produced.
//!
//! Each answer is read from the value that holds it and never from a second copy: the kind names itself, the account names what caused it, the context names the profile, the closure names the digest, and the door names who required it.

use crate::bounded::Bounded;
use crate::closure::Closure;
use crate::diagnostic::Door;
use crate::explanation::{AnsweredOutput, ExplanationError, UniversalAnswer};
use crate::identity::OwnerFact;
use crate::kind::Kind;
use crate::plan::Plan;

/// Answer the universal roster over one plan and the closure proved against it.
///
/// The related-disposition answer is empty and says so: one request produces one kind and accounts for no other.
/// A door that produces several answers for them all at once, by seating a disposition record beside the whole expansion.
/// The invalidators are read off the plan itself, never handed in beside it — the value that holds a fact is the value the answer is read from.
///
/// # Errors
///
/// Returns the coverage refusal where the assumed facts, the account's declared dependencies, or the rendered outputs outgrow the seat they are answered in.
pub(super) fn universal<K: Kind>(
    door: &Door,
    plan: &Plan<K>,
    closure: &Closure<K::Role>,
    assumptions: &[OwnerFact],
) -> Result<Vec<UniversalAnswer>, ExplanationError> {
    let account = plan.account();
    let dependencies =
        Bounded::new(account.dependencies().to_vec()).map_err(ExplanationError::bounded)?;
    let assumed = Bounded::new(assumptions.to_vec()).map_err(ExplanationError::bounded)?;
    let outputs = Bounded::new(AnsweredOutput::roster(closure.rendered()))
        .map_err(ExplanationError::bounded)?;
    let producer = door.producer();
    Ok(vec![
        UniversalAnswer::Kind { name: K::NAME },
        UniversalAnswer::Owner {
            owner: OwnerFact {
                home: producer.namespace,
                name: producer.name,
            },
        },
        UniversalAnswer::CausingDeclarations {
            commitment: account.commitment(),
            dependencies,
        },
        UniversalAnswer::Profile {
            profile: plan.context().profile(),
        },
        UniversalAnswer::OutputAndDigest { outputs },
        UniversalAnswer::Assumptions {
            assumptions: assumed,
        },
        UniversalAnswer::Invalidators {
            triggers: plan.invalidation().clone(),
        },
        UniversalAnswer::RelatedDispositions {
            related: Bounded::empty(),
        },
        UniversalAnswer::Repairs {
            repairs: Bounded::empty(),
        },
    ])
}

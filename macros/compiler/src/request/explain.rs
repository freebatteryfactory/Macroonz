//! The nine answers every kind owes, composed off the plan and the proof one request produced.
//!
//! Each answer is read from the value that holds it and never from a second copy: the kind names itself, the account names what caused it, the context names the profile, the closure names the digest, and the door names who required it.

use crate::bounded::Bounded;
use crate::closure::Closure;
use crate::diagnostic::Door;
use crate::explanation::{ExplanationError, UniversalAnswer};
use crate::identity::OwnerFact;
use crate::kind::{Kind, Role};
use crate::plan::{InvalidationSet, Plan};
use crate::render::{RenderedProjection, RenderedUnit};

/// Answer the universal roster over one plan and the closure proved against it.
///
/// The related-disposition answer is empty and says so: one request produces one kind and accounts for no other.
/// A door that produces several answers for them all at once, by seating a disposition record beside the whole expansion.
///
/// # Errors
///
/// Returns the coverage refusal where the assumed facts or the account's declared dependencies outgrow the seat they are answered in.
pub(super) fn universal<K: Kind>(
    door: &Door,
    plan: &Plan<K>,
    closure: &Closure<K::Role>,
    invalidation: InvalidationSet,
    assumptions: &[OwnerFact],
) -> Result<Vec<UniversalAnswer>, ExplanationError> {
    let account = plan.account();
    let dependencies =
        Bounded::new(account.dependencies().to_vec()).map_err(ExplanationError::bounded)?;
    let assumed = Bounded::new(assumptions.to_vec()).map_err(ExplanationError::bounded)?;
    let unit = answered(closure.rendered());
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
        UniversalAnswer::OutputAndDigest {
            output: Box::new(unit.reconstructed().output),
            digest: unit.digest(),
        },
        UniversalAnswer::Assumptions {
            assumptions: assumed,
        },
        UniversalAnswer::Invalidators {
            triggers: invalidation,
        },
        UniversalAnswer::RelatedDispositions {
            related: Bounded::empty(),
        },
        UniversalAnswer::Repairs {
            repairs: Bounded::empty(),
        },
    ])
}

/// The unit one output-and-digest answer is about: the first seat of the roster that a unit stands under.
///
/// Roster order and never rendering order, so the answer does not turn on the sequence a renderer happened to write its units in.
/// The fallback is unreachable: a rendering is non-empty and every unit in it stands under a row of its own roster.
fn answered<R: Role>(rendered: &RenderedProjection<R>) -> &RenderedUnit<R> {
    R::ALL
        .iter()
        .copied()
        .find_map(|role| rendered.under(role))
        .unwrap_or_else(|| rendered.first())
}
